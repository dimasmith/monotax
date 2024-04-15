//! Application configuration

use std::fs::File;

use log::warn;
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
    // todo: replace with proper logging.
    if !config_file_path.exists() {
        warn!(
            "the configuration file {} is missing. using the default configuration",
            config_file_path.display()
        );
        // creating example config file if the default is not present
        let example_config = xdg_dirs.place_config_file("config.toml.example")?;
        let mut example_config = File::create(example_config)?;
        let default_config = Config::default();
        let toml = toml::to_string(&default_config)?;
        std::io::Write::write_all(&mut example_config, toml.as_bytes())?;
        return Ok(Config::default());
    }

    let config: Config = toml::from_str(&std::fs::read_to_string(&config_file_path)?)?;
    Ok(config)
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
