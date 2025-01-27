//! Taxes that are dependent on incomes.

use chrono::NaiveDate;
use uuid::Uuid;

pub type TaxID = Uuid;

#[derive(Debug)]
pub struct IncomeTax {
    id: TaxID,
    name: String,
    rates: Vec<IncomeTaxRate>,
}

#[derive(Debug)]
pub struct IncomeTaxRate {
    start: NaiveDate,
    end: NaiveDate,
    rate: f64,
}

impl IncomeTax {
    pub fn new(id: TaxID, name: String, rates: Vec<IncomeTaxRate>) -> Self {
        Self { id, name, rates }
    }

    pub fn new_unchecked(id: String, name: String, rates: Vec<IncomeTaxRate>) -> Self {
        let id = Uuid::parse_str(&id).unwrap();
        Self { id, name, rates }
    }

    pub fn calculate_obligation(&self, income_amount: f64, income_date: NaiveDate) -> f64 {
        self.rates
            .iter()
            .rfind(|rate| rate.is_applicable(income_date))
            .map(|rate| rate.calculate_obligation(income_amount, income_date))
            .unwrap_or(0.0)
    }

    pub fn add_rate_unchecked(&mut self, start_date: NaiveDate, end_date: NaiveDate, rate: f64) {
        let period_rate = IncomeTaxRate::new(start_date, end_date, rate);
        self.rates.push(period_rate);
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn id(&self) -> &TaxID {
        &self.id
    }
}

impl IncomeTaxRate {
    pub fn new(start: NaiveDate, end: NaiveDate, rate: f64) -> Self {
        Self { start, end, rate }
    }

    fn is_applicable(&self, date: NaiveDate) -> bool {
        date >= self.start && date < self.end
    }

    fn calculate_obligation(&self, income_amount: f64, income_date: NaiveDate) -> f64 {
        let rate = if self.is_applicable(income_date) {
            self.rate
        } else {
            0.0
        };
        income_amount * rate
    }
}
