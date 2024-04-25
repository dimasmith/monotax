//! Initialize database schema.

use std::collections::HashSet;

use super::config::{connect, database_path};
use chrono::Local;
use log::info;
use rusqlite::{params, Connection};

pub fn create_schema(conn: &mut Connection) -> anyhow::Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS migration (
            date DATETIME NOT NULL,
            name VARCHAR(100),            
            PRIMARY KEY (date, name)
        )",
        [],
    )?;

    let applied_ids = read_applied_ids(conn)?;

    let migrations = [Migrations::CreateIncomeTable, Migrations::AddTaxPaidColumn];

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
    let mut find_migrations_stmt = conn.prepare("select name from migration order by date asc")?;
    let applied_ids = find_migrations_stmt.query_map([], |row| {
        let migration_id: String = row.get(0)?;
        Ok(migration_id)
    })?;
    let ids = applied_ids.into_iter().map(|r| r.unwrap()).collect();
    Ok(ids)
}

trait Migration {
    fn id(&self) -> String;

    fn apply(&self, conn: &mut Connection) -> anyhow::Result<()>;
}

enum Migrations {
    CreateIncomeTable,
    AddTaxPaidColumn,
}

impl Migration for Migrations {
    fn id(&self) -> String {
        match self {
            Migrations::CreateIncomeTable => "create_income_table".to_string(),
            Migrations::AddTaxPaidColumn => "add_tax_paid_column".to_string(),
        }
    }

    fn apply(&self, conn: &mut Connection) -> anyhow::Result<()> {
        match self {
            Migrations::CreateIncomeTable => {
                conn.execute(
                    "CREATE TABLE IF NOT EXISTS income (
                        date DATETIME NOT NULL,
                        amount DECIMAL(10,2) NOT NULL,
                        description TEXT,
                        year INTEGER NOT NULL,
                        quarter INTEGER NOT NULL,
                        PRIMARY KEY (date, amount)
                    )",
                    [],
                )?;
            }
            Migrations::AddTaxPaidColumn => {
                conn.execute(
                    "alter table income add column tax_paid bool default false",
                    [],
                )?;
            }
        }
        Ok(())
    }
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
