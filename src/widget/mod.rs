use std::cmp::Ordering;

pub mod cpu;
pub use cpu::CpuWidget;

pub mod memory;
pub use memory::MemoryWidget;

pub mod network;
pub use network::NetworkWidget;

pub mod battery;
pub use battery::BatteryWidget;

pub mod clock;
pub use clock::ClockWidget;

pub const ON_FULL: u8 = 60;
pub const ON_DIM: u8 = 30;
pub const OFF: u8 = 0;

/// Width/height dimensions for a widget matrix.
#[derive(Clone, Copy)]
pub struct Shape {
    pub x: usize,
    pub y: usize,
}

/// A standard set of instructions for widgets that can be updated from the system
pub trait Widget {
    fn update(&mut self);
    fn get_matrix(&self) -> &[u8];
    fn get_shape(&self) -> &Shape;
}

/// Helper function to draw an ascii character on the led display
pub fn write_char(mat: &mut [u8], position: usize, character: char) {
    debug_assert!(
        character.is_ascii_alphabetic(),
        "callers pass only latin header glyphs currently encoded as ASCII bits"
    );
    if !character.is_ascii_alphabetic() {
        return;
    }

    debug_assert!(
        position + 9 <= mat.len(),
        "widget allocates a full row before writing the 9-cell glyph"
    );
    if position + 9 > mat.len() {
        return;
    }

    mat[position] = ON_FULL;

    let c = character as u8;
    for bit in 0..8 {
        let is_on = (c >> (7 - bit)) & 1 == 1;
        if is_on {
            mat[position + 1 + bit] = 100;
        } else {
            mat[position + 1 + bit] = 20;
        }
    }
}

pub fn write_bar_1l(mat: &mut [u8], position: usize, width: usize, value: f32, max: f32) {
    debug_assert!(width > 0, "all widgets define non-zero row width");
    debug_assert!(max > 0.0, "resource capacities are expected to be positive");
    debug_assert!(
        position + width <= mat.len(),
        "caller writes bars only into the allocated widget matrix"
    );

    if width == 0 || max <= 0.0 || position + width > mat.len() {
        return;
    }

    let usage = (value / max).clamp(0.0, 1.0) * width as f32;
    let usage_int = usage.floor() as usize;
    let usage_fract = usage - usage_int as f32;
    for x in 0..width {
        match x.cmp(&usage_int) {
            Ordering::Less => mat[position + x] = ON_FULL,
            Ordering::Equal => mat[position + x] = (ON_FULL as f32 * usage_fract).max(10.0) as u8,
            _ => (),
        }
    }
}
