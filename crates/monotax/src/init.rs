//! Initialize the application configuration

use sqlx::SqlitePool;

use crate::config::dir::create_default_config;
use monotax_sqlite::init::initialize_db;

pub async fn init(db_pool: &SqlitePool, force: bool) -> anyhow::Result<()> {
    create_default_config(force)?;
    initialize_db(db_pool).await?;
    Ok(())
}
