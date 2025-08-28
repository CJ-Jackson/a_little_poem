use crate::common::validation::{StrValidationExtension, ValidationCheck};
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("Description Error")]
pub struct DescriptionError(pub Arc<[String]>);

impl ValidationCheck for DescriptionError {
    fn validation_check(strings: Vec<String>) -> Result<(), Self> {
        if strings.is_empty() {
            Ok(())
        } else {
            Err(Self(strings.into()))
        }
    }
}

#[derive(Default)]
pub struct Description(String);

impl Description {
    pub fn parse(description: String) -> Result<Self, DescriptionError> {
        let mut message: Vec<String> = vec![];
        let description_validator = description.as_string_validator();

        let mut check_count = true;
        description_validator.is_empty().then(|| {
            message.push("Required".to_string());
            check_count = false;
        });
        check_count.then(|| {
            (description_validator.count_graphemes() < 5)
                .then(|| message.push("Must be at least 5 characters".to_string()));
            (description_validator.count_graphemes() > 100)
                .then(|| message.push("Must be at most 100 characters".to_string()));
        });

        DescriptionError::validation_check(message)?;
        Ok(Description(description))
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_description() {
        let description = Description::parse("Hello".to_string());
        assert!(description.is_ok());
    }

    #[test]
    fn test_parse_description_error_empty_description() {
        let description = Description::parse("".to_string());
        assert!(description.is_err());
    }

    #[test]
    fn test_parse_description_error_description_length_too_short() {
        let description = Description::parse("a".to_string());
        assert!(description.is_err());
    }

    #[test]
    fn test_parse_description_error_description_length_too_long() {
        let description = Description::parse("a".repeat(101));
        assert!(description.is_err());
    }
}
