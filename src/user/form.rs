use crate::common::html::context_html::ContextHtmlBuilder;
use crate::common::html::validate::arc_string_to_html;
use crate::user::model::{
    UserLoginFormValidated, UserLoginFormValidationError, UserLoginFormValidationErrorMessage,
    UserRegisterFormValidated,
};
use crate::user::rules::{password_rules_for_login, username_rules_for_login};
use cjtoolkit_structured_validator::common::flag_error::flag_error;
use cjtoolkit_structured_validator::types::password::Password;
use cjtoolkit_structured_validator::types::username::{IsUsernameTakenAsync, Username};
use maud::{Markup, html};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct UserRegisterForm {
    pub username: String,
    pub password: String,
    pub password_confirm: String,
    pub csrf_token: String,
}

pub struct UserRegisterFormResult(
    pub Result<UserRegisterFormValidated, UserLoginFormValidationError>,
);

impl UserRegisterFormResult {
    async fn new<T: IsUsernameTakenAsync>(form: UserRegisterForm, service: &T) -> Self {
        Self(
            async {
                let mut flag = false;
                let default_password = Password::default();

                use flag_error as fe;
                let mut username = fe(
                    &mut flag,
                    Username::parse(Some(form.username.clone().as_str())),
                );
                if username.is_ok() {
                    username = fe(
                        &mut flag,
                        username
                            .unwrap_or_default()
                            .check_username_taken_async(service)
                            .await,
                    );
                }
                let password = fe(
                    &mut flag,
                    Password::parse(Some(form.password.clone().as_str())),
                );
                let password_confirm = fe(
                    &mut flag,
                    password
                        .as_ref()
                        .ok()
                        .unwrap_or(&default_password)
                        .parse_confirm(form.password_confirm.clone().as_str()),
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
            }
            .await,
        )
    }
}

impl UserRegisterForm {
    pub async fn as_validated<T: IsUsernameTakenAsync>(
        &self,
        service: &T,
    ) -> UserRegisterFormResult {
        UserRegisterFormResult::new(self.clone(), service).await
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
                Username::parse_custom(Some(self.username.as_str()), username_rules_for_login()),
            );
            let password = fe(
                &mut flag,
                Password::parse_custom(Some(self.password.as_str()), password_rules_for_login()),
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
}
