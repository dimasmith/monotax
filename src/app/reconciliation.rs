use chrono::Utc;

use crate::domain::{
    income::Income,
    reconciliation::{Completeness, Reconciliation},
    tax_payment::TaxPayment,
};

struct ReconcileResult {
    entries: Vec<Reconciliation>,
    balance: f64,
}

struct PartialReconciliations {
    reconciliations: Vec<Reconciliation>,
}

impl PartialReconciliations {
    fn new(reconciliations: Vec<Reconciliation>) -> Self {
        if reconciliations
            .iter()
            .any(|r| r.completeness() == Completeness::Full)
        {
            panic!("full reconciliation already exists");
        }
        Self { reconciliations }
    }

    fn iter(&self) -> impl Iterator<Item = &Reconciliation> {
        self.reconciliations.iter()
    }
}

impl From<&[Reconciliation]> for PartialReconciliations {
    fn from(reconciliations: &[Reconciliation]) -> Self {
        Self::new(reconciliations.to_vec())
    }
}

// applies payment to non-reconciled incomes.
// returns a list of reconciliations created by this operation.
fn reconcile(
    incomes: &[Income],
    reconciliations: PartialReconciliations,
    payment: TaxPayment,
    tax_rate: f64,
) -> ReconcileResult {
    let mut entries = Vec::new();
    let mut balance = payment.amount();

    for income in incomes {
        if balance <= 0.0 {
            break;
        }

        let tax_amount = income.amount() * tax_rate;
        let reconciled_amount = reconciliations
            .iter()
            .filter(|r| r.income_id() == income.id())
            .map(|r| r.reconciled_amount())
            .sum::<f64>();
        let expected_amount = tax_amount - reconciled_amount;
        if balance >= expected_amount {
            let entry = Reconciliation::new(
                income.id(),
                payment.id(),
                expected_amount,
                Utc::now().naive_local(),
                Completeness::Full,
            );
            entries.push(entry);
            balance -= expected_amount;
            continue;
        }

        let entry = Reconciliation::new(
            income.id(),
            payment.id(),
            balance,
            Utc::now().naive_local(),
            Completeness::Partial,
        );
        entries.push(entry);
        balance = 0.0;
    }

    ReconcileResult { entries, balance }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use speculoos::prelude::*;

    use crate::domain::reconciliation::Completeness;

    use super::*;

    #[test]
    fn fully_reconcile_single_income() {
        let tax_rate = 0.1;
        let expected_tax = 150.0;
        let income = Income::new(Utc::now().naive_local(), 1500.0);
        let tax_payment = TaxPayment::new(1, 150.0, Utc::now().naive_local());

        let res = reconcile(
            &[income],
            PartialReconciliations::new(vec![]),
            tax_payment,
            tax_rate,
        );

        assert_eq!(res.entries.len(), 1);
        expect_complete_entry(&res, 0, expected_tax);
        expect_remaining_balance(&res, 0.0);
    }

    #[test]
    fn fully_reconcile_single_income_with_additional_balance() {
        let tax_rate = 0.1;
        let expected_tax = 150.0;
        let income = Income::new(Utc::now().naive_local(), 1500.0);
        let tax_payment = TaxPayment::new(1, 200.0, Utc::now().naive_local());

        let res = reconcile(
            &[income],
            PartialReconciliations::new(vec![]),
            tax_payment,
            tax_rate,
        );

        assert_eq!(res.entries.len(), 1);
        expect_complete_entry(&res, 0, expected_tax);
        expect_remaining_balance(&res, 50.0);
    }

    #[test]
    fn partially_reconcile_single_income_with_additional_balance() {
        let tax_rate = 0.1;
        let income = Income::new(Utc::now().naive_local(), 1500.0);
        let tax_payment = TaxPayment::new(1, 50.0, Utc::now().naive_local());

        let res = reconcile(
            &[income],
            PartialReconciliations::new(vec![]),
            tax_payment,
            tax_rate,
        );

        assert_eq!(res.entries.len(), 1);
        expect_partial_entry(&res, 0, 50.0);
        expect_remaining_balance(&res, 0.0);
    }

    #[test]
    fn fully_reconcile_partially_reconciled_income() {
        let tax_rate = 0.1;
        let income = Income::new(Utc::now().naive_local(), 1500.0);
        let partial_reconciliation = Reconciliation::new(
            income.id(),
            1,
            100.0,
            Utc::now().naive_local(),
            Completeness::Partial,
        );
        let tax_payment = TaxPayment::new(1, 50.0, Utc::now().naive_local());

        let res = reconcile(
            &[income],
            PartialReconciliations::new(vec![partial_reconciliation]),
            tax_payment,
            tax_rate,
        );

        assert_eq!(res.entries.len(), 1);
        expect_complete_entry(&res, 0, 50.0);
        expect_remaining_balance(&res, 0.0);
    }

    #[test]
    fn reconcile_two_incomes_with_last_partial() {
        let tax_rate = 0.1;

        let income_1 = Income::new(Utc::now().naive_local(), 500.0);
        let income_2 = Income::new(Utc::now().naive_local(), 800.0);
        let tax_payment = TaxPayment::new(1, 100.0, Utc::now().naive_local());

        let res = reconcile(
            &[income_1, income_2],
            PartialReconciliations::new(vec![]),
            tax_payment,
            tax_rate,
        );

        assert_eq!(res.entries.len(), 2);

        expect_complete_entry(&res, 0, 50.0);
        expect_partial_entry(&res, 1, 50.0);

        expect_remaining_balance(&res, 0.0);
    }

    const TOLERANCE: f64 = 0.0001;

    fn expect_remaining_balance(response: &ReconcileResult, balance: f64) {
        asserting("remaining balance")
            .that(&response.balance)
            .is_close_to(balance, TOLERANCE);
    }

    fn expect_complete_entry(response: &ReconcileResult, idx: usize, balance: f64) {
        expect_entry(response, idx, balance, Completeness::Full);
    }

    fn expect_partial_entry(response: &ReconcileResult, idx: usize, balance: f64) {
        expect_entry(response, idx, balance, Completeness::Partial);
    }

    fn expect_entry(
        response: &ReconcileResult,
        idx: usize,
        balance: f64,
        completeness: Completeness,
    ) {
        let entry = &response.entries[idx];
        asserting("entry balance")
            .that(&entry.reconciled_amount())
            .is_close_to(balance, TOLERANCE);
        asserting("entry completeness")
            .that(&entry.completeness())
            .is_equal_to(completeness);
    }
}
