use clap::Subcommand;

use super::filter::FilterArgs;

#[derive(Debug, Subcommand)]
pub enum PaymentCommands {
    /// Report on tax payments.
    Report {
        #[command(flatten)]
        filter: FilterArgs,
    },
    /// Mark that taxes are paid for the income.
    Pay {
        /// Unique income number.
        payment_no: i64,
    },
    /// Mark that taxes are not paid for the income.
    Unpay {
        /// Unique income number.
        payment_no: i64,
    },
}
