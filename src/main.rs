use crate::common::config::Config;
use error_stack::{Report, ResultExt};
use poem::listener::TcpListener;
use poem::middleware::CookieJarManager;
use poem::web::Path;
use poem::{EndpointExt, Route, Server, get, handler};
use thiserror::Error;

pub mod common;
pub mod home;

#[handler]
async fn hello_root(Path(name): Path<String>) -> String {
    format!("Hello, {}!", name)
}

#[derive(Debug, Error)]
pub enum MainError {
    #[error("Config error")]
    ConfigError,
    #[error("IO error")]
    IoError,
}

#[tokio::main]
async fn main() -> Result<(), Report<MainError>> {
    let config = Config::fetch()
        .await
        .change_context(MainError::ConfigError)?;

    let app = Route::new().at("/hello/:name", get(hello_root));

    let app = app.with(CookieJarManager::new());

    match config.upgrade() {
        Some(config) => Server::new(TcpListener::bind(config.poem.parse_address().as_str()))
            .run(app)
            .await
            .change_context(MainError::IoError),
        None => Err(Report::new(MainError::ConfigError)),
    }
}
