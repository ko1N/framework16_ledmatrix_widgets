use super::{write_bar_1l, write_char, Shape, Widget, OFF};

/// Create a widget that displays the ram and swap usage
pub struct MemoryWidget {
    sys: sysinfo::System,
    matrix: Vec<u8>,
    shape: Shape,
}

impl MemoryWidget {
    pub fn new() -> Self {
        Self {
            shape: Shape { x: 9, y: 3 },
            sys: sysinfo::System::new(),
            matrix: Vec::new(),
        }
    }
}

impl Widget for MemoryWidget {
    fn update(&mut self) {
        self.sys.refresh_memory();

        // recreate matrix
        let width = self.get_shape().x;
        let height = self.get_shape().y;
        self.matrix = vec![OFF; width * height];

        // draw header
        write_char(&mut self.matrix, 0, 'R');

        // draw ram usage
        write_bar_1l(
            &mut self.matrix,
            width,
            width,
            self.sys.used_memory() as f32,
            self.sys.total_memory() as f32,
        );

        // draw swap usage
        write_bar_1l(
            &mut self.matrix,
            2 * width,
            width,
            self.sys.used_swap() as f32,
            self.sys.total_swap() as f32,
        );
    }

    fn get_matrix(&self) -> &[u8] {
        &self.matrix
    }

    fn get_shape(&self) -> &Shape {
        &self.shape
    }
}
