use std::path::Path;

use serde::{Deserialize, Serialize};

/// Top-level application configuration loaded from TOML.
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub general: GeneralConfig,
    pub widgets: Vec<WidgetConfig>,
}

/// Global display options shared by all widgets.
#[derive(Debug, Serialize, Deserialize)]
pub struct GeneralConfig {
    pub brightness: i32,
}

/// Per-widget placement and setup configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WidgetConfig {
    pub panel: usize,
    pub x: usize,
    pub y: usize,
    pub setup: WidgetSetup,
}

/// Supported widget variants and their setup payloads.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WidgetSetup {
    Cpu(WidgetCpuSetup),
    Memory(WidgetMemorySetup),
    Network(WidgetNetworkSetup),
    Battery,
    Clock,
}

/// CPU widget-specific settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WidgetCpuSetup {
    pub merge_threads: bool,
}

/// Memory widget-specific settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WidgetMemorySetup {
    pub swap: bool,
}

/// Network widget-specific settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WidgetNetworkSetup {
    pub devices: Vec<String>,
}

/// Load and deserialize a configuration file from a path.
pub fn load(path: impl AsRef<Path>) -> Result<Config, String> {
    log::info!("loading configuration");
    let config_str =
        std::fs::read_to_string(path).map_err(|err| format!("Unable to load config: {}", err))?;
    let config = toml::from_str::<Config>(&config_str)
        .map_err(|err| format!("Unable to load config: {}", err))?;
    Ok(config)
}
