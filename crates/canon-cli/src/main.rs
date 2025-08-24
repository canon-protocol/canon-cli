use clap::Parser;
use std::process;

mod cli;
mod commands;
mod config;
mod core;
mod utils;

use cli::Cli;
use utils::error::CanonResult;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    if let Err(err) = run(cli).await {
        eprintln!("Error: {}", err);
        process::exit(1);
    }
}

async fn run(cli: Cli) -> CanonResult<()> {
    match cli.command {
        Some(command) => commands::handle_command(command).await,
        None => {
            println!("Canon Protocol CLI v{}", env!("CARGO_PKG_VERSION"));
            println!("Transform chaos into structured specifications");
            println!();
            if cli.verbose {
                println!("Build info:");
                println!(
                    "  Git commit: {}",
                    option_env!("GIT_HASH").unwrap_or("unknown")
                );
                println!(
                    "  Git branch: {}",
                    option_env!("GIT_BRANCH").unwrap_or("unknown")
                );
                println!(
                    "  Build time: {}",
                    option_env!("BUILD_TIME").unwrap_or("unknown")
                );
                println!();
            }
            println!("Use 'canon --help' for more information.");
            Ok(())
        }
    }
}
