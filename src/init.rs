//! Initialize the application configuration

use crate::config::create_default_config;

pub fn init(force: bool) -> anyhow::Result<()> {
    create_default_config(force)
}
