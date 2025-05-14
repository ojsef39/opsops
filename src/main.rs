mod commands;
mod util;
use std::ffi::OsString;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "easy_sops")]
#[command(about = "A wrapper that integrates sops with 1Password", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Parse and display the .sops.yaml for this project
    #[command(arg_required_else_help = false)]
    ListConfig {},

    /// Generate an age key pair
    #[command(arg_required_else_help = false)]
    GenerateAgeKey {},

    /// Edit a file using sops with a key from 1password
    #[command(arg_required_else_help = true)]
    Edit {
        #[arg(value_name = "PATH", help = "Path to the file to edit")]
        path: OsString,
    },

    /// Encrypt a file using sops
    #[command(arg_required_else_help = true)]
    Encrypt {
        #[arg(value_name = "PATH", help = "Path to the file to encrypt")]
        path: OsString,
    },

    /// Decrypt a file using sops
    #[command(arg_required_else_help = true)]
    Decrypt {
        #[arg(value_name = "PATH", help = "Path to the encrypted file to decrypt")]
        path: OsString,
    },

    /// Initialize easy_sops
    Init {},
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::ListConfig {} => commands::list_config::list_config(),
        Commands::GenerateAgeKey {} => commands::generate_age_key::generate_age_key(),
        Commands::Edit { path } => commands::edit::edit(path),
        Commands::Encrypt { path } => commands::encrypt::encrypt(path),
        Commands::Decrypt { path } => commands::decrypt::decrypt(path),
        Commands::Init {} => commands::init::init(),
    }
}
