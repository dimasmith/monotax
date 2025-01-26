use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct DatabaseConfiguration {
    pub url: String,
}

impl DatabaseConfiguration {
    pub fn connection_url(&self) -> String {
        // TODO: accept file paths in additon to URLs.
        self.url.clone()
    }
}
impl Default for DatabaseConfiguration {
    fn default() -> Self {
        Self {
            url: "sqlite:monotax.db".to_string(),
        }
    }
}
