//! Initialize the application configuration

use crate::config::create_default_config;
use crate::db::init::initialize_db;

pub fn init(force: bool) -> anyhow::Result<()> {
    create_default_config(force)?;
    initialize_db()?;
    Ok(())
}
