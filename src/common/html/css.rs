use poem::web::WithContentType;
use poem::{IntoResponse, Route, get, handler};

#[handler]
async fn main_css() -> WithContentType<Vec<u8>> {
    #[cfg(debug_assertions)]
    let css = *include_bytes!("_asset/main.css");
    #[cfg(not(debug_assertions))]
    let css = *include_bytes!("_asset/main.min.css");

    css.to_vec().with_content_type("text/css; charset=utf-8")
}

pub fn route_css(route: Route) -> Route {
    route.at("/main.css", get(main_css))
}
