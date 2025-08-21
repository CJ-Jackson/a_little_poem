use crate::bucket_list::route::route_bucket_list;
use crate::common::cache_local::init_cache_local;
use crate::common::config::Config;
use crate::common::html::css::main_css;
use crate::home::{favicon, home_page};
use crate::user::model::UserIdContext;
use crate::user::route::route_user;
use error_stack::{Report, ResultExt};
use poem::listener::TcpListener;
use poem::middleware::CookieJarManager;
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
}

#[tokio::main]
async fn main() -> Result<(), Report<MainError>> {
    let config = Config::fetch()
        .await
        .change_context(MainError::ConfigError)?;

    let route = Route::new()
        .at("/", get(home_page))
        .at("/favicon.ico", get(favicon))
        .at("/main.css", get(main_css));

    let route = route
        .nest("/bucket-list/", route_bucket_list())
        .nest("/user/", route_user());

    let route = route
        .with(CookieJarManager::new())
        .with(CookieSession::new(CookieConfig::new()))
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
