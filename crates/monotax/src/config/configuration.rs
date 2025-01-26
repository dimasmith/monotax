use monotax_core::integration::taxer::TaxerImportConfig;
use monotax_sqlite::configuration::DatabaseConfiguration;
use serde::{Deserialize, Serialize};

/// Application configuration.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Configuration {
    taxer: TaxerImportConfig,
    pub database: DatabaseConfiguration,
}

impl Configuration {
    pub fn taxer(&self) -> &TaxerImportConfig {
        &self.taxer
    }

    pub fn database(&self) -> &DatabaseConfiguration {
        &self.database
    }
}
