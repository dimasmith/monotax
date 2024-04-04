use std::fs::File;
use std::io::{prelude::*, stdout, BufReader, BufWriter};
use std::path::PathBuf;

use anyhow::Context;
use clap::Parser;
use encoding_rs::WINDOWS_1251;
use encoding_rs_rw::DecodingReader;
use universalbank::UnivesralBankIncome;

mod income;
mod taxer;
mod universalbank;

#[derive(Debug, Parser)]
struct Cli {
    /// Path to the statement csv file
    statement: PathBuf,
    /// Path to the generated csv in taxer format
    taxer_csv: Option<PathBuf>,
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

    let writer = match cli.taxer_csv {
        Some(path) => {
            let file = File::create(path).with_context(|| format!("create taxer csv file"))?;
            Box::new(BufWriter::new(file)) as Box<dyn Write>
        }
        None => Box::new(BufWriter::new(stdout())) as Box<dyn Write>,
    };
    let mut csv_writer = csv::WriterBuilder::new().from_writer(writer);

    for income in &incomes {
        let taxer_income = taxer::TaxerIncome::from_income(income);
        taxer_income.write(&mut csv_writer)?;
    }

    Ok(())
}
