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
    pub fn new(merge_threads: bool) -> CpuWidget {
        let mut sys = sysinfo::System::new();
        sys.refresh_cpu_all();
        let num_cpus = sys.cpus().len();

        CpuWidget {
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
            for idy in 0..height {
                let inverse_y = height - (idy + 1);
                for (idx, chunk) in self.cpu_usages.chunks(2).enumerate() {
                    let usage = (chunk[0] + chunk[1]) / 2;
                    if usage as usize >= inverse_y * 10 {
                        self.matrix[(idy * width) + idx] = ON_FULL;
                    }
                }
            }
        } else {
            for y in 0..16 {
                write_bar_1l(
                    &mut self.matrix,
                    y * width,
                    width,
                    self.cpu_usages[y] as f32,
                    100.0,
                );
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
