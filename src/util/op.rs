use serde::Deserialize;
use std::process::Command;

#[derive(Debug, Deserialize)]
pub struct ItemField {
    label: String,
}

#[derive(Debug, Deserialize)]
pub struct ItemFields {
    fields: Vec<ItemField>,
}

#[derive(Debug, Deserialize)]
pub struct ListItem {
    title: String,
}

#[derive(Debug, Deserialize)]
pub struct Vault {
    id: String,
    name: String,
    content_version: u32,
    created_at: String,
    updated_at: String,
    items: u32,
}

/// Represents the category of a 1Password item.
pub enum OpCategory {
    Login,
    Password,
    Identity,
    Server,
}

impl OpCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            OpCategory::Login => "login",
            OpCategory::Password => "password",
            OpCategory::Identity => "identity",
            OpCategory::Server => "server",
        }
    }
}

/// Represents a field within a 1Password item.
pub struct OpItemField {
    pub section: Option<String>,
    pub field: String,
    pub field_type: Option<String>,
    pub value: String,
}

impl OpItemField {
    fn to_flag(&self) -> String {
        let mut flag = String::new();
        if let Some(section) = &self.section {
            flag.push_str(section);
            flag.push('.');
        }
        flag.push_str(&self.field);
        if let Some(field_type) = &self.field_type {
            flag.push_str(&format!("[{}]", field_type));
        }
        flag.push('=');
        flag.push_str(&self.value);
        flag
    }
}

/// Represents a 1Password item to be created.
pub struct OpItem {
    pub(crate) vault: String,
    pub(crate) title: String,
    pub(crate) category: OpCategory,
    pub(crate) fields: Vec<OpItemField>,
}

pub fn op_item_create(item: OpItem) {
    let mut cmd = Command::new("op");

    cmd.arg("item")
        .arg("create")
        .arg("--vault")
        .arg(&item.vault)
        .arg("--title")
        .arg(&item.title)
        .arg("--category")
        .arg(item.category.as_str()); // implement as_str on OpCategory

    for field in item.fields {
        let field_str = match (&field.section, &field.field_type) {
            (Some(section), Some(ftype)) => {
                format!("{}.{}[{}]={}", section, field.field, ftype, field.value)
            }
            (Some(section), None) => {
                format!("{}.{}={}", section, field.field, field.value)
            }
            (None, Some(ftype)) => {
                format!("{}[{}]={}", field.field, ftype, field.value)
            }
            (None, None) => {
                format!("{}={}", field.field, field.value)
            }
        };
        cmd.arg(field_str);
    }

    let status = cmd.status().expect("failed to run `op` command");

    if !status.success() {
        eprintln!("Failed to create item in 1Password");
    }
}

pub fn op_item_get(item_name: &str, field: &str) -> Option<String> {
    let output = Command::new("op")
        .arg("item")
        .arg("get")
        .arg(item_name)
        .arg("--field")
        .arg(field)
        .output()
        .ok()?;

    if output.status.success() {
        Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
        None
    }
}

pub fn get_vaults() -> Option<Vec<String>> {
    // Execute the `op vault list --format=json` command
    let output_json = Command::new("op")
        .arg("vault")
        .arg("list")
        .arg("--format=json")
        .output()
        .ok()?;

    if output_json.status.success() {
        // Parse the JSON response
        let vaults: Vec<Vault> = match serde_json::from_slice(&output_json.stdout) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Failed to parse JSON: {}", e);
                return None;
            }
        };

        // Extract vault names
        let vault_names: Vec<String> = vaults.into_iter().map(|vault| vault.name).collect();
        Some(vault_names)
    } else {
        eprintln!("Error: {}", String::from_utf8_lossy(&output_json.stderr));
        None
    }
}

pub fn get_items(vault: &String) -> Option<Vec<String>> {
    // Execute the `op vault list --format=json` command
    let output_json = Command::new("op")
        .arg("item")
        .arg("list")
        .arg("--vault")
        .arg(vault)
        .arg("--format=json")
        .output()
        .ok()?;

    if output_json.status.success() {
        // Parse the JSON response
        let vaults: Vec<ListItem> = match serde_json::from_slice(&output_json.stdout) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Failed to parse JSON: {}", e);
                return None;
            }
        };

        // Extract vault names
        let item_names: Vec<String> = vaults.into_iter().map(|item| item.title).collect();
        Some(item_names)
    } else {
        eprintln!("Error: {}", String::from_utf8_lossy(&output_json.stderr));
        None
    }
}

pub fn get_fields(item: &String, vault: &String) -> Option<Vec<String>> {
    // Execute the `op vault list --format=json` command
    let output_json = Command::new("op")
        .arg("item")
        .arg("get")
        .arg(item)
        .arg("--vault")
        .arg(vault)
        .arg("--format=json")
        .output()
        .ok()?;

    if output_json.status.success() {
        // Parse the JSON response
        let fields: ItemFields = match serde_json::from_slice(&output_json.stdout) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Failed to parse JSON: {}", e);
                return None;
            }
        };

        // Extract vault names
        let item_names: Vec<String> = fields.fields.into_iter().map(|item| item.label).collect();
        Some(item_names)
    } else {
        eprintln!("Error: {}", String::from_utf8_lossy(&output_json.stderr));
        None
    }
}
