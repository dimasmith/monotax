use common::connect_to_test_db;
use monotax_core::domain::repository::IncomeRepository;
use monotax_core::infra::sqlx::income_repository;

mod common;
mod income_repository_ctk;

async fn create_repository() -> impl IncomeRepository {
    let pool = connect_to_test_db().await;
    income_repository(pool)
}

#[tokio::test]
async fn save_and_load_incomes() {
    let mut repo = create_repository().await;

    income_repository_ctk::test_save_and_load_incomes(&mut repo).await;
}

#[tokio::test]
async fn ignore_duplicate_incomes() {
    let mut repo = create_repository().await;

    income_repository_ctk::test_ignore_duplicate_incomes(&mut repo).await;
}

#[tokio::test]
async fn filter_incomes_on_quarters() {
    let mut repo = create_repository().await;

    income_repository_ctk::test_filter_incomes_on_quarters(&mut repo).await;
}

#[tokio::test]
async fn filter_incomes_on_years() {
    let mut repo = create_repository().await;

    income_repository_ctk::test_filter_incomes_on_years(&mut repo).await;
}

#[tokio::test]
async fn filter_incomes_on_quarters_and_years() {
    let mut repo = create_repository().await;

    income_repository_ctk::test_filter_incomes_on_quarters_and_years(&mut repo).await;
}
