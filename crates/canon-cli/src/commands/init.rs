use crate::utils::{CanonError, CanonResult};
use canon_protocol::{CanonSpecification, Dependency, SpecificationMetadata};
use console::style;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub async fn run_init(force: bool) -> CanonResult<()> {
    let current_dir = std::env::current_dir().map_err(|e| CanonError::Command {
        message: format!("Failed to get current directory: {}", e),
    })?;

    let canon_yml_path = current_dir.join("canon.yml");

    // Check if canon.yml already exists
    if canon_yml_path.exists() && !force {
        return Err(CanonError::Command {
            message: "canon.yml already exists. Use --force to overwrite.".to_string(),
        });
    }

    println!("{} Canon project", style("Initializing").cyan().bold());
    println!();

    // Create progress bar
    let pb = ProgressBar::new(2);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} {msg}")
            .unwrap()
            .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ "),
    );

    // Download the project type specification
    pb.set_message("Fetching canon-protocol.org/project@1.0.0");
    let project_dep =
        Dependency::parse("canon-protocol.org/project@1.0.0").map_err(CanonError::Protocol)?;
    download_specification(&project_dep, &current_dir).await?;
    pb.inc(1);

    // Download the type meta-type specification
    pb.set_message("Fetching canon-protocol.org/type@1.0.0");
    let type_dep =
        Dependency::parse("canon-protocol.org/type@1.0.0").map_err(CanonError::Protocol)?;
    download_specification(&type_dep, &current_dir).await?;
    pb.inc(1);

    pb.finish_and_clear();

    // Create the project specification using Canon Protocol format
    let project_spec = CanonSpecification {
        canon: "1.0".to_string(),
        r#type: "canon-protocol.org/project@1.0.0".to_string(),
        metadata: SpecificationMetadata {
            id: "my-project".to_string(),
            version: "0.1.0".to_string(),
            publisher: "example.com".to_string(), // User should update this
            title: Some("My Canon Project".to_string()),
            description: Some("A new Canon Protocol project".to_string()),
        },
        includes: None,
        schema: None,
        content: {
            let mut content = HashMap::new();
            // Add project-specific fields based on the project type
            content.insert(
                "dependencies".to_string(),
                serde_yaml::Value::Sequence(vec![serde_yaml::Value::String(
                    "canon-protocol.org/type@1.0.0".to_string(),
                )]),
            );
            content
        },
    };

    // Write canon.yml with proper formatting
    let yaml_content = serde_yaml::to_string(&project_spec).map_err(CanonError::Serialization)?;
    fs::write(&canon_yml_path, yaml_content).map_err(CanonError::Io)?;

    // Create .canon directory
    let canon_dir = current_dir.join(".canon");
    if !canon_dir.exists() {
        fs::create_dir(&canon_dir).map_err(CanonError::Io)?;
    }

    // Create .gitignore if it doesn't exist or update it
    add_to_gitignore(&current_dir)?;

    println!("{} Canon project", style("Initialized").green().bold());
    println!();
    println!("Created:");
    println!("  • canon.yml (Canon Protocol project)");
    println!("  • .canon/ (dependency storage)");
    println!();
    println!("Downloaded specifications:");
    println!("  • canon-protocol.org/project@1.0.0");
    println!("  • canon-protocol.org/type@1.0.0");
    println!();
    println!("Next steps:");
    println!("  1. Update the 'publisher' field in canon.yml to your domain");
    println!("  2. Update the 'id' field to your project identifier");
    println!(
        "  3. Run {} to fetch any additional dependencies",
        style("canon install").yellow()
    );

    Ok(())
}

async fn download_specification(dep: &Dependency, base_dir: &Path) -> CanonResult<()> {
    // Create local directory for the specification
    let local_path = base_dir.join(dep.local_path());
    fs::create_dir_all(&local_path).map_err(CanonError::Io)?;

    // Download the canon.yml file
    let url = dep.canon_url();
    let client = reqwest::Client::builder()
        .user_agent("canon-cli/0.2.6")
        .build()
        .map_err(|e| CanonError::Network {
            message: format!("Failed to create HTTP client: {}", e),
        })?;

    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| CanonError::Network {
            message: format!("Failed to fetch {}: {}", url, e),
        })?;

    if !response.status().is_success() {
        return Err(CanonError::Network {
            message: format!("Failed to fetch {} (status: {})", url, response.status()),
        });
    }

    let content = response.text().await.map_err(|e| CanonError::Network {
        message: format!("Failed to read response: {}", e),
    })?;

    // Save the specification
    let spec_file = local_path.join("canon.yml");
    fs::write(&spec_file, content).map_err(CanonError::Io)?;

    Ok(())
}

fn add_to_gitignore(dir: &Path) -> CanonResult<()> {
    let gitignore_path = dir.join(".gitignore");
    let canon_entry = ".canon/";

    if gitignore_path.exists() {
        let content = fs::read_to_string(&gitignore_path).map_err(CanonError::Io)?;
        if !content.contains(canon_entry) {
            let new_content = if content.ends_with('\n') {
                format!("{}{}\n", content, canon_entry)
            } else {
                format!("{}\n{}\n", content, canon_entry)
            };
            fs::write(&gitignore_path, new_content).map_err(CanonError::Io)?;
        }
    } else {
        fs::write(&gitignore_path, format!("{}\n", canon_entry)).map_err(CanonError::Io)?;
    }

    Ok(())
}
