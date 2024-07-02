//! Initialize the application configuration

use crate::config::create_default_config;
use crate::db::init::initialize_db;

pub async fn init(force: bool) -> anyhow::Result<()> {
    create_default_config(force)?;
    initialize_db().await?;
    Ok(())
}
