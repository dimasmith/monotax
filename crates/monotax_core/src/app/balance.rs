use crate::domain::filter::income::IncomeCriteria;
use crate::domain::repository::income_tax::IncomeTaxRepository;
use crate::domain::repository::IncomeRepository;
use crate::report::balance::BalanceReport;

pub async fn generate_balance_report(
    criteria: IncomeCriteria,
    income_repository: &mut impl IncomeRepository,
    income_tax_repository: &impl IncomeTaxRepository,
) -> anyhow::Result<BalanceReport> {
    let incomes = income_repository.find_by(criteria).await?;
    let income_taxes = income_tax_repository.find_all().await?;
    Ok(BalanceReport::new(incomes, income_taxes))
}
