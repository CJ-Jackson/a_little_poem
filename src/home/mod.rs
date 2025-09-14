use crate::common::context::user::JustDep;
use crate::common::embed::{Asset, AssetFileEndpoint, EmbedAsString};
use crate::common::html::context_html::ContextHtmlBuilder;
use crate::common::icon::plus_icon;
use maud::{Markup, PreEscaped, html};
use poem::web::Json;
use poem::{Route, get, handler};
use serde_json::{Value, json};

#[handler]
pub async fn home_page(JustDep(context_html_builder, _): JustDep<ContextHtmlBuilder>) -> Markup {
    let title = "Rust Vue Exercise";
    context_html_builder
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
    let js = if cfg!(debug_assertions) {
        Asset::get("js/root.js").as_string()
    } else {
        Asset::get("js/root.min.js").as_string()
    };
    html! {
        script type="module" { (PreEscaped(js)) }
    }
}

#[handler]
pub async fn js_array() -> Json<Value> {
    Json(json!(["Apple", "Orange", "Banana", "Strawberry", "Mango"]))
}

pub fn route_home_page() -> Route {
    Route::new()
        .at("/", get(home_page))
        .at("/array", get(js_array))
        .at(
            "/favicon.ico",
            AssetFileEndpoint::new("/favicon/favicon.ico"),
        )
}
