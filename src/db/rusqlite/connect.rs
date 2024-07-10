use crate::db::config::database_path;

pub fn connect() -> anyhow::Result<rusqlite::Connection> {
    let db_path = database_path()?;
    let conn = rusqlite::Connection::open(db_path)?;
    Ok(conn)
}
