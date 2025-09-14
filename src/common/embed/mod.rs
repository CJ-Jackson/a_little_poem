use poem::endpoint::{EmbeddedFileEndpoint, EmbeddedFilesEndpoint};
use rust_embed::{Embed, EmbeddedFile};

pub const EMBED_PATH: &'static str = "/assets/";

#[derive(Embed)]
#[folder = "asset/embed/"]
#[exclude = "*.mjs"]
pub struct Asset;

pub type AssetFileEndpoint = EmbeddedFileEndpoint<Asset>;
pub type AssetFilesEndpoint = EmbeddedFilesEndpoint<Asset>;

pub trait EmbedAsString {
    fn as_string(&self) -> String;
}

impl EmbedAsString for Option<EmbeddedFile> {
    fn as_string(&self) -> String {
        self.as_ref()
            .map(|f| String::from_utf8(f.data.to_vec()).unwrap_or_default())
            .unwrap_or_default()
    }
}
