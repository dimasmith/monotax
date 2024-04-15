//! Initialize the application configuration

use std::fs::File;

use log::{info, warn};
use xdg::BaseDirectories;

use crate::config::Config;

const APP_NAME: &str = "monotax";

pub fn init(force: bool) -> anyhow::Result<()> {
    let xdg_dirs = BaseDirectories::with_prefix(APP_NAME)?;
    let config_file_path = xdg_dirs.place_config_file("config.toml")?;
    if config_file_path.exists() && !force {
        warn!(
            "Not writing the configuration. The configuration file {} already exists",
            config_file_path.display()
        );
        info!("Use -f flag to override the configuration");
        return Ok(());
    }

    let mut config_file = File::create(config_file_path)?;
    let default_config = Config::default();
    let toml = toml::to_string_pretty(&default_config)?;
    std::io::Write::write_all(&mut config_file, toml.as_bytes())?;

    Ok(())
}
