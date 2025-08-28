use crate::common::validation::{StrValidationExtension, ValidationCheck};
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("Name Error")]
pub struct NameError(pub Arc<[String]>);

impl ValidationCheck for NameError {
    fn validation_check(strings: Vec<String>) -> Result<(), Self> {
        if strings.is_empty() {
            Ok(())
        } else {
            Err(Self(strings.into()))
        }
    }
}

#[derive(Default)]
pub struct Name(String);

impl Name {
    pub fn parse(name: String) -> Result<Self, NameError> {
        let mut message: Vec<String> = vec![];
        let name_validator = name.as_string_validator();

        let mut check_count = true;
        name_validator.is_empty().then(|| {
            message.push("Required".to_string());
            check_count = false;
        });
        check_count.then(|| {
            (name_validator.count_graphemes() < 5)
                .then(|| message.push("Must be at least 5 characters".to_string()));
            (name_validator.count_graphemes() > 20)
                .then(|| message.push("Must be at most 20 characters".to_string()));
        });

        NameError::validation_check(message)?;
        Ok(Name(name))
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_name() {
        let name = Name::parse("Hello".to_string());
        assert!(name.is_ok());
    }

    #[test]
    fn test_parse_name_error_empty_name() {
        let name = Name::parse("".to_string());
        assert!(name.is_err());
    }

    #[test]
    fn test_parse_name_error_name_length_too_short() {
        let name = Name::parse("a".to_string());
        assert!(name.is_err());
    }

    #[test]
    fn test_parse_name_error_name_length_too_long() {
        let name = Name::parse("a".repeat(21));
        assert!(name.is_err());
    }
}
