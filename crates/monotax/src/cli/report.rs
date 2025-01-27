use clap::Subcommand;
use anyhow::Result;
use monotax_core::domain::filter::income::IncomeCriteria;
use monotax_core::app::balance::generate_balance_report;
use monotax_core::domain::repository::income_tax::IncomeTaxRepository;
use monotax_core::domain::repository::IncomeRepository;
use super::filter::FilterArgs;

#[derive(Debug, Subcommand)]
pub enum ReportCommands {
    /// Generate balance report
    Balance {
        #[command(flatten)]
        filter: FilterArgs,
    }
}

pub async fn handle_report(
    command: &ReportCommands,
    income_repo: &mut impl IncomeRepository,
    income_tax_repo: &impl IncomeTaxRepository,
) -> Result<()> {
    match command {
        ReportCommands::Balance { filter } => {
            let criteria = IncomeCriteria::from(filter.criteria());
            let report = generate_balance_report(criteria, income_repo, income_tax_repo).await?;            
            println!("{:?}", report);
            Ok(())
        }
    }
}

