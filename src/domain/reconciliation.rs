use chrono::NaiveDateTime;

use super::{income::IncomeID, tax_payment::PaymentID};

pub type ReconciliationID = uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Completeness {
    Full,
    Partial,
}

#[derive(Debug, Clone)]
pub struct Reconciliation {
    id: ReconciliationID,
    income_id: IncomeID,
    payment_id: PaymentID,
    reconciled_amount: f64,
    reconciled_on: NaiveDateTime,
    completeness: Completeness,
}

impl Reconciliation {
    pub fn new(
        income_id: IncomeID,
        payment_id: PaymentID,
        reconciled_amount: f64,
        reconciled_on: NaiveDateTime,
        completeness: Completeness,
    ) -> Self {
        Self {
            id: uuid::Uuid::now_v7(),
            income_id,
            payment_id,
            reconciled_amount,
            reconciled_on,
            completeness,
        }
    }

    pub fn id(&self) -> ReconciliationID {
        self.id
    }

    pub fn income_id(&self) -> IncomeID {
        self.income_id
    }

    pub fn payment_id(&self) -> PaymentID {
        self.payment_id
    }

    pub fn reconciled_amount(&self) -> f64 {
        self.reconciled_amount
    }

    pub fn reconciled_on(&self) -> NaiveDateTime {
        self.reconciled_on
    }

    pub fn completeness(&self) -> Completeness {
        self.completeness
    }
}

impl PartialEq for Reconciliation {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
