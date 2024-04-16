//! Initialize database schema.

use log::info;
use rusqlite::Connection;

use crate::config::base_directories;

pub fn create_schema(conn: &mut Connection) -> anyhow::Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS incomes (
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
    let base_dirs = base_directories()?;
    let db_path = base_dirs.place_data_file("monotax.db")?;
    if !db_path.exists() {
        let mut conn = Connection::open(&db_path)?;
        create_schema(&mut conn)?;
        info!("Database created at {}", &db_path.display());
    }

    Ok(())
}
