use crate::db::TaxPaymentRepository;
use crate::domain::tax_payment::{NewTaxPayment, TaxPayment, ID};
use async_trait::async_trait;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use sqlx::{FromRow, SqlitePool};

pub struct SqlxTaxPaymentRepository {
    pool: SqlitePool,
}

#[derive(Debug, Clone, FromRow)]
struct PaymentRecord {
    id: ID,
    amount: f64,
    payment_date: NaiveDateTime,
}

impl SqlxTaxPaymentRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TaxPaymentRepository for SqlxTaxPaymentRepository {
    async fn insert_payment(&mut self, new_payment: NewTaxPayment) -> anyhow::Result<ID> {
        let id = self.generate_id().await?;
        let pool = &self.pool;

        let payment_record = PaymentRecord {
            id,
            amount: new_payment.amount(),
            payment_date: new_payment.payment_date(),
        };

        sqlx::query!(
            r#"INSERT INTO payment (id, amount, payment_date) VALUES (?, ?, ?)"#,
            payment_record.id,
            payment_record.amount,
            payment_record.payment_date
        )
        .execute(pool)
        .await?;

        Ok(id)
    }

    async fn find_by_year(&mut self, year: i32) -> anyhow::Result<Vec<TaxPayment>> {
        let year_start = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(year, 1, 1).unwrap(),
            NaiveTime::from_hms_nano_opt(0, 0, 0, 0).unwrap(),
        );
        let year_end = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap(),
            NaiveTime::from_hms_nano_opt(0, 0, 0, 0).unwrap(),
        );

        let pool = &self.pool;
        let records = sqlx::query_as!(PaymentRecord, r#"select id, amount, payment_date from payment where payment_date >= ? and payment_date < ?"#, year_start, year_end)
            .fetch_all(pool)
            .await?;

        let payments = records.into_iter().map(TaxPayment::from).collect();
        Ok(payments)
    }
}

impl SqlxTaxPaymentRepository {
    async fn generate_id(&mut self) -> anyhow::Result<ID> {
        Ok(0)
    }
}

impl From<PaymentRecord> for TaxPayment {
    fn from(value: PaymentRecord) -> Self {
        TaxPayment::new(value.id, value.amount, value.payment_date)
    }
}
