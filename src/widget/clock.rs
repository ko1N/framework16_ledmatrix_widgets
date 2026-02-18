use chrono::{Local, Timelike};

use super::{Shape, Widget, OFF, ON_DIM, ON_FULL};

const DIGIT_0: &[u8] = [
    OFF, ON_FULL, OFF, ON_FULL, OFF, ON_FULL, ON_FULL, OFF, ON_FULL, ON_FULL, OFF, ON_FULL, OFF,
    ON_FULL, OFF,
]
.as_slice();

const DIGIT_1: &[u8] = [
    OFF, OFF, ON_FULL, OFF, ON_DIM, ON_FULL, OFF, OFF, ON_FULL, OFF, OFF, ON_FULL, OFF, OFF,
    ON_FULL,
]
.as_slice();

const DIGIT_2: &[u8] = [
    ON_FULL, ON_FULL, ON_FULL, OFF, OFF, ON_FULL, ON_FULL, ON_FULL, ON_FULL, ON_FULL, OFF, OFF,
    ON_FULL, ON_FULL, ON_FULL,
]
.as_slice();

const DIGIT_3: &[u8] = [
    ON_FULL, ON_FULL, ON_FULL, OFF, OFF, ON_FULL, ON_FULL, ON_FULL, OFF, OFF, OFF, ON_FULL,
    ON_FULL, ON_FULL, ON_FULL,
]
.as_slice();

const DIGIT_4: &[u8] = [
    ON_FULL, OFF, ON_FULL, ON_FULL, OFF, ON_FULL, ON_FULL, ON_FULL, ON_FULL, OFF, OFF, ON_FULL,
    OFF, OFF, ON_FULL,
]
.as_slice();

const DIGIT_5: &[u8] = [
    ON_FULL, ON_FULL, ON_FULL, ON_FULL, OFF, OFF, ON_FULL, ON_FULL, ON_FULL, OFF, OFF, ON_FULL,
    ON_FULL, ON_FULL, ON_FULL,
]
.as_slice();

const DIGIT_6: &[u8] = [
    OFF, ON_FULL, ON_DIM, ON_FULL, OFF, OFF, ON_FULL, ON_FULL, ON_FULL, ON_FULL, OFF, ON_FULL,
    ON_FULL, ON_FULL, ON_FULL,
]
.as_slice();

const DIGIT_7: &[u8] = [
    ON_FULL, ON_FULL, ON_FULL, ON_DIM, OFF, ON_FULL, OFF, OFF, ON_FULL, OFF, ON_FULL, OFF, OFF,
    ON_FULL, OFF,
]
.as_slice();

const DIGIT_8: &[u8] = [
    ON_FULL, ON_FULL, ON_FULL, ON_FULL, OFF, ON_FULL, ON_FULL, ON_FULL, ON_FULL, ON_FULL, OFF,
    ON_FULL, ON_FULL, ON_FULL, ON_FULL,
]
.as_slice();

const DIGIT_9: &[u8] = [
    ON_FULL, ON_FULL, ON_FULL, ON_FULL, OFF, ON_FULL, ON_FULL, ON_FULL, ON_FULL, OFF, OFF, ON_FULL,
    ON_DIM, ON_FULL, OFF,
]
.as_slice();

pub struct ClockWidget {
    matrix: Vec<u8>,
    shape: Shape,
}

impl ClockWidget {
    /// Construct a digital clock widget in HH:MM 24-hour format.
    pub fn new() -> Self {
        Self {
            shape: Shape { x: 9, y: 11 },
            matrix: Vec::new(),
        }
    }

    fn render_digit(num: u32) -> &'static [u8] {
        match num {
            0 => DIGIT_0,
            1 => DIGIT_1,
            2 => DIGIT_2,
            3 => DIGIT_3,
            4 => DIGIT_4,
            5 => DIGIT_5,
            6 => DIGIT_6,
            7 => DIGIT_7,
            8 => DIGIT_8,
            9 => DIGIT_9,
            _ => {
                debug_assert!(false, "render_number only passes base-10 digits");
                DIGIT_0
            }
        }
    }

    fn render_number(num: u32) -> Vec<u8> {
        let mut numrow = vec![0; 9 * 5];
        let first_digit = Self::render_digit(num / 10);
        let second_digit = Self::render_digit(num % 10);
        for idx in 0..(9 * 5) {
            let cell = match idx % 9 {
                1..=3 => first_digit[((idx / 9) * 3) + (idx % 9) - 1],
                5..=7 => second_digit[((idx / 9) * 3) + idx % 9 - 5],
                _ => OFF,
            };
            numrow[idx] = cell;
        }
        numrow
    }
}

impl Widget for ClockWidget {
    fn update(&mut self) {
        let time = Local::now();
        self.matrix = Vec::with_capacity(9 * 11);
        self.matrix.extend(Self::render_number(time.hour()));
        self.matrix.extend([OFF; 9]);
        self.matrix.extend(Self::render_number(time.minute()));
    }

    fn get_matrix(&self) -> &[u8] {
        &self.matrix
    }

    fn get_shape(&self) -> &Shape {
        &self.shape
    }
}
