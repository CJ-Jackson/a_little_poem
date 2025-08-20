use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PoemConfig {
    pub address: String,
    pub port: u16,
}

impl Default for PoemConfig {
    fn default() -> Self {
        Self {
            address: "127.0.0.1".to_string(),
            port: 8000,
        }
    }
}

impl PoemConfig {
    pub fn parse_address(&self) -> String {
        format!("{}:{}", self.address, self.port)
    }
}
