use crate::common::validation::StringValidator;

#[derive(Default)]
pub struct StringMandatoryRule {
    pub is_mandatory: bool,
}

impl StringMandatoryRule {
    pub fn check(&self, msgs: &mut Vec<String>, subject: &StringValidator) {
        if self.is_mandatory && subject.is_empty() {
            msgs.push("Cannot be empty".to_string());
        }
    }
}

#[derive(Default)]
pub struct StringLengthRule {
    pub min_length: Option<usize>,
    pub max_length: Option<usize>,
}

impl StringLengthRule {
    pub fn check(&self, msgs: &mut Vec<String>, subject: &StringValidator) {
        if let Some(min_length) = self.min_length {
            if subject.count_graphemes() < min_length {
                msgs.push(format!("Must be at least {} characters", min_length));
            }
        }
        if let Some(max_length) = self.max_length {
            if subject.count_graphemes() > max_length {
                msgs.push(format!("Must be at most {} characters", max_length));
            }
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
    pub fn check(&self, msgs: &mut Vec<String>, subject: &StringValidator) {
        if self.must_have_special_chars {
            if !subject.has_special_chars() {
                msgs.push("Must contain at least one special character".to_string());
            }
        }
        if self.must_have_uppercase && self.must_have_lowercase {
            if !subject.has_ascii_uppercase_and_lowercase() {
                msgs.push("Must contain at least one uppercase and lowercase letter".to_string());
            }
        } else {
            if self.must_have_uppercase {
                if !subject.has_ascii_uppercase() {
                    msgs.push("Must contain at least one uppercase letter".to_string());
                }
            }
            if self.must_have_lowercase {
                if !subject.has_ascii_lowercase() {
                    msgs.push("Must contain at least one lowercase letter".to_string());
                }
            }
        }
        if self.must_have_digit {
            if !subject.has_ascii_digit() {
                msgs.push("Must contain at least one digit".to_string());
            }
        }
    }
}
