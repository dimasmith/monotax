//! Connect to the database specified in configuration.
use sqlx::SqlitePool;

use crate::configuration::DatabaseConfiguration;

pub async fn connection_pool(config: &DatabaseConfiguration) -> anyhow::Result<SqlitePool> {
    SqlitePool::connect(&config.connection_url())
        .await
        .map_err(anyhow::Error::from)
}
