//! Initialize database schema.

use super::migration::base::*;
use super::migration::v0_2_0::*;
use super::{
    config::{connect, database_path},
    migration::{apply_migrations, Migration},
};
use log::info;
use rusqlite::Connection;

pub fn create_schema(conn: &mut Connection) -> anyhow::Result<()> {
    let migrations: Vec<Box<dyn Migration>> = vec![
        Box::new(CreateIncomeTableMigration),
        Box::new(AddTaxPaidColumnMigration),
        Box::new(AddPaymentNoColumnMigration),
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
