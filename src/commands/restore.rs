use crate::cli::RestoreArgs;
use crate::config::{load_config, SortOrder};
use crate::utils::{
    ask_yes_no, check_dirty, find_snapshot, format_snapshot_line, gather_metadata, get_snapshots,
    load_metadata_for_snapshot, run_command,
};
use anyhow::{anyhow, Context, Result};
use colored::*;
use inquire::Select;
use std::collections::HashSet;
use std::env;
use std::fs;

pub fn execute(args: RestoreArgs) -> Result<()> {
    let config = load_config()?;
    let mut snapshots = get_snapshots()?;
    if snapshots.is_empty() {
        return Err(anyhow!("No snapshots found to restore."));
    }

    if config.options.order_by == SortOrder::Label {
        snapshots.sort_by(|a, b| b.tag.cmp(&a.tag));
    }

    let snapshot_to_restore = match args.id_or_label {
        Some(key) => find_snapshot(&snapshots, &key)
            .cloned()
            .with_context(|| format!("Snapshot \"{}\" not found.", key)),
        None => {
            let choices = snapshots
                .iter()
                .map(|s| format_snapshot_line(s, config.options.show_ids))
                .collect();
            let choice = Select::new("Select snapshot to restore:", choices).prompt()?;
            snapshots
                .iter()
                .find(|s| format_snapshot_line(s, config.options.show_ids) == choice)
                .cloned()
                .context("Could not find selected snapshot.")
        }
    }?;

    if check_dirty()? {
        println!("\n{}", "[snap] WARNING: Your project has uncommitted changes.".yellow());
        if !ask_yes_no("To restore a snapshot, all local changes must be discarded. Continue?", false)? {
            println!("{}", "[snap] Restore cancelled.".yellow());
            return Ok(());
        }
        println!("{}", "[snap] Discarding all local changes...".cyan());
        run_command("git reset --hard HEAD", None)?;
        run_command("git clean -fd", None)?;
        println!("{}\n", "[snap] Workspace is now clean. Proceeding with restore.".green());
    }

    println!("\n{} \"{}\"...", "[snap] Restoring project files for snapshot".cyan(), snapshot_to_restore.tag);
    run_command(&format!("git checkout --force {}", snapshot_to_restore.tag), None)?;

    println!("{}", "[snap] Synchronizing metadata...".cyan());

    let source_metadata = gather_metadata()?;
    let target_metadata = load_metadata_for_snapshot(&snapshot_to_restore)?;

    let source_hidden: HashSet<_> = source_metadata.hidden_paths.into_iter().collect();
    let source_readonly: HashSet<_> = source_metadata.readonly_paths.into_iter().collect();
    let target_hidden: HashSet<_> = target_metadata.hidden_paths.into_iter().collect();
    let target_readonly: HashSet<_> = target_metadata.readonly_paths.into_iter().collect();
    let source_empty: HashSet<_> = source_metadata.empty_dirs.into_iter().collect();
    let target_empty: HashSet<_> = target_metadata.empty_dirs.into_iter().collect();

    let cwd = env::current_dir()?;

    // Reconcile empty directories
    let mut dirs_to_remove: Vec<_> = source_empty.difference(&target_empty).collect();
    dirs_to_remove.sort_by(|a, b| b.len().cmp(&a.len()));
    for path_str in dirs_to_remove {
        let full_path = cwd.join(path_str);
        if full_path.exists() {
            if let Ok(mut read_dir) = full_path.read_dir() {
                if read_dir.next().is_none() {
                    let _ = fs::remove_dir(full_path);
                }
            }
        }
    }
    for path_str in target_empty.difference(&source_empty) {
        let full_path = cwd.join(path_str);
        if !full_path.exists() {
            let _ = fs::create_dir_all(full_path);
        }
    }

    // Reconcile hidden attributes
    for path_str in source_hidden.difference(&target_hidden) {
        let full_path = cwd.join(path_str);
        if full_path.exists() {
            let _ = crate::os::set_hidden(&full_path, false);
        }
    }
    for path_str in target_hidden.difference(&source_hidden) {
        let full_path = cwd.join(path_str);
        if full_path.exists() {
            let _ = crate::os::set_hidden(&full_path, true);
        }
    }

    // Reconcile read-only attributes
    // Paths to make WRITABLE: are read-only in source but NOT in target.
    for path_str in source_readonly.difference(&target_readonly) {
        let full_path = cwd.join(path_str);
        if full_path.exists() {
            if let Ok(metadata) = fs::metadata(&full_path) {
                let mut perms = metadata.permissions();
                perms.set_readonly(false);
                let _ = fs::set_permissions(&full_path, perms);
            }
        }
    }
    // Paths to make READ-ONLY: are read-only in target but NOT in source.
    for path_str in target_readonly.difference(&source_readonly) {
        let full_path = cwd.join(path_str);
        if full_path.exists() {
            if let Ok(metadata) = fs::metadata(&full_path) {
                let mut perms = metadata.permissions();
                perms.set_readonly(true);
                let _ = fs::set_permissions(&full_path, perms);
            }
        }
    }

    println!("\n{}", "[snap] Restore complete. Your project is now at the state of this snapshot.".green());
    Ok(())
}