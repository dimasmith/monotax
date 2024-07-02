use crate::config::base_directories;
use anyhow::Context;
use std::env;
use std::path::PathBuf;

pub(super) fn database_path() -> anyhow::Result<PathBuf> {
    if let Ok(dev_db_url) = dotenvy::var("DATABASE_URL") {
        let db_file_name = dev_db_url
            .strip_prefix("sqlite:")
            .with_context(|| format!("incorrect DATABASE_URL variable value {}", dev_db_url))?;
        let curr_dir = env::current_dir()?;
        return Ok(curr_dir.join(db_file_name));
    }
    let base_dirs = base_directories()?;
    base_dirs.place_data_file("monotax.db")
}

pub(super) fn connect() -> anyhow::Result<rusqlite::Connection> {
    let db_path = database_path()?;
    let conn = rusqlite::Connection::open(db_path)?;
    Ok(conn)
}
