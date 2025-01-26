use log::info;

use crate::domain::{filter::income::IncomeCriteria, repository::IncomeRepository, Income};

pub async fn import_incomes(
    incomes: Vec<Income>,
    income_repo: &mut impl IncomeRepository,
) -> anyhow::Result<usize> {
    let imported_count = income_repo.save_all(&incomes).await?;
    info!("imported {} incomes", imported_count);
    Ok(imported_count)
}

pub async fn read_incomes(
    criteria: IncomeCriteria,
    income_repo: &mut impl IncomeRepository,
) -> anyhow::Result<Vec<Income>> {
    income_repo.find_by(criteria).await
}
