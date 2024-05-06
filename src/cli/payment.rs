use clap::Subcommand;

use super::filter::FilterArgs;

#[derive(Debug, Subcommand)]
pub enum PaymentCommands {
    /// Report on tax payments.
    Report {
        #[command(flatten)]
        filter: FilterArgs,
    },
    Pay {
        payment_no: i64,
    },
}
