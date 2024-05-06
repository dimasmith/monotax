use rusqlite::Connection;

use super::Migration;

pub struct CreateIncomeTableMigration;

impl Migration for CreateIncomeTableMigration {
    fn id(&self) -> String {
        "create_income_table".to_string()
    }

    fn apply(&self, conn: &mut Connection) -> anyhow::Result<()> {
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
        Ok(())
    }
}

pub struct AddTaxPaidColumnMigration;

impl Migration for AddTaxPaidColumnMigration {
    fn id(&self) -> String {
        "add_tax_paid_column".to_string()
    }

    fn apply(&self, conn: &mut Connection) -> anyhow::Result<()> {
        conn.execute(
            "ALTER TABLE income ADD COLUMN tax_paid BOOL DEFAULT false",
            [],
        )?;
        Ok(())
    }
}
