//! Build script for TabSSH

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    
    // Embed themes
    println!("cargo:rerun-if-changed=assets/themes/");
    
    // Platform-specific build configurations
    #[cfg(target_os = "macos")]
    {
        println!("cargo:rustc-link-lib=framework=Security");
    }
    
    #[cfg(target_os = "windows")]
    {
        println!("cargo:rustc-link-lib=advapi32");
    }
}
