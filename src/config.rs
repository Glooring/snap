use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortOrder {
    Timestamp,
    Label,
}

impl Default for SortOrder {
    fn default() -> Self {
        SortOrder::Timestamp
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SnapConfig {
    pub options: Options,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Options {
    #[serde(default = "default_bool_false")]
    pub show_ids: bool,
    #[serde(default = "default_bool_true")]
    pub confirm_command: bool,
    #[serde(default)]
    pub order_by: SortOrder,
    // --- START: NEW TIMESTAMP OPTION ---
    #[serde(default = "default_bool_false")]
    pub edit_updates_timestamp: bool,
    // --- END: NEW TIMESTAMP OPTION ---
}

fn default_bool_false() -> bool { false }
fn default_bool_true() -> bool { true }

impl Default for SnapConfig {
    fn default() -> Self {
        SnapConfig {
            options: Options {
                show_ids: false,
                confirm_command: true,
                order_by: SortOrder::default(),
                // --- START: SET DEFAULT FOR NEW OPTION ---
                edit_updates_timestamp: false, // Default is NOT to update the timestamp
                // --- END: SET DEFAULT FOR NEW OPTION ---
            },
        }
    }
}

pub fn get_config_path() -> Result<PathBuf> {
    let exe_path = env::current_exe().context("Failed to get current executable path")?;
    let snap_dir = exe_path
        .parent()
        .ok_or_else(|| anyhow!("Failed to get parent directory of executable"))?;
    Ok(snap_dir.join(".snapconfig"))
}

pub fn load_config() -> Result<SnapConfig> {
    let config_path = get_config_path()?;
    if !config_path.exists() {
        return Ok(SnapConfig::default());
    }
    let content = fs::read_to_string(&config_path)
        .with_context(|| format!("Failed to read config file at {:?}", config_path))?;
    Ok(serde_json::from_str(&content).unwrap_or_else(|_| SnapConfig::default()))
}

pub fn save_config(config: &SnapConfig) -> Result<()> {
    let config_path = get_config_path()?;
    let content = serde_json::to_string_pretty(config)
        .context("Failed to serialize config to JSON")?;
    fs::write(&config_path, content)
        .with_context(|| format!("Failed to write config file to {:?}", config_path))
}

pub fn ensure_repo_exists() -> Result<()> {
    if !is_git_repo() {
        return Err(anyhow!(
            "Not a snap repository (or any of the parent directories).\nHint: Run `snap init` to create one."
        ));
    }
    Ok(())
}

fn is_git_repo() -> bool {
    Path::new(".git").exists() && Path::new(".git").is_dir()
}