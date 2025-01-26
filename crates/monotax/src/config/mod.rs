//! Application configuration.
//!
//! The configuration is stored in the XDG directories. The configuration file is a TOML file.
//! By default, the configuration file is located in the `~/.config/monotax/config.toml`.
//!
//! It is expected that users will edit the configuration file manually.
//! It's rarely needed to change the configuration after the initial setup.
//!
//! Running the `monotax init` command will create the default configuration file.

pub mod configuration;
pub mod dir;

pub use configuration::*;
pub use dir::load_config;
