use super::{write_bar_1l, Shape, Widget, OFF, ON_FULL};

/// Create a widget that displays the usage of all CPU cores, one per row.
pub struct CpuWidget {
    cpu_usages: Vec<u8>,
    merge_threads: bool,
    sys: sysinfo::System,
    matrix: Vec<u8>,
    shape: Shape,
}

impl CpuWidget {
    pub fn new(merge_threads: bool) -> Self {
        let mut sys = sysinfo::System::new();
        sys.refresh_cpu_all();
        let num_cpus = sys.cpus().len();

        Self {
            shape: match merge_threads {
                false => Shape { x: 9, y: num_cpus },
                true => Shape { x: 9, y: 8 },
            },
            cpu_usages: vec![0; num_cpus],
            merge_threads,
            sys,
            matrix: Vec::new(),
        }
    }

    fn draw_merged_threads(&mut self, width: usize, height: usize) {
        for idy in 0..height {
            let inverse_y = height - (idy + 1);
            for (idx, chunk) in self.cpu_usages.chunks(2).enumerate() {
                if idx >= width {
                    break;
                }
                let sum: u16 = chunk.iter().map(|&usage| u16::from(usage)).sum();
                let usage = sum as f32 / chunk.len() as f32;

                if usage >= (inverse_y * 10) as f32 {
                    self.matrix[(idy * width) + idx] = ON_FULL;
                }
            }
        }
    }

    fn draw_per_core_bars(&mut self, width: usize, height: usize) {
        for row in 0..height {
            let usage = self.cpu_usages.get(row).copied().unwrap_or(0) as f32;
            write_bar_1l(&mut self.matrix, row * width, width, usage, 100.0);
        }
    }
}

impl Widget for CpuWidget {
    fn update(&mut self) {
        // refresh the cpu usage
        self.sys.refresh_cpu_all();

        for (idx, usage) in self.sys.cpus().iter().enumerate() {
            self.cpu_usages[idx] = usage.cpu_usage().round() as u8;
        }

        // recreate matrix
        let width = self.get_shape().x;
        let height = self.get_shape().y;
        self.matrix = vec![OFF; width * height];

        if self.merge_threads {
            self.draw_merged_threads(width, height);
        } else {
            self.draw_per_core_bars(width, height);
        }
    }

    fn get_matrix(&self) -> &[u8] {
        &self.matrix
    }

    fn get_shape(&self) -> &Shape {
        &self.shape
    }
}
