use crate::common::config::Config;
use error_stack::Report;
use poem::http::StatusCode;
use poem::{FromRequest, Request, RequestBody};
use std::sync::Weak;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ContextError {
    #[error("Config error")]
    ConfigError,
    #[error("Request error")]
    RequestError,
    #[error("Other error")]
    Other,
    #[error("Other status code")]
    Status(StatusCode, String),
}

impl ContextError {
    pub fn status_code(&self) -> (StatusCode, String) {
        match self {
            ContextError::ConfigError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Config Error".to_string(),
            ),
            ContextError::RequestError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Request Error".to_string(),
            ),
            ContextError::Other => (StatusCode::INTERNAL_SERVER_ERROR, "Other Error".to_string()),
            ContextError::Status(code, v) => (*code, v.clone()),
        }
    }
}

pub trait FromContext: Sized + Send + Sync {
    fn from_context(
        ctx: &'_ Context,
    ) -> impl Future<Output = Result<Self, Report<ContextError>>> + Send;
}

pub struct Context<'a> {
    pub config: Weak<Config>,
    pub req: &'a Request,
}

impl Context<'_> {
    pub async fn inject<T: FromContext>(&self) -> Result<T, Report<ContextError>> {
        T::from_context(self).await
    }
}

pub struct Dependency<T: FromContext>(pub T);

pub type Dep<T> = Dependency<T>;

impl<'a, T: FromContext> FromRequest<'a> for Dependency<T> {
    async fn from_request(req: &'a Request, _body: &mut RequestBody) -> poem::Result<Self> {
        let config = match Config::fetch().await {
            Ok(config) => config,
            Err(_) => return Err(poem::Error::from_status(StatusCode::INTERNAL_SERVER_ERROR)),
        };
        let context = Box::pin(Context { config, req });
        Ok(Self(T::from_context(&context).await.map_err(|e| {
            let status_code = e.current_context().status_code();
            poem::Error::from_string(status_code.1, status_code.0)
        })?))
    }
}
