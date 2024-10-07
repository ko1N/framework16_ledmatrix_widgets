use std::time::Instant;

use sysinfo::Networks;

use super::{Shape, Widget, OFF, ON_FULL};

/// Create a widget that displays the ram and swap usage
pub struct NetworkWidget {
    networks: Networks,
    last_update_time: Instant,
    matrix: Vec<u8>,
    shape: Shape,
}

impl NetworkWidget {
    pub fn new() -> Self {
        Self {
            networks: Networks::new_with_refreshed_list(),
            last_update_time: Instant::now(),
            shape: Shape { x: 9, y: 3 },
            matrix: Vec::new(),
        }
    }
}

impl Widget for NetworkWidget {
    fn update(&mut self) {
        self.networks.refresh();

        // recreate matrix
        let width = self.get_shape().x;
        let height = self.get_shape().y;
        self.matrix = vec![OFF; width * height];

        // accumulate network traffic
        // TODO: filter lo, virbr & docker networks
        let mut download = 0;
        let total_download = 500u64 * 1024 * 1024 / 8; // 500 mbit/s
        let mut upload = 0;
        let total_upload = 100u64 * 1024 * 1024 / 8; // 100 mbit/s
        for (_interface_name, data) in self
            .networks
            .iter()
            .filter(|(k, _)| *k != "lo" && !k.contains("virbr"))
        {
            //println!(
            //    "{interface_name}: {} B (down) / {} B (up)",
            //    data.received(),
            //    data.transmitted(),
            //);
            download += data.received();
            upload += data.transmitted();
        }

        let elapsed_secs = self.last_update_time.elapsed().as_secs_f32();
        self.last_update_time = Instant::now();

        // draw header
        // binary N 01001110
        self.matrix[0] = ON_FULL;
        self.matrix[1] = 20;
        self.matrix[2] = 100;
        self.matrix[3] = 20;
        self.matrix[4] = 20;
        self.matrix[5] = 100;
        self.matrix[6] = 100;
        self.matrix[7] = 100;
        self.matrix[8] = 20;

        // draw accumulated network usage
        let download_usage =
            (download as f32 / elapsed_secs / total_download as f32 * width as f32) as usize;
        let mut line = width;
        for x in 0..width {
            if x <= download_usage {
                self.matrix[line + x] = ON_FULL;
            }
        }

        let upload_usage =
            (upload as f32 / elapsed_secs / total_upload as f32 * width as f32) as usize;
        line = 2 * width;
        for x in 0..width {
            if x <= upload_usage {
                self.matrix[line + x] = ON_FULL;
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
