use super::{Shape, Widget, OFF, ON_FULL};

/// Create a widget that displays the ram and swap usage
pub struct MemoryWidget {
    total_memory: u64,
    used_memory: u64,
    total_swap: u64,
    used_swap: u64,
    sys: sysinfo::System,
    matrix: Vec<u8>,
    shape: Shape,
}

impl MemoryWidget {
    pub fn new() -> Self {
        Self {
            shape: Shape { x: 9, y: 3 },
            total_memory: 0,
            used_memory: 0,
            total_swap: 0,
            used_swap: 0,
            sys: sysinfo::System::new(),
            matrix: Vec::new(),
        }
    }
}

impl Widget for MemoryWidget {
    fn update(&mut self) {
        self.sys.refresh_memory();

        self.total_memory = self.sys.total_memory();
        self.used_memory = self.sys.used_memory();
        self.total_swap = self.sys.total_swap();
        self.used_swap = self.sys.used_swap();

        // recreate matrix
        let width = self.get_shape().x;
        let height = self.get_shape().y;
        self.matrix = vec![OFF; width * height];

        // draw header
        // binary R 01010010
        self.matrix[0] = ON_FULL;
        self.matrix[1] = 20;
        self.matrix[2] = 100;
        self.matrix[3] = 20;
        self.matrix[4] = 100;
        self.matrix[5] = 20;
        self.matrix[6] = 20;
        self.matrix[7] = 100;
        self.matrix[8] = 20;

        let mut line = width;

        // draw ram usage
        let ram_usage = self.used_memory as f32 / self.total_memory as f32 * width as f32;
        let ram_usage_int = ram_usage as usize;
        let ram_usage_fract = ram_usage - ram_usage_int as f32;
        for x in 0..width {
            if x < ram_usage_int {
                self.matrix[line + x] = ON_FULL;
            } else if x == ram_usage_int {
                self.matrix[line + x] = (ON_FULL as f32 * ram_usage_fract).max(10.0) as u8;
            }
        }

        line = 2 * width;

        // draw swap usage
        let swap_usage = self.used_swap as f32 / self.total_swap as f32 * width as f32;
        let swap_usage_int = swap_usage as usize;
        let swap_usage_fract = swap_usage - swap_usage_int as f32;
        for x in 0..width {
            if x < swap_usage_int {
                self.matrix[line + x] = ON_FULL;
            } else if x == swap_usage_int {
                self.matrix[line + x] = (ON_FULL as f32 * swap_usage_fract).max(10.0) as u8;
            }
        }
    }

    fn get_matrix(&self) -> &Vec<u8> {
        &self.matrix
    }

    fn get_shape(&self) -> &Shape {
        &self.shape
    }
}
