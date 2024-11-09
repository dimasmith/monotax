use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};

use self::filter::FilterArgs;
use self::payment::PaymentCommands;

pub mod app;
pub mod filter;
pub mod handler;
pub(super) mod payment;

#[derive(Debug, Parser)]
#[command(version, about, long_about)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Initialize storage and configuration
    Init {
        /// Forces monotax to recreate configuration
        #[clap(short, long)]
        force: bool,
    },
    /// Import incomes into the database
    Import {
        /// Path to the statement csv file
        statement: PathBuf,
        #[command(flatten)]
        filter: FilterArgs,
    },
    /// Export statement csv to taxer csv
    Taxer {
        /// Input file to export. If specified, the database is ignored.
        input: Option<PathBuf>,
        /// Output file for taxer csv
        #[clap(short, long)]
        output: Option<PathBuf>,
        #[command(flatten)]
        filter: FilterArgs,
    },
    /// Generates quarterly tax report of incomes.
    Report {
        /// Input file to export. If specified, the database is ignored.
        input: Option<PathBuf>,
        #[clap(short, long)]
        #[arg(value_enum)]
        #[arg(value_enum, default_value_t)]
        format: ReportFormat,
        #[clap(short, long)]
        output: Option<PathBuf>,
        #[command(flatten)]
        filter: FilterArgs,
    },
    /// Work with tax payments.
    Payments {
        #[clap(subcommand)]
        command: PaymentCommands,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, ValueEnum, Default)]
pub enum ReportFormat {
    /// Print report to console
    #[default]
    Console,
    /// Export report to csv file
    Csv,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum, Default)]
pub enum IncludeQuarters {
    #[default]
    Any,
    One,
    Ytd,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum, Default)]
pub enum IncludeYears {
    All,
    One,
    #[default]
    Current,
}
