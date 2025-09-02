use crate::common::validation::string_rules::{StringLengthRule, StringMandatoryRule};
use crate::common::validation::validate_locale::{ValidateErrorCollector, ValidateErrorStore};
use crate::common::validation::{StrValidationExtension, StringValidator, ValidationCheck};
use thiserror::Error;

pub struct NameRule {
    pub is_mandatory: bool,
    pub min_length: Option<usize>,
    pub max_length: Option<usize>,
}

impl Default for NameRule {
    fn default() -> Self {
        Self {
            is_mandatory: true,
            min_length: Some(5),
            max_length: Some(20),
        }
    }
}

impl Into<(StringMandatoryRule, StringLengthRule)> for &NameRule {
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

impl NameRule {
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
#[error("Name Error")]
pub struct NameError(pub ValidateErrorStore);

impl ValidationCheck for NameError {
    fn validation_check(strings: ValidateErrorCollector) -> Result<(), Self> {
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
    pub fn parse_custom(name: String, name_rule: NameRule) -> Result<Self, NameError> {
        let mut message = ValidateErrorCollector::new();
        let name_validator = name.as_string_validator();

        name_rule.check(&mut message, &name_validator);

        NameError::validation_check(message)?;
        Ok(Name(name))
    }

    pub fn parse(name: String) -> Result<Self, NameError> {
        Self::parse_custom(name, NameRule::default())
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}
