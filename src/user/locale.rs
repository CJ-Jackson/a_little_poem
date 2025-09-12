use cjtoolkit_structured_validator::common::locale::{LocaleData, LocaleMessage, LocaleValue};
use std::sync::Arc;

pub struct PasswordEntropyLocale(pub f64);

impl LocaleMessage for PasswordEntropyLocale {
    fn get_locale_data(&self) -> Arc<LocaleData> {
        LocaleData::new_with_vec(
            "validate-password-entropy",
            vec![("min".to_string(), LocaleValue::from(self.0))],
        )
    }
}
