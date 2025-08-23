use crate::core::CanonRepository;
use crate::utils::{CanonError, CanonResult};
use console::style;
use std::fs;

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

    // Create repository configuration with core dependencies
    let repo = CanonRepository::new();

    // Write canon.yml
    let yaml_content = serde_yaml::to_string(&repo).map_err(CanonError::Serialization)?;
    fs::write(&canon_yml_path, yaml_content).map_err(CanonError::Io)?;

    // Create .canon directory
    let canon_dir = current_dir.join(".canon");
    if !canon_dir.exists() {
        fs::create_dir(&canon_dir).map_err(CanonError::Io)?;
    }

    // Create .gitignore if it doesn't exist or update it
    add_to_gitignore(&current_dir)?;

    println!("{} Canon repository", style("Initialized").green().bold());
    println!();
    println!("Created:");
    println!("  • canon.yml (with core dependencies)");
    println!("  • .canon/ (dependency storage)");
    println!();
    println!("Core dependencies configured:");
    for dep in &repo.dependencies {
        println!("  • {}", style(dep).cyan());
    }
    println!();
    println!("Run {} to fetch dependencies", style("canon install").yellow());

    Ok(())
}

fn add_to_gitignore(dir: &std::path::Path) -> CanonResult<()> {
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
