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
    /// Initialize new specification
    Init {
        /// Specification name (defaults to current directory name)
        name: Option<String>,

        /// Specification type URI
        #[arg(long, default_value = "canon-protocol.org/specification@1.0.0")]
        r#type: String,

        /// Author name (default: git config user.name)
        #[arg(long)]
        author: Option<String>,

        /// License identifier
        #[arg(long)]
        license: Option<String>,

        /// Use specification template
        #[arg(long)]
        template: Option<String>,

        /// Overwrite existing canon.yml
        #[arg(long)]
        force: bool,
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

    /// Remove generated artifacts
    Clean,

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
