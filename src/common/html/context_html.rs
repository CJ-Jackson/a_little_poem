use crate::common::context::user::{FromUserContext, UserContext};
use crate::common::context::{Context, ContextError, FromContext};
use crate::common::flash::{Flash, FlashMessage, FlashMessageHtml};
use crate::common::html::HtmlBuilder;
use crate::user::model::UserIdContext;
use error_stack::Report;
use maud::{Markup, PreEscaped, html};
use poem::session::Session;
use std::sync::{Arc, RwLock};

pub struct NavigationItem {
    name: String,
    url: String,
    tag: String,
}

impl NavigationItem {
    fn navigations() -> Box<[Self]> {
        [
            Self {
                name: "Home".to_string(),
                url: "/".to_string(),
                tag: "home".to_string(),
            },
            Self {
                name: "Bucket List".to_string(),
                url: "/bucket-list/".to_string(),
                tag: "bucket-list".to_string(),
            },
            Self {
                name: "User".to_string(),
                url: "/user/".to_string(),
                tag: "user".to_string(),
            },
        ]
        .into()
    }
}

struct ContextHtmlCellData {
    title: Option<String>,
    content: Option<Markup>,
    head: Option<Markup>,
    footer: Option<Markup>,
    current_tag: String,
}

pub struct ContextHtmlBuilder {
    flash: Option<Flash>,
    user_id_context: Option<Arc<UserIdContext>>,
    data: RwLock<ContextHtmlCellData>,
}

impl ContextHtmlBuilder {
    pub fn new(flash: Option<Flash>) -> Self {
        Self {
            flash,
            user_id_context: None,
            data: RwLock::new(ContextHtmlCellData {
                title: None,
                content: None,
                head: None,
                footer: None,
                current_tag: "".to_string(),
            }),
        }
    }

    pub fn set_user_id_context(&mut self, user_id_context: Arc<UserIdContext>) {
        self.user_id_context = Some(user_id_context);
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

    pub fn set_current_tag(&self, tag: &str) -> &Self {
        match self.data.try_write() {
            Ok(mut data) => {
                data.current_tag = tag.to_string();
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
                let current_tag = data.current_tag.clone();

                let new_content = html! {
                    (self.flash.flash_message_html())
                    (self.build_navigation(current_tag))
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

    fn build_navigation(&self, tag: String) -> Markup {
        let user_context = self.user_id_context.as_ref();
        html! {
            nav .nav-content {
                span .nav-home {
                    a href="/" { "A Little Poem" }
                }
                (self.parse_navigation(tag))
                @if let Some(user_context) = user_context {
                    span .nav-user {
                        @if user_context.is_user {
                            a href="/user/" { "Hello, " (user_context.username) }
                        } @else {
                            a href="/user/login/" { "You're a visitor, click here to login" }
                        }
                    }
                } @else {
                    span .nav-user {
                        a .nav-user href="/user/login/" { "Login" }
                    }
                }
            }
        }
    }

    fn parse_navigation(&self, tag: String) -> Markup {
        let mut output = "".to_string();
        for item in NavigationItem::navigations() {
            let html = if item.tag == tag {
                html! {
                    span .nav-item .nav-item-active {
                        a href=(item.url) { (item.name) }
                    }
                }
            } else {
                html! {
                    span .nav-item {
                        a href=(item.url) { (item.name) }
                    }
                }
            };
            output.push_str(html.into_string().as_str());
        }
        PreEscaped(output)
    }
}

impl FromContext for ContextHtmlBuilder {
    async fn from_context(ctx: &'_ Context<'_>) -> Result<Self, Report<ContextError>> {
        let flash: Option<Flash> = match ctx.req.data::<Session>() {
            None => None,
            Some(session) => session.get_flash(),
        };
        Ok(Self::new(flash))
    }
}

impl FromUserContext for ContextHtmlBuilder {
    async fn from_user_context(ctx: &'_ UserContext<'_>) -> Result<Self, Report<ContextError>> {
        let mut context_html_builder: Self = ctx.inject_global().await?;
        context_html_builder.set_user_id_context(Arc::clone(&ctx.user_context));
        Ok(context_html_builder)
    }
}
