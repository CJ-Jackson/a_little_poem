use maud::{Markup, html};
use std::sync::Arc;

fn arc_string_to_html_error(messages: Arc<[String]>) -> Markup {
    if messages.is_empty() {
        return html! {};
    }
    html! {
        ul .validation-error-list {
            @for message in messages.iter() {
                li .validation-error-message { (message) }
            }
        }
    }
}

pub trait ValidateErrorMessageExt {
    fn into_error_html(self) -> Markup;
}

impl ValidateErrorMessageExt for Arc<[String]> {
    fn into_error_html(self) -> Markup {
        arc_string_to_html_error(self)
    }
}

impl ValidateErrorMessageExt for Vec<String> {
    fn into_error_html(self) -> Markup {
        arc_string_to_html_error(self.into())
    }
}
