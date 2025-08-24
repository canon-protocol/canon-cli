use std::process::Command;

fn main() {
    // Tell Cargo to rerun this build script if the git HEAD changes
    println!("cargo:rerun-if-changed=.git/HEAD");

    // Get git commit hash
    let git_hash = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .ok()
        .and_then(|output| {
            if output.status.success() {
                Some(
                    String::from_utf8(output.stdout)
                        .unwrap_or_default()
                        .trim()
                        .to_string(),
                )
            } else {
                None
            }
        })
        .unwrap_or_else(|| "unknown".to_string());

    // Get git branch
    let git_branch = Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .ok()
        .and_then(|output| {
            if output.status.success() {
                Some(
                    String::from_utf8(output.stdout)
                        .unwrap_or_default()
                        .trim()
                        .to_string(),
                )
            } else {
                None
            }
        })
        .unwrap_or_else(|| "unknown".to_string());

    // Pass git info to the compiler
    println!("cargo:rustc-env=GIT_HASH={}", git_hash);
    println!("cargo:rustc-env=GIT_BRANCH={}", git_branch);

    // Set build timestamp
    let build_time = chrono::Utc::now()
        .format("%Y-%m-%d %H:%M:%S UTC")
        .to_string();
    println!("cargo:rustc-env=BUILD_TIME={}", build_time);
}
