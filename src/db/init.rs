//! Initialize database schema.

use super::config::{connect, database_path};
use log::info;
use rusqlite::Connection;

pub fn create_schema(conn: &mut Connection) -> anyhow::Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS income (
            date DATEtIME NOT NULL,
            amount DECIMAL(10,2) NOT NULL,
            description TEXT,
            year INTEGER NOT NULL,
            quarter INTEGER NOT NULL,
            PRIMARY KEY (date, amount)
        )",
        [],
    )?;
    Ok(())
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

pub fn initialize_db() -> anyhow::Result<()> {
    initialize_db_file()?;
    let mut conn = connect()?;
    create_schema(&mut conn)?;
    Ok(())
}
