use std::fs::File;
use std::io::{prelude::*, stdout, BufWriter};
use std::path::PathBuf;

use anyhow::Context;
use clap::{Parser, Subcommand};
use date_filter::{Quarter, QuarterFilter, YearFilter};

mod date_filter;
mod income;
mod taxer;
mod universalbank;

#[derive(Debug, Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Command,

    /// Path to the statement csv file
    statement: PathBuf,

    /// A qarter to filter incomes. Optional.
    #[clap(short, long)]
    quarter: Option<u32>,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Export statement csv to taxer csv
    TaxerCsv { csv_file: Option<PathBuf> },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let stmt_path = cli.statement.as_path();
    let stmt_file = File::open(stmt_path)
        .with_context(|| format!("open statement file {}", stmt_path.display()))?;

    let quarter_filter = match cli.quarter {
        Some(quarter) => QuarterFilter::OneQuarter(Quarter::try_from(quarter)?),
        None => QuarterFilter::AllQuarters,
    };
    let year_filter = YearFilter::CurrentYear;

    let incomes = universalbank::read_incomes(stmt_file)?
        .into_iter()
        .filter(|income| year_filter.filter_income(income))
        .filter(|income| quarter_filter.filter_income(income))
        .collect::<Vec<_>>();

    match cli.command {
        Command::TaxerCsv { csv_file } => {
            let writer: Box<dyn Write> = match csv_file {
                Some(path) => Box::new(BufWriter::new(File::create(path)?)),
                None => Box::new(BufWriter::new(stdout())),
            };
            taxer::export_csv(&incomes, writer)?;
        }
    }

    Ok(())
}
