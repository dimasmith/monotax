use std::fs::File;
use std::io::{prelude::*, stdout, BufWriter};
use std::path::PathBuf;

use anyhow::Context;
use clap::{Parser, Subcommand};
use monotax::filter::date::{QuarterFilter, YearFilter};
use monotax::filter::{IncomeFilter, IncomePredicate};
use monotax::report::generate_report;
use monotax::time::Quarter;
use monotax::{config, report, taxer, universalbank};

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
    /// Generates quaretly tax report of incomes.
    Report,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let stmt_path = cli.statement.as_path();
    let stmt_file = File::open(stmt_path)
        .with_context(|| format!("open statement file {}", stmt_path.display()))?;

    let filter = create_filters(&cli)?;
    let mut incomes = universalbank::read_incomes(stmt_file, &filter)?;

    let config = config::load_config()?;
    match cli.command {
        Command::TaxerCsv { csv_file } => {
            let writer: Box<dyn Write> = match csv_file {
                Some(path) => Box::new(BufWriter::new(File::create(path)?)),
                None => Box::new(BufWriter::new(stdout())),
            };
            taxer::export_csv(&incomes, config.taxer(), writer)?;
        }
        Command::Report => {
            let report = generate_report(&mut incomes, config.tax());
            report::console::pretty_print(&report, stdout())?;
        }
    }

    Ok(())
}

fn create_filters(cli: &Cli) -> anyhow::Result<IncomeFilter> {
    let quarter_filter = match cli.quarter {
        Some(quarter) => QuarterFilter::One(Quarter::try_from(quarter)?),
        None => QuarterFilter::Any,
    }
    .boxed();
    let year_filter = YearFilter::Current.boxed();
    let predicates = vec![year_filter, quarter_filter];
    Ok(IncomeFilter::new(predicates))
}
