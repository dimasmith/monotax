//! Pretty-print reports to console

use std::io::Write;

use super::QuarterlyReport;

pub fn pretty_print(report: &QuarterlyReport, mut writer: impl Write) -> anyhow::Result<()> {
    let delimiter = "-".repeat(80);
    writeln!(&mut writer, "{}", delimiter)?;
    for line in report.quarters() {
        writeln!(&mut writer, "{} {}", line.year(), line.quarter())?;
        writeln!(&mut writer, "{}", "-".repeat(7))?;
        writeln!(&mut writer, "\t\tTotal\t\t\tCumulative")?;
        writeln!(
            &mut writer,
            "Income\t\t{:.2}\t\t{:.2}",
            line.total_income(),
            line.cumulative_income()
        )?;
        writeln!(
            &mut writer,
            "Tax\t\t{:.2}\t\t{:.2}",
            line.total_tax(),
            line.cumulative_tax()
        )?;
        writeln!(&mut writer, "{}", delimiter)?;
    }
    writeln!(
        &mut writer,
        "Total:\t\t{:.2}\t\t{:.2}",
        report.total_income(),
        report.total_tax()
    )?;
    Ok(())
}
