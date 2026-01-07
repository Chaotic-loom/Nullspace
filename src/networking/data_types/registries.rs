use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "registries/"]
pub struct RegistryData;