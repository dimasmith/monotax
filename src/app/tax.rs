use crate::db::{IncomeRepository, PaymentRepository, TaxPaymentRepository};
use crate::domain::NewTaxPayment;
use chrono::Utc;

pub async fn mark_income_paid(
    payment_no: i64,
    payments_repo: &mut impl PaymentRepository,
    incomes_repo: &mut impl IncomeRepository,
    tax_payment_repo: &mut impl TaxPaymentRepository,
) -> anyhow::Result<()> {
    // mark income as paid
    payments_repo.mark_paid(payment_no).await?;
    // read income and create a necessary payment record
    let income = incomes_repo.find_by_payment_no(payment_no).await?;
    if income.is_none() {
        anyhow::bail!("no income found for payment no {}", payment_no);
    }
    let income = income.unwrap();
    let tax_payment = NewTaxPayment::new(income.amount(), Utc::now().naive_local());
    let _ = tax_payment_repo.insert_payment(tax_payment).await?;
    Ok(())
}
