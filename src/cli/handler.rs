//! Handlers for cli app requests.

use std::{fs::File, path::Path};

use anyhow::Context;
use tokio::task::block_in_place;

use crate::app::tax::mark_income_paid;
use crate::config;
use crate::domain::repository::{IncomeRepository, PaymentRepository, TaxPaymentRepository};
use crate::domain::Income;
use crate::infra::io::writer;
use crate::integration::{taxer, universalbank};
use crate::payment::report::{plaintext::plaintext_report, PaymentReport};
use crate::report::{self, QuarterlyReport};

use super::{filter::FilterArgs, ReportFormat};

pub async fn cancel_tax_payment(
    payments_repo: &mut impl PaymentRepository,
    payment_no: &i64,
) -> anyhow::Result<()> {
    payments_repo.mark_unpaid(*payment_no).await?;
    Ok(())
}

pub async fn pay_tax(
    payments_repo: &mut impl PaymentRepository,
    incomes_repo: &mut impl IncomeRepository,
    tax_payments_repo: &mut impl TaxPaymentRepository,
    payment_no: i64,
) -> anyhow::Result<()> {
    mark_income_paid(payment_no, payments_repo, incomes_repo, tax_payments_repo).await?;
    Ok(())
}

pub async fn report_payments(
    payments_repo: &mut impl PaymentRepository,
    output: Option<&Path>,
    filter: &FilterArgs,
) -> anyhow::Result<()> {
    let criteria = filter.criteria();
    let payments = payments_repo.find_by(criteria).await?;
    let report = PaymentReport::from_payments(payments);
    let writer = writer(output)?;
    plaintext_report(&report, writer)?;
    Ok(())
}

pub async fn generate_incomes_report(
    income_repo: &mut impl IncomeRepository,
    input: Option<&Path>,
    format: &ReportFormat,
    output: Option<&Path>,
    filter: &FilterArgs,
) -> anyhow::Result<()> {
    let config = config::load_config()?;
    let incomes = read_incomes(income_repo, input, filter).await?;
    let report = QuarterlyReport::build_report(incomes, config.tax());
    let writer = writer(output)?;
    match format {
        ReportFormat::Console => report::console::pretty_print(&report, writer)?,
        ReportFormat::Csv => report::csv::render_csv(&report, writer)?,
    };
    Ok(())
}

pub async fn generate_taxer_report(
    income_repo: &mut impl IncomeRepository,
    input: Option<&Path>,
    output: Option<&Path>,
    filter: &FilterArgs,
) -> anyhow::Result<()> {
    let config = config::load_config()?;
    let incomes = read_incomes(income_repo, input, filter).await?;
    let writer = writer(output)?;
    taxer::export_csv(incomes, config.taxer(), writer)?;
    Ok(())
}

pub async fn import_incomes(
    income_repo: &mut impl IncomeRepository,
    statement: &Path,
    filter: &FilterArgs,
) -> anyhow::Result<()> {
    let incomes = read_incomes(income_repo, Some(statement), filter).await?;
    let imported = income_repo
        .save_all(&incomes.into_iter().collect::<Vec<_>>())
        .await?;
    log::info!("Imported {} incomes", imported);
    Ok(())
}

pub async fn read_incomes(
    income_repo: &mut impl IncomeRepository,
    input: Option<&Path>,
    filter: &FilterArgs,
) -> anyhow::Result<impl IntoIterator<Item = Income>> {
    let incomes = match input {
        Some(stmt) => block_in_place(move || {
            let file = File::open(stmt).context("opening input file")?;
            universalbank::read_incomes(file, filter.criteria())
        })?,
        None => income_repo.find_all().await?,
    };
    Ok(incomes)
}
