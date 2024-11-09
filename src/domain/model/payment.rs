use crate::domain::Income;

/// Tax payment for a single income.
#[derive(Debug, Clone)]
pub struct Payment {
    income: Income,
    tax_amount: f64,
    paid: bool,
}

impl Payment {
    /// Default constructor for the payment.
    fn new(income: Income, tax_amount: f64, paid: bool) -> Self {
        Self {
            income,
            tax_amount,
            paid,
        }
    }

    pub fn tax_rate(income: Income, tax_rate: f64, paid: bool) -> Self {
        let tax_amount = income.amount() * tax_rate;
        Self::new(income, tax_amount, paid)
    }
}

impl Payment {
    pub fn income(&self) -> &Income {
        &self.income
    }

    pub fn tax_amount(&self) -> f64 {
        self.tax_amount
    }

    pub fn paid(&self) -> bool {
        self.paid
    }

    pub fn payment_no(&self) -> i64 {
        self.income.income_no()
    }
}

impl Ord for Payment {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.income.cmp(&other.income)
    }
}

impl PartialOrd for Payment {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Payment {
    fn eq(&self, other: &Self) -> bool {
        self.income == other.income
    }
}

impl Eq for Payment {}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, Local, NaiveDate};

    #[test]
    fn calculate_payment_by_rate() {
        let income = Income::from_date(today(), 1000.0);
        let tax_rate = 0.08; // 8%

        let payment = Payment::tax_rate(income, tax_rate, false);

        assert_eq!(
            payment.tax_amount(),
            80.0,
            "incorrect tax amount calculation"
        );
    }

    #[test]
    fn payments_are_comparable_by_income_date() {
        let new_payment = Payment::tax_rate(Income::from_date(today(), 200.0), 0.1, false);
        let older_payment = Payment::tax_rate(
            Income::from_date(today() + Duration::days(1), 400.0),
            0.05,
            true,
        );

        assert!(older_payment > new_payment, "incorrect order of payments");
    }

    fn today() -> NaiveDate {
        Local::now().date_naive()
    }
}
