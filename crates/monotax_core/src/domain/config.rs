use serde::{Deserialize, Serialize};

/// Tax rate configuration. Deprecated.
/// It will be replaced via the proper tax management.
#[derive(Debug, Serialize, Deserialize)]
pub struct TaxConfig {
    tax_rate: f64,
}

impl TaxConfig {
    pub fn new(tax_rate: f64) -> Self {
        Self { tax_rate }
    }

    pub fn tax_rate(&self) -> f64 {
        self.tax_rate
    }
}

impl Default for TaxConfig {
    fn default() -> Self {
        Self { tax_rate: 0.05 }
    }
}
