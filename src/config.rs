//! Application configuration

use std::fs::File;

use serde::{Deserialize, Serialize};
use xdg::BaseDirectories;

const APP_NAME: &str = "monotax";

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub(crate) taxer: TaxerImportConfig,
    pub(crate) tax: TaxConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaxConfig {
    pub(crate) tax_rate: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaxerImportConfig {
    pub(crate) id: String,
    pub(crate) account_name: String,
    pub(crate) default_comment: String,
}

pub fn load_config() -> anyhow::Result<Config> {
    let xdg_dirs = BaseDirectories::with_prefix(APP_NAME)?;
    let config_file_path = xdg_dirs.place_config_file("config.toml")?;
    // todo: replace with proper logging.
    if !config_file_path.exists() {
        println!(
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

impl Default for Config {
    fn default() -> Self {
        Self {
            taxer: TaxerImportConfig::default(),
            tax: TaxConfig::default(),
        }
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
