use chrono::{Datelike, NaiveDateTime};
use rusqlite::{named_params, Connection, Row, ToSql};

use crate::{income::Income, time::Quarter};

use super::criteria::Criteria;

pub fn save_incomes(conn: &mut Connection, incomes: &[Income]) -> anyhow::Result<usize> {
    let income_records = incomes.iter().map(IncomeRecord::from);

    let mut updated = 0;
    let tx = conn.transaction()?;
    for income in income_records {
        updated += tx.execute(
            "INSERT OR IGNORE INTO income (date, amount, description, year, quarter, tax_paid) 
            VALUES (:date, :amount, :description, :year, :quarter, :tax_paid)",
            named_params![
                ":date": income.date.to_string(),
                ":amount": income.amount,
                ":description": income.description,
                ":year": income.year,
                ":quarter": income.quarter,
                ":tax_paid": income.tax_paid,
            ],
        )?;
    }
    tx.commit()?;

    Ok(updated)
}

pub fn load_all_incomes(conn: &mut Connection) -> anyhow::Result<Vec<Income>> {
    find_incomes(conn, &Criteria::And(vec![]))
}

pub fn find_incomes(conn: &mut Connection, criteria: &Criteria) -> anyhow::Result<Vec<Income>> {
    let records = find_records_by(conn, criteria)?;
    let incomes: Vec<Income> = records
        .into_iter()
        .map(|r| r.into_income())
        .collect::<Vec<_>>();

    Ok(incomes)
}

pub(super) fn find_records_by(
    conn: &mut Connection,
    criteria: &Criteria,
) -> anyhow::Result<Vec<IncomeRecord>> {
    let where_clause = if criteria.where_clause().is_empty() {
        String::default()
    } else {
        format!("WHERE {}", criteria.where_clause())
    };
    let query =format!(
            "SELECT date, amount, description, tax_paid, year, quarter FROM income {} ORDER BY date ASC",
            where_clause
        );

    let params = criteria.params();
    let params: Vec<(&str, &dyn ToSql)> = params
        .iter()
        .map(|(name, value)| (*name, value as &dyn ToSql))
        .collect();

    let mut stmt = conn.prepare(&query)?;
    let incomes_records = stmt.query_map(params.as_slice(), |row| map_income_records(row))?;

    let incomes = incomes_records.map(|r| r.unwrap()).collect::<Vec<_>>();

    Ok(incomes)
}

fn map_income_records(row: &Row) -> Result<IncomeRecord, rusqlite::Error> {
    let date = row.get("date")?;
    let amount: f64 = row.get("amount")?;
    let description: String = row.get("description")?;
    let tax_paid = row.get("tax_paid")?;
    Ok(IncomeRecord::new(date, amount, description, tax_paid))
}

#[derive(Debug, Clone)]
pub(super) struct IncomeRecord {
    date: NaiveDateTime,
    amount: f64,
    description: String,
    year: i32,
    quarter: u32,
    tax_paid: bool,
}

impl IncomeRecord {
    fn new(date: NaiveDateTime, amount: f64, description: String, tax_paid: bool) -> Self {
        let year = date.year();
        let quarter = Quarter::from(&date).index() as u32;
        Self {
            date,
            amount,
            description,
            year,
            quarter,
            tax_paid,
        }
    }

    pub(super) fn tax_paid(&self) -> bool {
        self.tax_paid
    }

    pub(super) fn income(&self) -> Income {
        Income::new(self.date, self.amount)
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
            false,
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
