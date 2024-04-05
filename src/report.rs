//! Generate quarterly income reports with tax information.

use chrono::{Datelike, NaiveDate};

use crate::{date_filter::Quarter, income::Income};

pub mod console;

#[derive(Debug, Clone, PartialEq)]
pub struct QuarterReport {
    year: i32,
    quarter: Quarter,
    total_income: f64,
    cumulative_income: f64,
    total_tax: f64,
    cumulative_tax: f64,
}

pub fn generate_report(incomes: &mut [Income], tax: f64) -> Vec<QuarterReport> {
    if incomes.is_empty() {
        return vec![];
    }
    let mut reports = vec![];
    incomes.sort_by(|lhs, rhs| lhs.date().cmp(&rhs.date()));

    let mut prev_report = QuarterReport::income(
        incomes.first().expect("at least one income is present"),
        tax,
    );
    for income in &incomes[1..] {
        let date = income.date();
        if prev_report.is_for_date(&date) {
            prev_report.add_income(income, tax);
        } else {
            let mut new_report = QuarterReport::income(income, tax);
            new_report.add_cumulative_values(&prev_report);
            reports.push(prev_report);
            prev_report = new_report;
        }
    }
    reports.push(prev_report);
    reports
}

impl QuarterReport {
    fn income(income: &Income, tax: f64) -> Self {
        let date = income.date();
        let year = date.year();
        let quarter = Quarter::of_date(&date);
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
        let quarter = Quarter::of_date(&date);
        return self.year == year && self.quarter == quarter;
    }

    fn add_income(&mut self, income: &Income, tax: f64) {
        assert!(
            self.is_for_date(&income.date()),
            "income is for incorrect report"
        );
        let amount = income.amount();
        let tax_amount = amount * tax;
        self.total_income += amount;
        self.cumulative_income += amount;
        self.total_tax += tax_amount;
        self.cumulative_tax += tax_amount;
    }

    fn add_cumulative_values(&mut self, prev: &QuarterReport) {
        if self.year != prev.year {
            return;
        }
        self.cumulative_income += prev.cumulative_income;
        self.cumulative_tax += prev.cumulative_tax;
    }
}

impl QuarterReport {
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
    use chrono::NaiveDate;

    use super::*;

    #[test]
    fn single_income_report() {
        let q1_income = Income::new(NaiveDate::from_ymd_opt(2024, 2, 5).unwrap(), 275000.0);

        let report = generate_report(&mut [q1_income], 0.05);

        assert_eq!(
            &report,
            &[QuarterReport {
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
        let tax = 0.1;
        let feb_income = Income::new(NaiveDate::from_ymd_opt(2024, 2, 5).unwrap(), 1000.0);
        let mar_income = Income::new(NaiveDate::from_ymd_opt(2024, 3, 12).unwrap(), 1500.0);

        let report = generate_report(&mut [feb_income, mar_income], tax);

        assert_eq!(
            &report,
            &[QuarterReport {
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
        let tax = 0.1;
        let feb_income = Income::new(NaiveDate::from_ymd_opt(2024, 2, 5).unwrap(), 1000.0);
        let mar_income = Income::new(NaiveDate::from_ymd_opt(2024, 3, 12).unwrap(), 1500.0);
        let apr_income = Income::new(NaiveDate::from_ymd_opt(2024, 4, 1).unwrap(), 2000.0);

        let report = generate_report(&mut [feb_income, mar_income, apr_income], tax);

        assert_eq!(
            &report,
            &[
                QuarterReport {
                    year: 2024,
                    quarter: Quarter::Q1,
                    total_income: 2500.0,
                    cumulative_income: 2500.0,
                    total_tax: 250.0,
                    cumulative_tax: 250.0,
                },
                QuarterReport {
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
            let tax = 0.1;
            let dec_income = Income::new(NaiveDate::from_ymd_opt(2023, 12, 5).unwrap(), 1000.0);
            let jan_income = Income::new(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(), 1500.0);
            
    
            let report = generate_report(&mut [dec_income, jan_income], tax);
    
            assert_eq!(
                &report,
                &[
                    QuarterReport {
                        year: 2023,
                        quarter: Quarter::Q4,
                        total_income: 1000.0,
                        cumulative_income: 1000.0,
                        total_tax: 100.0,
                        cumulative_tax: 100.0,
                    },
                    QuarterReport {
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
