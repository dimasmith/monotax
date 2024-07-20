use sqlx::SqlitePool;

use crate::db::config::database_path;

pub async fn default_connection_pool() -> anyhow::Result<SqlitePool> {
    let db_path = database_path()?;
    let url = format!("sqlite:{}", db_path.as_path().display());
    let pool = SqlitePool::connect(&url).await?;
    Ok(pool)
}
