use crate::common::context::user::UserDep;
use crate::common::html::context_html::ContextHtmlBuilder;
use crate::common::icon::plus_icon;
use maud::{Markup, PreEscaped, html};
use poem::web::WithContentType;
use poem::{IntoResponse, Route, get, handler};

#[handler]
async fn home(context_html_builder: UserDep<ContextHtmlBuilder>) -> Markup {
    let title = "Rust Vue Exercise";
    context_html_builder
        .0
        .attach_title(title)
        .set_current_tag("home")
        .attach_content(html! {
            h1 .mt-3 { (title) }
            p .mt-3 { "This is Rust Vue Exercise." }
            h2 .mt-3 { "Exercise 1" }
            div #app .mt-3 v-cloak { "{{ message }}" }
            h2 .mt-3 { "Exercise 2" }
            div #counter .mt-3 v-cloak {
                button .btn .btn-sky-blue "@click"="count++" {
                    "Count is: {{ count }}  "
                    (plus_icon())
                }
            }
            h2 .mt-3 { "Exercise 3" }
            div #array .mt-3 v-cloak {
                ul .ul-bullet {
                    li "v-for"="(item) in items" { "{{ item }}" }
                }
            }
        })
        .attach_footer(root_js())
        .build()
}

fn root_js() -> Markup {
    #[cfg(debug_assertions)]
    let js = include_str!("_asset/root.js");
    #[cfg(not(debug_assertions))]
    let js = include_str!("_asset/root.min.js");
    html! {
        script type="module" { (PreEscaped(js)) }
    }
}

#[handler]
async fn favicon() -> WithContentType<Vec<u8>> {
    (*include_bytes!("_asset/favicon.ico"))
        .to_vec()
        .with_content_type("image/x-icon")
}

pub fn route_home(route: Route) -> Route {
    route.at("/", get(home)).at("/favicon.ico", get(favicon))
}
