use crate::core::CanonSpecification;
use crate::utils::{CanonError, CanonResult};
use std::fs;

pub async fn run_init(
    name: Option<String>,
    spec_type: String,
    author: Option<String>,
    license: Option<String>,
    _template: Option<String>,
    force: bool,
) -> CanonResult<()> {
    let current_dir = std::env::current_dir().map_err(|e| CanonError::Command {
        message: format!("Failed to get current directory: {}", e),
    })?;

    let spec_name = name.unwrap_or_else(|| {
        current_dir
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("canon-spec")
            .to_string()
    });

    let canon_yml_path = current_dir.join("canon.yml");

    // Check if canon.yml already exists
    if canon_yml_path.exists() && !force {
        return Err(CanonError::Command {
            message: "canon.yml already exists. Use --force to overwrite.".to_string(),
        });
    }

    // Get author from git config if not provided
    let final_author = match author {
        Some(a) => Some(a),
        None => get_git_user_name().await,
    };

    // Create specification
    let spec = CanonSpecification::new(spec_name.clone(), spec_type, final_author, license);

    // Write canon.yml
    let yaml_content = serde_yaml::to_string(&spec).map_err(CanonError::Serialization)?;

    fs::write(&canon_yml_path, yaml_content).map_err(CanonError::Io)?;

    // Create sources directory
    let sources_dir = current_dir.join("sources");
    if !sources_dir.exists() {
        fs::create_dir(&sources_dir).map_err(CanonError::Io)?;
    }

    // Create .canonignore file
    let canonignore_path = current_dir.join(".canonignore");
    if !canonignore_path.exists() {
        let ignore_content = "# Canon ignore patterns\n.canon/\n*.tmp\n.DS_Store\n";
        fs::write(&canonignore_path, ignore_content).map_err(CanonError::Io)?;
    }

    println!("Initialized Canon specification '{}'", spec_name);
    println!("Created:");
    println!("  - canon.yml");
    println!("  - sources/");
    println!("  - .canonignore");

    Ok(())
}

async fn get_git_user_name() -> Option<String> {
    use std::process::Command;

    let output = Command::new("git")
        .args(["config", "user.name"])
        .output()
        .ok()?;

    if output.status.success() {
        Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        None
    }
}
