use chrono::NaiveDate;

use crate::domain::{model::income_tax::IncomeTax, Income};

#[derive(Debug)]
pub struct BalanceReport {
    income_obligations: Vec<IncomeRow>,
}

#[derive(Debug)]
pub struct IncomeRow {
    amount: f64,
    date: NaiveDate,
    obligations: Vec<IncomeTaxObligation>,
}

#[derive(Debug)]
pub struct IncomeTaxObligation {
    name: String,
    obligation: f64,
}

impl BalanceReport {
    pub fn new(incomes: Vec<Income>, income_taxes: Vec<IncomeTax>) -> Self {
        let mut income_obligations = vec![];
        for income in incomes {
            let income_amount = income.amount();
            let income_date = income.date();
            let obligations = income_taxes
                .iter()
                .map(|tax| {
                    let obligation = tax.calculate_obligation(income_amount, income_date);
                    IncomeTaxObligation::new(tax.name().to_string(), obligation)
                })
                .collect::<Vec<_>>();
                
            income_obligations.push(IncomeRow::new(income_amount, income_date, obligations));
        }
        Self { income_obligations }
    }



    pub fn income_obligations(&self) -> &Vec<IncomeRow> {
        &self.income_obligations
    }
}

impl IncomeRow {
    pub fn new(amount: f64, date: NaiveDate, obligations: Vec<IncomeTaxObligation>) -> Self {
        Self {
            amount,
            date,
            obligations,
        }
    }

    pub fn amount(&self) -> f64 {
        self.amount
    }

    pub fn date(&self) -> NaiveDate {
        self.date
    }

    pub fn obligations(&self) -> &Vec<IncomeTaxObligation> {
        &self.obligations
    }

    pub fn total_obligations(&self) -> f64 {
        self.obligations.iter().map(|t| t.obligation).sum()
    }
}

impl IncomeTaxObligation {
    pub fn new(name: String, obligation: f64) -> Self {
        Self { name, obligation }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn obligation(&self) -> f64 {
        self.obligation
    }
}
