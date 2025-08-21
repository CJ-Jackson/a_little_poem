use crate::common::context::user::UserDep;
use crate::common::flash::{Flash, FlashMessage};
use crate::common::html::context_html::ContextHtmlBuilder;
use crate::user::flag::{LoginFlag, LogoutFlag};
use crate::user::form::UserRegisterForm;
use crate::user::service::{UserLoginService, UserRegisterService};
use maud::{Markup, html};
use poem::session::Session;
use poem::web::cookie::{Cookie, CookieJar};
use poem::web::{Form, Redirect};
use poem::{IntoResponse, Route, get, handler};
use serde::Deserialize;
use std::time::Duration;

#[handler]
async fn display_user(context_html_builder: UserDep<ContextHtmlBuilder>) -> Markup {
    let title = if context_html_builder.1.is_user {
        format!("User: {}", context_html_builder.1.username)
    } else {
        "Visitor".to_string()
    };

    context_html_builder
        .0
        .attach_title(title.as_str())
        .set_current_tag("user")
        .attach_content(html! {
            h1 .mt-3 { (title) }
            p { "Welcome to the user page!" }
            @if context_html_builder.1.is_user {
                p { "You are logged in as a user '" (context_html_builder.1.username) "'." }
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
async fn login(context_html_builder: UserDep<ContextHtmlBuilder>) -> Markup {
    let title = "Login".to_string();
    context_html_builder
        .0
        .attach_title(title.as_str())
        .attach_content(html! {
            h1 .mt-3 { (title) }
            form method="post" .form {
                input .form-item type="text" name="username" placeholder="Username";
                input .form-item type="password" name="password" placeholder="Password";
                button .btn .btn-sky-blue .mt-3 type="submit" { "Login" };
            }
            p { "If you don't have an account, you can register by clicking the button below." }
            a .btn .btn-sky-blue .mt-3 href="/user/register/" { "Register" }
        })
        .build()
}

#[derive(Deserialize)]
struct UserLoginForm {
    pub username: String,
    pub password: String,
}

#[handler]
async fn login_post(
    data: Form<UserLoginForm>,
    user_login: UserDep<UserLoginService, LoginFlag>,
    session: &Session,
    cookie: &CookieJar,
) -> Redirect {
    let token = user_login
        .0
        .validate_login(data.username.clone(), data.password.clone());
    if let Some(token) = token {
        let mut new_cookie = Cookie::new_with_str("login-token", token);
        new_cookie.set_path("/");
        // 30 days
        new_cookie.set_max_age(Duration::from_secs(30 * 24 * 60 * 60));

        cookie.add(new_cookie);
        session.flash(Flash::Success {
            msg: "Login succeeded".to_string(),
        });
        return Redirect::see_other("/user/");
    }

    session.flash(Flash::Error {
        msg: "Login failed".to_string(),
    });
    Redirect::see_other("/user/login/")
}

#[handler]
async fn logout(
    user_login_service: UserDep<UserLoginService, LogoutFlag>,
    session: &Session,
    cookie: &CookieJar,
) -> Redirect {
    user_login_service.0.logout();
    cookie.remove("login-token");
    session.flash(Flash::Success {
        msg: "Logout succeeded".to_string(),
    });
    Redirect::see_other("/user/")
}

#[handler]
async fn register(context_html_builder: UserDep<ContextHtmlBuilder, LoginFlag>) -> Markup {
    UserRegisterForm::html_form("Register".to_string(), &context_html_builder.0, None, None)
}

enum RegisterPostResponse {
    Redirect(Redirect),
    Markup(Markup),
}

impl IntoResponse for RegisterPostResponse {
    fn into_response(self) -> poem::Response {
        match self {
            RegisterPostResponse::Redirect(redirect) => redirect.into_response(),
            RegisterPostResponse::Markup(markup) => markup.into_response(),
        }
    }
}

#[handler]
async fn register_post(
    data: Form<UserRegisterForm>,
    user_register_service: UserDep<UserRegisterService, LoginFlag>,
    context_html_builder: UserDep<ContextHtmlBuilder>,
    session: &Session,
) -> RegisterPostResponse {
    let validated_data = data.as_validated(&user_register_service.0).await;
    match validated_data {
        Ok(data) => {
            if user_register_service.0.register_user(
                data.username.as_str().to_string(),
                data.password.as_str().to_string(),
            ) {
                session.flash(Flash::Success {
                    msg: "Register succeeded".to_string(),
                });
                RegisterPostResponse::Redirect(Redirect::see_other("/user/login/"))
            } else {
                session.flash(Flash::Error {
                    msg: "Register failed".to_string(),
                });
                RegisterPostResponse::Redirect(Redirect::see_other("/user/register/"))
            }
        }
        Err(err) => RegisterPostResponse::Markup(UserRegisterForm::html_form(
            "Register".to_string(),
            &context_html_builder.0,
            Some(data.0.clone()),
            Some(err.as_map()),
        )),
    }
}

pub fn route_user() -> Route {
    Route::new()
        .at("/", get(display_user))
        .at("/login/", get(login).post(login_post))
        .at("/logout/", get(logout))
        .at("/register/", get(register).post(register_post))
}
