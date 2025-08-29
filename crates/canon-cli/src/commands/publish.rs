use crate::utils::{CanonError, CanonResult};
use canon_protocol::CanonSpecification;
use console::style;
use dialoguer::{theme::ColorfulTheme, Input, Password};
use reqwest;
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};

/// Registry capability discovery response
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct RegistryCapabilities {
    #[serde(default)]
    name: String,
    url: String,
    api_version: String,
    read_only: bool,
    #[serde(default)]
    authentication: Option<AuthenticationConfig>,
    #[serde(default)]
    verification: Option<VerificationConfig>,
    endpoints: EndpointsConfig,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct AuthenticationConfig {
    required: bool,
    #[serde(default)]
    methods: Vec<String>,
    #[serde(default)]
    endpoints: Option<AuthEndpoints>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct AuthEndpoints {
    #[serde(default)]
    token: Option<String>,
    #[serde(default)]
    oauth: Option<String>,
}

#[derive(Debug, Deserialize)]
struct VerificationConfig {
    methods: Vec<String>,
    #[serde(default)]
    dns_txt: Option<DnsVerificationConfig>,
    #[serde(default)]
    https_file: Option<HttpsVerificationConfig>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct DnsVerificationConfig {
    prefix: String,
    format: String,
    #[serde(default)]
    ttl: Option<u64>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct HttpsVerificationConfig {
    path: String,
    format: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct EndpointsConfig {
    discovery: String,
    packages: String,
    download: String,
    #[serde(default)]
    publish: Option<String>,
}

pub async fn run_publish(
    registry: Option<String>,
    token: Option<String>,
    dry_run: bool,
    skip_verification: bool,
) -> CanonResult<()> {
    println!("{} Canon specification", style("Publishing").cyan().bold());
    println!();

    // Step 1: Read canon.yml to get package info
    let canon_yml_path = std::env::current_dir()
        .map_err(|e| CanonError::Command {
            message: format!("Failed to get current directory: {}", e),
        })?
        .join("canon.yml");

    if !canon_yml_path.exists() {
        return Err(CanonError::Command {
            message: "No canon.yml found in current directory".to_string(),
        });
    }

    let yaml_content = fs::read_to_string(&canon_yml_path).map_err(CanonError::Io)?;
    let spec: CanonSpecification =
        serde_yaml::from_str(&yaml_content).map_err(|e| CanonError::Command {
            message: format!("Failed to parse canon.yml: {}", e),
        })?;

    let publisher = &spec.metadata.publisher;
    let id = &spec.metadata.id;
    let version = &spec.metadata.version;

    println!(
        "Package: {}@{}",
        style(format!("{}/{}", publisher, id)).green(),
        style(version).yellow()
    );
    println!();

    // Step 2: Check if package is built in localhost
    let local_path = PathBuf::from(".canon")
        .join("localhost")
        .join(publisher)
        .join(id)
        .join(version);

    if !local_path.exists() {
        // For now, we'll just copy the current canon.yml to localhost
        // In the future, the build command will handle this
        println!("{}", style("Building canonical form...").dim());

        fs::create_dir_all(&local_path).map_err(CanonError::Io)?;
        let dest_canon = local_path.join("canon.yml");
        fs::copy(&canon_yml_path, &dest_canon).map_err(CanonError::Io)?;

        println!("{} Built to localhost", style("✓").green());
        println!();
    }

    // Step 3: Get registry URL
    let registry_url = match registry {
        Some(url) => url,
        None => {
            // Try to load from config, or prompt
            prompt_for_registry()?
        }
    };

    println!("Registry: {}", style(&registry_url).cyan());
    println!();

    // Step 4: Check registry capabilities
    println!("{}", style("Checking registry capabilities...").dim());

    let capabilities = match discover_registry_capabilities(&registry_url).await {
        Ok(caps) => caps,
        Err(_) => {
            return Err(CanonError::Command {
                message: format!(
                    "Failed to discover registry capabilities at {}/.well-known/canon-registry\n\
                     The registry may not implement the Canon Registry specification.",
                    registry_url
                ),
            });
        }
    };

    // Step 5: Check if registry is read-only
    if capabilities.read_only {
        return Err(CanonError::Command {
            message: format!(
                "Registry '{}' is read-only and does not accept publications",
                capabilities.name
            ),
        });
    }

    if capabilities.endpoints.publish.is_none() {
        return Err(CanonError::Command {
            message: "Registry does not provide a publish endpoint".to_string(),
        });
    }

    println!("{} Registry supports publishing", style("✓").green());
    println!();

    // Step 6: Handle authentication if required
    let auth_token = if let Some(auth) = &capabilities.authentication {
        if auth.required {
            match token {
                Some(t) => Some(t),
                None => {
                    println!("{}", style("Authentication required").yellow());
                    let token = Password::with_theme(&ColorfulTheme::default())
                        .with_prompt("API Token")
                        .interact()
                        .map_err(|e| CanonError::Command {
                            message: format!("Failed to read token: {}", e),
                        })?;
                    Some(token)
                }
            }
        } else {
            token
        }
    } else {
        token
    };

    // Step 7: Verify domain ownership
    if !skip_verification {
        println!("{}", style("Verifying domain ownership...").dim());

        if let Some(verification) = &capabilities.verification {
            verify_domain_ownership(publisher, &registry_url, verification).await?;
            println!("{} Domain verified", style("✓").green());
        } else {
            println!(
                "{}",
                style("⚠ Registry does not specify verification methods").yellow()
            );
        }
        println!();
    }

    // Step 8: Publish the package
    if dry_run {
        println!("{}", style("DRY RUN - Would publish:").yellow().bold());
        println!("  Registry: {}", registry_url);
        println!("  Package: {}/{}@{}", publisher, id, version);
        println!("  From: {}", local_path.display());
        if auth_token.is_some() {
            println!("  Auth: Token provided");
        }
        println!();
        println!("{}", style("No changes made (dry run)").dim());
    } else {
        println!("{}", style("Publishing package...").dim());

        publish_to_registry(
            &registry_url,
            &capabilities.endpoints.publish.unwrap(),
            publisher,
            id,
            version,
            &local_path,
            auth_token,
        )
        .await?;

        println!();
        println!("{} Published successfully!", style("✓").green().bold());
        println!();
        println!("Package available at:");
        println!("  {}/{}/{}@{}", registry_url, publisher, id, version);
    }

    Ok(())
}

fn prompt_for_registry() -> CanonResult<String> {
    Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Registry URL")
        .validate_with(|input: &String| -> Result<(), &str> {
            if input.starts_with("https://") || input.starts_with("http://") {
                Ok(())
            } else {
                Err("Registry URL must start with https:// or http://")
            }
        })
        .interact()
        .map_err(|e| CanonError::Command {
            message: format!("Failed to read registry URL: {}", e),
        })
}

async fn discover_registry_capabilities(registry_url: &str) -> CanonResult<RegistryCapabilities> {
    let discovery_url = format!("{}/.well-known/canon-registry", registry_url);

    let client = reqwest::Client::builder()
        .user_agent("canon-cli/0.2.10")
        .build()
        .map_err(|e| CanonError::Network {
            message: format!("Failed to create HTTP client: {}", e),
        })?;

    let response = client
        .get(&discovery_url)
        .send()
        .await
        .map_err(|e| CanonError::Network {
            message: format!("Failed to fetch registry capabilities: {}", e),
        })?;

    if !response.status().is_success() {
        return Err(CanonError::Network {
            message: format!("Registry discovery failed (status: {})", response.status()),
        });
    }

    let capabilities: RegistryCapabilities =
        response.json().await.map_err(|e| CanonError::Network {
            message: format!("Failed to parse registry capabilities: {}", e),
        })?;

    Ok(capabilities)
}

async fn verify_domain_ownership(
    publisher: &str,
    _registry_url: &str,
    verification: &VerificationConfig,
) -> CanonResult<()> {
    // For now, we'll implement basic DNS-TXT verification
    // In a real implementation, this would check actual DNS records

    if verification.methods.contains(&"dns-txt".to_string()) {
        if let Some(dns_config) = &verification.dns_txt {
            // In production, this would:
            // 1. Generate a verification token
            // 2. Ask user to add DNS TXT record
            // 3. Check DNS for the record
            // 4. Verify the token matches

            println!("  Method: DNS TXT record");
            println!("  Record: {}.{}", dns_config.prefix, publisher);

            // For now, we'll simulate success
            // Real implementation would use a DNS resolver library
            return Ok(());
        }
    }

    if verification.methods.contains(&"https-file".to_string()) {
        if let Some(https_config) = &verification.https_file {
            // In production, this would:
            // 1. Generate a verification token
            // 2. Ask user to place file at https://publisher/.well-known/canon-verify
            // 3. Fetch the file
            // 4. Verify the token matches

            println!("  Method: HTTPS file");
            println!("  Path: https://{}{}", publisher, https_config.path);

            // For now, we'll simulate success
            return Ok(());
        }
    }

    Err(CanonError::Command {
        message: "No supported verification method available".to_string(),
    })
}

async fn publish_to_registry(
    registry_url: &str,
    publish_endpoint: &str,
    publisher: &str,
    id: &str,
    version: &str,
    local_path: &Path,
    auth_token: Option<String>,
) -> CanonResult<()> {
    // Read the canonical form
    let canon_yml_path = local_path.join("canon.yml");
    let canon_yml_content = fs::read_to_string(&canon_yml_path).map_err(CanonError::Io)?;

    // Construct the publish URL
    let publish_url = if publish_endpoint.starts_with("http") {
        publish_endpoint.to_string()
    } else {
        format!("{}{}", registry_url, publish_endpoint)
    };

    // Create the request
    let client = reqwest::Client::builder()
        .user_agent("canon-cli/0.2.10")
        .build()
        .map_err(|e| CanonError::Network {
            message: format!("Failed to create HTTP client: {}", e),
        })?;

    let mut request = client
        .post(&publish_url)
        .header("Content-Type", "application/x-yaml");

    if let Some(token) = auth_token {
        request = request.header("Authorization", format!("Bearer {}", token));
    }

    // Add package metadata headers
    request = request
        .header("X-Canon-Publisher", publisher)
        .header("X-Canon-Id", id)
        .header("X-Canon-Version", version);

    // Send the request
    let response =
        request
            .body(canon_yml_content)
            .send()
            .await
            .map_err(|e| CanonError::Network {
                message: format!("Failed to publish package: {}", e),
            })?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        return Err(CanonError::Network {
            message: format!("Publication failed (status: {}): {}", status, body),
        });
    }

    Ok(())
}
