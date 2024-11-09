use serde::{Deserialize, Serialize};

/// Application configuration.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    taxer: TaxerImportConfig,
    tax: TaxConfig,
}

/// Tax rate configuration. Deprecated.
/// It will be replaced via the proper tax management.
#[derive(Debug, Serialize, Deserialize)]
pub struct TaxConfig {
    tax_rate: f64,
}

/// Configuration for the Taxer import.
///
/// - The `id` is person's national tax identifier.
/// - The `account_name` is the name of the account in the Taxer.
/// - The `default_comment` is a comment that will be used if the income has no comment.
#[derive(Debug, Serialize, Deserialize)]
pub struct TaxerImportConfig {
    id: String,
    account_name: String,
    default_comment: String,
}

impl Config {
    pub fn taxer(&self) -> &TaxerImportConfig {
        &self.taxer
    }

    pub fn tax(&self) -> &TaxConfig {
        &self.tax
    }
}

impl TaxConfig {
    pub fn new(tax_rate: f64) -> Self {
        Self { tax_rate }
    }

    pub fn tax_rate(&self) -> f64 {
        self.tax_rate
    }
}

impl TaxerImportConfig {
    /// Provide the national tax identifier.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Provide the name of the account in the Taxer.
    pub fn account_name(&self) -> &str {
        &self.account_name
    }

    /// Provide the default comment that will be used if the income has no comment.
    pub fn default_comment(&self) -> &str {
        &self.default_comment
    }
}

impl Default for TaxConfig {
    fn default() -> Self {
        Self { tax_rate: 0.05 }
    }
}

impl Default for TaxerImportConfig {
    fn default() -> Self {
        Self {
            id: "1234567890".to_string(),
            account_name: Default::default(),
            default_comment: Default::default(),
        }
    }
}
