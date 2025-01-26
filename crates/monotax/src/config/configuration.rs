use monotax_core::domain::config::TaxConfig;
use monotax_core::integration::taxer::TaxerImportConfig;
use monotax_sqlite::configuration::DatabaseConfiguration;
use serde::{Deserialize, Serialize};

/// Application configuration.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Configuration {
    taxer: TaxerImportConfig,
    tax: TaxConfig,
    pub database: DatabaseConfiguration,
}

impl Configuration {
    pub fn taxer(&self) -> &TaxerImportConfig {
        &self.taxer
    }

    pub fn tax(&self) -> &TaxConfig {
        &self.tax
    }
}
