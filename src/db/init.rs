//! Initialize database schema.

use rusqlite::Connection;

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
