use std::fs::File;
use std::io::{prelude::*, stdout, BufReader, BufWriter};
use std::path::PathBuf;

use anyhow::Context;
use clap::{Parser, Subcommand};
use encoding_rs::WINDOWS_1251;
use encoding_rs_rw::DecodingReader;
use universalbank::UnivesralBankIncome;

mod income;
mod taxer;
mod universalbank;

#[derive(Debug, Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Command,

    /// Path to the statement csv file
    statement: PathBuf,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Export statement csv to taxer csv
    TaxerCsv { csv_file: Option<PathBuf> },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let stmt_path = cli.statement.as_path();
    let stmt_file = BufReader::new(
        std::fs::File::open(stmt_path)
            .with_context(|| format!("open statement file {}", stmt_path.display()))?,
    );
    let mut reader = DecodingReader::new(stmt_file, WINDOWS_1251.new_decoder());
    let mut csv = csv::ReaderBuilder::new()
        .delimiter(b';')
        .flexible(true)
        .from_reader(&mut reader);

    let mut incomes = vec![];
    for record in csv.records() {
        let record = record?;
        let income = UnivesralBankIncome::try_from(record)?;
        incomes.push(income);
    }

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
