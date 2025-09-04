use crate::common::etag::{EtagCheck, EtagStamp};
use poem::handler;

pub mod adapter;
pub mod cache_local;
pub mod config;
pub mod context;
pub mod cookie_builder;
pub mod csrf;
pub mod db;
pub mod error;
pub mod etag;
pub mod flash;
pub mod html;
pub mod icon;
pub mod locale;
pub mod password;

#[handler]
pub async fn common_js(_etag_check: EtagCheck) -> EtagStamp {
    #[cfg(debug_assertions)]
    let v = *include_bytes!("_asset/common.js");
    #[cfg(not(debug_assertions))]
    let v = *include_bytes!("_asset/common.min.js");

    EtagStamp {
        data: v.into(),
        content_type: "application/javascript; charset=utf-8",
    }
}
