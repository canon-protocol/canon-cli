pub mod add;
pub mod build;
pub mod clean;
pub mod config;
pub mod init;
pub mod install;
pub mod validate;

use crate::cli::{Commands, ConfigCommands};
use crate::utils::CanonResult;

pub async fn handle_command(command: Commands) -> CanonResult<()> {
    match command {
        Commands::Init {
            force,
            non_interactive,
        } => init::run_init(force, non_interactive).await,
        Commands::Install => install::run_install().await,
        Commands::Add { uri } => add::run_add(&uri).await,
        Commands::Validate {
            path,
            strict,
            schema,
            fix,
        } => validate::run_validate(path, strict, schema, fix).await,
        Commands::Build {
            engine,
            output,
            sign,
            key,
            no_cache,
            parallel,
        } => build::run_build(engine, output, sign, key, no_cache, parallel).await,
        Commands::Clean { all, purge } => clean::run_clean(all, purge).await,
        Commands::Config { command } => match command {
            ConfigCommands::Get { key } => config::get_config(&key).await,
            ConfigCommands::Set { key, value } => config::set_config(&key, &value).await,
            ConfigCommands::Unset { key } => config::unset_config(&key).await,
            ConfigCommands::List => config::list_config().await,
            ConfigCommands::Edit => config::edit_config().await,
        },
    }
}
