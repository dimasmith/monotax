//! Initialize database schema.

use sqlx::{migrate, SqlitePool};

pub async fn initialize_db(pool: &SqlitePool) -> anyhow::Result<()> {
    migrate!("./migrations").run(pool).await?;
    Ok(())
}
