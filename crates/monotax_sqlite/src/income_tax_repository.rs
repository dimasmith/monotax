use async_trait::async_trait;
use chrono::NaiveDate;
use monotax_core::domain::model::income_tax::{IncomeTax, IncomeTaxRate, TaxRate};
use monotax_core::domain::repository::income_tax::IncomeTaxRepository;
use sqlx::SqlitePool;

pub struct SqlxIncomeTaxRepository {
    db_pool: SqlitePool,
}

impl SqlxIncomeTaxRepository {
    pub fn new(db_pool: SqlitePool) -> Self {
        Self { db_pool }
    }
}

#[derive(Debug, Clone)]
struct IncomeTaxRecord {
    id: String,
    title: String,
}

#[derive(Debug, Clone)]
struct IncomeTaxRateRecord {
    income_tax_id: String,
    rate: f64,
    start_date: NaiveDate,
}

#[async_trait]
impl IncomeTaxRepository for SqlxIncomeTaxRepository {
    async fn find_all(&self) -> anyhow::Result<Vec<IncomeTax>> {
        // select all income taxes
        let tax_records = sqlx::query_as!(
            IncomeTaxRecord,
            r#"
            SELECT id, title 
            FROM income_tax
            ORDER BY id
            "#
        )
        .fetch_all(&self.db_pool)
        .await?;

        let rate_records = sqlx::query_as!(
            IncomeTaxRateRecord,
            r#"
            SELECT income_tax_id, rate, start_date
            FROM income_tax_rate
            ORDER BY start_date
            "#
        )
        .fetch_all(&self.db_pool)
        .await?;

        let mut taxes = vec![];

        for tax_record in tax_records {
            let rate_recs = rate_records
                .iter()
                .filter(|rr| rr.income_tax_id == tax_record.id)
                .collect::<Vec<_>>();
            let mut income_tax = IncomeTax::from(tax_record);
            if rate_recs.is_empty() {
                taxes.push(income_tax);
                continue;
            }
            rate_recs.windows(2).for_each(|rates| {
                let earlier = rates[0];
                let later = rates[1];
                let rate = IncomeTaxRate::closed(
                    earlier.start_date,
                    later.start_date,
                    TaxRate::new(earlier.rate).unwrap(),
                )
                .unwrap();
                income_tax.add_rate(rate);
            });

            let last_record = rate_recs.last().unwrap();
            let rate = IncomeTaxRate::open(
                last_record.start_date,
                TaxRate::new(last_record.rate).unwrap(),
            );
            income_tax.add_rate(rate);
        }

        Ok(taxes)
    }
}

impl From<IncomeTaxRecord> for IncomeTax {
    fn from(value: IncomeTaxRecord) -> Self {
        IncomeTax::new_unchecked(value.id, value.title, vec![])
    }
}
