//! Generate quarterly income reports with tax information.

use chrono::{Datelike, NaiveDate};

use crate::config::TaxConfig;
use crate::domain::Income;
use crate::time::Quarter;

pub mod console;
pub mod csv;

#[derive(Debug, Clone, PartialEq)]
pub struct QuarterlyReport {
    quarters: Vec<QuarterReportLine>,
    total_income: f64,
    total_tax: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct QuarterReportLine {
    year: i32,
    quarter: Quarter,
    total_income: f64,
    cumulative_income: f64,
    total_tax: f64,
    cumulative_tax: f64,
}

impl QuarterlyReport {
    pub fn build_report<I>(incomes: I, tax_props: &TaxConfig) -> Self
    where
        I: IntoIterator<Item = Income>,
    {
        let quarters = generate_report(incomes, tax_props);
        // todo: optimize this calculations
        let total_income = quarters.iter().map(|q| q.total_income()).sum();
        let total_tax = quarters.iter().map(|q| q.total_tax()).sum();
        Self {
            quarters,
            total_income,
            total_tax,
        }
    }
}

impl QuarterlyReport {
    pub fn quarters(&self) -> &[QuarterReportLine] {
        &self.quarters
    }

    pub fn total_income(&self) -> f64 {
        self.total_income
    }

    pub fn total_tax(&self) -> f64 {
        self.total_tax
    }
}

fn generate_report<I>(incomes: I, tax_props: &TaxConfig) -> Vec<QuarterReportLine>
where
    I: IntoIterator<Item = Income>,
{
    let mut incomes = incomes.into_iter().collect::<Vec<_>>();
    incomes.sort();
    let mut incomes = incomes.iter();

    let Some(income) = incomes.next() else {
        return vec![];
    };

    let tax_rate = tax_props.tax_rate();
    let mut prev_report = QuarterReportLine::income(income, tax_rate);

    let mut reports = vec![];
    for income in incomes {
        let date = income.date();
        if prev_report.is_for_date(&date) {
            prev_report.add_income(income, tax_rate);
        } else {
            let mut new_report = QuarterReportLine::income(income, tax_rate);
            new_report.add_cumulative_values(&prev_report);
            reports.push(prev_report);
            prev_report = new_report;
        }
    }
    reports.push(prev_report);
    reports
}

impl QuarterReportLine {
    fn income(income: &Income, tax: f64) -> Self {
        let date = income.date();
        let year = date.year();
        let quarter = Quarter::of(income);
        let amount = income.amount();
        let tax = amount * tax;
        Self {
            year,
            quarter,
            total_income: amount,
            cumulative_income: amount,
            total_tax: tax,
            cumulative_tax: tax,
        }
    }

    fn is_for_date(&self, date: &NaiveDate) -> bool {
        let year = date.year();
        let quarter = Quarter::from(date);
        self.year == year && self.quarter == quarter
    }

    fn add_income(&mut self, income: &Income, tax_rate: f64) {
        assert!(
            self.is_for_date(&income.date()),
            "income is for incorrect report"
        );
        let amount = income.amount();
        self.add_amount(amount);
        self.add_tax(amount, tax_rate)
    }

    fn add_cumulative_values(&mut self, prev: &QuarterReportLine) {
        if self.year != prev.year {
            return;
        }
        self.cumulative_income += prev.cumulative_income;
        self.cumulative_tax += prev.cumulative_tax;
    }

    fn add_amount(&mut self, amount: f64) {
        self.total_income += amount;
        self.cumulative_income += amount;
    }

    fn add_tax(&mut self, amount: f64, tax_rate: f64) {
        let tax_amount = amount * tax_rate;
        self.total_tax += tax_amount;
        self.cumulative_tax += tax_amount;
    }
}

impl QuarterReportLine {
    pub fn year(&self) -> i32 {
        self.year
    }

    pub fn quarter(&self) -> &Quarter {
        &self.quarter
    }

    pub fn total_income(&self) -> f64 {
        self.total_income
    }

    pub fn cumulative_income(&self) -> f64 {
        self.cumulative_income
    }

    pub fn total_tax(&self) -> f64 {
        self.total_tax
    }

    pub fn cumulative_tax(&self) -> f64 {
        self.cumulative_tax
    }
}

#[cfg(test)]
mod tests {
    use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDateTime {
        NaiveDate::from_ymd_opt(y, m, d)
            .unwrap()
            .and_time(NaiveTime::from_hms_opt(12, 0, 0).unwrap())
    }

    #[test]
    fn single_income_report() {
        let q1_income = Income::new(date(2024, 2, 5), 275000.0);
        let tax_config = TaxConfig::new(0.05);

        let report = generate_report(vec![q1_income], &tax_config);

        assert_eq!(
            &report,
            &[QuarterReportLine {
                year: 2024,
                quarter: Quarter::Q1,
                total_income: 275000.0,
                cumulative_income: 275000.0,
                total_tax: 13750.0,
                cumulative_tax: 13750.0,
            }]
        )
    }

    #[test]
    fn single_quarter_report() {
        let feb_income = Income::new(date(2024, 2, 5), 1000.0);
        let mar_income = Income::new(date(2024, 3, 12), 1500.0);
        let tax_config = TaxConfig::new(0.1);
        let report = generate_report(&mut [feb_income, mar_income].into_iter(), &tax_config);

        assert_eq!(
            &report,
            &[QuarterReportLine {
                year: 2024,
                quarter: Quarter::Q1,
                total_income: 2500.0,
                cumulative_income: 2500.0,
                total_tax: 250.0,
                cumulative_tax: 250.0,
            }]
        )
    }

    #[test]
    fn two_quarters_report() {
        let feb_income = Income::new(date(2024, 2, 5), 1000.0);
        let mar_income = Income::new(date(2024, 3, 12), 1500.0);
        let apr_income = Income::new(date(2024, 4, 1), 2000.0);
        let tax_config = TaxConfig::new(0.1);
        let report = generate_report(
            &mut [feb_income, mar_income, apr_income].into_iter(),
            &tax_config,
        );

        assert_eq!(
            &report,
            &[
                QuarterReportLine {
                    year: 2024,
                    quarter: Quarter::Q1,
                    total_income: 2500.0,
                    cumulative_income: 2500.0,
                    total_tax: 250.0,
                    cumulative_tax: 250.0,
                },
                QuarterReportLine {
                    year: 2024,
                    quarter: Quarter::Q2,
                    total_income: 2000.0,
                    cumulative_income: 4500.0,
                    total_tax: 200.0,
                    cumulative_tax: 450.0,
                }
            ]
        )
    }

    #[test]
    fn cross_year_report() {
        let dec_income = Income::new(date(2023, 12, 5), 1000.0);
        let jan_income = Income::new(date(2024, 1, 1), 1500.0);
        let tax_config = TaxConfig::new(0.1);
        let report = generate_report(&mut [dec_income, jan_income].into_iter(), &tax_config);

        assert_eq!(
            &report,
            &[
                QuarterReportLine {
                    year: 2023,
                    quarter: Quarter::Q4,
                    total_income: 1000.0,
                    cumulative_income: 1000.0,
                    total_tax: 100.0,
                    cumulative_tax: 100.0,
                },
                QuarterReportLine {
                    year: 2024,
                    quarter: Quarter::Q1,
                    total_income: 1500.0,
                    cumulative_income: 1500.0,
                    total_tax: 150.0,
                    cumulative_tax: 150.0,
                }
            ]
        )
    }
}
