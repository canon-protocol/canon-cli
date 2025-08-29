use crate::utils::{CanonError, CanonResult};
use canon_protocol::{CanonSpecification, Dependency, SpecificationMetadata};
use console::style;
use dialoguer::{theme::ColorfulTheme, Input};
use indicatif::{ProgressBar, ProgressStyle};
use reqwest;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub async fn run_init(force: bool, non_interactive: bool) -> CanonResult<()> {
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

    // Get project metadata (interactive or defaults)
    let (project_id, project_title, project_description, publisher, version) = if non_interactive {
        get_default_values(&current_dir)
    } else {
        get_interactive_values(&current_dir)?
    };

    println!();

    // Create progress bar
    let pb = ProgressBar::new(3);
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

    // Create the project specification using Canon Protocol format
    pb.set_message("Creating canon.yml");
    let project_spec = create_project_spec(
        &project_id,
        &version,
        &publisher,
        &project_title,
        &project_description,
    );

    // Write canon.yml with proper formatting
    let yaml_content = serde_yaml::to_string(&project_spec).map_err(CanonError::Serialization)?;
    fs::write(&canon_yml_path, yaml_content).map_err(CanonError::Io)?;

    // Create .canon directory
    let canon_dir = current_dir.join(".canon");
    if !canon_dir.exists() {
        fs::create_dir(&canon_dir).map_err(CanonError::Io)?;
    }

    // Create or update .gitignore
    add_to_gitignore(&current_dir)?;

    pb.inc(1);
    pb.finish_and_clear();

    println!("{} Canon project", style("✓ Initialized").green().bold());
    println!();
    println!("Created:");
    println!("  • canon.yml");
    println!("  • .canon/");
    println!("  • .gitignore (updated with .canon/)");
    println!();
    println!("Downloaded:");
    println!("  • canon-protocol.org/project@1.0.0");
    println!("  • canon-protocol.org/type@1.0.0");
    println!();
    println!("Next steps:");
    if publisher == "example.com" {
        println!(
            "  1. Update the {} field in canon.yml to your domain",
            style("publisher").yellow()
        );
        println!(
            "  2. Run {} to validate your specification",
            style("canon validate").yellow()
        );
        println!(
            "  3. Run {} to add dependencies as needed",
            style("canon add <uri>").yellow()
        );
    } else {
        println!(
            "  1. Run {} to validate your specification",
            style("canon validate").yellow()
        );
        println!(
            "  2. Run {} to add dependencies as needed",
            style("canon add <uri>").yellow()
        );
    }

    Ok(())
}

fn create_project_spec(
    id: &str,
    version: &str,
    publisher: &str,
    title: &str,
    description: &str,
) -> CanonSpecification {
    let mut content = HashMap::new();

    // Add base dependencies - every project needs at least the type definition
    let deps = vec![serde_yaml::Value::String(
        "canon-protocol.org/type@1.0.0".to_string(),
    )];

    content.insert(
        "dependencies".to_string(),
        serde_yaml::Value::Sequence(deps),
    );

    CanonSpecification {
        canon: "1.0".to_string(),
        r#type: "canon-protocol.org/project@1.0.0".to_string(),
        metadata: SpecificationMetadata {
            id: id.to_string(),
            version: version.to_string(),
            publisher: publisher.to_string(),
            title: Some(title.to_string()),
            description: Some(description.to_string()),
        },
        includes: None,
        schema: None,
        content,
    }
}

fn format_title(id: &str) -> String {
    id.split('-')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

async fn download_specification(dep: &Dependency, base_dir: &Path) -> CanonResult<()> {
    // Create local directory for the specification
    let local_path = base_dir.join(dep.local_path());
    fs::create_dir_all(&local_path).map_err(CanonError::Io)?;

    // Download the canon.yml file
    let url = dep.canon_url();
    let client = reqwest::Client::builder()
        .user_agent("canon-cli/0.2.8")
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

fn get_default_values(current_dir: &Path) -> (String, String, String, String, String) {
    let project_id = current_dir
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("my-project")
        .to_string()
        .to_lowercase()
        .replace([' ', '_'], "-");

    let project_title = format_title(&project_id);
    let project_description = "A Canon Protocol project".to_string();
    let publisher = "example.com".to_string();
    let version = "0.1.0".to_string();

    (
        project_id,
        project_title,
        project_description,
        publisher,
        version,
    )
}

fn get_interactive_values(
    current_dir: &Path,
) -> CanonResult<(String, String, String, String, String)> {
    let project_id = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Project ID")
        .default(
            current_dir
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("my-project")
                .to_string()
                .to_lowercase()
                .replace([' ', '_'], "-"),
        )
        .validate_with(|input: &String| -> Result<(), &str> {
            if input
                .chars()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
            {
                Ok(())
            } else {
                Err("Project ID must be lowercase alphanumeric with hyphens only")
            }
        })
        .interact()
        .map_err(|e| CanonError::Command {
            message: format!("Failed to read input: {}", e),
        })?;

    let project_title = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Project title")
        .default(format_title(&project_id))
        .interact()
        .map_err(|e| CanonError::Command {
            message: format!("Failed to read input: {}", e),
        })?;

    let project_description = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Project description")
        .default("A Canon Protocol project".to_string())
        .interact()
        .map_err(|e| CanonError::Command {
            message: format!("Failed to read input: {}", e),
        })?;

    let publisher = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Publisher (your domain or subdomain)")
        .default("example.com".to_string())
        .validate_with(|input: &String| -> Result<(), &str> {
            if input.contains('.') || input == "localhost" {
                Ok(())
            } else {
                Err("Publisher should be a valid domain or subdomain")
            }
        })
        .interact()
        .map_err(|e| CanonError::Command {
            message: format!("Failed to read input: {}", e),
        })?;

    let version = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Initial version")
        .default("0.1.0".to_string())
        .validate_with(|input: &String| -> Result<(), &str> {
            let parts: Vec<&str> = input.split('.').collect();
            if parts.len() == 3 && parts.iter().all(|p| p.parse::<u32>().is_ok()) {
                Ok(())
            } else {
                Err("Version must be semantic version (MAJOR.MINOR.PATCH)")
            }
        })
        .interact()
        .map_err(|e| CanonError::Command {
            message: format!("Failed to read input: {}", e),
        })?;

    Ok((
        project_id,
        project_title,
        project_description,
        publisher,
        version,
    ))
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
