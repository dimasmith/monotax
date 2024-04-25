use std::path::PathBuf;

use crate::config::base_directories;

pub(super) fn database_path() -> anyhow::Result<PathBuf> {
    let base_dirs = base_directories()?;
    base_dirs.place_data_file("monotax.db")
}

pub(super) fn connect() -> anyhow::Result<rusqlite::Connection> {
    let db_path = database_path()?;
    let conn = rusqlite::Connection::open(db_path)?;
    Ok(conn)
}
