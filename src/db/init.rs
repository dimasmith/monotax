//! Initialize database schema.

use super::{
    config::{connect, database_path},
    migration::{
        apply_migrations,
        base::{AddTaxPaidColumnMigration, CreateIncomeTableMigration},
        Migration,
    },
};
use log::info;
use rusqlite::Connection;

pub fn create_schema(conn: &mut Connection) -> anyhow::Result<()> {
    let migrations: Vec<Box<dyn Migration>> = vec![
        Box::new(CreateIncomeTableMigration),
        Box::new(AddTaxPaidColumnMigration),
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

pub fn initialize_db() -> anyhow::Result<()> {
    initialize_db_file()?;
    let mut conn = connect()?;
    create_schema(&mut conn)?;
    Ok(())
}
