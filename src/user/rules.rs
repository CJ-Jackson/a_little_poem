use crate::user::locale::PasswordEntropyLocale;
use cjtoolkit_structured_validator::common::locale::ValidateErrorCollector;
use cjtoolkit_structured_validator::common::validation_check::ValidationCheck;
use cjtoolkit_structured_validator::types::password::{Password, PasswordError, PasswordRules};
use cjtoolkit_structured_validator::types::username::{
    IsUsernameTakenAsync, Username, UsernameError, UsernameRules,
};
use paspio::entropy;

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
        if let Ok(password_ref) = password.as_ref() {
            return match PasswordStatus::check(password_ref, password_confirm) {
                PasswordStatus::ConfirmOk(password_confirm) => (password, Ok(password_confirm)),
                PasswordStatus::ConfirmError(err) => (password, Err(err)),
                PasswordStatus::EntropyError(err) => (Err(err), Err(PasswordError::default())),
            };
        }
        (password, Err(PasswordError::default()))
    }

    fn parse_user_login(s: Option<&str>) -> Result<Password, PasswordError> {
        Password::parse_custom(s, password_rules_for_login())
    }
}

enum PasswordStatus {
    ConfirmError(PasswordError),
    ConfirmOk(Password),
    EntropyError(PasswordError),
}

impl PasswordStatus {
    fn check(password: &Password, password_confirm: &str) -> Self {
        if let Err(err) = Self::check_password_entropy(password) {
            return Self::EntropyError(err);
        }
        match password.parse_confirm(password_confirm) {
            Ok(ok) => Self::ConfirmOk(ok),
            Err(err) => Self::ConfirmError(err),
        }
    }

    const PASSWORD_ENTROPY_MIN: f64 = 60.0;

    fn check_password_entropy(password: &Password) -> Result<(), PasswordError> {
        let mut messages = ValidateErrorCollector::new();
        if entropy(password.as_str()) < Self::PASSWORD_ENTROPY_MIN {
            messages.push((
                format!(
                    "Password entropy score must be over {}",
                    Self::PASSWORD_ENTROPY_MIN
                ),
                Box::new(PasswordEntropyLocale(Self::PASSWORD_ENTROPY_MIN)),
            ));
        }
        PasswordError::validate_check(messages)?;
        Ok(())
    }
}
