use std::{
    fs::{self, File},
    path::{Path, PathBuf},
};

use directories::ProjectDirs;
use log::{info, warn};

use super::Config;

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

/// Load application configuration from the default configuration files.
///
/// The configuration reads the `XDG_CONFIG/monotax/config.toml` file if it is found.
/// You can override the settings using the environment variables.
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
