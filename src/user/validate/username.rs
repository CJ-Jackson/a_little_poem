use crate::common::validation::{StrValidationExtension, ValidationCheck};
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("Username is invalid")]
pub struct UsernameError(pub Arc<[String]>);

impl ValidationCheck for UsernameError {
    fn validation_check(strings: Vec<String>) -> Result<(), Self> {
        if strings.is_empty() {
            Ok(())
        } else {
            Err(Self(strings.into()))
        }
    }
}

#[derive(Default)]
pub struct Username(String);

impl Username {
    pub fn parse(username: String) -> Result<Self, UsernameError> {
        let mut message: Vec<String> = vec![];
        let username_validator = username.as_string_validator();

        let mut check_count = true;
        username_validator.is_empty().then(|| {
            message.push("Cannot be empty".to_string());
            check_count = false;
        });
        check_count.then(|| {
            (username_validator.count_graphemes() < 5)
                .then(|| message.push("Must be at least 5 characters".to_string()));
            (username_validator.count_graphemes() > 30)
                .then(|| message.push("Must be at most 30 characters".to_string()));
        });

        UsernameError::validation_check(message)?;
        Ok(Self(username))
    }

    pub fn parse_login(username: String) -> Result<Self, UsernameError> {
        let mut message: Vec<String> = vec![];
        let username_validator = username.as_string_validator();

        (username_validator.count_graphemes() > 30)
            .then(|| message.push("Must be at most 30 characters".to_string()));

        UsernameError::validation_check(message)?;
        Ok(Self(username))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

pub trait IsUsernameTaken {
    fn is_username_taken(&self, username: &str) -> impl Future<Output = bool>;
}

trait Sealed {}

#[allow(private_bounds)]
pub trait UsernameCheckResult: Sealed {
    fn check_username_result<T: IsUsernameTaken>(self, service: &T) -> impl Future<Output = Self>;
}

impl Sealed for Result<Username, UsernameError> {}

impl UsernameCheckResult for Result<Username, UsernameError> {
    async fn check_username_result<T: IsUsernameTaken>(self, service: &T) -> Self {
        match self {
            Ok(v) => {
                let mut message: Vec<String> = vec![];

                service.is_username_taken(v.as_str()).await.then(|| {
                    message.push("Already taken".to_string());
                });

                UsernameError::validation_check(message)?;
                Ok(v)
            }
            Err(_) => self,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_username_parse() {
        let username = Username::parse("Hello".to_string());
        assert!(username.is_ok());
    }

    #[test]
    fn test_username_parse_error_empty_string() {
        let username = Username::parse("".to_string());
        assert!(username.is_err());
    }

    #[test]
    fn test_username_parse_error_too_short() {
        let username = Username::parse("a".to_string());
        assert!(username.is_err());
    }

    #[test]
    fn test_username_parse_error_too_long() {
        let username_str = "a".repeat(31);
        let username = Username::parse(username_str);
        assert!(username.is_err());
    }

    struct FakeUsernameCheckService(String);

    impl IsUsernameTaken for FakeUsernameCheckService {
        async fn is_username_taken(&self, username: &str) -> bool {
            username == self.0.as_str()
        }
    }

    #[tokio::test]
    async fn username_is_taken() {
        let username_result: Result<Username, UsernameError> = Ok(Username("taken".to_string()));

        assert!(
            username_result
                .check_username_result(&FakeUsernameCheckService("taken".to_string()))
                .await
                .is_err()
        )
    }

    #[tokio::test]
    async fn username_is_not_taken() {
        let username_result: Result<Username, UsernameError> =
            Ok(Username("not_taken".to_string()));

        assert!(
            username_result
                .check_username_result(&FakeUsernameCheckService("taken".to_string()))
                .await
                .is_ok()
        )
    }
}
