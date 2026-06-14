//! Compile-time asset embedding via rust-embed.
//!
//! Requires `rust-embed` in Cargo.toml:
//!   rust-embed = { version = "8", features = ["compression"] }
//!
//! Also requires `mod assets;` in src/lib.rs.

use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "assets/"]
pub struct Assets;

impl Assets {
    pub fn get_theme(name: &str) -> Option<Vec<u8>> {
        let path = format!("themes/{}.json", name);
        Self::get(&path).map(|f| f.data.into_owned())
    }

    pub fn theme_names() -> Vec<String> {
        Self::iter()
            .filter(|p| p.starts_with("themes/") && p.ends_with(".json"))
            .map(|p| {
                p.strip_prefix("themes/")
                    .and_then(|s| s.strip_suffix(".json"))
                    .unwrap_or(&p)
                    .to_string()
            })
            .collect()
    }
}
