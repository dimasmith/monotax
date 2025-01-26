use std::fs::{self, File};
use std::path::{Path, PathBuf};

use config::{Config, Environment, FileFormat};
use directories::ProjectDirs;
use dotenvy::{dotenv, var};
use log::{info, warn};

use super::Configuration;

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

    fn config_file<P: AsRef<Path>>(&self, path: P) -> anyhow::Result<PathBuf> {
        let mut file_path = self.data_dir.clone();
        file_path.push(path);
        Ok(file_path)
    }
}

/// Load application configuration from the default configuration files.
///
/// The configuration loading depends on presence of `.env` file in the application directory.
/// Application starts in `dev` mode when the file is present.
/// The `dev` mode:
/// - does not include configuration from XDG directories.
/// - does not force database override with `DATABASE_URL` environment variable.
pub fn load_config() -> anyhow::Result<Configuration> {
    // TODO: add more ways to enable dev mode.
    let has_env_file = Path::new(".env").exists();
    let is_dev_mode = has_env_file;
    if is_dev_mode {
        load_dev_config()
    } else {
        load_prod_config()
    }
}

fn load_dev_config() -> anyhow::Result<Configuration> {
    let _ = dotenv(); // make sure to read .env file
    let default_config_str = include_str!("../../monotax.toml");
    let builder = Config::builder()
        .add_source(config::File::from_str(default_config_str, FileFormat::Toml))
        .add_source(
            Environment::with_prefix("MONOTAX")
                .convert_case(config::Case::Snake)
                .separator("_")
                .ignore_empty(true),
        )
        // use the value from the .env file. it is the same one as set for the sqlx cli.
        .set_override_option("database.url", var("DATABASE_URL").ok())?
        .build()?;
    let configuration = builder.try_deserialize::<Configuration>()?;

    Ok(configuration)
}

fn load_prod_config() -> anyhow::Result<Configuration> {
    let default_config_str = include_str!("../../monotax.toml");
    let xdg_dirs = base_directories()?;
    let xdg_config = xdg_dirs.config_file("config.toml")?;
    let builder = Config::builder()
        .add_source(config::File::from_str(default_config_str, FileFormat::Toml))
        .add_source(config::File::from(xdg_config).required(true))
        .add_source(
            Environment::with_prefix("MONOTAX")
                .convert_case(config::Case::Snake)
                .separator("_")
                .ignore_empty(true),
        )
        .build()?;
    let configuration = builder.try_deserialize::<Configuration>()?;

    Ok(configuration)
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
    let default_config = Configuration::default();
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
