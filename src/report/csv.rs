/// Renders quaterly reports to CSV format.
use std::io::Write;

use super::QuarterlyReport;

/// Renders reports to CSV format.
pub fn render_csv(report: &QuarterlyReport, mut writer: impl Write) -> anyhow::Result<()> {
    writeln!(
        &mut writer,
        "Year,Quarter,Total Income,Cumulative Income,Total Tax,Cumulative Tax"
    )?;
    for line in report.quarters() {
        writeln!(
            &mut writer,
            "{},{},{:.2},{:.2},{:.2},{:.2}",
            line.year(),
            line.quarter(),
            line.total_income(),
            line.cumulative_income(),
            line.total_tax(),
            line.cumulative_tax()
        )?;
    }
    Ok(())
}
