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

#[derive(Clone)]
pub struct Shape {
    pub x: usize,
    pub y: usize,
}

/// A standard set of instructions for widgets that can be updated from the system
pub trait Widget {
    fn update(&mut self);
    fn get_matrix(&self) -> &Vec<u8>;
    fn get_shape(&self) -> &Shape;
}
