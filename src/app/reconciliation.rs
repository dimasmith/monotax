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

// applies payment to non-reconciled incomes.
// returns a list of reconciliations created by this operation.
fn reconcile(
    incomes: PendingIncomes,
    reconciliations: PartialReconciliations,
    payment: TaxPayment,
    tax_rate: f64,
) -> ReconcileResult {
    let mut entries = Vec::new();
    let mut balance = payment.amount();

    for income in incomes.iter() {
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

struct PendingIncomes<'a> {
    incomes: &'a [Income],
}

impl<'a> PendingIncomes<'a> {
    fn new(incomes: &'a [Income]) -> Self {
        assert!(
            incomes
                .windows(2)
                .all(|w| w[0].datetime() <= w[1].datetime()),
            "incomes are not sorted by datetime"
        );
        Self { incomes }
    }

    fn iter(&self) -> impl Iterator<Item = &Income> {
        self.incomes.iter()
    }
}

impl AsRef<[Income]> for PendingIncomes<'_> {
    fn as_ref(&self) -> &[Income] {
        self.incomes
    }
}

#[derive(Debug, Clone, Default)]
struct PartialReconciliations<'a> {
    reconciliations: &'a [Reconciliation],
}

impl<'a> PartialReconciliations<'a> {
    fn new(reconciliations: &'a [Reconciliation]) -> Self {
        assert!(
            reconciliations
                .iter()
                .all(|r| r.completeness() == Completeness::Partial),
            "must not contain full reconciliations"
        );
        assert!(
            reconciliations
                .windows(2)
                .all(|w| w[0].reconciled_on() <= w[1].reconciled_on()),
            "reconciliations must be sorted by date"
        );
        Self { reconciliations }
    }

    fn iter(&self) -> impl Iterator<Item = &Reconciliation> {
        self.reconciliations.iter()
    }
}

impl AsRef<[Reconciliation]> for PartialReconciliations<'_> {
    fn as_ref(&self) -> &[Reconciliation] {
        self.reconciliations
    }
}

#[cfg(test)]
mod tests {
    use chrono::{NaiveDate, Utc};
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
            PendingIncomes::new(&[income]),
            PartialReconciliations::default(),
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
            PendingIncomes::new(&[income]),
            PartialReconciliations::default(),
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
            PendingIncomes::new(&[income]),
            PartialReconciliations::default(),
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
            PendingIncomes::new(&[income]),
            PartialReconciliations::new(&[partial_reconciliation]),
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
            PendingIncomes::new(&[income_1, income_2]),
            PartialReconciliations::default(),
            tax_payment,
            tax_rate,
        );

        assert_eq!(res.entries.len(), 2);

        expect_complete_entry(&res, 0, 50.0);
        expect_partial_entry(&res, 1, 50.0);

        expect_remaining_balance(&res, 0.0);
    }

    #[test]
    fn create_pending_incomes_from_incomes_sorted_by_date() {
        let base_date = NaiveDate::from_yo_opt(2023, 22)
            .unwrap()
            .and_hms_opt(0, 24, 0)
            .unwrap();
        let oldest = Income::new(base_date, 1000.0);
        let old = Income::new(base_date + chrono::Duration::days(1), 500.0);
        let new = Income::new(base_date + chrono::Duration::days(10), 800.0);
        let incomes = &[oldest, old, new];

        let pending_incomes = PendingIncomes::new(incomes);

        assert_eq!(pending_incomes.as_ref(), incomes);
    }

    #[test]
    #[should_panic]
    fn fail_to_create_pending_incomes_from_unsorted_incomes() {
        let base_date = NaiveDate::from_yo_opt(2023, 22)
            .unwrap()
            .and_hms_opt(0, 24, 0)
            .unwrap();
        let oldest = Income::new(base_date, 1000.0);
        let old = Income::new(base_date + chrono::Duration::days(1), 500.0);
        let new = Income::new(base_date + chrono::Duration::days(10), 800.0);

        let _ = PendingIncomes::new(&[oldest, new, old]);
    }

    #[test]
    #[should_panic]
    fn pending_reconciliations_cannot_have_complete_entries() {
        let partial =
            Reconciliation::new(1, 1, 100.0, Utc::now().naive_local(), Completeness::Partial);
        let complete =
            Reconciliation::new(1, 1, 100.0, Utc::now().naive_local(), Completeness::Full);

        let _ = PartialReconciliations::new(&[partial, complete]);
    }

    #[test]
    fn pending_reconciliations_can_contain_only_partial_entries() {
        let partial =
            Reconciliation::new(1, 1, 100.0, Utc::now().naive_local(), Completeness::Partial);
        let partial_2 =
            Reconciliation::new(1, 1, 100.0, Utc::now().naive_local(), Completeness::Partial);

        let entries = &[partial, partial_2];

        let recs = PartialReconciliations::new(entries);

        assert_eq!(recs.as_ref(), entries);
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
