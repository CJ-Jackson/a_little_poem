use crate::common::context::{Context, ContextError, FromContext};
use error_stack::{Report, ResultExt};
use poem::FromRequest;
use poem::error::I18NError;
use poem::i18n::{I18NResources, Locale};

pub fn build_resources() -> Result<I18NResources, I18NError> {
    let english = include_str!("_locale/english.ftl");
    let french = include_str!("_locale/french.ftl");

    I18NResources::builder()
        .add_ftl("en-GB", english)
        .add_ftl("en-US", english)
        .add_ftl("fr-FR", french)
        .build()
}

impl FromContext for Locale {
    async fn from_context(ctx: &'_ Context<'_>) -> Result<Self, Report<ContextError>> {
        Locale::from_request_without_body(ctx.req)
            .await
            .change_context(ContextError::RequestError)
    }
}
