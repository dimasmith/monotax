//! Render plaintext payments report.

use std::io::Write;

use super::PaymentReport;

/// Render plaintext report on payments.
pub fn plaintext_report<W>(report: &PaymentReport, mut writer: W) -> anyhow::Result<()>
where
    W: Write,
{
    writeln!(&mut writer, "No\tDate\t\tIncome\t\tTax\t\tPaid")?;
    for (i, payment) in report.payments().iter().enumerate() {
        let date = payment.income().date();
        let income_amount = payment.income().amount();
        let tax_amount = payment.tax_amount();
        let paid = if payment.paid() { "Yes" } else { "No" };

        writeln!(
            &mut writer,
            "{}:\t{}\t{:10.2}\t{:10.2}\t{}",
            i + 1,
            date,
            income_amount,
            tax_amount,
            paid
        )?;
    }
    writeln!(&mut writer, "{}", "-".repeat(40))?;
    writeln!(&mut writer, "\t\t\tTax debt:\t{:10.2}", report.total_debt())?;

    Ok(())
}
