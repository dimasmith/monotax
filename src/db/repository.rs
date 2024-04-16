use chrono::{Datelike, NaiveDateTime};
use rusqlite::{params, Connection};

use crate::{income::Income, time::Quarter};

pub fn save_incomes(conn: &mut Connection, incomes: &[Income]) -> anyhow::Result<usize> {
    let income_records = incomes.iter().map(IncomeRecord::from);

    let mut updated = 0;
    let tx = conn.transaction()?;
    for income in income_records {
        updated += tx.execute(
            "INSERT OR IGNORE INTO incomes (date, amount, description, year, quarter) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                income.date.to_string(),
                income.amount,
                income.description,
                income.year,
                income.quarter
            ],
        )?;
    }
    tx.commit()?;

    Ok(updated)
}

pub fn load_all_incomes(conn: &mut Connection) -> anyhow::Result<Vec<Income>> {
    let mut stmt = conn.prepare(
        "SELECT date, amount, description, year, quarter FROM incomes order by date asc",
    )?;
    let incomes_records = stmt.query_map([], |row| {
        let date_str: String = row.get(0)?;
        let date = NaiveDateTime::parse_from_str(&date_str, "%Y-%m-%d %H:%M:%S")
            .expect("broken db date format");
        let amount: f64 = row.get(1)?;
        let description: String = row.get(2)?;
        Ok(IncomeRecord::new(date, amount, description))
    })?;

    let incomes: Vec<Income> = incomes_records
        .map(|r| r.unwrap().into_income())
        .collect::<Vec<_>>();

    Ok(incomes)
}

#[derive(Debug, Clone)]
struct IncomeRecord {
    date: NaiveDateTime,
    amount: f64,
    description: String,
    year: i32,
    quarter: u32,
}

impl IncomeRecord {
    fn new(date: NaiveDateTime, amount: f64, description: String) -> Self {
        let year = date.year();
        let quarter = Quarter::from(&date).index() as u32;
        Self {
            date,
            amount,
            description,
            year,
            quarter,
        }
    }

    fn into_income(self) -> Income {
        Income::new(self.date, self.amount)
    }
}

impl From<&Income> for IncomeRecord {
    fn from(income: &Income) -> Self {
        Self::new(
            income.datetime(),
            income.amount(),
            income.comment().unwrap_or_default().to_string(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn income(datetime: &str, amount: f64) -> Income {
        let datetime = NaiveDateTime::parse_from_str(datetime, "%Y-%m-%d %H:%M:%S").unwrap();
        Income::new(datetime, amount)
    }

    #[test]
    fn income_record_from_income() {
        let income = income("2024-04-13 14:00:00", 225.0);
        let record = IncomeRecord::from(&income);
        assert_eq!(record.amount, income.amount(), "amouts are note equal");
        assert_eq!(record.description, "", "unexpected description");
        assert_eq!(record.year, 2024, "incorrect year");
        assert_eq!(record.quarter, 2, "incorrect quarter");
        assert_eq!(record.date, income.datetime(), "incorrect date");
    }
}
