use crate::util::sops_config::{get_sops_config, read_or_create_config};
use colored::Colorize;
use std::process::Command;

/// Retrieves the Age key from 1Password using the reference stored in .sops.yaml
/// Returns the key as a string if successful, or an error message if not
pub fn get_age_key_from_1password() -> Result<String, String> {
    // Read the SOPS config to get the 1Password reference
    let config = read_or_create_config()
        .map_err(|e| format!("Failed to read SOPS config: {}", e))?;
    
    // Check if onepassworditem is set
    if config.onepassworditem.is_empty() {
        return Err("No 1Password reference found in .sops.yaml. Run 'easy_sops init' to configure.".to_string());
    }
    
    // Extract the 1Password reference
    let op_reference = config.onepassworditem;
    println!("{} {}", "🔑 Retrieving Age key from".dimmed(), op_reference.dimmed());
    
    // Run the op command to get the key
    // Format: op://<vault>/<item>/<field>
    let output = Command::new("op")
        .arg("read")
        .arg(op_reference)
        .output()
        .map_err(|e| format!("Failed to execute 1Password CLI: {}", e))?;
    
    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(format!("1Password CLI returned an error: {}", error));
    }
    
    // Get the output as a string
    let key = String::from_utf8_lossy(&output.stdout).trim().to_string();
    
    // Validate that we got a proper Age key
    if !key.starts_with("AGE-SECRET-KEY-") {
        return Err("Retrieved value is not a valid Age key. It should start with 'AGE-SECRET-KEY-'.".to_string());
    }
    
    Ok(key)
}