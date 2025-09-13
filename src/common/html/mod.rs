pub mod context_html;
pub mod validate;

use crate::common::embed::{Asset, EmbedAsString};
use maud::{DOCTYPE, Markup, PreEscaped, html};

fn html_import_map() -> Markup {
    let map = if cfg!(debug_assertions) {
        Asset::get("import_map/import_map.dev.min.json").as_string()
    } else {
        Asset::get("import_map/import_map.prod.min.json").as_string()
    };
    html! {
        script type="importmap" { (PreEscaped(map)) }
    }
}

fn main_css_name() -> String {
    if cfg!(debug_assertions) {
        "/assets/css/main.css".to_string()
    } else {
        "/assets/css/main.min.css".to_string()
    }
}

fn html_doc(title: &str, content: Markup, head: Markup, footer: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title { (title) " | A Little Poem" }
                link rel="stylesheet" type="text/css" href=(main_css_name());
                (html_import_map())
                (head)
            }
            body {
                (content)
                (footer)
            }
        }
    }
}

pub struct HtmlBuilder {
    title: String,
    content: Markup,
    head: Option<Markup>,
    footer: Option<Markup>,
}

impl HtmlBuilder {
    pub fn new(title: String, content: Markup) -> Self {
        Self {
            title,
            content,
            head: None,
            footer: None,
        }
    }

    pub fn attach_head(mut self, head: Markup) -> Self {
        self.head = Some(head);
        self
    }

    pub fn attach_footer(mut self, footer: Markup) -> Self {
        self.footer = Some(footer);
        self
    }

    pub fn build(self) -> Markup {
        html_doc(
            &self.title,
            self.content,
            self.head.unwrap_or(html! {}),
            self.footer.unwrap_or(html! {}),
        )
    }
}
