use crate::core::{CanonSpecification, Dependency};
use crate::utils::{CanonError, CanonResult};
use console::style;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs;
use std::path::Path;

pub async fn run_install() -> CanonResult<()> {
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

    // Read and parse canon.yml as a Canon specification
    let yaml_content = fs::read_to_string(&canon_yml_path).map_err(CanonError::Io)?;
    let spec: CanonSpecification =
        serde_yaml::from_str(&yaml_content).map_err(|e| CanonError::Config {
            message: format!("Failed to parse canon.yml: {}", e),
        })?;

    // Extract dependencies from the content field
    let dependencies = spec
        .content
        .get("dependencies")
        .and_then(|v| v.as_sequence())
        .map(|seq| {
            seq.iter()
                .filter_map(|v| v.as_str())
                .map(String::from)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    if dependencies.is_empty() {
        println!("{} No dependencies to install", style("✓").green().bold());
        return Ok(());
    }

    // Create .canon directory if it doesn't exist
    let canon_dir = current_dir.join(".canon");
    if !canon_dir.exists() {
        fs::create_dir(&canon_dir).map_err(CanonError::Io)?;
    }

    println!(
        "{} {} dependencies",
        style("Installing").green().bold(),
        dependencies.len()
    );
    println!();

    let total_deps = dependencies.len();
    let mut installed = 0;
    let mut skipped = 0;
    let mut failed = 0;

    for dep_uri in &dependencies {
        // Parse dependency
        let dep = match Dependency::parse(dep_uri) {
            Ok(d) => d,
            Err(e) => {
                eprintln!("  {} {} - {}", style("✗").red(), dep_uri, e);
                failed += 1;
                continue;
            }
        };

        // Check if already installed
        if dep.is_installed() {
            println!("  {} {} (already installed)", style("✓").green(), dep_uri);
            skipped += 1;
            continue;
        }

        // Install dependency
        match install_dependency(&dep, &current_dir).await {
            Ok(_) => {
                println!("  {} {}", style("✓").green(), dep_uri);
                installed += 1;
            }
            Err(e) => {
                eprintln!("  {} {} - {}", style("✗").red(), dep_uri, e);
                failed += 1;
            }
        }
    }

    println!();
    println!("{}", style("Summary:").bold());
    println!("  • {} dependencies total", total_deps);
    if installed > 0 {
        println!("  • {} installed", style(installed).green());
    }
    if skipped > 0 {
        println!("  • {} already installed", style(skipped).yellow());
    }
    if failed > 0 {
        println!("  • {} failed", style(failed).red());
    }

    if failed > 0 {
        Err(CanonError::Command {
            message: format!("{} dependencies failed to install", failed),
        })
    } else {
        Ok(())
    }
}

async fn install_dependency(dep: &Dependency, base_dir: &Path) -> CanonResult<()> {
    let local_path = base_dir.join(dep.local_path());

    // Create parent directories
    if let Some(parent) = local_path.parent() {
        fs::create_dir_all(parent).map_err(CanonError::Io)?;
    }

    // Create progress bar
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("    {spinner:.green} Fetching {msg}")
            .unwrap(),
    );
    pb.set_message(format!("{}/{}", dep.publisher, dep.id));

    // Fetch canon.yml from canon.canon-protocol.org
    let canon_yml_url = dep.canon_url();
    let canon_yml_content = fetch_file(&canon_yml_url).await?;

    // Optionally fetch manifest and signature (these might not exist for all specs)
    let manifest_url = canon_yml_url.replace("/canon.yml", "/canon-manifest.yml");
    let manifest_content = fetch_file(&manifest_url).await.ok();

    let signature_url = canon_yml_url.replace("/canon.yml", "/canon-signature.yml");
    let signature_content = fetch_file(&signature_url).await.ok();

    pb.finish_and_clear();

    // Create local directory and save files
    fs::create_dir_all(&local_path).map_err(CanonError::Io)?;

    let canon_yml_path = local_path.join("canon.yml");
    fs::write(canon_yml_path, canon_yml_content).map_err(CanonError::Io)?;

    if let Some(manifest) = manifest_content {
        let manifest_path = local_path.join("canon-manifest.yml");
        fs::write(manifest_path, manifest).map_err(CanonError::Io)?;
    }

    if let Some(signature) = signature_content {
        let signature_path = local_path.join("canon-signature.yml");
        fs::write(signature_path, signature).map_err(CanonError::Io)?;
    }

    Ok(())
}

async fn fetch_file(url: &str) -> CanonResult<String> {
    let client = reqwest::Client::builder()
        .user_agent("canon-cli/0.2.6")
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .map_err(|e| CanonError::Network {
            message: format!("Failed to create HTTP client: {}", e),
        })?;

    let response = client
        .get(url)
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

    response.text().await.map_err(|e| CanonError::Network {
        message: format!("Failed to read response: {}", e),
    })
}
