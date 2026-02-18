use std::{process::exit, thread, time::Duration};

use clap::Parser;
use config::{Config, WidgetConfig};
use ledmatrix::LedMatrix;
use matrix::{MATRIX_HEIGHT, MATRIX_WIDTH};

use crate::widget::{BatteryWidget, ClockWidget, CpuWidget, MemoryWidget, NetworkWidget, Widget};

mod config;
mod ledmatrix;
mod matrix;
mod widget;

#[derive(Parser)]
#[command(version, about, long_about=None)]
struct Cli {
    #[arg(long)]
    list_modules: bool,

    #[arg(long)]
    list_widgets: bool,

    #[arg(long)]
    config: Option<String>,
}

enum Program {
    ListMod,
    ListWid,
    Default,
}

fn validate_widget_placements(
    widgets: &[(WidgetConfig, Box<dyn Widget>)],
    panel_count: usize,
) -> Result<(), String> {
    for (cfg, widget) in widgets {
        if cfg.panel >= panel_count {
            return Err(format!(
                "widget targets panel {} but only {} panel(s) were detected",
                cfg.panel, panel_count
            ));
        }

        let shape = widget.get_shape();
        let x_end = cfg
            .x
            .checked_add(shape.x)
            .ok_or_else(|| "widget x position overflowed usize".to_string())?;
        let y_end = cfg
            .y
            .checked_add(shape.y)
            .ok_or_else(|| "widget y position overflowed usize".to_string())?;

        if x_end > MATRIX_WIDTH || y_end > MATRIX_HEIGHT {
            return Err(format!(
                "widget at panel {} with origin ({}, {}) and shape {}x{} exceeds panel bounds {}x{}",
                cfg.panel, cfg.x, cfg.y, shape.x, shape.y, MATRIX_WIDTH, MATRIX_HEIGHT
            ));
        }
    }

    Ok(())
}

fn parse_program(cli: &Cli) -> Program {
    if cli.list_modules {
        Program::ListMod
    } else if cli.list_widgets {
        Program::ListWid
    } else {
        Program::Default
    }
}

fn main() {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // TODO possible options:
    // each widget + Y placement + LED module (both as default) (for now, x maybe later)
    // Overall brightness
    // update rate

    let cli = Cli::parse();
    let program = parse_program(&cli);

    let config_path = cli.config.unwrap_or_else(|| "./config.toml".to_string());
    let config = match config::load(&config_path) {
        Ok(config) => config,
        Err(err) => {
            log::error!("failed to load config at {}: {}", config_path, err);
            exit(1);
        }
    };

    match program {
        Program::Default => loop {
            if let Err(err) = run(&config) {
                log::warn!("widget runner exited early: {err}");
            }
            thread::sleep(Duration::from_millis(1000));
        },
        Program::ListMod => {
            if let Err(err) = LedMatrix::detect() {
                log::error!("unable to detect led matrix modules: {err}");
                exit(1);
            }
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

fn run(config: &Config) -> Result<(), String> {
    let mut mats = LedMatrix::detect()?;
    if mats.is_empty() {
        log::warn!("no led modules found");
        return Ok(());
    }

    // load all widgets
    let mut widgets: Vec<(WidgetConfig, Box<dyn Widget>)> = Vec::new();
    for widget in config.widgets.iter() {
        match &widget.setup {
            config::WidgetSetup::Cpu(cfg) => {
                widgets.push((widget.clone(), Box::new(CpuWidget::new(cfg.merge_threads))));
            }
            config::WidgetSetup::Memory(_) => {
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

    validate_widget_placements(&widgets, mats.len())?;

    loop {
        for (idx, mat) in mats.iter_mut().enumerate() {
            let mut dots = [[0; MATRIX_WIDTH]; MATRIX_HEIGHT];
            for (config, widget) in widgets.iter_mut().filter(|(c, _)| c.panel == idx) {
                widget.update();
                dots = matrix::emplace(dots, widget.as_ref(), config.x, config.y);
            }
            mat.draw_matrix(dots)?;
        }

        thread::sleep(Duration::from_millis(500));
    }
}
