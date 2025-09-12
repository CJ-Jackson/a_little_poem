use cjtoolkit_structured_validator::types::password::{Password, PasswordError, PasswordRules};
use cjtoolkit_structured_validator::types::username::{
    IsUsernameTakenAsync, Username, UsernameError, UsernameRules,
};

#[inline]
fn username_rules_for_login() -> UsernameRules {
    UsernameRules {
        is_mandatory: true,
        min_length: None,
        max_length: None,
    }
}

#[inline]
fn password_rules_for_login() -> PasswordRules {
    PasswordRules {
        is_mandatory: true,
        must_have_uppercase: false,
        must_have_lowercase: false,
        must_have_special_chars: false,
        must_have_digit: false,
        min_length: None,
        max_length: Some(64),
    }
}

pub trait UsernameRulesExt {
    fn parse_user_register<T: IsUsernameTakenAsync>(
        s: Option<&str>,
        service: &T,
    ) -> impl Future<Output = Result<Username, UsernameError>>;
    fn parse_user_login(s: Option<&str>) -> Result<Username, UsernameError>;
}

impl UsernameRulesExt for Username {
    async fn parse_user_register<T: IsUsernameTakenAsync>(
        s: Option<&str>,
        service: &T,
    ) -> Result<Username, UsernameError> {
        let mut username = Username::parse(s);
        if let Ok(username_ref) = username.as_ref() {
            username = username_ref.check_username_taken_async(service).await;
        }
        username
    }

    fn parse_user_login(s: Option<&str>) -> Result<Username, UsernameError> {
        Username::parse_custom(s, username_rules_for_login())
    }
}

pub type PasswordTuple = (
    Result<Password, PasswordError>,
    Result<Password, PasswordError>,
);

pub trait PasswordRulesExt {
    fn parse_user_register(password: Option<&str>, password_confirm: &str) -> PasswordTuple;
    fn parse_user_login(s: Option<&str>) -> Result<Password, PasswordError>;
}

impl PasswordRulesExt for Password {
    fn parse_user_register(password: Option<&str>, password_confirm: &str) -> PasswordTuple {
        let password = Password::parse(password);
        let password_confirm = if let Ok(password_ref) = password.as_ref() {
            password_ref.parse_confirm(password_confirm)
        } else {
            password.clone()
        };
        (password, password_confirm)
    }

    fn parse_user_login(s: Option<&str>) -> Result<Password, PasswordError> {
        Password::parse_custom(s, password_rules_for_login())
    }
}
