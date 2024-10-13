use std::time::Instant;

use sysinfo::Networks;

use crate::matrix::Matrix;

use super::{write_bar_1l, write_char, Shape, Widget, OFF, ON_FULL};

/// Create a widget that displays the ram and swap usage
pub struct NetworkWidget {
    networks: Networks,
    last_update_time: Instant,
    matrix: Vec<u8>,
    shape: Shape,
    config: NetworkWidgetConfig,
}

enum NetworkWidgetConfig {
    Device(String),
    DeviceList(Vec<String>),
}

impl NetworkWidget {
    pub fn with_device(device: &str) -> Self {
        Self {
            networks: Networks::new_with_refreshed_list(),
            last_update_time: Instant::now(),
            shape: Shape { x: 9, y: 3 },
            matrix: Vec::new(),
            config: NetworkWidgetConfig::Device(device.to_string()),
        }
    }

    pub fn with_devices(devices: &[String]) -> Self {
        Self {
            networks: Networks::new_with_refreshed_list(),
            last_update_time: Instant::now(),
            shape: Shape { x: 9, y: 3 },
            matrix: Vec::new(),
            config: NetworkWidgetConfig::DeviceList(devices.to_vec()),
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
            download += data.received();
            upload += data.transmitted();
        }

        let elapsed_secs = self.last_update_time.elapsed().as_secs_f32();
        self.last_update_time = Instant::now();

        // draw header
        write_char(&mut self.matrix, 0, 'N');

        // draw download
        write_bar_1l(
            &mut self.matrix,
            width,
            width,
            download as f32 / elapsed_secs,
            total_download as f32,
        );

        // draw upload
        write_bar_1l(
            &mut self.matrix,
            2 * width,
            width,
            upload as f32 / elapsed_secs,
            total_upload as f32,
        );
    }

    fn get_matrix(&self) -> &Vec<u8> {
        &self.matrix
    }

    fn get_shape(&self) -> &Shape {
        &self.shape
    }
}
