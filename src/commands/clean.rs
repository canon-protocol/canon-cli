use crate::utils::{CanonError, CanonResult};
use console::style;
use std::fs;
use std::path::Path;

pub async fn run_clean(all: bool, purge: bool) -> CanonResult<()> {
    // Check if we're in a Canon project
    let canon_yml = Path::new("canon.yml");
    let canon_dir = Path::new(".canon");

    if !canon_yml.exists() && !canon_dir.exists() {
        return Err(CanonError::Command {
            message: "No Canon project found in current directory".to_string(),
        });
    }

    if purge {
        // Remove everything - complete uninstall
        println!("{} Canon project...", style("Purging").red().bold());

        let mut removed_items = Vec::new();

        if canon_dir.exists() {
            fs::remove_dir_all(canon_dir).map_err(CanonError::Io)?;
            removed_items.push(".canon/");
        }

        if canon_yml.exists() {
            fs::remove_file(canon_yml).map_err(CanonError::Io)?;
            removed_items.push("canon.yml");
        }

        // Also remove from .gitignore if present
        let gitignore = Path::new(".gitignore");
        if gitignore.exists() {
            let content = fs::read_to_string(gitignore).map_err(CanonError::Io)?;
            let lines: Vec<&str> = content.lines().filter(|line| *line != ".canon/").collect();
            let new_content = lines.join("\n");

            // Only update if we actually removed something
            if new_content.len() != content.len() {
                fs::write(gitignore, new_content).map_err(CanonError::Io)?;
                removed_items.push(".gitignore entry");
            }
        }

        if removed_items.is_empty() {
            println!("Nothing to remove");
        } else {
            println!();
            println!("Removed:");
            for item in removed_items {
                println!("  {} {}", style("✓").green(), item);
            }
            println!();
            println!(
                "{}",
                style("Canon has been completely removed from this project").yellow()
            );
        }
    } else if all {
        // Remove all cached data
        println!("{} all cached data...", style("Removing").yellow().bold());

        if canon_dir.exists() {
            fs::remove_dir_all(canon_dir).map_err(CanonError::Io)?;
            println!("  {} Removed .canon/", style("✓").green());
            println!();
            println!("All cached data has been removed");
            println!(
                "Run {} to re-download dependencies",
                style("canon install").cyan()
            );
        } else {
            println!("No cached data found");
        }
    } else {
        // Default: just remove specs
        println!(
            "{} cached specifications...",
            style("Cleaning").cyan().bold()
        );

        let specs_dir = canon_dir.join("specs");
        if specs_dir.exists() {
            fs::remove_dir_all(&specs_dir).map_err(CanonError::Io)?;
            println!("  {} Removed .canon/specs/", style("✓").green());
            println!();
            println!("Cached specifications removed");
            println!(
                "Run {} to re-download dependencies",
                style("canon install").cyan()
            );
        } else {
            println!("No cached specifications found");
        }
    }

    Ok(())
}