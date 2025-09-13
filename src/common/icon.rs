use crate::common::embed::{Asset, EmbedAsString};
use maud::{Markup, PreEscaped};

pub fn plus_icon() -> Markup {
    PreEscaped(Asset::get("icon/plus.svg").as_string())
}
