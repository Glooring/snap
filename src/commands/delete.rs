use crate::cli::DeleteArgs;
use crate::config::{load_config, SortOrder}; // Import SortOrder
use crate::utils::{ask_yes_no, find_snapshot, format_snapshot_line, get_snapshots, run_command};
use anyhow::{anyhow, Context, Result};
use colored::*;
use inquire::Select;

pub fn execute(args: DeleteArgs) -> Result<()> {
    let config = load_config()?;

    let mut snapshots = get_snapshots()?; // Make the list mutable
    if snapshots.is_empty() {
        return Err(anyhow!("No snapshots found to delete."));
    }

    // --- START: NEW SORTING LOGIC ---
    if config.options.order_by == SortOrder::Label {
        snapshots.sort_by(|a, b| b.tag.cmp(&a.tag));
    }
    // --- END: NEW SORTING LOGIC ---

    let snapshot_to_delete = match args.id_or_label {
        Some(key) => find_snapshot(&snapshots, &key)
            .cloned()
            .with_context(|| format!("Snapshot \"{}\" not found.", key)),
        None => {
            // This list of choices will now be sorted according to the user's preference
            let choices: Vec<String> = snapshots
                .iter()
                .map(|s| format_snapshot_line(s, config.options.show_ids))
                .collect();
            let choice = Select::new("Select snapshot to delete:", choices).prompt()?;
            snapshots
                .iter()
                .find(|s| format_snapshot_line(s, config.options.show_ids) == choice)
                .cloned()
                .context("Could not find selected snapshot.")
        }
    }?;

    println!("\n{}", "[snap] You are about to delete snapshot:".cyan());
    println!("  Label: {}", snapshot_to_delete.tag.bold());

    if config.options.confirm_command {
        let proceed = ask_yes_no(
            "[snap] WARNING: This will permanently delete the snapshot tag. Continue?",
            false,
        )?;
        if !proceed {
            println!("{}", "[snap] Deletion cancelled.".yellow());
            return Ok(());
        }
    }

    println!("{}", format!("[snap] Deleting tag \"{}\"...", snapshot_to_delete.tag).cyan());
    run_command(&format!("git tag -d {}", snapshot_to_delete.tag), None)?;

    println!("{}", format!("[snap] Snapshot \"{}\" deleted successfully.", snapshot_to_delete.tag).green());
    println!();
    Ok(())
}