use async_trait::async_trait;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use monotax_core::domain::repository::TaxPaymentRepository;
use monotax_core::domain::{NewTaxPayment, TaxPayment, TaxPaymentID};
use sqlx::{FromRow, SqlitePool};

pub struct SqlxTaxPaymentRepository {
    pool: SqlitePool,
}

#[derive(Debug, Clone, FromRow)]
struct PaymentRecord {
    id: TaxPaymentID,
    amount: f64,
    payment_date: NaiveDateTime,
}

impl SqlxTaxPaymentRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

pub fn payment_tax_repository(pool: SqlitePool) -> impl TaxPaymentRepository {
    SqlxTaxPaymentRepository::new(pool)
}

#[async_trait]
impl TaxPaymentRepository for SqlxTaxPaymentRepository {
    async fn insert_payment(&mut self, new_payment: NewTaxPayment) -> anyhow::Result<TaxPaymentID> {
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
    async fn generate_id(&mut self) -> anyhow::Result<TaxPaymentID> {
        let pool = &self.pool;
        let id = sqlx::query_scalar!(r#"select max(id) from payment"#)
            .fetch_optional(pool)
            .await?
            .unwrap_or_default()
            .unwrap();
        Ok(id + 1)
    }
}

impl From<PaymentRecord> for TaxPayment {
    fn from(value: PaymentRecord) -> Self {
        TaxPayment::new(value.id, value.amount, value.payment_date)
    }
}
