use clap::CommandFactory;
use clap_complete::{generate_to, shells::Fish};
use clap_mangen::Man;
use std::env;
use std::fs;
use std::io::Result;
use std::path::Path;

// Import your CLI struct
// Note: We need to pull this in from the main package
#[path = "src/main.rs"]
mod main_mod {
    pub mod commands;
    pub mod util;
    use clap::{Parser, Subcommand};
    use std::ffi::OsString;

    #[derive(Debug, Parser)]
    #[command(name = "opsops")]
    #[command(about = "A wrapper that integrates sops with 1Password", long_about = None)]
    pub struct Cli {
        #[command(subcommand)]
        pub command: Commands,
    }

    #[derive(Debug, Subcommand)]
    pub enum Commands {
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

        /// Troubleshoot your current config
        #[command(arg_required_else_help = false)]
        Doctor {},

        /// Initialize opsops
        Init {},
    }
}

use main_mod::Cli;

fn main() -> Result<()> {
    // Get the output directory from cargo
    let out_dir = env::var("OUT_DIR").unwrap();
    let man_dir = Path::new(&out_dir).join("man");
    let completion_dir = Path::new(&out_dir).join("completions");

    // Create directories if they don't exist
    fs::create_dir_all(&man_dir)?;
    fs::create_dir_all(&completion_dir)?;

    // Generate man page
    let cmd = Cli::command();
    let man = Man::new(cmd.clone());
    let mut buffer = Vec::<u8>::new();
    man.render(&mut buffer)?;
    fs::write(man_dir.join("opsops.1"), buffer)?;

    // Generate Fish completions
    let mut cmd = Cli::command();
    generate_to(Fish, &mut cmd, "opsops", &completion_dir)?;

    // Print message during build
    println!(
        "cargo:warning=Generated man pages and fish completions in {}",
        out_dir
    );

    Ok(())
}
