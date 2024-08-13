use chrono::NaiveDateTime;

pub type PaymentID = i64;

#[derive(Debug, Clone)]
pub struct TaxPayment {
    id: PaymentID,
    amount: f64,
    payment_date: NaiveDateTime,
}

#[derive(Debug, Clone)]
pub struct NewTaxPayment {
    amount: f64,
    payment_date: NaiveDateTime,
}

impl NewTaxPayment {
    pub fn new(amount: f64, payment_date: NaiveDateTime) -> Self {
        Self {
            amount,
            payment_date,
        }
    }

    pub fn amount(&self) -> f64 {
        self.amount
    }

    pub fn payment_date(&self) -> NaiveDateTime {
        self.payment_date
    }
}

impl TaxPayment {
    pub fn new(id: PaymentID, amount: f64, payment_date: NaiveDateTime) -> Self {
        Self {
            id,
            amount,
            payment_date,
        }
    }

    pub fn id(&self) -> PaymentID {
        self.id
    }

    pub fn amount(&self) -> f64 {
        self.amount
    }

    pub fn payment_date(&self) -> NaiveDateTime {
        self.payment_date
    }
}
