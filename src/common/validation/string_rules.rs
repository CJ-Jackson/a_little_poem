use crate::common::validation::StringValidator;
use crate::common::validation::validate_locale::{LocaleMessage, ValidateErrorCollector};
use poem::i18n::Locale;

pub struct StringMandatoryLocale;

impl LocaleMessage for StringMandatoryLocale {
    fn get_locale_message(&self, locale: &Locale, original: String) -> String {
        locale.text("validate-cannot-be-empty").unwrap_or(original)
    }
}

#[derive(Default)]
pub struct StringMandatoryRule {
    pub is_mandatory: bool,
}

impl StringMandatoryRule {
    pub fn check(&self, msgs: &mut ValidateErrorCollector, subject: &StringValidator) {
        if self.is_mandatory && subject.is_empty() {
            msgs.push((
                "Cannot be empty".to_string(),
                Box::new(StringMandatoryLocale),
            ));
        }
    }
}

pub enum StringLengthLocale {
    MinLength(usize),
    MaxLength(usize),
}

impl LocaleMessage for StringLengthLocale {
    fn get_locale_message(&self, locale: &Locale, original: String) -> String {
        match self {
            Self::MinLength(min) => locale
                .text_with_args("validate-min-length", (("min", min),))
                .unwrap_or(original),
            Self::MaxLength(max) => locale
                .text_with_args("validate-max-length", (("max", max),))
                .unwrap_or(original),
        }
    }
}

#[derive(Default)]
pub struct StringLengthRule {
    pub min_length: Option<usize>,
    pub max_length: Option<usize>,
}

impl StringLengthRule {
    pub fn check(&self, msgs: &mut ValidateErrorCollector, subject: &StringValidator) {
        if let Some(min_length) = self.min_length {
            if subject.count_graphemes() < min_length {
                msgs.push((
                    format!("Must be at least {} characters", min_length),
                    Box::new(StringLengthLocale::MinLength(min_length)),
                ));
            }
        }
        if let Some(max_length) = self.max_length {
            if subject.count_graphemes() > max_length {
                msgs.push((
                    format!("Must be at most {} characters", max_length),
                    Box::new(StringLengthLocale::MaxLength(max_length)),
                ));
            }
        }
    }
}

pub enum StringSpecialCharLocale {
    MustHaveSpecialChars,
    MustHaveUppercaseAndLowercase,
    MustHaveUppercase,
    MustHaveLowercase,
    MustHaveDigit,
}

impl LocaleMessage for StringSpecialCharLocale {
    fn get_locale_message(&self, locale: &Locale, original: String) -> String {
        match self {
            Self::MustHaveSpecialChars => locale
                .text("validate-must-have-special-chars")
                .unwrap_or(original),
            Self::MustHaveUppercaseAndLowercase => locale
                .text("validate-must-have-uppercase-and-lowercase")
                .unwrap_or(original),
            Self::MustHaveUppercase => locale
                .text("validate-must-have-uppercase")
                .unwrap_or(original),
            Self::MustHaveLowercase => locale
                .text("validate-must-have-lowercase")
                .unwrap_or(original),
            Self::MustHaveDigit => locale.text("validate-must-have-digit").unwrap_or(original),
        }
    }
}

#[derive(Default)]
pub struct StringSpecialCharRule {
    pub must_have_uppercase: bool,
    pub must_have_lowercase: bool,
    pub must_have_special_chars: bool,
    pub must_have_digit: bool,
}

