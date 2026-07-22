use rust_embed::RustEmbed;

#[derive(Debug, Clone, RustEmbed)]
#[folder = "web/dist/"]
pub struct Assets;
