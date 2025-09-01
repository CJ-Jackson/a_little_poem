use crate::common::validation::string_rules::{
    StringLengthRule, StringMandatoryRule, StringSpecialCharRule,
};
use crate::common::validation::{StrValidationExtension, StringValidator, ValidationCheck};
use std::sync::Arc;
use thiserror::Error;

pub struct PasswordRule {
    pub is_mandatory: bool,
    pub must_have_uppercase: bool,
    pub must_have_lowercase: bool,
    pub must_have_special_chars: bool,
    pub must_have_digit: bool,
    pub min_length: Option<usize>,
    pub max_length: Option<usize>,
}

impl Default for PasswordRule {
    fn default() -> Self {
        Self {
            is_mandatory: true,
            must_have_uppercase: true,
            must_have_lowercase: true,
            must_have_special_chars: true,
            must_have_digit: true,
            min_length: Some(8),
            max_length: Some(64),
        }
    }
}

impl Into<(StringMandatoryRule, StringLengthRule, StringSpecialCharRule)> for &PasswordRule {
    fn into(self) -> (StringMandatoryRule, StringLengthRule, StringSpecialCharRule) {
        (
            StringMandatoryRule {
                is_mandatory: self.is_mandatory,
            },
            StringLengthRule {
                min_length: self.min_length,
                max_length: self.max_length,
            },
            StringSpecialCharRule {
                must_have_uppercase: self.must_have_uppercase,
                must_have_lowercase: self.must_have_lowercase,
                must_have_special_chars: self.must_have_special_chars,
                must_have_digit: self.must_have_digit,
            },
        )
    }
}

impl PasswordRule {
    fn rules(&self) -> (StringMandatoryRule, StringLengthRule, StringSpecialCharRule) {
        self.into()
    }

    fn check(&self, msgs: &mut Vec<String>, subject: &StringValidator) {
        let (mandatory, length, special_char) = self.rules();
        mandatory.check(msgs, subject);
        if !msgs.is_empty() {
            return;
        }
        length.check(msgs, subject);
        special_char.check(msgs, subject);
    }
}

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
    pub fn parse_custom(
        password: String,
        password_rule: PasswordRule,
    ) -> Result<Self, PasswordError> {
        let mut message: Vec<String> = vec![];
        let password_validator = password.as_string_validator();

        password_rule.check(&mut message, &password_validator);

        PasswordError::validation_check(message)?;
        Ok(Self(password))
    }

    pub fn parse(password: String) -> Result<Self, PasswordError> {
        Self::parse_custom(password, PasswordRule::default())
    }

    pub fn parse_login(password: String) -> Result<Self, PasswordError> {
        Self::parse_custom(
            password,
            PasswordRule {
                is_mandatory: false,
                must_have_uppercase: false,
                must_have_lowercase: false,
                must_have_special_chars: false,
                must_have_digit: false,
                min_length: None,
                max_length: Some(64),
            },
        )
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