impl StringSpecialCharRule {
    pub fn check(&self, msgs: &mut ValidateErrorCollector, subject: &StringValidator) {
        if self.must_have_special_chars {
            if !subject.has_special_chars() {
                msgs.push((
                    "Must contain at least one special character".to_string(),
                    Box::new(StringSpecialCharLocale::MustHaveSpecialChars),
                ));
            }
        }
        if self.must_have_uppercase && self.must_have_lowercase {
            if !subject.has_ascii_uppercase_and_lowercase() {
                msgs.push((
                    "Must contain at least one uppercase and lowercase letter".to_string(),
                    Box::new(StringSpecialCharLocale::MustHaveUppercaseAndLowercase),
                ));
            }
        } else {
            if self.must_have_uppercase {
                if !subject.has_ascii_uppercase() {
                    msgs.push((
                        "Must contain at least one uppercase letter".to_string(),
                        Box::new(StringSpecialCharLocale::MustHaveUppercase),
                    ));
                }
            }
            if self.must_have_lowercase {
                if !subject.has_ascii_lowercase() {
                    msgs.push((
                        "Must contain at least one lowercase letter".to_string(),
                        Box::new(StringSpecialCharLocale::MustHaveLowercase),
                    ));
                }
            }
        }
        if self.must_have_digit {
            if !subject.has_ascii_digit() {
                msgs.push((
                    "Must contain at least one digit".to_string(),
                    Box::new(StringSpecialCharLocale::MustHaveDigit),
                ));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod string_mandatory_rule {
        use super::*;
        use crate::common::validation::StrValidationExtension;

        #[test]
        fn test_string_mandatory_rule_check_empty_string() {
            let mut msgs = ValidateErrorCollector::new();
            let subject = "".as_string_validator();
            let rule = StringMandatoryRule { is_mandatory: true };
            rule.check(&mut msgs, &subject);
            assert_eq!(msgs.len(), 1);
            assert_eq!(msgs.0[0].0, "Cannot be empty");
        }

        #[test]
        fn test_string_mandatory_rule_check_not_empty_string() {
            let mut msgs = ValidateErrorCollector::new();
            let subject = "Hello".as_string_validator();
            let rule = StringMandatoryRule { is_mandatory: true };
            rule.check(&mut msgs, &subject);
            assert_eq!(msgs.len(), 0);
        }
    }

    mod string_length_rule {
        use super::*;
        use crate::common::validation::StrValidationExtension;

        #[test]
        fn test_string_length_rule_check_empty_string() {
            let mut msgs = ValidateErrorCollector::new();
            let subject = "".as_string_validator();
            let rule = StringLengthRule {
                min_length: Some(5),
                max_length: Some(10),
            };
            rule.check(&mut msgs, &subject);
            assert_eq!(msgs.len(), 1);
            assert_eq!(msgs.0[0].0, "Must be at least 5 characters");
        }

        #[test]
        fn test_string_length_rule_check_too_long_string() {
            let mut msgs = ValidateErrorCollector::new();
            let subject = "Hello".as_string_validator();
            let rule = StringLengthRule {
                min_length: Some(2),
                max_length: Some(4),
            };
            rule.check(&mut msgs, &subject);
            assert_eq!(msgs.len(), 1);
            assert_eq!(msgs.0[0].0, "Must be at most 4 characters");
        }
    }

    mod string_special_char_rule {
        use super::*;
        use crate::common::validation::StrValidationExtension;

        #[test]
        fn test_string_special_char_rule_check_empty_string() {
            let mut msgs = ValidateErrorCollector::new();
            let subject = "".as_string_validator();
            let rule = StringSpecialCharRule {
                must_have_uppercase: true,
                must_have_lowercase: true,
                must_have_special_chars: true,
                must_have_digit: true,
            };
            rule.check(&mut msgs, &subject);
            assert_eq!(msgs.len(), 3);
            assert_eq!(msgs.0[0].0, "Must contain at least one special character");
            assert_eq!(
                msgs.0[1].0,
                "Must contain at least one uppercase and lowercase letter"
            );
            assert_eq!(msgs.0[2].0, "Must contain at least one digit");
        }

        #[test]
        fn test_string_special_char_rule_check_not_empty_string() {
            let mut msgs = ValidateErrorCollector::new();
            let subject = "Hello".as_string_validator();
            let rule = StringSpecialCharRule {
                must_have_uppercase: true,
                must_have_lowercase: true,
                must_have_special_chars: true,
                must_have_digit: true,
            };
            rule.check(&mut msgs, &subject);
            assert_eq!(msgs.len(), 2);
            assert_eq!(msgs.0[0].0, "Must contain at least one special character");
            assert_eq!(msgs.0[1].0, "Must contain at least one digit");
        }

        #[test]
        fn test_string_special_char_rule_check_not_empty_string_with_uppercase_and_lowercase_and_symbol()
         {
            let mut msgs = ValidateErrorCollector::new();
            let subject = "Hello@".as_string_validator();
            let rule = StringSpecialCharRule {
                must_have_uppercase: true,
                must_have_lowercase: true,
                must_have_special_chars: true,
                must_have_digit: true,
            };
            rule.check(&mut msgs, &subject);
            assert_eq!(msgs.len(), 1);
            assert_eq!(msgs.0[0].0, "Must contain at least one digit");
        }

        #[test]
        fn test_string_special_char_rule_check_not_empty_string_with_uppercase_and_lowercase_and_digit()
         {
            let mut msgs = ValidateErrorCollector::new();
            let subject = "Hello1".as_string_validator();
            let rule = StringSpecialCharRule {
                must_have_uppercase: true,
                must_have_lowercase: true,
                must_have_special_chars: true,
                must_have_digit: true,
            };
            rule.check(&mut msgs, &subject);
            assert_eq!(msgs.len(), 1);
            assert_eq!(msgs.0[0].0, "Must contain at least one special character");
        }

        #[test]
        fn test_string_special_char_rule_check_not_empty_string_with_uppercase_and_lowercase_digit_and_symbol()
         {
            let mut msgs = ValidateErrorCollector::new();
            let subject = "Hello1@".as_string_validator();
            let rule = StringSpecialCharRule {
                must_have_uppercase: true,
                must_have_lowercase: true,
                must_have_special_chars: true,
                must_have_digit: true,
            };
            rule.check(&mut msgs, &subject);
            assert_eq!(msgs.len(), 0);
        }
    }
}
