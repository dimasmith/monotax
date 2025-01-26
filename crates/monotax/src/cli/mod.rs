use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};

use self::filter::FilterArgs;

pub mod app;
pub mod filter;
pub mod handler;

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
