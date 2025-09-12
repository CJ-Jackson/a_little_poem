use crate::common::locale::LocaleExtForResult;
use cjtoolkit_structured_validator::types::password::{Password, PasswordError};
use cjtoolkit_structured_validator::types::username::{Username, UsernameError};
use poem::i18n::Locale;
use std::sync::Arc;

#[derive(Debug)]
pub struct UserIdContext {
    pub id: i64,
    pub is_user: bool,
    pub username: String,
}

pub struct IdPassword {
    pub id: i64,
    pub password: Box<[u8]>,
}

pub struct IdUsername {
    pub id: i64,
    pub username: String,
}

pub struct UserRegisterFormValidated {
    pub username: Username,
    pub password: Password,
    pub password_confirm: Password,
}

impl Into<UserLoginFormValidationError> for UserRegisterFormValidated {
    fn into(self) -> UserLoginFormValidationError {
        UserLoginFormValidationError {
            username: Ok(self.username),
            password: Ok(self.password),
            password_confirm: Ok(self.password_confirm),
        }
    }
}

pub struct UserLoginFormValidated {
    pub username: Username,
    pub password: Password,
}

#[derive(Clone)]
pub struct UserLoginFormValidationError {
    pub username: Result<Username, UsernameError>,
    pub password: Result<Password, PasswordError>,
    pub password_confirm: Result<Password, PasswordError>,
}

impl Default for UserLoginFormValidationError {
    fn default() -> Self {
        Self {
            username: Ok(Username::default()),
            password: Ok(Password::default()),
            password_confirm: Ok(Password::default()),
        }
    }
}

impl Into<UserLoginFormValidationErrorMessage> for UserLoginFormValidationError {
    fn into(self) -> UserLoginFormValidationErrorMessage {
        UserLoginFormValidationErrorMessage {
            username: self.username.as_original_message(),
            password: self.password.as_original_message(),
            password_confirm: self.password_confirm.as_original_message(),
        }
    }
}

impl Into<UserLoginFormValidationErrorMessage> for (UserLoginFormValidationError, &Locale) {
    fn into(self) -> UserLoginFormValidationErrorMessage {
        UserLoginFormValidationErrorMessage {
            username: self.0.username.as_translated_message(self.1),
            password: self.0.password.as_translated_message(self.1),
            password_confirm: self.0.password_confirm.as_translated_message(self.1),
        }
    }
}

impl Into<UserRegisterFormValidated> for UserLoginFormValidationError {
    fn into(self) -> UserRegisterFormValidated {
        UserRegisterFormValidated {
            username: self.username.unwrap_or_default(),
            password: self.password.unwrap_or_default(),
            password_confirm: self.password_confirm.unwrap_or_default(),
        }
    }
}

#[derive(Debug, Default)]
pub struct UserLoginFormValidationErrorMessage {
    pub username: Arc<[String]>,
    pub password: Arc<[String]>,
    pub password_confirm: Arc<[String]>,
}
