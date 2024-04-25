use super::Payment;

pub mod plaintext;

/// Consolidated report on payments.
/// Report has a list of payments ordered chronologically.
#[derive(Debug, Clone)]
pub struct PaymentReport {
    payments: Vec<Payment>,
}

impl PaymentReport {
    /// Creates a report from the collection of payments.
    /// Payments in report are guaranteed to be ordered by the income date.
    pub fn from_payments<I>(payments: I) -> Self
    where
        I: IntoIterator<Item = Payment>,
    {
        let mut p: Vec<_> = payments.into_iter().collect();
        p.sort();
        Self { payments: p }
    }
}

impl PaymentReport {
    /// List of all individual report payments
    /// sorted by the income date.
    pub fn payments(&self) -> &[Payment] {
        &self.payments
    }

    /// Returns a total of unpaid taxes.
    pub fn total_debt(&self) -> f64 {
        self.payments
            .iter()
            .filter(|p| !p.paid)
            .map(|p| p.tax_amount)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use chrono::{Duration, Local, NaiveDate};

    use crate::income::Income;

    use super::*;

    #[test]
    fn report_payments_are_ordered_by_date() {
        let tax_rate = 0.05;
        let paid = false;
        let new_payment = Payment::tax_rate(Income::from_date(today(), 250.0), tax_rate, paid);
        let older_payment = Payment::tax_rate(
            Income::from_date(today() - Duration::days(5), 480.0),
            tax_rate,
            paid,
        );
        let oldest_payment = Payment::tax_rate(
            Income::from_date(today() - Duration::days(12), 75.0),
            tax_rate,
            paid,
        );
        let payments = vec![
            new_payment.clone(),
            oldest_payment.clone(),
            older_payment.clone(),
        ];

        let report = PaymentReport::from_payments(payments);

        assert_eq!(
            report.payments,
            vec![
                oldest_payment.clone(),
                older_payment.clone(),
                new_payment.clone()
            ],
            "payments in report are in incorrect order"
        );
    }

    #[test]
    fn calculate_tax_debt() {
        let tax_rate = 0.05;
        let unpaid_1 = Payment::tax_rate(Income::from_date(today(), 250.0), tax_rate, false);
        let unpaid_2 = Payment::tax_rate(
            Income::from_date(today() - Duration::days(5), 480.0),
            tax_rate,
            false,
        );
        let paid = Payment::tax_rate(
            Income::from_date(today() - Duration::days(12), 75.0),
            tax_rate,
            true,
        );
        let payments = vec![unpaid_1.clone(), paid.clone(), unpaid_2.clone()];

        let report = PaymentReport::from_payments(payments);

        assert_eq!(report.total_debt(), 36.5);
    }

    fn today() -> NaiveDate {
        Local::now().date_naive()
    }
}
