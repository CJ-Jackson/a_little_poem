use crate::common::context::{Context, ContextError, FromContext};
use crate::common::html::HtmlBuilder;
use error_stack::Report;
use maud::{Markup, html};
use std::cell::RefCell;
use std::sync::{RwLock, TryLockResult};

struct ContextHtmlCellData {
    title: Option<String>,
    content: Option<Markup>,
    head: Option<Markup>,
    footer: Option<Markup>,
}

pub struct ContextHtmlBuilder {
    data: RwLock<ContextHtmlCellData>,
}

impl ContextHtmlBuilder {
    pub fn new() -> Self {
        Self {
            data: RwLock::new(ContextHtmlCellData {
                title: None,
                content: None,
                head: None,
                footer: None,
            }),
        }
    }

    pub fn attach_title(&self, title: &str) -> &Self {
        match self.data.try_write() {
            Ok(mut data) => {
                data.title = Some(title.to_string());
            }
            Err(_) => {}
        }
        self
    }

    pub fn attach_content(&self, content: Markup) -> &Self {
        match self.data.try_write() {
            Ok(mut data) => {
                data.content = Some(content);
            }
            Err(_) => {}
        }
        self
    }

    pub fn attach_head(&self, head: Markup) -> &Self {
        match self.data.try_write() {
            Ok(mut data) => {
                data.head = Some(head);
            }
            Err(_) => {}
        }
        self
    }

    pub fn attach_footer(&self, footer: Markup) -> &Self {
        match self.data.try_write() {
            Ok(mut data) => {
                data.footer = Some(footer);
            }
            Err(_) => {}
        }
        self
    }

    pub fn build(&self) -> Markup {
        match self.data.try_read() {
            Ok(data) => {
                let title = data.title.clone().unwrap_or_else(|| "Untitled".to_string());
                let content = data.content.clone().unwrap_or_else(|| html! {});
                let head = data.head.clone().unwrap_or_else(|| html! {});
                let footer = data.footer.clone().unwrap_or_else(|| html! {});

                let new_content = html! {
                    div .content-wrapper {
                        div .container .main-content {
                            (content)
                        }
                    }
                };

                HtmlBuilder::new(title, new_content)
                    .attach_head(head)
                    .attach_footer(footer)
                    .build()
            }
            Err(_) => {
                html! {}
            }
        }
    }
}

impl FromContext for ContextHtmlBuilder {
    async fn from_context(_ctx: &'_ Context<'_>) -> Result<Self, Report<ContextError>> {
        Ok(Self::new())
    }
}
