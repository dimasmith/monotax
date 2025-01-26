//! File and console IO.

use std::fs::File;
use std::io::{stdout, BufWriter, Write};
use std::path::Path;

/// Provides a writer that writes to a file when the path is present,
/// or to the standard output if not.
pub fn writer(output: Option<&Path>) -> anyhow::Result<Box<dyn Write>> {
    let writer: Box<dyn Write> = match output {
        Some(path) => Box::new(BufWriter::new(File::create(path)?)),
        None => Box::new(BufWriter::new(stdout())),
    };
    Ok(writer)
}
