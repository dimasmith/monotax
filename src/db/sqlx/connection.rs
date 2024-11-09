use sqlx::SqlitePool;

use crate::config::DatabaseConfiguration;

pub async fn connection_pool(config: &DatabaseConfiguration) -> anyhow::Result<SqlitePool> {
    SqlitePool::connect(&config.connection_url())
        .await
        .map_err(anyhow::Error::from)
}
