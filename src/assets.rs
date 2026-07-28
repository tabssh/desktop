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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_names_nonempty_and_no_extension() {
        let names = Assets::theme_names();
        assert!(!names.is_empty());
        for name in &names {
            assert!(!name.ends_with(".json"));
            assert!(!name.starts_with("themes/"));
        }
    }

    #[test]
    fn test_theme_names_contains_known_theme() {
        let names = Assets::theme_names();
        assert!(names.contains(&"dracula".to_string()));
    }

    #[test]
    fn test_get_theme_returns_data_for_known_theme() {
        let data = Assets::get_theme("dracula");
        assert!(data.is_some());
        assert!(!data.unwrap().is_empty());
    }

    #[test]
    fn test_get_theme_returns_none_for_unknown_theme() {
        assert!(Assets::get_theme("does-not-exist").is_none());
    }

    #[test]
    fn test_get_theme_empty_name_returns_none() {
        assert!(Assets::get_theme("").is_none());
    }

    #[test]
    fn test_all_theme_names_are_loadable() {
        for name in Assets::theme_names() {
            let data = Assets::get_theme(&name);
            assert!(data.is_some(), "theme {} should be loadable", name);
        }
    }
}
