use anyhow::{anyhow, Context, Result};
use chrono::{DateTime, Local};
use colored::*;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::io::{self, Write};
use std::os::windows::fs::MetadataExt;
use std::path::Path; // Corrected: Removed unused PathBuf
use std::process::{Command, Stdio};
use walkdir::{DirEntry, WalkDir};

const METADATA_REF_KEY: &str = "Snap-Metadata-Ref";

#[derive(Debug, Clone)]
pub struct Snapshot {
    pub full_id: String,
    pub id: String,
    pub tag: String,
    pub description: String,
    pub timestamp: String,
    pub raw_tag_message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct SnapMetadata {
    pub hidden_paths: Vec<String>,
    pub readonly_paths: Vec<String>,
    pub empty_dirs: Vec<String>,
}

pub fn run_command(cmd_str: &str, input: Option<&str>) -> Result<String> {
    run_command_with_env(cmd_str, input, &HashMap::new())
}

pub fn run_command_with_env(
    cmd_str: &str,
    input: Option<&str>,
    env_vars: &HashMap<&str, &str>,
) -> Result<String> {
    let parts = shlex::split(cmd_str).ok_or_else(|| anyhow!("Invalid command string"))?;
    if parts.is_empty() {
        return Err(anyhow!("Empty command"));
    }
    let command = &parts[0];
    let args = &parts[1..];
    let mut cmd = Command::new(command);
    cmd.args(args).envs(env_vars);

    if input.is_some() {
        cmd.stdin(Stdio::piped());
    }
    cmd.stdout(Stdio::piped()).stderr(Stdio::piped());

    let mut child = cmd.spawn().with_context(|| format!("Failed to spawn command: {}", cmd_str))?;

    if let (Some(stdin), Some(input_data)) = (child.stdin.as_mut(), input) {
        stdin.write_all(input_data.as_bytes())?;
    }

    let output = child.wait_with_output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!(
            "Command failed: '{}'\n---\n{}",
            cmd_str,
            stderr.trim()
        ));
    }
    Ok(String::from_utf8(output.stdout)?)
}

pub fn check_dirty() -> Result<bool> {
    Ok(!run_command("git status --porcelain", None)?
        .trim()
        .is_empty())
}

pub fn get_active_commit_full() -> Result<Option<String>> {
    match run_command("git rev-parse HEAD", None) {
        Ok(output) => Ok(Some(output.trim().to_string())),
        Err(_) => Ok(None),
    }
}

pub fn get_snapshots() -> Result<Vec<Snapshot>> {
    let command = r#"git for-each-ref refs/tags --sort=-taggerdate --format="%(refname:short)	%(*objectname)	%(taggerdate:iso-strict)	%(contents)""#;
    let output = match run_command(command, None) {
        Ok(out) => out,
        Err(_) => return Ok(vec![]),
    };
    Ok(output
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split('\t').collect();
            if parts.len() < 4 {
                return None;
            }
            let full_id = parts[1].to_string();
            let raw_message = parts[3].to_string();

            let metadata_key_with_colon = format!("{}:", METADATA_REF_KEY);

            let description = raw_message
                .lines()
                .take_while(|line| !line.starts_with(&metadata_key_with_colon))
                .collect::<Vec<_>>()
                .join("\n")
                .trim()
                .to_string();

            Some(Snapshot {
                id: full_id.chars().take(7).collect(),
                full_id,
                tag: parts[0].to_string(),
                timestamp: parts[2].to_string(),
                description,
                raw_tag_message: raw_message,
            })
        })
        .collect())
}

pub fn find_snapshot<'a>(snaps: &'a [Snapshot], key: &str) -> Option<&'a Snapshot> {
    snaps
        .iter()
        .find(|s| s.tag == key || s.id.starts_with(key) || s.full_id.starts_with(key))
}

