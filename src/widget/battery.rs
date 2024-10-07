use super::{Shape, Widget, OFF, ON_DIM};

/// Create a widget that displays the battery remaining in the laptop
pub struct BatteryWidget {
    matrix: Vec<u8>,
    shape: Shape,
    chrg_ind: bool,
}

impl BatteryWidget {
    pub fn new() -> BatteryWidget {
        BatteryWidget {
            matrix: vec![],
            chrg_ind: false,
            shape: Shape { x: 9, y: 2 },
        }
    }
}

impl Widget for BatteryWidget {
    fn update(&mut self) {
        // Update the battery percentage
        let battery_dev = battery::Manager::new()
            .unwrap()
            .batteries()
            .unwrap()
            .enumerate()
            .next()
            .unwrap()
            .1
            .unwrap();

        // Update whether or not the device is charging
        let bat_level_pct = battery_dev
            .state_of_charge()
            .get::<battery::units::ratio::percent>();
        let is_charging = battery_dev.state() == battery::State::Charging;

        // recreate matrix
        let width = self.get_shape().x;
        let height = self.get_shape().y;
        self.matrix = vec![OFF; width * height];

        let num_illum = (bat_level_pct * ((width * 2) - 1) as f32 / 100.0).round();

        let row_1 = (num_illum / 2.0 + 0.5) as usize;
        let row_2 = (num_illum / 2.0) as usize;

        // draw battery bar
        for i in 0..width {
            if i <= row_1 {
                self.matrix[i] = ON_DIM;
            }
            if i <= row_2 {
                self.matrix[self.shape.x + i] = ON_DIM;
            }
        }

        // draw charging indicator
        if is_charging && bat_level_pct < 99.0 {
            if row_1 > row_2 {
                self.matrix[row_1] = if self.chrg_ind { ON_DIM } else { OFF };
            } else {
                self.matrix[self.shape.x + row_2] = if self.chrg_ind { ON_DIM } else { OFF };
            }

            self.chrg_ind = !self.chrg_ind;
        }
    }

    fn get_matrix(&self) -> &Vec<u8> {
        &self.matrix
    }

    fn get_shape(&self) -> &Shape {
        &self.shape
    }
}
