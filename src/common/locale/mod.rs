use crate::common::context::{Context, ContextError, FromContext};
use crate::common::embed::EmbedAsString;
use cjtoolkit_structured_validator::common::locale::{LocaleData, LocaleValue, ValidateErrorStore};
use cjtoolkit_structured_validator::common::validation_collector::AsValidateErrorStore;
use error_stack::{Report, ResultExt};
use poem::FromRequest;
use poem::error::I18NError;
use poem::i18n::{I18NArgs, I18NResources, Locale};
use rust_embed::Embed;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Embed)]
#[folder = "$CARGO_MANIFEST_DIR/asset/locale/"]
struct LocaleAsset;

impl LocaleAsset {
    fn locale_map() -> HashMap<String, String> {
        let mut map = HashMap::new();
        for value in Self::iter() {
            let locale = value.split("/").next().unwrap_or_default();
            let mut str = map
                .get(locale)
                .map(|v: &String| v.to_string())
                .unwrap_or_default();
            let file = Self::get(String::from(value.clone()).as_str());
            str.push_str(&file.as_string());
            str.push('\n');
            map.insert(locale.to_string(), str);
        }
        map
    }
}

pub fn build_resources() -> Result<I18NResources, I18NError> {
    let locale_map = LocaleAsset::locale_map();
    let mut resources = I18NResources::builder();
    for (locale, content) in locale_map {
        resources = resources.add_ftl(locale, content);
    }
    resources.build()
}

impl FromContext for Locale {
    async fn from_context(ctx: &'_ Context<'_>) -> Result<Self, Report<ContextError>> {
        Locale::from_request_without_body(ctx.req)
            .await
            .change_context(ContextError::RequestError)
    }
}

pub trait LocaleExtForData {
    fn get_translation(&self, locale: &Locale, original: String) -> String;
}

impl LocaleExtForData for LocaleData {
    fn get_translation(&self, locale: &Locale, original: String) -> String {
        if !self.args.is_empty() {
            let mut values = I18NArgs::default();
            for (key, value) in self.args.iter() {
                match value {
                    LocaleValue::String(string) => {
                        values = values.set::<String, String>(key.clone(), string.clone());
                    }
                    LocaleValue::Uint(unit) => {
                        values = values.set::<String, usize>(key.clone(), *unit);
                    }
                    LocaleValue::Int(int) => {
                        values = values.set::<String, isize>(key.clone(), *int);
                    }
                    LocaleValue::Float(float) => {
                        values = values.set::<String, f64>(key.clone(), *float);
                    }
                }
            }
            locale
                .text_with_args(self.name.clone(), values)
                .unwrap_or(original)
        } else {
            locale.text(self.name.clone()).unwrap_or(original)
        }
    }
}

pub trait LocaleExtForStore {
    fn as_translated_message(&self, locale: &Locale) -> Arc<[String]>;
}

impl LocaleExtForStore for ValidateErrorStore {
    fn as_translated_message(&self, locale: &Locale) -> Arc<[String]> {
        self.0
            .iter()
            .map(|e| e.1.get_locale_data().get_translation(locale, e.0.clone()))
            .collect()
    }
}

pub trait LocaleExtForResult: AsValidateErrorStore {
    fn as_translated_message(&self, locale: &Locale) -> Arc<[String]>;

    fn as_original_message(&self) -> Arc<[String]>;
}

impl<T, E> LocaleExtForResult for Result<T, E>
where
    for<'a> &'a E: Into<ValidateErrorStore>,
{
    fn as_translated_message(&self, locale: &Locale) -> Arc<[String]> {
        self.as_validate_store().as_translated_message(locale)
    }

    fn as_original_message(&self) -> Arc<[String]> {
        self.as_validate_store().as_original_message()
    }
}
