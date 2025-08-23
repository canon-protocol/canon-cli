use crate::core::{CanonRepository, Dependency};
use crate::utils::{CanonError, CanonResult};
use console::style;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs;
use url::Url;

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

    // Read canon.yml
    let yaml_content = fs::read_to_string(&canon_yml_path).map_err(CanonError::Io)?;
    let repo: CanonRepository =
        serde_yaml::from_str(&yaml_content).map_err(|e| CanonError::Config {
            message: format!("Failed to parse canon.yml: {}", e),
        })?;

    // Create .canon directory if it doesn't exist
    let canon_dir = current_dir.join(".canon");
    if !canon_dir.exists() {
        fs::create_dir(&canon_dir).map_err(CanonError::Io)?;
    }

    // Extract registry domain from URL
    let registry_domain = extract_domain(&repo.registry.default)?;

    println!(
        "{} dependencies from {}",
        style("Installing").green().bold(),
        style(&repo.registry.default).cyan()
    );
    println!();

    let total_deps = repo.dependencies.len();
    let mut installed = 0;
    let mut skipped = 0;
    let mut failed = 0;

    for dep_uri in &repo.dependencies {
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
        if dep.is_installed(&registry_domain) {
            println!("  {} {} (already installed)", style("✓").green(), dep_uri);
            skipped += 1;
            continue;
        }

        // Install dependency
        match install_dependency(&dep, &repo.registry.default, &registry_domain).await {
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

async fn install_dependency(
    dep: &Dependency,
    registry_base: &str,
    registry_domain: &str,
) -> CanonResult<()> {
    let local_path = dep.local_path(registry_domain);

    // Create parent directories
    if let Some(parent) = local_path.parent() {
        fs::create_dir_all(parent).map_err(CanonError::Io)?;
    }

    // Construct URLs for files to fetch
    let base_url = dep.registry_url(registry_base);

    // Create progress bar
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("    {spinner:.green} Fetching {msg}")
            .unwrap(),
    );
    pb.set_message(format!("{}/{}", dep.publisher, dep.name));

    // Fetch canon.yml
    let canon_yml_url = format!("{}canon.yml", base_url);
    let canon_yml_content = fetch_file(&canon_yml_url).await?;

    // Fetch canon-manifest.yml
    let manifest_url = format!("{}canon-manifest.yml", base_url);
    let manifest_content = fetch_file(&manifest_url).await.ok(); // Optional file

    pb.finish_and_clear();

    // Create local directory and save files
    fs::create_dir_all(&local_path).map_err(CanonError::Io)?;

    let canon_yml_path = local_path.join("canon.yml");
    fs::write(canon_yml_path, canon_yml_content).map_err(CanonError::Io)?;

    if let Some(manifest) = manifest_content {
        let manifest_path = local_path.join("canon-manifest.yml");
        fs::write(manifest_path, manifest).map_err(CanonError::Io)?;
    }

    Ok(())
}

async fn fetch_file(url: &str) -> CanonResult<String> {
    let client = reqwest::Client::builder()
        .user_agent("canon-cli")
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .map_err(CanonError::Http)?;

    let response = client.get(url).send().await.map_err(CanonError::Http)?;

    if !response.status().is_success() {
        return Err(CanonError::RegistryError {
            url: url.to_string(),
            status: response.status().to_string(),
        });
    }

    response.text().await.map_err(CanonError::Http)
}

fn extract_domain(url_str: &str) -> CanonResult<String> {
    let url = Url::parse(url_str).map_err(|e| CanonError::Config {
        message: format!("Invalid registry URL: {}", e),
    })?;

    url.host_str()
        .map(|h| h.to_string())
        .ok_or_else(|| CanonError::Config {
            message: "Registry URL must have a host".to_string(),
        })
}
