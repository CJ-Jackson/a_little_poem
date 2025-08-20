use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SqliteConfig {
    pub path: String,
}

impl Default for SqliteConfig {
    fn default() -> Self {
        Self {
            path: "./sqlite.db".to_string(),
        }
    }
}
