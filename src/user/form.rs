use crate::common::adapter::unified;
use crate::common::html::context_html::ContextHtmlBuilder;
use crate::common::validation::{arc_string_to_html, error_flag};
use crate::user::model::{
    UserLoginFormValidated, UserLoginFormValidationError, UserLoginFormValidationErrorMessage,
    UserRegisterFormValidated,
};
use crate::user::validate::password::Password;
use crate::user::validate::username::{IsUsernameTaken, Username};
use maud::{Markup, html};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct UserRegisterForm {
    pub username: String,
    pub password: String,
    pub password_confirm: String,
    pub csrf_token: String,
}
impl Into<UserRegisterFormResult> for UserRegisterForm {
    fn into(self) -> UserRegisterFormResult {
        UserRegisterFormResult((|| {
            let mut flag = false;
            let default_password = Password::default();

            use error_flag as ef;
            let username = ef(&mut flag, Username::parse(self.username.clone()));
            let password = ef(&mut flag, Password::parse(self.password.clone()));
            let password_confirm = ef(
                &mut flag,
                password
                    .as_ref()
                    .ok()
                    .unwrap_or(&default_password)
                    .parse_confirm(self.password_confirm.clone()),
            );

            if flag {
                return Err(UserLoginFormValidationError {
                    username,
                    password,
                    password_confirm,
                });
            }

            Ok(UserRegisterFormValidated {
                username: username.unwrap_or_default(),
                password: password.unwrap_or_default(),
                password_confirm: password_confirm.unwrap_or_default(),
            })
        })())
    }
}

pub struct UserRegisterFormResult(
    pub Result<UserRegisterFormValidated, UserLoginFormValidationError>,
);

impl UserRegisterFormResult {
    pub async fn check_username_taken<T: IsUsernameTaken>(
        self,
        service: &T,
    ) -> UserRegisterFormResult {
        let current: UserLoginFormValidationError =
            unified(async { self.0.map(|v| v.into()) }).await;
        if current.username.is_err() {
            return Self(Err(current));
        }
        let current_clone = current.clone();
        let username = current
            .username
            .unwrap_or_default()
            .check_if_username_taken(service)
            .await;
        if let Err(username) = username {
            return Self(Err(UserLoginFormValidationError {
                username: Err(username),
                password: current.password,
                password_confirm: current.password_confirm,
            }));
        } else if current_clone.has_error() {
            return Self(Err(current_clone));
        }
        Self(Ok(current_clone.into()))
    }
}

impl UserRegisterForm {
    pub fn as_validated(&self) -> UserRegisterFormResult {
        self.clone().into()
    }

    pub fn html_form(
        title: String,
        context_html_builder: &ContextHtmlBuilder,
        user_register_form: Option<UserRegisterForm>,
        errors: Option<UserLoginFormValidationErrorMessage>,
        token: Option<Markup>,
    ) -> Markup {
        let user_register_form = user_register_form.unwrap_or_default();
        let errors = errors.unwrap_or_default();
        let token = token.unwrap_or_default();
        context_html_builder
            .attach_title(title.as_str())
            .attach_content(html! {
                h1 .mt-3 { (title) }
                form method="post" .form {
                    (token)
                    input .form-item type="text" name="username" placeholder="Username" value=(user_register_form.username);
                    (arc_string_to_html(errors.username))
                    input .form-item type="password" name="password" placeholder="Password";
                    (arc_string_to_html(errors.password))
                    input .form-item type="password" name="password_confirm" placeholder="Confirm password";
                    (arc_string_to_html(errors.password_confirm))
                    button .btn .btn-sky-blue .mt-3 type="submit" { "Register" };
                }
            })
            .build()
    }
}

#[derive(Deserialize)]
pub struct UserLoginForm {
    pub username: String,
    pub password: String,
    pub csrf_token: String,
}

impl UserLoginForm {
    pub fn as_validated(&self) -> Result<UserLoginFormValidated, UserLoginFormValidationError> {
        let mut flag = false;

        use error_flag as ef;
        let username = ef(&mut flag, Username::parse_login(self.username.clone()));
        let password = ef(&mut flag, Password::parse_login(self.password.clone()));

        if flag {
            return Err(UserLoginFormValidationError {
                username,
                password,
                password_confirm: Ok(Password::default()),
            });
        }

        Ok(UserLoginFormValidated {
            username: username.unwrap_or_default(),
            password: password.unwrap_or_default(),
        })
    }
}
