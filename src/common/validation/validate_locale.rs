use poem::i18n::Locale;
use std::fmt::Debug;
use std::sync::Arc;

pub trait LocaleMessage: Send + Sync {
    fn get_locale_message(&self, locale: &Locale, original: String) -> String;
}

#[derive(Default)]
pub struct ValidateErrorStore(pub Arc<[(String, Box<dyn LocaleMessage>)]>);

impl Debug for ValidateErrorStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, error) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{:?}", error.0)?;
        }
        Ok(())
    }
}

impl Clone for ValidateErrorStore {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

impl ValidateErrorStore {
    pub fn as_original_message(&self) -> Arc<[String]> {
        self.0.iter().map(|x| x.0.clone()).collect()
    }

    pub fn as_locale_message(&self, locale: &Locale) -> Arc<[String]> {
        self.0
            .iter()
            .map(|x| x.1.get_locale_message(locale, x.0.clone()))
            .collect()
    }
}

#[derive(Default)]
pub struct ValidateErrorCollector(pub Vec<(String, Box<dyn LocaleMessage>)>);

impl Into<ValidateErrorStore> for ValidateErrorCollector {
    fn into(self) -> ValidateErrorStore {
        ValidateErrorStore(self.0.into())
    }
}

impl ValidateErrorCollector {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn push(&mut self, error: (String, Box<dyn LocaleMessage>)) {
        self.0.push(error);
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}
