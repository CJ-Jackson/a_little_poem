use error_stack::{Report, ResultExt};
use maud::{Markup, html};
use poem::error::ResponseError;
use poem::http::StatusCode;
use poem::web::{CsrfToken, CsrfVerifier, Json};
use poem::{FromRequest, IntoResponse, Request, RequestBody, Response, handler};
use serde_json::{Value, json};
use std::error::Error;
use thiserror::Error;

pub const CSRF_PATH: &'static str = "/csrf/";

pub trait CsrfTokenHtml {
    fn as_html(&self) -> Markup;
}

impl CsrfTokenHtml for CsrfToken {
    fn as_html(&self) -> Markup {
        html! {
            input type="hidden" name="csrf_token" value=(self.0);
        }
    }
}

#[derive(Debug, Error)]
#[error("csrf error")]
pub struct CsrfError;

impl ResponseError for CsrfError {
    fn status(&self) -> StatusCode {
        StatusCode::FORBIDDEN
    }

    fn as_response(&self) -> Response
    where
        Self: Error + Send + Sync + 'static,
    {
        ().with_status(self.status()).into_response()
    }
}

pub trait CsrfVerifierError {
    fn verify(&self, token: &str) -> Result<(), Report<CsrfError>>;
}

impl CsrfVerifierError for CsrfVerifier {
    fn verify(&self, token: &str) -> Result<(), Report<CsrfError>> {
        self.validate(token).change_context(CsrfError)
    }
}

pub struct CsrfHeaderChecker;

impl<'a> FromRequest<'a> for CsrfHeaderChecker {
    async fn from_request(req: &'a Request, _body: &mut RequestBody) -> poem::Result<Self> {
        let token = req.header("X-Csrf-Token").ok_or_else(|| CsrfError)?;

        match req.data::<CsrfVerifier>() {
            None => Ok(Self),
            Some(csrf_verifier) => {
                if csrf_verifier.is_valid(token) {
                    Ok(Self)
                } else {
                    Err(CsrfError.into())
                }
            }
        }
    }
}

#[handler]
async fn fetch_csrf_token(token: &CsrfToken) -> Json<Value> {
    Json(json!({"token": token.0}))
}

pub fn route_csrf() -> poem::Route {
    poem::Route::new().at("/token", fetch_csrf_token)
}
