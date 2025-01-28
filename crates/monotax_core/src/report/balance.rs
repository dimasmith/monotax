use chrono::NaiveDate;

use crate::domain::{
    model::{income::Amount, income_tax::IncomeTax},
    Income,
};

#[derive(Debug)]
pub struct BalanceReport {
    income_obligations: Vec<IncomeRow>,
}

#[derive(Debug)]
pub struct IncomeRow {
    amount: Amount,
    date: NaiveDate,
    obligations: Vec<IncomeTaxObligation>,
}

#[derive(Debug)]
pub struct IncomeTaxObligation {
    name: String,
    obligation: Amount,
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
    pub fn new(amount: Amount, date: NaiveDate, obligations: Vec<IncomeTaxObligation>) -> Self {
        Self {
            amount,
            date,
            obligations,
        }
    }

    pub fn amount(&self) -> Amount {
        self.amount
    }

    pub fn date(&self) -> NaiveDate {
        self.date
    }

    pub fn obligations(&self) -> &Vec<IncomeTaxObligation> {
        &self.obligations
    }

    pub fn total_obligations(&self) -> Amount {
        self.obligations.iter().map(|t| t.obligation).sum()
    }
}

impl IncomeTaxObligation {
    pub fn new(name: String, obligation: Amount) -> Self {
        Self { name, obligation }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn obligation(&self) -> Amount {
        self.obligation
    }
}
