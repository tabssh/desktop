//! Build script for TabSSH

use std::{fs, path::Path};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=assets/themes/");
    println!("cargo:rerun-if-changed=release.txt");
    println!("cargo:rerun-if-changed=site.txt");

    if Path::new("release.txt").exists() {
        let version = fs::read_to_string("release.txt").unwrap();
        println!("cargo:rustc-env=APP_VERSION={}", version.trim());
    }

    if Path::new("site.txt").exists() {
        let site = fs::read_to_string("site.txt").unwrap();
        println!("cargo:rustc-env=APP_OFFICIAL_SITE={}", site.trim());
    }

    // Platform-specific link libraries
    #[cfg(target_os = "macos")]
    {
        println!("cargo:rustc-link-lib=framework=Security");
    }

    #[cfg(target_os = "windows")]
    {
        println!("cargo:rustc-link-lib=advapi32");
    }
}
