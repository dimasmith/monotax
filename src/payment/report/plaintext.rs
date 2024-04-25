//! Render plaintext payments report.

use std::io::Write;

use super::PaymentReport;

pub fn plaintext_report<W>(report: &PaymentReport, mut writer: W) -> anyhow::Result<()>
where
    W: Write,
{
    for (i, payment) in report.payments().iter().enumerate() {
        let date = payment.income().date();
        let income_amount = payment.income().amount();
        let tax_amount = payment.tax_amount();
        let paid = payment.paid();
        writeln!(
            &mut writer,
            "{}:\t{}\t{:.2}\t{:.2}\t{}",
            i + 1,
            date,
            income_amount,
            tax_amount,
            paid
        )?;
    }
    writeln!(&mut writer, "{}", "-".repeat(40))?;
    writeln!(&mut writer, "Tax dept:\t{}", report.total_debt())?;

    Ok(())
}
