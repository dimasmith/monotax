use chrono::NaiveDateTime;

use crate::domain::income::Income;

pub struct IncomeRecord {
    pub date: NaiveDateTime,
    pub amount: f64,
    pub payment_no: i64,
    pub description: Option<String>,
    pub year: u16,
    pub quarter: u8,
    pub tax_paid: bool,
}

impl From<IncomeRecord> for Income {
    fn from(record: IncomeRecord) -> Self {
        Income::new(record.date, record.amount).with_no(record.payment_no)
    }
}
