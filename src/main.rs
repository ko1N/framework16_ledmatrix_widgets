use std::{
    env::{args, args_os},
    process::exit,
    thread,
    time::Duration,
};

use clap::Parser;
use config::WidgetConfig;
use ledmatrix::LedMatrix;

use crate::widget::{BatteryWidget, ClockWidget, CpuWidget, MemoryWidget, NetworkWidget, Widget};

mod config;
mod ledmatrix;
mod matrix;
mod widget;

#[derive(Parser)]
#[command(version, about, long_about=None)]
struct Cli {
    // ======== Info about system ========
    #[arg(long)]
    /// List all connected matrix modules
    list_modules: bool,

    /// List all widgets available for placement
    #[arg(long)]
    list_widgets: bool, // ======== Program Control ========
                        // #[arg(long)]
                        // Start the background service updating the matrix
                        // start: bool,

                        // #[arg(long)]
                        // JSON config file path
                        // config: Option<String>,
}

enum Program {
    ListMod,
    ListWid,
    Default,
}

fn main() {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // TODO possible options:
    // each widget + Y placement + LED module (both as default) (for now, x maybe later)
    // Overall brightness
    // update rate

    let config = config::load().unwrap();

    let mut program = Program::Default;

    if args_os().len() > 1 {
        let cli = Cli::parse();
        if cli.list_modules {
            program = Program::ListMod;
        } else if cli.list_widgets {
            program = Program::ListWid;
        }
    }

    match program {
        Program::Default => {
            let mut mats = LedMatrix::detect();
            if mats.is_empty() {
                println!("No modules found, unable to continue.");
                exit(1);
            }

            // load all widgets
            let mut widgets: Vec<(WidgetConfig, Box<dyn Widget>)> = Vec::new();
            for widget in config.widgets.iter() {
                match &widget.setup {
                    config::WidgetSetup::Cpu(cfg) => {
                        widgets.push((widget.clone(), Box::new(CpuWidget::new(cfg.merge_threads))))
                    }
                    config::WidgetSetup::Memory(cfg) => {
                        widgets.push((widget.clone(), Box::new(MemoryWidget::new())));
                    }
                    config::WidgetSetup::Network(cfg) => {
                        widgets.push((widget.clone(), Box::new(NetworkWidget::new(&cfg.devices))));
                    }
                    config::WidgetSetup::Battery => {
                        widgets.push((widget.clone(), Box::new(BatteryWidget::new())));
                    }
                    config::WidgetSetup::Clock => {
                        widgets.push((widget.clone(), Box::new(ClockWidget::new())));
                    }
                }
            }

            // No arguments provided? Start the
            if args().len() <= 1 {
                loop {
                    for (idx, mat) in mats.iter_mut().enumerate() {
                        let mut dots = [[0; 9]; 34];
                        for (config, widget) in widgets.iter_mut().filter(|(c, _)| c.panel == idx) {
                            widget.update();
                            dots = matrix::emplace(dots, widget.as_mut(), config.x, config.y);
                        }
                        mat.draw_matrix(dots);
                    }

                    thread::sleep(Duration::from_millis(500));
                }
            }
        }
        Program::ListMod => {
            LedMatrix::detect();
        }
        Program::ListWid => {
            println!(
                "Battery Indicator:\n \
                A 9x4 widget in the shape of a battery, with an internal bar indicating remaining capacity.\n"
            );
            println!(
                "CPU Usage Indicator:\n \
                A 9x16 widget where each row of LEDs is a bar that represents the CPU usage of one core.\n"
            );
            println!(
                "Clock Widget:\n \
                A 9x11 widget that displays the system time in 24hr format.\n"
            );
        } // _ => {}
    }

    exit(0);
}
