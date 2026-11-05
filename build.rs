// records build info for `neofetch` (see src/routes/sysinfo.rs)
use std::process::Command;

fn main() {
    let rustc = std::env::var("RUSTC").unwrap_or_else(|_| "rustc".into());
    let version = Command::new(rustc)
        .arg("--version")
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .unwrap_or_default();
    // "rustc 1.99.0 (b940084d7 2026-09-28)" -> "rustc 1.99.0"
    let version: Vec<&str> = version.split_whitespace().take(2).collect();
    println!("cargo:rustc-env=BUILD_RUSTC={}", version.join(" "));

    let date = Command::new("date")
        .args(["-u", "+%Y-%m-%d"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .unwrap_or_default();
    println!("cargo:rustc-env=BUILD_DATE={}", date.trim());
    println!("cargo:rerun-if-changed=build.rs");
}
