use maud::{Markup, html};
use std::sync::Arc;

pub fn arc_string_to_html(vec: Arc<[String]>) -> Markup {
    if vec.is_empty() {
        return html! {};
    }
    html! {
        ul .validation-error-list {
            @for message in vec.iter() {
                li .validation-error-message { (message) }
            }
        }
    }
}
