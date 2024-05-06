use std::fs::File;
use std::io::{prelude::*, stdout, BufWriter};

use anyhow::Context;

use clap::Parser;
use cli::payment::PaymentCommands;
use cli::ReportFormat;
use cli::{Cli, Command};
use env_logger::{Builder, Env};
use monotax::db::{self, find_payments_by_criteria, mark_paid, mark_unpaid};
use monotax::payment::report::plaintext::plaintext_report;
use monotax::payment::report::PaymentReport;
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
        Command::Import { statement, filter } => {
            let stmt_path = statement.as_path();
            let stmt_file = File::open(stmt_path)
                .with_context(|| format!("open statement file {}", stmt_path.display()))?;
            let incomes = universalbank::read_incomes(stmt_file, filter.criteria())?;
            let imported = db::save_all(&incomes)?;
            log::info!("Imported {} incomes", imported);
        }

        Command::Taxer { output, filter } => {
            let config = config::load_config()?;
            let incomes = db::find_by_criteria(filter.criteria())?;
            let writer: Box<dyn Write> = match output {
                Some(path) => Box::new(BufWriter::new(File::create(path)?)),
                None => Box::new(BufWriter::new(stdout())),
            };
            taxer::export_csv(&incomes, config.taxer(), writer)?;
        }
        Command::Report {
            format,
            output,
            filter,
        } => {
            let config = config::load_config()?;
            let incomes = db::find_by_criteria(filter.criteria())?;
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
        Command::Payments { command } => match command {
            PaymentCommands::Report { filter } => {
                let payments = find_payments_by_criteria(filter.criteria())?;
                let report = PaymentReport::from_payments(payments);
                plaintext_report(&report, stdout())?;
            }
            PaymentCommands::Pay { payment_no } => {
                mark_paid(*payment_no)?;
            }
            PaymentCommands::Unpay { payment_no } => {
                mark_unpaid(*payment_no)?;
            }
        },
    }

    Ok(())
}
