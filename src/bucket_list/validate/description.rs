use crate::common::validation::string_rules::{StringLengthRule, StringMandatoryRule};
use crate::common::validation::validate_locale::{ValidateErrorCollector, ValidateErrorStore};
use crate::common::validation::{StrValidationExtension, StringValidator, ValidationCheck};
use thiserror::Error;

pub struct DescriptionRule {
    pub is_mandatory: bool,
    pub min_length: Option<usize>,
    pub max_length: Option<usize>,
}

impl Default for DescriptionRule {
    fn default() -> Self {
        Self {
            is_mandatory: true,
            min_length: Some(5),
            max_length: Some(100),
        }
    }
}

impl Into<(StringMandatoryRule, StringLengthRule)> for &DescriptionRule {
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

impl DescriptionRule {
    fn rules(&self) -> (StringMandatoryRule, StringLengthRule) {
        self.into()
    }

    fn check(&self, msgs: &mut ValidateErrorCollector, subject: &StringValidator) {
        let (mandatory, length) = self.rules();
        mandatory.check(msgs, subject);
        if !msgs.is_empty() {
            return;
        }
        length.check(msgs, subject);
    }
}

#[derive(Debug, Error)]
#[error("Description Error")]
pub struct DescriptionError(pub ValidateErrorStore);

impl ValidationCheck for DescriptionError {
    fn validation_check(strings: ValidateErrorCollector) -> Result<(), Self> {
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
    pub fn parse_custom(
        description: String,
        description_rule: DescriptionRule,
    ) -> Result<Self, DescriptionError> {
        let mut message = ValidateErrorCollector::new();
        let description_validator = description.as_string_validator();

        description_rule.check(&mut message, &description_validator);

        DescriptionError::validation_check(message)?;
        Ok(Description(description))
    }

    pub fn parse(description: String) -> Result<Self, DescriptionError> {
        Self::parse_custom(description, DescriptionRule::default())
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}
