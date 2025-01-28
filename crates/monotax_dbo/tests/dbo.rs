// Test import incomes from the UniversalBank csv format.

use std::fs::File;

use chrono::NaiveDateTime;
use monotax_core::domain::filter::income::{IncomeCriteria, IncomeCriterion, QuarterFilter};
use monotax_core::domain::model::income::Amount;
use monotax_core::domain::Income;
use monotax_core::domain::Quarter;
use monotax_dbo::dbo;

fn income(date: &str, amount: f64) -> Income {
    let income_date = NaiveDateTime::parse_from_str(date, "%d.%m.%Y %H:%M:%S").unwrap();
    Income::new(income_date, Amount::new(amount).unwrap())
}

#[test]
fn import_all_from_csv() {
    let balance_file = File::open("tests/test_files/balance.csv").unwrap();
    let allow_all_filter = IncomeCriteria::new(&[]);
    let incomes = dbo::read_incomes(balance_file, allow_all_filter).unwrap();

    assert_eq!(4, incomes.len());
    assert_eq!(
        incomes,
        vec![
            income("18.01.2024 12:36:00", 3302.00),
            income("05.02.2024 15:18:00", 265654.00),
            income("05.03.2024 14:20:00", 269359.00),
            income("05.04.2024 14:11:00", 275674.00),
        ]
    );
}

#[test]
fn import_one_quarter_from_csv() {
    let balance_file = File::open("tests/test_files/balance.csv").unwrap();
    let quarter_filter =
        IncomeCriteria::new(&[IncomeCriterion::Quarter(QuarterFilter::Only(Quarter::Q1))]);
    let incomes = dbo::read_incomes(balance_file, quarter_filter).unwrap();

    assert_eq!(3, incomes.len());
    assert_eq!(
        incomes,
        vec![
            income("18.01.2024 12:36:00", 3302.00),
            income("05.02.2024 15:18:00", 265654.00),
            income("05.03.2024 14:20:00", 269359.00),
        ]
    );
}
