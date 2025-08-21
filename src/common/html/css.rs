use poem::web::WithContentType;
use poem::{IntoResponse, handler};

#[handler]
pub async fn main_css() -> WithContentType<Vec<u8>> {
    #[cfg(debug_assertions)]
    let css = *include_bytes!("_asset/main.css");
    #[cfg(not(debug_assertions))]
    let css = *include_bytes!("_asset/main.min.css");

    css.to_vec().with_content_type("text/css; charset=utf-8")
}
