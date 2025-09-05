use cjtoolkit_structured_validator::types::password::PasswordRules;
use cjtoolkit_structured_validator::types::username::UsernameRules;

#[inline]
pub fn username_rules_for_login() -> UsernameRules {
    UsernameRules {
        is_mandatory: true,
        min_length: None,
        max_length: None,
    }
}

#[inline]
pub fn password_rules_for_login() -> PasswordRules {
    PasswordRules {
        is_mandatory: true,
        must_have_uppercase: false,
        must_have_lowercase: false,
        must_have_special_chars: false,
        must_have_digit: false,
        min_length: None,
        max_length: Some(64),
    }
}
