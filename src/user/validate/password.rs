use crate::common::validation::{StrValidationExtension, ValidationCheck};
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("Password is invalid")]
pub struct PasswordError(pub Arc<[String]>);

impl ValidationCheck for PasswordError {
    fn validation_check(strings: Vec<String>) -> Result<(), Self> {
        if strings.is_empty() {
            Ok(())
        } else {
            Err(Self(strings.into()))
        }
    }
}

impl Clone for PasswordError {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

#[derive(Default, Clone)]
pub struct Password(String);

impl Password {
    pub fn parse(password: String) -> Result<Self, PasswordError> {
        let mut message: Vec<String> = vec![];
        let password_validator = password.as_string_validator();

        let mut check_count_and_chars = true;
        password_validator.is_empty().then(|| {
            message.push("Cannot be empty".to_string());
            check_count_and_chars = false;
        });
        check_count_and_chars.then(|| {
            (password_validator.count_graphemes() < 8).then(|| {
                message.push("Must be at least 8 characters".to_string());
            });
            (password_validator.count_graphemes() > 64).then(|| {
                message.push("Must be at most 64 characters".to_string());
            });
            (!password_validator.has_ascii_uppercase_and_lowercase()).then(|| {
                message
                    .push("Must contain at least one uppercase and lowercase letter".to_string());
            });
            (!password_validator.has_special_chars()).then(|| {
                message.push("Must contain at least one special character".to_string());
            });
            (!password_validator.has_ascii_digit()).then(|| {
                message.push("Must contain at least one digit".to_string());
            })
        });

        PasswordError::validation_check(message)?;
        Ok(Self(password))
    }

    pub fn parse_login(password: String) -> Result<Self, PasswordError> {
        let mut message: Vec<String> = vec![];
        let password_validator = password.as_string_validator();

        (password_validator.count_graphemes() > 64).then(|| {
            message.push("Must be at most 64 characters".to_string());
        });

        PasswordError::validation_check(message)?;
        Ok(Self(password))
    }

    pub fn parse_confirm(&self, password_confirm: String) -> Result<Self, PasswordError> {
        let mut message: Vec<String> = vec![];

        (password_confirm != self.as_str()).then(|| message.push("Does not match".to_string()));

        PasswordError::validation_check(message)?;
        Ok(Self(password_confirm))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_parse() {
        let password = Password::parse("Hello@Wor1d".to_string());
        assert!(password.is_ok());
    }

    #[test]
    fn test_password_parse_error_empty_string() {
        let password = Password::parse("".to_string());
        assert!(password.is_err());
    }

    #[test]
    fn test_password_parse_error_too_short() {
        let password = Password::parse("a".to_string());
        assert!(password.is_err());
    }

    #[test]
    fn test_password_parse_error_too_long() {
        let password_str = "a".repeat(65);
        let password = Password::parse(password_str);
        assert!(password.is_err());
    }

    #[test]
    fn test_password_parse_error_lower_case_only() {
        let password = Password::parse("hello@wor1d".to_string());
        assert!(password.is_err());
    }

    #[test]
    fn test_password_parse_error_upper_case_only() {
        let password = Password::parse("HELLO@WOR1D".to_string());
        assert!(password.is_err());
    }

    #[test]
    fn test_password_parse_error_special_char_only() {
        let password = Password::parse("!@#$%^&*()".to_string());
        assert!(password.is_err());
    }

    #[test]
    fn test_password_parse_error_digit_only() {
        let password = Password::parse("1234567890".to_string());
        assert!(password.is_err());
    }

    #[test]
    fn test_password_parse_error_password_confirmation_mismatch() {
        let password = Password("match".to_string());
        let password = password.parse_confirm("mismatch".to_string());
        assert!(password.is_err());
    }

    #[test]
    fn test_password_parse_error_password_confirmation_match() {
        let password = Password("match".to_string());
        let password = password.parse_confirm("match".to_string());
        assert!(password.is_ok());
    }
}
