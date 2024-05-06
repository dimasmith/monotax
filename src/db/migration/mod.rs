//! Keeps various database migrations

use std::collections::HashSet;

use chrono::Local;
use log::info;
use rusqlite::{params, Connection};

pub mod base;
pub mod v0_2_0;

pub trait Migration {
    fn id(&self) -> String;

    fn apply(&self, conn: &mut Connection) -> anyhow::Result<()>;
}

fn initialize_migrations_table(conn: &mut Connection) -> anyhow::Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS migration (
            date DATETIME NOT NULL,
            name VARCHAR(100),            
            PRIMARY KEY (date, name)
        )",
        [],
    )?;
    Ok(())
}

pub fn apply_migrations(
    conn: &mut Connection,
    migrations: &[Box<dyn Migration>],
) -> anyhow::Result<()> {
    initialize_migrations_table(conn)?;

    let applied_ids = read_applied_ids(conn)?;

    for migration in migrations.iter() {
        let migration_id = migration.id();
        if applied_ids.contains(&migration_id) {
            info!(
                "skipped migration {}. it was already applied",
                &migration_id
            );
            continue;
        }
        migration.apply(conn)?;
        conn.execute(
            "insert into migration (date, name) values (?, ?)",
            params![Local::now().naive_local(), &migration_id],
        )?;
        info!("applied migration {}", &migration_id);
    }

    Ok(())
}

fn read_applied_ids(conn: &mut Connection) -> anyhow::Result<HashSet<String>> {
    let mut find_migrations_stmt = conn.prepare("SELECT name FROM migration ORDER BY date ASC")?;
    let applied_ids = find_migrations_stmt.query_map([], |row| {
        let migration_id: String = row.get(0)?;
        Ok(migration_id)
    })?;
    let ids = applied_ids.into_iter().map(|r| r.unwrap()).collect();
    Ok(ids)
}
