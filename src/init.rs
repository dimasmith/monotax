//! Initialize the application configuration

use crate::config::create_default_config;

pub fn init(force: bool) -> anyhow::Result<()> {
    create_default_config(force)?;
    #[cfg(feature = "sqlite")]
    crate::db::init::initialize_db()?;
    Ok(())
}
