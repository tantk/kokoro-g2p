//! Build script for kokoro-g2p
//!
//! This script handles:
//! - Android NDK configuration
//! - Dictionary embedding

fn main() {
    // Re-run if dictionaries change
    println!("cargo:rerun-if-changed=dictionaries/us_gold.json");
    println!("cargo:rerun-if-changed=dictionaries/us_silver.json");
    println!("cargo:rerun-if-changed=dictionaries/gb_gold.json");
    println!("cargo:rerun-if-changed=dictionaries/gb_silver.json");

    // Android-specific configuration
    #[cfg(target_os = "android")]
    {
        println!("cargo:rustc-link-lib=log");
    }
}
