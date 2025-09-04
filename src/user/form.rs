use crate::common::html::context_html::ContextHtmlBuilder;
use crate::common::html::validate::arc_string_to_html;
use crate::user::model::{
    UserLoginFormValidated, UserLoginFormValidationError, UserLoginFormValidationErrorMessage,
    UserRegisterFormValidated,
};
use cjtoolkit_structured_validator::common::flag_error::flag_error;
use cjtoolkit_structured_validator::types::password::{Password, PasswordError, PasswordRules};
use cjtoolkit_structured_validator::types::username::{
    IsUsernameTakenAsync, Username, UsernameError, UsernameRules,
};
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

            use flag_error as fe;
            let username = fe(
                &mut flag,
                Username::parse(Some(self.username.clone().as_str())),
            );
            let password = fe(
                &mut flag,
                Password::parse(Some(self.password.clone().as_str())),
            );
            let password_confirm = fe(
                &mut flag,
                password
                    .as_ref()
                    .ok()
                    .unwrap_or(&default_password)
                    .parse_confirm(self.password_confirm.clone().as_str()),
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
    fn into_error(self) -> UserLoginFormValidationError {
        self.0.map(|v| v.into()).unwrap_or_else(|e| e)
    }

    pub async fn check_username_taken<T: IsUsernameTakenAsync>(
        self,
        service: &T,
    ) -> UserRegisterFormResult {
        let is_error = self.0.is_err();
        let current = self.into_error();
        if current.username.is_err() {
            return Self(Err(current));
        }
        let current_clone = current.clone();
        let username = current
            .username
            .unwrap_or_default()
            .check_username_taken_async(service)
            .await;
        if let Err(username) = username {
            return Self(Err(UserLoginFormValidationError {
                username: Err(username),
                password: current.password,
                password_confirm: current.password_confirm,
            }));
        } else if is_error {
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

#[derive(Deserialize, Clone)]
pub struct UserLoginForm {
    pub username: String,
    pub password: String,
    pub csrf_token: String,
}

pub struct UserLoginFormResult(pub Result<UserLoginFormValidated, UserLoginFormValidationError>);

impl Into<UserLoginFormResult> for UserLoginForm {
    fn into(self) -> UserLoginFormResult {
        UserLoginFormResult((|| {
            let mut flag = false;

            use flag_error as fe;
            let username = fe(
                &mut flag,
                UserLoginForm::username_parse_login(self.username.clone().as_str()),
            );
            let password = fe(
                &mut flag,
                UserLoginForm::password_parse_login(self.password.clone().as_str()),
            );

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
        })())
    }
}

impl UserLoginForm {
    pub fn as_validated(&self) -> UserLoginFormResult {
        self.clone().into()
    }

    pub fn username_parse_login(s: &str) -> Result<Username, UsernameError> {
        Username::parse_custom(
            Some(s),
            UsernameRules {
                is_mandatory: true,
                min_length: None,
                max_length: None,
            },
        )
    }

    pub fn password_parse_login(s: &str) -> Result<Password, PasswordError> {
        Password::parse_custom(
            Some(s),
            PasswordRules {
                is_mandatory: true,
                must_have_uppercase: false,
                must_have_lowercase: false,
                must_have_special_chars: false,
                must_have_digit: false,
                min_length: None,
                max_length: Some(64),
            },
        )
    }
}
