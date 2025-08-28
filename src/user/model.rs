use crate::user::validate::password::{Password, PasswordError};
use crate::user::validate::username::{Username, UsernameError};
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

pub struct UserLoginFormValidated {
    pub username: Username,
    pub password: Password,
}

pub struct UserLoginFormValidationError {
    pub username: Result<Username, UsernameError>,
    pub password: Result<Password, PasswordError>,
    pub password_confirm: Result<Password, PasswordError>,
}

impl Into<UserLoginFormValidationErrorMessage> for UserLoginFormValidationError {
    fn into(self) -> UserLoginFormValidationErrorMessage {
        UserLoginFormValidationErrorMessage {
            username: self.username.err().map(|e| e.0).unwrap_or_default(),
            password: self.password.err().map(|e| e.0).unwrap_or_default(),
            password_confirm: self.password_confirm.err().map(|e| e.0).unwrap_or_default(),
        }
    }
}

#[derive(Debug, Default)]
pub struct UserLoginFormValidationErrorMessage {
    pub username: Arc<[String]>,
    pub password: Arc<[String]>,
    pub password_confirm: Arc<[String]>,
}
