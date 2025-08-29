use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "canon")]
#[command(author = "Paul Fryer <https://github.com/paulfryer>")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "Canon Protocol CLI - Transform chaos into structured specifications")]
#[command(long_about = None)]
pub struct Cli {
    /// Enable verbose output
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// Suppress non-error output
    #[arg(short, long, global = true)]
    pub quiet: bool,

    /// Use custom config file
    #[arg(long, global = true, value_name = "PATH")]
    pub config: Option<String>,

    /// Override default registry
    #[arg(long, global = true, value_name = "URL")]
    pub registry: Option<String>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize new Canon repository with core dependencies
    Init {
        /// Overwrite existing canon.yml
        #[arg(long)]
        force: bool,

        /// Skip interactive prompts and use defaults
        #[arg(long)]
        non_interactive: bool,
    },

    /// Install dependencies from canon.yml
    Install,

    /// Add a new dependency to canon.yml
    Add {
        /// Dependency URI (e.g., "api.io/openapi@2.0.0")
        uri: String,
    },

    /// Validate specification syntax and structure
    Validate {
        /// Path to canon.yml or directory (default: current directory)
        path: Option<String>,

        /// Enable strict validation (warnings become errors)
        #[arg(long)]
        strict: bool,

        /// Validate against specific schema version
        #[arg(long)]
        schema: Option<String>,

        /// Attempt to fix common issues automatically
        #[arg(long)]
        fix: bool,
    },

    /// Generate canonical artifacts, manifest, and signature
    Build {
        /// Transformation engine (default: auto)
        #[arg(long)]
        engine: Option<String>,

        /// Output directory (default: .canon/)
        #[arg(long)]
        output: Option<String>,

        /// Generate cryptographic signature
        #[arg(long)]
        sign: bool,

        /// Private key for signing
        #[arg(long)]
        key: Option<String>,

        /// Disable transformation caching
        #[arg(long)]
        no_cache: bool,

        /// Number of parallel transformations
        #[arg(long)]
        parallel: Option<usize>,
    },

    /// Remove Canon artifacts
    Clean {
        /// Remove all cached data (.canon/ directory)
        #[arg(long)]
        all: bool,

        /// Remove everything including canon.yml (complete uninstall)
        #[arg(long)]
        purge: bool,
    },

    /// Manage configuration
    Config {
        #[command(subcommand)]
        command: ConfigCommands,
    },
}

#[derive(Subcommand)]
pub enum ConfigCommands {
    /// Get configuration value
    Get { key: String },

    /// Set configuration value
    Set { key: String, value: String },

    /// Remove configuration value
    Unset { key: String },

    /// List all configuration
    List,

    /// Edit configuration file
    Edit,
}
