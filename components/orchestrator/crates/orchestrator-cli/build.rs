use std::process::Command;

fn main() {
    // Get git commit hash
    let commit = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    // Get hostname
    let host = hostname::get()
        .ok()
        .and_then(|h| h.into_string().ok())
        .unwrap_or_else(|| "unknown".to_string());

    // Get build time
    let build_time = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC").to_string();

    println!("cargo:rustc-env=BUILD_COMMIT={commit}");
    println!("cargo:rustc-env=BUILD_HOST={host}");
    println!("cargo:rustc-env=BUILD_TIME={build_time}");
    println!("cargo:rerun-if-changed=.git/HEAD");
}