pub fn gather_metadata() -> Result<SnapMetadata> {
    let root = env::current_dir()?;
    let mut hidden_paths = Vec::new();
    let mut readonly_paths = Vec::new();
    let mut all_dirs = Vec::new();

    let walker = WalkDir::new(&root)
        .into_iter()
        .filter_entry(|e| !is_ignored(e));

    for entry_result in walker {
        let entry = match entry_result {
            Ok(e) => e,
            Err(e) => {
                eprintln!(
                    "{} Failed to process a path during scan: {}. Skipping.",
                    "[snap] Warning:".yellow(),
                    e
                );
                continue;
            }
        };

        let path = entry.path();
        if path == root {
            continue;
        }

        // We collect all directories regardless of whether we can get metadata for them,
        // as even a directory with unreadable contents is still a directory.
        if entry.file_type().is_dir() {
            all_dirs.push(path.to_path_buf());
        }

        // --- START OF COMPILER FIX ---
        // Correctly handle the `walkdir::Error` type returned by `entry.metadata()`
        let metadata = match entry.metadata() {
            Ok(m) => m,
            Err(e) => {
                // `e` is a `walkdir::Error`. Check if it's an IO error we can ignore.
                if let Some(io_err) = e.io_error() {
                    if io_err.kind() == io::ErrorKind::NotFound {
                        // This is a transient file that was deleted during the scan. It's safe to ignore.
                        continue;
                    }
                }
                // For any other kind of error, warn the user and skip this entry.
                eprintln!(
                    "{} Could not read metadata for '{}': {}. Skipping.",
                    "[snap] Warning:".yellow(),
                    path.display(),
                    e
                );
                continue;
            }
        };
        // --- END OF COMPILER FIX ---

        let relative_path = path
            .strip_prefix(&root)
            .unwrap()
            .to_string_lossy()
            .replace('\\', "/");

        let attrs = metadata.file_attributes();
        const FILE_ATTRIBUTE_READONLY: u32 = 0x1;
        const FILE_ATTRIBUTE_HIDDEN: u32 = 0x2;

        if (attrs & FILE_ATTRIBUTE_HIDDEN) != 0 {
            hidden_paths.push(relative_path.to_string());
        }

        if (attrs & FILE_ATTRIBUTE_READONLY) != 0 {
            readonly_paths.push(relative_path.to_string());
        }
    }

    let mut empty_dirs: Vec<String> = all_dirs
        .into_par_iter()
        .filter(|path| is_dir_empty(path).unwrap_or(false))
        .filter_map(|path| {
            path.strip_prefix(&root)
                .ok()
                .map(|p| p.to_string_lossy().replace('\\', "/").to_string())
        })
        .collect();

    hidden_paths.sort();
    readonly_paths.sort();
    empty_dirs.sort();

    Ok(SnapMetadata {
        hidden_paths,
        readonly_paths,
        empty_dirs,
    })
}

pub fn hash_metadata_blob(metadata: &SnapMetadata) -> Result<Option<String>> {
    if metadata.hidden_paths.is_empty()
        && metadata.empty_dirs.is_empty()
        && metadata.readonly_paths.is_empty()
    {
        return Ok(None);
    }
    let json_content =
        serde_json::to_string(metadata).context("Failed to serialize metadata to JSON")?;

    let blob_hash = run_command("git hash-object -w --stdin", Some(&json_content))?;
    Ok(Some(blob_hash.trim().to_string()))
}

pub fn create_tag_message(description: &str, blob_hash: Option<&str>) -> String {
    let desc = description.trim();
    let Some(hash) = blob_hash else {
        return desc.to_string();
    };

    let metadata_line = format!("{}: {}", METADATA_REF_KEY, hash);

    if desc.is_empty() {
        metadata_line
    } else {
        format!("{}\n\n{}", desc, metadata_line)
    }
}

fn get_blob_hash_from_message(raw_message: &str) -> Option<String> {
    raw_message
        .lines()
        .find(|line| line.starts_with(METADATA_REF_KEY))
        .and_then(|line| line.split(':').nth(1))
        .map(|hash| hash.trim().to_string())
}

pub fn load_metadata_for_snapshot(snapshot: &Snapshot) -> Result<SnapMetadata> {
    let Some(blob_hash) = get_blob_hash_from_message(&snapshot.raw_tag_message) else {
        return Ok(SnapMetadata::default());
    };

    let json_content = run_command(&format!("git cat-file blob {}", blob_hash), None)
        .with_context(|| format!("Failed to read metadata blob object '{}'", blob_hash))?;

    serde_json::from_str(&json_content).with_context(|| "Failed to deserialize metadata from blob")
}

fn is_ignored(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s == ".git" || s.starts_with("target") || s.starts_with("node_modules"))
        .unwrap_or(false)
}

fn is_dir_empty(path: &Path) -> Result<bool> {
    Ok(path.read_dir()?.next().is_none())
}

pub fn ask_yes_no(question: &str, default: bool) -> Result<bool> {
    let prompt = if default { "[Y/n]" } else { "[y/N]" };
    let answer = inquire::Text::new(&format!("{} {}", question.yellow(), prompt))
        .with_help_message("Press Enter to accept the default")
        .prompt_skippable()?
        .map(|s| s.trim().to_lowercase());
    Ok(match answer.as_deref() {
        Some("") | None => default,
        Some(s) => s.starts_with('y'),
    })
}

pub fn format_timestamp(iso_str: &str) -> String {
    match DateTime::parse_from_rfc3339(iso_str) {
        Ok(dt) => dt.with_timezone(&Local).format("%Y-%m-%d %H:%M").to_string(),
        Err(_) => iso_str.to_string(),
    }
}

pub fn format_snapshot_line(s: &Snapshot, show_ids: bool) -> String {
    let desc_text = if s.description.is_empty() {
        "".to_string()
    } else {
        s.description.clone()
    };
    if show_ids {
        format!("{} - {} {}", s.id.bold(), s.tag, desc_text.dimmed())
    } else {
        format!("{} {}", s.tag.bold(), desc_text.dimmed())
    }
}