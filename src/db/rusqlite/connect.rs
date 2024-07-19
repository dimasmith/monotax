use r2d2_sqlite::SqliteConnectionManager;

use crate::db::config::database_path;

pub fn connect() -> anyhow::Result<rusqlite::Connection> {
    let db_path = database_path()?;
    let conn = rusqlite::Connection::open(db_path)?;
    Ok(conn)
}

pub fn connection_pool() -> anyhow::Result<r2d2::Pool<SqliteConnectionManager>> {
    let db_path = database_path()?;
    let manager = SqliteConnectionManager::file(db_path);
    let pool = r2d2::Pool::new(manager)?;
    Ok(pool)
}
