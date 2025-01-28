use super::filter::FilterArgs;
use anyhow::Result;
use clap::Subcommand;
use monotax_core::app::balance::generate_balance_report;
use monotax_core::domain::repository::income_tax::IncomeTaxRepository;
use monotax_core::domain::repository::IncomeRepository;

#[derive(Debug, Subcommand)]
pub enum ReportCommands {
    /// Generate balance report
    Balance {
        #[command(flatten)]
        filter: FilterArgs,
    },
}

pub async fn handle_report(
    command: &ReportCommands,
    income_repo: &mut impl IncomeRepository,
    income_tax_repo: &impl IncomeTaxRepository,
) -> Result<()> {
    match command {
        ReportCommands::Balance { filter } => {
            let criteria = filter.criteria();
            let report = generate_balance_report(criteria, income_repo, income_tax_repo).await?;
            println!("{:?}", report);
            Ok(())
        }
    }
}
