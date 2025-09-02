use poem::error::I18NError;
use poem::i18n::I18NResources;

pub fn build_resources() -> Result<I18NResources, I18NError> {
    let english = include_str!("_locale/english.ftl");
    let french = include_str!("_locale/french.ftl");

    I18NResources::builder()
        .add_ftl("en-GB", english)
        .add_ftl("en-US", english)
        .add_ftl("fr-FR", french)
        .build()
}
