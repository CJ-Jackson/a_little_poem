use crate::common::config::poem::PoemConfig;
use error_stack::{Report, ResultExt};
use figment::providers::{Format, Serialized, Toml};
use figment::{Figment, Profile};
use serde::{Deserialize, Serialize};
use sqlite::SqliteConfig;
use std::env::var;
use std::sync::{Arc, Weak};
use thiserror::Error;
use tokio::sync::OnceCell;

pub mod poem;
pub mod sqlite;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Config did not parse")]
    ParseError,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub poem: Arc<PoemConfig>,
    pub sqlite: Arc<SqliteConfig>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            poem: Arc::new(PoemConfig::default()),
            sqlite: Arc::new(SqliteConfig::default()),
        }
    }
}

static CONFIG_CACHE: OnceCell<Arc<Config>> = OnceCell::const_new();

impl Config {
    fn build_figment() -> Figment {
        Figment::new()
            .merge(Serialized::defaults(Config::default()))
            .merge(Toml::file("little_poem.toml").nested())
            .merge(
                Toml::file(
                    var("LITTLE_POEM_CONFIG_PATH")
                        .unwrap_or_else(|_| "little_poem.local.toml".to_string()),
                )
                .nested(),
            )
            .select(Profile::from_env_or("LITTLE_POEM_PROFILE", "default"))
    }

    fn parse() -> Result<Self, Report<ConfigError>> {
        Self::build_figment()
            .extract::<Self>()
            .change_context(ConfigError::ParseError)
    }

    pub async fn fetch() -> Result<Weak<Config>, Report<ConfigError>> {
        let config: Result<&Arc<Config>, Report<ConfigError>> = CONFIG_CACHE
            .get_or_try_init(|| async {
                let config = Self::parse()?;
                Ok(Arc::new(config))
            })
            .await;

        Ok(Arc::downgrade(config?))
    }
}
