use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};
use monotax::time::Quarter;

#[cfg(feature = "sqlite")]
pub mod criterion;
pub mod filter;
pub mod predicate;

#[derive(Debug, Parser)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,

    /// A quarter to filter incomes. Optional.
    #[clap(short, long)]
    #[arg(value_enum)]
    pub quarter: Option<Quarter>,
    #[clap(long)]
    #[arg(value_enum, default_value_t)]
    pub include_quarters: IncludeQuarters,

    /// What years to include in the report.
    #[clap(long)]
    #[arg(value_enum, default_value_t)]
    pub include_years: IncludeYears,

    /// A specific year to filter incomes. Optional.
    #[clap(short, long)]
    #[arg(value_enum)]
    pub year: Option<i32>,
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
    #[cfg(feature = "sqlite")]
    Import {
        /// Path to the statement csv file
        statement: PathBuf,
    },
    /// Export statement csv to taxer csv
    #[cfg(not(feature = "sqlite"))]
    Taxer {
        /// Path to the statement csv file
        statement: PathBuf,
        #[clap(short, long)]
        output: Option<PathBuf>,
    },
    #[cfg(feature = "sqlite")]
    Taxer {
        /// Output file for taxer csv
        #[clap(short, long)]
        output: Option<PathBuf>,
    },
    /// Generates quarterly tax report of incomes.
    #[cfg(not(feature = "sqlite"))]
    Report {
        /// Path to the statement csv file
        statement: PathBuf,
        #[clap(short, long)]
        #[arg(value_enum)]
        #[arg(value_enum, default_value_t)]
        format: ReportFormat,
        #[clap(short, long)]
        output: Option<PathBuf>,
    },
    #[cfg(feature = "sqlite")]
    Report {
        #[clap(short, long)]
        #[arg(value_enum)]
        #[arg(value_enum, default_value_t)]
        format: ReportFormat,
        #[clap(short, long)]
        output: Option<PathBuf>,
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
