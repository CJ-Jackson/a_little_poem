use crate::bucket_list::route::route_bucket_list;
use crate::common::cache_local::init_cache_local;
use crate::common::config::Config;
use crate::common::csrf::route_csrf;
use crate::common::embed::Asset;
use crate::common::locale::build_resources;
use crate::home::{home_page, js_array};
use crate::user::model::UserIdContext;
use crate::user::route::route_user;
use error_stack::{Report, ResultExt};
use poem::endpoint::{EmbeddedFileEndpoint, EmbeddedFilesEndpoint};
use poem::listener::TcpListener;
use poem::middleware::{CookieJarManager, Csrf};
use poem::session::{CookieConfig, CookieSession};
use poem::{EndpointExt, Route, Server, get};
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

    let route = Route::new()
        .at("/", get(home_page))
        .at("/array", get(js_array))
        .at(
            "/favicon.ico",
            EmbeddedFileEndpoint::<Asset>::new("/favicon/favicon.ico"),
        );

    let route = route
        .nest("/bucket-list/", route_bucket_list())
        .nest("/user/", route_user())
        .nest("/csrf/", route_csrf())
        .nest("/assets/", EmbeddedFilesEndpoint::<Asset>::new());

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
