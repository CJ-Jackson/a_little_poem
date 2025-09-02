use crate::common::adapter::unified;
use crate::common::context::user::{JustDep, UserDep};
use crate::common::cookie_builder::CookieBuilderExt;
use crate::common::csrf::{CsrfError, CsrfTokenHtml, CsrfVerifierError};
use crate::common::flash::{Flash, FlashMessage};
use crate::common::html::context_html::ContextHtmlBuilder;
use crate::user::flag::{LoginFlag, LogoutFlag};
use crate::user::form::{UserLoginForm, UserLoginFormResult, UserRegisterForm};
use crate::user::service::{UserLoginService, UserRegisterService};
use chrono::TimeDelta;
use error_stack::Report;
use maud::{Markup, html};
use poem::error::ResponseError;
use poem::http::StatusCode;
use poem::i18n::Locale;
use poem::session::Session;
use poem::web::cookie::{Cookie, CookieJar};
use poem::web::{CsrfToken, CsrfVerifier, Form, Redirect};
use poem::{IntoResponse, Route, get, handler};

#[handler]
async fn display_user(
    UserDep(context_html_builder, user, _): UserDep<ContextHtmlBuilder>,
) -> Markup {
    let title = if user.is_user {
        format!("User: {}", user.username)
    } else {
        "Visitor".to_string()
    };

    context_html_builder
        .attach_title(title.as_str())
        .set_current_tag("user")
        .attach_content(html! {
            h1 .mt-3 { (title) }
            p { "Welcome to the user page!" }
            @if user.is_user {
                p { "You are logged in as a user '" (user.username) "'." }
                p { "You can log out by clicking the button below." }
                a .btn .btn-sky-blue .mt-3 href="/user/logout/" { "Log out" }
            } @else {
                p { "You are logged in as a visitor." }
                p { "You can log in as a user by clicking the button below." }
                a .btn .btn-sky-blue .mt-3 href="/user/login/" { "Log in as a user" }
            }
        })
        .build()
}

#[handler]
async fn login(
    JustDep(context_html_builder, _): JustDep<ContextHtmlBuilder>,
    csrf_token: &CsrfToken,
) -> Markup {
    let title = "Login".to_string();
    context_html_builder
        .attach_title(title.as_str())
        .attach_content(html! {
            h1 .mt-3 { (title) }
            form method="post" .form {
                (csrf_token.as_html())
                input .form-item type="text" name="username" placeholder="Username";
                input .form-item type="password" name="password" placeholder="Password";
                button .btn .btn-sky-blue .mt-3 type="submit" { "Login" };
            }
            p { "If you don't have an account, you can register by clicking the button below." }
            a .btn .btn-sky-blue .mt-3 href="/user/register/" { "Register" }
        })
        .build()
}

enum LoginPostResponse {
    Redirect(Redirect),
    Csrf(Report<CsrfError>),
}

impl IntoResponse for LoginPostResponse {
    fn into_response(self) -> poem::Response {
        match self {
            Self::Redirect(redirect) => redirect.into_response(),
            Self::Csrf(csrf) => csrf.current_context().as_response(),
        }
    }
}

#[handler]
async fn login_post(
    JustDep(user_login, _): JustDep<UserLoginService, LoginFlag>,
    Form(data): Form<UserLoginForm>,
    session: &Session,
    cookie_jar: &CookieJar,
    csrf_verifier: &CsrfVerifier,
) -> LoginPostResponse {
    unified(async {
        csrf_verifier
            .verify(data.csrf_token.as_str())
            .map_err(|err| LoginPostResponse::Csrf(err))?;
        if let UserLoginFormResult(Ok(data)) = data.as_validated() {
            let token = user_login.validate_login(
                data.username.as_str().to_string(),
                data.password.as_str().to_string(),
            );
            if let Some(token) = token {
                let new_cookie = Cookie::new_with_str("login-token", token)
                    .into_builder()
                    .path("/")
                    .expires_by_delta(TimeDelta::days(30))
                    .build();

                cookie_jar.add(new_cookie);
                session.flash(Flash::Success {
                    msg: "Login succeeded".to_string(),
                });
                return Ok(LoginPostResponse::Redirect(Redirect::see_other("/user/")));
            }
        }

        session.flash(Flash::Error {
            msg: "Login failed".to_string(),
        });
        Err(LoginPostResponse::Redirect(Redirect::see_other(
            "/user/login/",
        )))
    })
    .await
}

#[handler]
async fn logout(
    JustDep(user_login_service, _): JustDep<UserLoginService, LogoutFlag>,
    session: &Session,
    cookie: &CookieJar,
) -> Redirect {
    user_login_service.logout();
    cookie.remove("login-token");
    session.flash(Flash::Success {
        msg: "Logout succeeded".to_string(),
    });
    Redirect::see_other("/user/")
}

#[handler]
async fn register(
    JustDep(context_html_builder, _): JustDep<ContextHtmlBuilder, LoginFlag>,
    csrf_token: &CsrfToken,
) -> Markup {
    UserRegisterForm::html_form(
        "Register".to_string(),
        &context_html_builder,
        None,
        None,
        Some(csrf_token.as_html()),
    )
}

enum RegisterPostResponse {
    Redirect(Redirect),
    MarkupValidationError(Markup),
    Csrf(Report<CsrfError>),
}

impl IntoResponse for RegisterPostResponse {
    fn into_response(self) -> poem::Response {
        match self {
            Self::Redirect(redirect) => redirect.into_response(),
            Self::MarkupValidationError(markup) => markup
                .with_status(StatusCode::UNPROCESSABLE_ENTITY)
                .into_response(),
            Self::Csrf(csrf) => csrf.current_context().as_response(),
        }
    }
}

#[handler]
async fn register_post(
    JustDep(user_register_service, _): JustDep<UserRegisterService, LoginFlag>,
    Form(data): Form<UserRegisterForm>,
    JustDep(context_html_builder, _): JustDep<ContextHtmlBuilder>,
    session: &Session,
    csrf_verifier: &CsrfVerifier,
    csrf_token: &CsrfToken,
    locale: Locale,
) -> RegisterPostResponse {
    unified(async {
        csrf_verifier
            .verify(data.csrf_token.as_str())
            .map_err(|err| RegisterPostResponse::Csrf(err))?;
        let validated_data = data
            .as_validated()
            .check_username_taken(&user_register_service)
            .await;
        match validated_data.0 {
            Ok(data) => {
                if user_register_service.register_user(
                    data.username.as_str().to_string(),
                    data.password.as_str().to_string(),
                ) {
                    session.flash(Flash::Success {
                        msg: "Register succeeded".to_string(),
                    });
                    Ok(RegisterPostResponse::Redirect(Redirect::see_other(
                        "/user/login/",
                    )))
                } else {
                    session.flash(Flash::Error {
                        msg: "Register failed".to_string(),
                    });
                    Err(RegisterPostResponse::Redirect(Redirect::see_other(
                        "/user/register/",
                    )))
                }
            }
            Err(err) => Err(RegisterPostResponse::MarkupValidationError(
                UserRegisterForm::html_form(
                    "Register".to_string(),
                    &context_html_builder,
                    Some(data),
                    Some((err, &locale).into()),
                    Some(csrf_token.as_html()),
                ),
            )),
        }
    })
    .await
}

pub fn route_user() -> Route {
    Route::new()
        .at("/", get(display_user))
        .at("/login/", get(login).post(login_post))
        .at("/logout/", get(logout))
        .at("/register/", get(register).post(register_post))
}
