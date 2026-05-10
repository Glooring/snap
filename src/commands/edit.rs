use crate::cli::EditArgs;
use crate::config::{load_config, SortOrder};
use crate::git_health::ensure_git_healthy_for_write;
use crate::utils::{
    create_tag_message, find_snapshot, format_snapshot_line, get_snapshots,
    load_metadata_for_snapshot, pin_metadata_blob, run_command, run_command_with_env,
};
use anyhow::{anyhow, Context, Result};
use colored::*;
use inquire::Select;
use std::collections::HashMap; // Keep HashMap import
use std::io::{self, Write};

fn sanitize_tag_name(label: &str) -> String {
    label
        .trim()
        .replace(char::is_whitespace, "-")
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '-' || *c == '.' || *c == '_')
        .collect()
}

pub fn execute(args: EditArgs) -> Result<()> {
    ensure_git_healthy_for_write(false)?;

    let config = load_config()?;

    let mut snapshots = get_snapshots()?;
    if snapshots.is_empty() {
        return Err(anyhow!("No snapshots found to edit."));
    }

    if config.options.order_by == SortOrder::Label {
        snapshots.sort_by(|a, b| b.tag.cmp(&a.tag));
    }

    let snapshot_to_edit = match args.id_or_label {
        Some(key) => find_snapshot(&snapshots, &key)
            .cloned()
            .with_context(|| format!("Snapshot \"{}\" not found.", key)),
        None => {
            let choices: Vec<String> = snapshots
                .iter()
                .map(|s| format_snapshot_line(s, config.options.show_ids))
                .collect();
            let choice = Select::new("Select snapshot to edit:", choices).prompt()?;
            snapshots
                .iter()
                .find(|s| format_snapshot_line(s, config.options.show_ids) == choice)
                .cloned()
                .context("Could not find selected snapshot.")
        }
    }?;

    println!(
        "\n{} \"{}\":",
        "[snap] Editing snapshot".cyan(),
        snapshot_to_edit.tag
    );

    let blob_hash_key = "Snap-Metadata-Ref:";
    let metadata_blob_hash = snapshot_to_edit
        .raw_tag_message
        .lines()
        .find(|line| line.starts_with(blob_hash_key))
        .and_then(|line| line.split(':').nth(1))
        .map(|hash| hash.trim().to_string());
    if let Some(hash) = metadata_blob_hash.as_deref() {
        load_metadata_for_snapshot(&snapshot_to_edit)?;
        pin_metadata_blob(hash)?;
    }

    let new_label = prompt_text_with_default("Enter new label (tag name)", &snapshot_to_edit.tag)?;
    if new_label.trim().is_empty() {
        return Err(anyhow!("Label cannot be empty."));
    }

    let new_description =
        prompt_text_with_default("Enter new description", &snapshot_to_edit.description)?;

    let new_tag_name = sanitize_tag_name(&new_label);
    let new_description_trimmed = new_description.trim();

    if new_tag_name == snapshot_to_edit.tag
        && new_description_trimmed == snapshot_to_edit.description
    {
        println!(
            "{}",
            "[snap] No changes detected. Edit cancelled.\n".yellow()
        );
        return Ok(());
    }

    if new_tag_name != snapshot_to_edit.tag && snapshots.iter().any(|s| s.tag == new_tag_name) {
        return Err(anyhow!(
            "A snapshot with the label \"{}\" already exists.\n",
            new_tag_name
        ));
    }

    println!(
        "\n{}",
        "[snap] Applying changes... This will replace the old tag.".yellow()
    );

    let new_tag_message =
        create_tag_message(new_description_trimmed, metadata_blob_hash.as_deref());
    let tag_cmd = format!(
        "git tag -a -f {} -F - {}",
        new_tag_name, snapshot_to_edit.full_id
    );

    // --- START: CORRECTED TIMESTAMP LOGIC ---
    // Explicitly declare the HashMap's type to match the function signature.
    let mut env_vars: HashMap<&str, &str> = HashMap::new();

    if !config.options.edit_updates_timestamp {
        // Now, the compiler knows to coerce `&snapshot_to_edit.timestamp` (a &String)
        // into a `&str`, which matches the HashMap's value type.
        env_vars.insert("GIT_COMMITTER_DATE", &snapshot_to_edit.timestamp);
    }

    run_command_with_env(&tag_cmd, Some(&new_tag_message), &env_vars)?;
    // --- END: CORRECTED TIMESTAMP LOGIC ---

    if new_tag_name != snapshot_to_edit.tag {
        // Use the simpler run_command here, as no special environment is needed.
        run_command(&format!("git tag -d {}", snapshot_to_edit.tag), None)?;
    }

    println!(
        "\n{}",
        format!(
            "[snap] Snapshot successfully updated to \"{}\".",
            new_tag_name
        )
        .green()
    );

    println!();
    Ok(())
}

fn prompt_text_with_default(prompt: &str, default: &str) -> Result<String> {
    print!("{} [{}]: ", prompt, default);
    io::stdout().flush()?;

    let mut answer = String::new();
    io::stdin().read_line(&mut answer)?;
    let answer = answer.trim_end_matches(&['\r', '\n'][..]);
    if answer.is_empty() {
        Ok(default.to_string())
    } else {
        Ok(answer.to_string())
    }
}
