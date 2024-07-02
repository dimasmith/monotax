//! Application configuration.
//!
//! The configuration is stored in the XDG directories. The configuration file is a TOML file.
//! By default, the configuration file is located in the `~/.config/monotax/config.toml`.
//!
//! It is expected that users will edit the configuration file manually.
//! It's rarely needed to change the configuration after the initial setup.
//!
//! Running the `monotax init` command will create the default configuration file.

use directories::ProjectDirs;
use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};

use log::{info, warn};
use serde::{Deserialize, Serialize};

const APP_NAME: &str = "monotax";

pub struct ConfigurationDirs {
    config_dir: PathBuf,
    data_dir: PathBuf,
}

impl ConfigurationDirs {
    fn new(config_dir: PathBuf, data_dir: PathBuf) -> Self {
        Self {
            config_dir,
            data_dir,
        }
    }
}

impl ConfigurationDirs {
    pub fn place_config_file<P: AsRef<Path>>(&self, path: P) -> anyhow::Result<PathBuf> {
        fs::create_dir_all(&self.config_dir)?;
        let mut file_path = self.config_dir.clone();
        file_path.push(path);
        Ok(file_path)
    }

    pub fn place_data_file<P: AsRef<Path>>(&self, path: P) -> anyhow::Result<PathBuf> {
        fs::create_dir_all(&self.data_dir)?;
        let mut file_path = self.data_dir.clone();
        file_path.push(path);
        Ok(file_path)
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    taxer: TaxerImportConfig,
    tax: TaxConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaxConfig {
    tax_rate: f64,
}

/// Configuration for the Taxer import.
///
/// - The `id` is person's national tax identifier.
/// - The `account_name` is the name of the account in the Taxer.
/// - The `default_comment` is a comment that will be used if the income has no comment.
#[derive(Debug, Serialize, Deserialize)]
pub struct TaxerImportConfig {
    id: String,
    account_name: String,
    default_comment: String,
}

pub fn load_config() -> anyhow::Result<Config> {
    let xdg_dirs = base_directories()?;
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
    let xdg_dirs = base_directories()?;
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

pub fn base_directories() -> anyhow::Result<ConfigurationDirs> {
    let Some(project_dirs) = ProjectDirs::from("", "", APP_NAME) else {
        anyhow::bail!("cannot retrieve configuration directories")
    };
    let dirs = ConfigurationDirs::new(
        project_dirs.config_dir().to_path_buf(),
        project_dirs.data_dir().to_path_buf(),
    );
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
    /// Provide the national tax identifier.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Provide the name of the account in the Taxer.
    pub fn account_name(&self) -> &str {
        &self.account_name
    }

    /// Provide the default comment that will be used if the income has no comment.
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
