use crate::util::sops_command::SopsCommandBuilder;
use crate::util::sops_status::is_file_unchanged_status;
use colored::Colorize;
use std::ffi::OsString;

/// Entry point for the `edit` command.
pub fn edit(path: OsString) {
    // Convert the path from OsString to String
    let path_str = match path.into_string() {
        Ok(p) => p,
        Err(os) => {
            eprintln!("{} {:?}", "❌ Invalid UTF-8 in path:".red(), os);
            std::process::exit(1);
        }
    };

    // Check if the file exists
    if !std::path::Path::new(&path_str).is_file() {
        eprintln!("{} {}", "❌ File not found:".red(), path_str);
        std::process::exit(1);
    }

    // Ensure sops is installed
    if which::which("sops").is_err() {
        eprintln!(
            "{} {}",
            "❌ 'sops' is not installed or not in PATH.".red(),
            "Please install it first.".dimmed()
        );
        std::process::exit(1);
    }

    println!("{} {}", "📝 Opening file for editing:".green(), path_str);

    // Create a SOPS command with the Age key from 1Password
    let sops_command = match SopsCommandBuilder::new().arg(&path_str).with_age_key() {
        Ok(cmd) => cmd,
        Err(e) => {
            eprintln!("{} {}", "❌ Failed to get Age key:".red(), e);
            std::process::exit(1);
        }
    };

    // Run the command
    match sops_command.status() {
        Ok(status) if status.success() => {
            println!("{}", "✅ File edited and saved successfully.".green());
        }
        Ok(status) if is_file_unchanged_status(&status) => {
            println!("{}", "ℹ️ File has not changed.".blue());
        }
        Ok(status) => {
            eprintln!(
                "{} Exit code: {}",
                "❌ Error while editing the file.".red(),
                status
            );
            std::process::exit(status.code().unwrap_or(1));
        }
        Err(e) => {
            eprintln!("{} {:?}", "❌ Failed to launch sops:".red(), e);
            std::process::exit(1);
        }
    }
}
