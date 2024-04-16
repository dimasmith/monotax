//! Application configuration

use std::fs::File;

use log::{info, warn};
use serde::{Deserialize, Serialize};
use xdg::BaseDirectories;

const APP_NAME: &str = "monotax";

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    taxer: TaxerImportConfig,
    tax: TaxConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaxConfig {
    tax_rate: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaxerImportConfig {
    id: String,
    account_name: String,
    default_comment: String,
}

pub fn load_config() -> anyhow::Result<Config> {
    let xdg_dirs = BaseDirectories::with_prefix(APP_NAME)?;
    let config_file_path = xdg_dirs.place_config_file("config.toml")?;
    if !config_file_path.exists() {
        warn!(
            "The configuration file {} is missing. Using the default configuration",
            config_file_path.display()
        );
        warn!("It's highly unlikely that the default configuration fits your needs");
        info!("Create the configuration file with init command");
        return Ok(Config::default());
    }

    let config: Config = toml::from_str(&std::fs::read_to_string(&config_file_path)?)?;
    Ok(config)
}

pub fn create_default_config(force: bool) -> anyhow::Result<()> {
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

pub fn base_directories() -> anyhow::Result<BaseDirectories> {
    let dirs = BaseDirectories::with_prefix(APP_NAME)?;
    Ok(dirs)
}

impl Config {
    pub fn taxer(&self) -> &TaxerImportConfig {
        &self.taxer
    }

    pub fn tax(&self) -> &TaxConfig {
        &self.tax
    }
}

impl TaxConfig {
    pub fn new(tax_rate: f64) -> Self {
        Self { tax_rate }
    }

    pub fn tax_rate(&self) -> f64 {
        self.tax_rate
    }
}

impl TaxerImportConfig {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn account_name(&self) -> &str {
        &self.account_name
    }

    pub fn default_comment(&self) -> &str {
        &self.default_comment
    }
}

impl Default for TaxConfig {
    fn default() -> Self {
        Self { tax_rate: 0.05 }
    }
}

impl Default for TaxerImportConfig {
    fn default() -> Self {
        Self {
            id: "1234567890".to_string(),
            account_name: Default::default(),
            default_comment: Default::default(),
        }
    }
}
