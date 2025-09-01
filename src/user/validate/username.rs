use crate::common::validation::string_rules::{StringLengthRule, StringMandatoryRule};
use crate::common::validation::{StrValidationExtension, StringValidator, ValidationCheck};
use std::sync::Arc;
use thiserror::Error;

pub struct UsernameRule {
    pub is_mandatory: bool,
    pub min_length: Option<usize>,
    pub max_length: Option<usize>,
}

impl Default for UsernameRule {
    fn default() -> Self {
        Self {
            is_mandatory: true,
            min_length: Some(5),
            max_length: Some(30),
        }
    }
}

impl Into<(StringMandatoryRule, StringLengthRule)> for &UsernameRule {
    fn into(self) -> (StringMandatoryRule, StringLengthRule) {
        (
            StringMandatoryRule {
                is_mandatory: self.is_mandatory,
            },
            StringLengthRule {
                min_length: self.min_length,
                max_length: self.max_length,
            },
        )
    }
}

impl UsernameRule {
    fn rules(&self) -> (StringMandatoryRule, StringLengthRule) {
        self.into()
    }

    fn check(&self, msgs: &mut Vec<String>, subject: &StringValidator) {
        let (mandatory, length) = self.rules();
        mandatory.check(msgs, subject);
        if !msgs.is_empty() {
            return;
        }
        length.check(msgs, subject);
    }
}

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

impl Clone for UsernameError {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

#[derive(Default, Clone)]
pub struct Username(String);

pub trait IsUsernameTaken {
    fn is_username_taken(&self, username: &str) -> impl Future<Output = bool>;
}

impl Username {
    pub fn parse_custom(
        username: String,
        username_rule: UsernameRule,
    ) -> Result<Self, UsernameError> {
        let mut message: Vec<String> = vec![];
        let username_validator = username.as_string_validator();

        username_rule.check(&mut message, &username_validator);

        UsernameError::validation_check(message)?;
        Ok(Self(username))
    }

    pub fn parse(username: String) -> Result<Self, UsernameError> {
        Self::parse_custom(username, UsernameRule::default())
    }

    pub fn parse_login(username: String) -> Result<Self, UsernameError> {
        Self::parse_custom(
            username,
            UsernameRule {
                is_mandatory: false,
                min_length: None,
                max_length: Some(30),
            },
        )
    }

    pub async fn check_if_username_taken<T: IsUsernameTaken>(
        &self,
        service: &T,
    ) -> Result<Self, UsernameError> {
        let mut message: Vec<String> = vec![];

        service.is_username_taken(self.as_str()).await.then(|| {
            message.push("Already taken".to_string());
        });

        UsernameError::validation_check(message)?;
        Ok(self.clone())
    }

    pub fn as_str(&self) -> &str {
        &self.0
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
        let username_result = Username("taken".to_string());

        assert!(
            username_result
                .check_if_username_taken(&FakeUsernameCheckService("taken".to_string()))
                .await
                .is_err()
        )
    }

    #[tokio::test]
    async fn username_is_not_taken() {
        let username_result = Username("not_taken".to_string());

        assert!(
            username_result
                .check_if_username_taken(&FakeUsernameCheckService("taken".to_string()))
                .await
                .is_ok()
        )
    }
}
