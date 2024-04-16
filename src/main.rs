use std::fs::File;
use std::io::{prelude::*, stdout, BufWriter};

use anyhow::Context;

use clap::Parser;
use cli::{Cli, Command, IncludeQuarters, IncludeYears, ReportFormat};
use env_logger::{Builder, Env};
use monotax::filter::date::{QuarterFilter, YearFilter};
use monotax::filter::{IncomeFilter, IncomePredicate};
use monotax::report::generate_report;
use monotax::{config, db, init, report, taxer, universalbank};

mod cli;

fn main() -> anyhow::Result<()> {
    // Initialize logging
    let env = Env::default().filter_or("RUST_LOG", "monotax=info");
    Builder::from_env(env).init();

    let cli = Cli::parse();

    match &cli.command {
        Command::Init { force } => init::init(*force)?,
        #[cfg(feature = "sqlite")]
        Command::Import { statement } => {
            let filter = create_filters(&cli)?;
            let stmt_path = statement.as_path();
            let stmt_file = File::open(stmt_path)
                .with_context(|| format!("open statement file {}", stmt_path.display()))?;
            let incomes = universalbank::read_incomes(stmt_file, &filter)?;
            let imported = db::save_all(&incomes)?;
            log::info!("Imported {} incomes", imported);
        }
        Command::Taxer { statement, output } => {
            let config = config::load_config()?;
            let filter = create_filters(&cli)?;
            let stmt_path = statement.as_path();
            let stmt_file = File::open(stmt_path)
                .with_context(|| format!("open statement file {}", stmt_path.display()))?;

            let incomes = universalbank::read_incomes(stmt_file, &filter)?;
            let writer: Box<dyn Write> = match output {
                Some(path) => Box::new(BufWriter::new(File::create(path)?)),
                None => Box::new(BufWriter::new(stdout())),
            };
            taxer::export_csv(&incomes, config.taxer(), writer)?;
        }
        Command::Report {
            statement,
            format,
            output,
        } => {
            let config = config::load_config()?;
            let filter = create_filters(&cli)?;
            let stmt_path = statement.as_path();
            let stmt_file = File::open(stmt_path)
                .with_context(|| format!("open statement file {}", stmt_path.display()))?;

            let incomes = universalbank::read_incomes(stmt_file, &filter)?;
            let report = generate_report(incomes.into_iter(), config.tax());
            let writer: Box<dyn Write> = match output {
                Some(path) => Box::new(BufWriter::new(File::create(path)?)),
                None => Box::new(BufWriter::new(stdout())),
            };
            match format {
                ReportFormat::Console => report::console::pretty_print(&report, writer)?,
                ReportFormat::Csv => report::csv::render_csv(&report, writer)?,
            };
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
