use std::fs::File;
use std::io::{prelude::*, stdout, BufWriter};
use std::path::PathBuf;

use anyhow::Context;
use clap::{Parser, Subcommand};
use cli::{IncludeQuarters, IncludeYears};
use monotax::filter::date::{QuarterFilter, YearFilter};
use monotax::filter::{IncomeFilter, IncomePredicate};
use monotax::report::generate_report;
use monotax::time::Quarter;
use monotax::{config, report, taxer, universalbank};

mod cli;

#[derive(Debug, Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Command,

    /// Path to the statement csv file
    statement: PathBuf,

    /// A qarter to filter incomes. Optional.
    #[clap(short, long)]
    #[arg(value_enum)]
    quarter: Option<Quarter>,
    #[clap(long)]
    #[arg(value_enum, default_value_t)]
    include_quarters: IncludeQuarters,

    /// What years to include in the report.
    #[clap(long)]
    #[arg(value_enum, default_value_t)]
    include_years: IncludeYears,

    /// A specific year to filter incomes. Optional.
    #[clap(short, long)]
    #[arg(value_enum)]
    year: Option<i32>,
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
    let include_quarters = cli.include_quarters;
    let quarter = cli.quarter;

    let quarter_filter = match (include_quarters, quarter) {
        (IncludeQuarters::Any, None) => QuarterFilter::Any,
        (IncludeQuarters::Any, Some(q)) => QuarterFilter::Only(q),
        (IncludeQuarters::One, None) => QuarterFilter::Current,
        (IncludeQuarters::One, Some(q)) => QuarterFilter::Only(q),
        (IncludeQuarters::Ytd, None) => QuarterFilter::CurrentToDate,
        (IncludeQuarters::Ytd, Some(q)) => QuarterFilter::Ytd(q),
    };
    let year_filter = match (cli.include_years, cli.year) {
        (IncludeYears::All, None) => YearFilter::Any,
        (IncludeYears::All, Some(y)) => YearFilter::One(y),
        (IncludeYears::One, None) => YearFilter::Current,
        (IncludeYears::One, Some(y)) => YearFilter::One(y),
        (IncludeYears::Current, None) => YearFilter::Current,
        (IncludeYears::Current, Some(y)) => YearFilter::One(y),
    };
    let predicates = vec![year_filter.boxed(), quarter_filter.boxed()];
    Ok(IncomeFilter::new(predicates))
}
