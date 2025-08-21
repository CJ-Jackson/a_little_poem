use crate::common::etag::{EtagCheck, EtagStamp};
use poem::handler;

#[handler]
pub async fn main_css(_etag_check: EtagCheck) -> EtagStamp {
    #[cfg(debug_assertions)]
    let css = *include_bytes!("_asset/main.css");
    #[cfg(not(debug_assertions))]
    let css = *include_bytes!("_asset/main.min.css");

    EtagStamp {
        data: css.into(),
        content_type: "text/css; charset=utf-8",
    }
}
