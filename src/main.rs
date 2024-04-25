use std::fs::File;
use std::io::{prelude::*, stdout, BufWriter};

use anyhow::Context;

use clap::Parser;
#[cfg(feature = "sqlite")]
use cli::criterion::build_criteria;
use cli::predicate::build_predicates;
use cli::ReportFormat;
use cli::{Cli, Command};
use env_logger::{Builder, Env};
#[cfg(feature = "sqlite")]
use monotax::db;
#[cfg(feature = "sqlite")]
use monotax::db::criteria::Criteria;
use monotax::filter::IncomeFilter;
use monotax::report::QuarterlyReport;
use monotax::{config, init, report, taxer, universalbank};

mod cli;

fn main() -> anyhow::Result<()> {
    // Initialize logging
    let env = Env::default().filter_or("RUST_LOG", "monotax=info");
    Builder::from_env(env).init();

    let cli = Cli::parse();

    match &cli.command {
        Command::Init { force } => init::init(*force)?,
        #[cfg(feature = "sqlite")]
        Command::Import { statement, filter } => {
            let predicates = build_predicates(filter)?;
            let stmt_path = statement.as_path();
            let stmt_file = File::open(stmt_path)
                .with_context(|| format!("open statement file {}", stmt_path.display()))?;
            let incomes = universalbank::read_incomes(stmt_file, &IncomeFilter::new(predicates))?;
            let imported = db::save_all(&incomes)?;
            log::info!("Imported {} incomes", imported);
        }
        #[cfg(not(feature = "sqlite"))]
        Command::Taxer {
            statement,
            output,
            filter,
        } => {
            let config = config::load_config()?;
            let predicates = build_predicates(filter)?;
            let stmt_path = statement.as_path();
            let stmt_file = File::open(stmt_path)
                .with_context(|| format!("open statement file {}", stmt_path.display()))?;

            let incomes = universalbank::read_incomes(stmt_file, &IncomeFilter::new(predicates))?;
            let writer: Box<dyn Write> = match output {
                Some(path) => Box::new(BufWriter::new(File::create(path)?)),
                None => Box::new(BufWriter::new(stdout())),
            };
            taxer::export_csv(&incomes, config.taxer(), writer)?;
        }
        #[cfg(feature = "sqlite")]
        Command::Taxer { output, filter } => {
            let config = config::load_config()?;
            let criteria = build_criteria(filter)?;
            let incomes = db::find_by_criteria(&Criteria::And(criteria))?;
            let writer: Box<dyn Write> = match output {
                Some(path) => Box::new(BufWriter::new(File::create(path)?)),
                None => Box::new(BufWriter::new(stdout())),
            };
            taxer::export_csv(&incomes, config.taxer(), writer)?;
        }
        #[cfg(not(feature = "sqlite"))]
        Command::Report {
            statement,
            format,
            output,
            filter,
        } => {
            let config = config::load_config()?;
            let predicates = build_predicates(filter)?;
            let stmt_path = statement.as_path();
            let stmt_file = File::open(stmt_path)
                .with_context(|| format!("open statement file {}", stmt_path.display()))?;

            let incomes = universalbank::read_incomes(stmt_file, &IncomeFilter::new(predicates))?;
            let report = QuarterlyReport::build_report(incomes, config.tax());
            let writer: Box<dyn Write> = match output {
                Some(path) => Box::new(BufWriter::new(File::create(path)?)),
                None => Box::new(BufWriter::new(stdout())),
            };
            match format {
                ReportFormat::Console => report::console::pretty_print(&report, writer)?,
                ReportFormat::Csv => report::csv::render_csv(&report, writer)?,
            };
        }
        #[cfg(feature = "sqlite")]
        Command::Report {
            format,
            output,
            filter,
        } => {
            let config = config::load_config()?;
            let criteria = build_criteria(filter)?;
            let incomes = db::find_by_criteria(&Criteria::And(criteria))?;
            let report = QuarterlyReport::build_report(incomes, config.tax());
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
