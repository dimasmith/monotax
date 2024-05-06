use chrono::NaiveDateTime;
use rusqlite::{named_params, Transaction};

use super::Migration;

pub struct AddPaymentNoColumnMigration;

impl Migration for AddPaymentNoColumnMigration {
    fn id(&self) -> String {
        "add_payment_no_column".to_string()
    }

    fn apply(&self, conn: &mut rusqlite::Connection) -> anyhow::Result<()> {
        let tx = conn.transaction()?;
        self.add_temporary_table(&tx)?;
        self.copy_records(&tx)?;
        self.rename_table(&tx)?;
        tx.commit()?;
        Ok(())
    }
}

impl AddPaymentNoColumnMigration {
    fn add_temporary_table(&self, tx: &Transaction) -> anyhow::Result<()> {
        tx.execute(
            "CREATE TABLE IF NOT EXISTS income_tmp (
            date DATETIME NOT NULL,
            amount DECIMAL(10,2) NOT NULL,
            payment_no INTEGER NOT NULL UNIQUE,
            description TEXT,
            year INTEGER NOT NULL,
            quarter INTEGER NOT NULL,
            tax_paid BOOL DEFAULT false,
            PRIMARY KEY (date, amount)
        )",
            [],
        )?;
        Ok(())
    }

    fn copy_records(&self, tx: &Transaction) -> anyhow::Result<()> {
        let mut insert_stmt = tx.prepare(
            "INSERT INTO income_tmp 
        (date, amount, payment_no, description, year, quarter, tax_paid) 
        VALUES 
        (:date, :amount, :payment_no, :description, :year, :quarter, :tax_paid)",
        )?;
        let mut read_stmt = tx.prepare("SELECT * FROM income")?;
        let mut payment_no = 0;
        let _ = read_stmt.query_map([], |r| {
            let date: NaiveDateTime = r.get("date")?;
            let amount: f64 = r.get("amount")?;
            let description: String = r.get("description")?;
            let year: i32 = r.get("year")?;
            let quarter: u32 = r.get("quarter")?;
            let tax_paid: bool = r.get("tax_paid")?;

            payment_no += 1;
            insert_stmt.execute(named_params! {
                ":date": date,
                ":amount": amount,
                ":payment_no": payment_no,
                ":description": description,
                ":year": year,
                ":quarter": quarter,
                ":tax_paid": tax_paid
            })?;
            Ok(())
        });
        Ok(())
    }

    fn rename_table(&self, tx: &Transaction) -> anyhow::Result<()> {
        tx.execute("DROP TABLE income", [])?;
        tx.execute("ALTER TABLE income_tmp RENAME TO income", [])?;
        Ok(())
    }
}
