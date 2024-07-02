//! Initialize database schema.

use sqlx::{migrate, SqlitePool};

use super::config::database_path;

pub async fn initialize_db() -> anyhow::Result<()> {
    let db_path = database_path()?;
    let url = format!("sqlite:{}", db_path.as_path().display());
    let conn = SqlitePool::connect(&url).await?;
    migrate!("./migrations").run(&conn).await?;
    Ok(())
}
