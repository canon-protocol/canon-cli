use crate::core::{CanonRepository, Dependency};
use crate::utils::{CanonError, CanonResult};
use console::style;
use std::fs;

pub async fn run_add(uri: &str) -> CanonResult<()> {
    let current_dir = std::env::current_dir().map_err(|e| CanonError::Command {
        message: format!("Failed to get current directory: {}", e),
    })?;

    let canon_yml_path = current_dir.join("canon.yml");

    // Check if canon.yml exists
    if !canon_yml_path.exists() {
        return Err(CanonError::Command {
            message: "No canon.yml found. Run 'canon init' first.".to_string(),
        });
    }

    // Validate the dependency URI format
    Dependency::parse(uri)?;

    // Read canon.yml
    let yaml_content = fs::read_to_string(&canon_yml_path).map_err(CanonError::Io)?;
    let mut repo: CanonRepository =
        serde_yaml::from_str(&yaml_content).map_err(|e| CanonError::Config {
            message: format!("Failed to parse canon.yml: {}", e),
        })?;

    // Check if dependency already exists
    if repo.dependencies.contains(&uri.to_string()) {
        println!(
            "{} {} already exists in canon.yml",
            style("Dependency").yellow(),
            style(uri).cyan()
        );
        return Ok(());
    }

    // Add the dependency
    repo.add_dependency(uri.to_string());

    // Write updated canon.yml
    let yaml_content = serde_yaml::to_string(&repo).map_err(CanonError::Serialization)?;
    fs::write(&canon_yml_path, yaml_content).map_err(CanonError::Io)?;

    println!(
        "{} {} to canon.yml",
        style("Added").green().bold(),
        style(uri).cyan()
    );
    println!();
    println!(
        "Running {} to fetch the new dependency...",
        style("canon install").yellow()
    );
    println!();

    // Run install to fetch the new dependency
    crate::commands::install::run_install().await?;

    Ok(())
}

