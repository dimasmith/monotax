//! Initialize database schema.

use super::config::database_path;
use super::migration::apply_migrations;
use super::migration::base::*;
use super::migration::v0_2_0::*;
use super::migration::Migration;
use log::info;
use rusqlite::Connection;
use sqlx::{migrate, SqlitePool};

pub fn create_schema(conn: &mut Connection) -> anyhow::Result<()> {
    let migrations: Vec<Box<dyn Migration>> = vec![
        Box::new(CreateIncomeTableMigration),
        Box::new(AddTaxPaidColumnMigration),
        Box::new(AddPaymentNoColumnMigration),
    ];
    apply_migrations(conn, &migrations)
}

pub fn initialize_db_file() -> anyhow::Result<()> {
    let db_path = database_path()?;
    if !db_path.exists() {
        let mut conn = Connection::open(&db_path)?;
        create_schema(&mut conn)?;
        info!("Database created at {}", &db_path.display());
    }

    Ok(())
}

pub async fn initialize_db() -> anyhow::Result<()> {
    initialize_db_file()?;
    let db_path = database_path()?;
    let url = format!("sqlite:{}", db_path.as_path().display());
    let conn = SqlitePool::connect(&url).await?;
    migrate!("./migrations").run(&conn).await?;
    Ok(())
}
