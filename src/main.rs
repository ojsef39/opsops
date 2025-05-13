mod commands;
mod util;
use std::ffi::OsString;

use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "simple-sops")]
#[command(about = "A simple wrapper around sops and age", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(arg_required_else_help = false)]
    ListConfig {},

    #[command(arg_required_else_help = false)]
    GenerateAgeKey {},

    #[command(arg_required_else_help = true)]
    Edit {
        #[arg(value_name = "PATH", help = "Path to the file to edit")]
        path: OsString,
    },

    #[command(arg_required_else_help = true)]
    Encrypt {
        #[arg(value_name = "PATH", help = "Path to the file to encrypt")]
        path: OsString,
    },

    #[command(arg_required_else_help = true)]
    Decrypt {
        #[arg(value_name = "PATH", help = "Path to the encrypted file to decrypt")]
        path: OsString,
    },

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
