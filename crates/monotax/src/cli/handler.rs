//! Handlers for cli app requests.

use std::{fs::File, path::Path};

use anyhow::Context;
use monotax_core::app::income::{import_incomes, read_incomes};
use monotax_dbo::dbo;
use tokio::task::block_in_place;

use monotax_core::app::tax::mark_income_paid;
use monotax_core::domain::repository::{IncomeRepository, PaymentRepository, TaxPaymentRepository};
use monotax_core::domain::Income;
use monotax_core::infra::io::writer;
use monotax_core::integration::taxer;
use monotax_core::payment::report::{plaintext::plaintext_report, PaymentReport};
use monotax_core::report::{self, QuarterlyReport};

use crate::config;

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
    let incomes = read_incomes_from_file_or_db(income_repo, input, filter).await?;
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
    let incomes = read_incomes_from_file_or_db(income_repo, input, filter).await?;
    let writer = writer(output)?;
    taxer::export_csv(incomes, config.taxer(), writer)?;
    Ok(())
}

pub async fn import_incomes_from_dbo_csv(
    income_repo: &mut impl IncomeRepository,
    statement: &Path,
    filter: &FilterArgs,
) -> anyhow::Result<()> {
    let incomes = incomes_from_dbo_file(statement, filter).await?;
    let _ = import_incomes(incomes, income_repo).await?;
    Ok(())
}

async fn read_incomes_from_file_or_db(
    income_repo: &mut impl IncomeRepository,
    input: Option<&Path>,
    filter: &FilterArgs,
) -> anyhow::Result<Vec<Income>> {
    let incomes = match input {
        Some(statement) => incomes_from_dbo_file(statement, filter).await?,
        None => read_incomes(filter.criteria(), income_repo).await?,
    };
    Ok(incomes)
}

async fn incomes_from_dbo_file(input: &Path, filter: &FilterArgs) -> anyhow::Result<Vec<Income>> {
    let incomes = block_in_place(move || {
        let file = File::open(input).context("opening input file")?;
        dbo::read_incomes(file, filter.criteria())
    })?;
    Ok(incomes)
}
