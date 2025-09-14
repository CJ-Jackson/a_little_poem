use crate::bucket_list::route::{BUCKET_LIST_PATH, route_bucket_list};
use crate::common::cache_local::init_cache_local;
use crate::common::config::Config;
use crate::common::csrf::{CSRF_PATH, route_csrf};
use crate::common::embed::{AssetFilesEndpoint, EMBED_PATH};
use crate::common::locale::build_resources;
use crate::home::route_home_page;
use crate::user::model::UserIdContext;
use crate::user::route::{USER_PATH, route_user};
use error_stack::{Report, ResultExt};
use poem::listener::TcpListener;
use poem::middleware::{CookieJarManager, Csrf};
use poem::session::{CookieConfig, CookieSession};
use poem::{EndpointExt, Server};
use std::sync::Arc;
use thiserror::Error;

pub mod bucket_list;
pub mod common;
pub mod home;
pub mod user;

#[derive(Debug, Error)]
pub enum MainError {
    #[error("Config error")]
    ConfigError,
    #[error("IO error")]
    IoError,
    #[error("Locale error")]
    LocaleError,
}

#[tokio::main]
async fn main() -> Result<(), Report<MainError>> {
    let config = Config::fetch()
        .await
        .change_context(MainError::ConfigError)?;

    let route = route_home_page();

    let route = route
        .nest(BUCKET_LIST_PATH, route_bucket_list())
        .nest(USER_PATH, route_user())
        .nest(CSRF_PATH, route_csrf())
        .nest(EMBED_PATH, AssetFilesEndpoint::new());

    let route = route
        .with(CookieJarManager::new())
        .with(CookieSession::new(CookieConfig::new()))
        .with(Csrf::new())
        .data(build_resources().change_context(MainError::LocaleError)?)
        .around(init_cache_local::<Arc<UserIdContext>, _>);

    match config.upgrade() {
        Some(config) => {
            println!("Listening on http://{}", config.poem.parse_address());
            Server::new(TcpListener::bind(config.poem.parse_address().as_str()))
                .run(route)
                .await
                .change_context(MainError::IoError)
        }
        None => Err(Report::new(MainError::ConfigError)),
    }
}
