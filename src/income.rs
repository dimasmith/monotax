use chrono::NaiveDate;

pub trait Income {
    fn tax_number(&self) -> String;
    fn date(&self) -> NaiveDate;
    fn amount(&self) -> f64;
    fn comment(&self) -> String;
}
