use crate::cli::DeleteArgs;
use crate::config::{load_config, SortOrder}; // Import SortOrder
use crate::git_health::{
    collect_health_report, ensure_git_healthy_for_write, resolve_snapshot_commit, run_git,
    run_git_success,
};
use crate::utils::{
    ask_yes_no, find_snapshot, format_snapshot_line, get_snapshots,
    metadata_blob_hash_for_snapshot, metadata_ref_name, pin_metadata_blob, Snapshot,
};
use anyhow::{anyhow, Context, Result};
use chrono::Local;
use colored::*;
use inquire::Select;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub fn execute(args: DeleteArgs) -> Result<()> {
    ensure_git_healthy_for_write(false)?;

    let config = load_config()?;
    let DeleteArgs {
        id_or_label,
        purge: purge_arg,
        no_backup,
    } = args;

    let mut snapshots = get_snapshots()?; // Make the list mutable
    if snapshots.is_empty() {
        return Err(anyhow!("No snapshots found to delete."));
    }

    // --- START: NEW SORTING LOGIC ---
    if config.options.order_by == SortOrder::Label {
        snapshots.sort_by(|a, b| b.tag.cmp(&a.tag));
    }
    // --- END: NEW SORTING LOGIC ---

    let selected_interactively = id_or_label.is_none();
    let snapshot_to_delete = match id_or_label {
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

    let mut purge = purge_arg;
    let mut create_backup = !no_backup;
    if selected_interactively && !purge {
        let choices = vec![
            "Delete snapshot tag only".to_string(),
            "Delete snapshot tag and purge unreachable Git objects".to_string(),
        ];
        let choice = Select::new("Choose delete mode:", choices).prompt()?;
        purge = choice.contains("purge");

        if purge {
            create_backup = ask_yes_no(
                "[snap] Create the recommended purge bundle backup first?",
                true,
            )?;
        }
    }

    if no_backup && !purge {
        return Err(anyhow!(
            "`--no-backup` can only be used together with `--purge`."
        ));
    }

    println!("\n{}", "[snap] You are about to delete snapshot:".cyan());
    println!("  Label: {}", snapshot_to_delete.tag.bold());

    if purge {
        return purge_snapshot(
            &snapshot_to_delete,
            &snapshots,
            create_backup,
            config.options.confirm_command,
        );
    }

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

    println!(
        "{}",
        format!("[snap] Deleting tag \"{}\"...", snapshot_to_delete.tag).cyan()
    );
    run_git_success(&["tag", "-d", &snapshot_to_delete.tag], None)?;

    println!(
        "{}",
        format!(
            "[snap] Snapshot \"{}\" deleted successfully.",
            snapshot_to_delete.tag
        )
        .green()
    );
    println!(
        "{}",
        "[snap] Disk space was not reclaimed. To remove objects reachable only from this snapshot, run with `--purge`.".yellow()
    );
    println!();
    Ok(())
}

fn purge_snapshot(
    snapshot: &Snapshot,
    snapshots: &[Snapshot],
    create_backup: bool,
    confirm_command: bool,
) -> Result<()> {
    let target_commit = resolve_snapshot_commit(&snapshot.tag)?;
    ensure_not_active_snapshot(&target_commit)?;
    ensure_not_reachable_from_other_refs(&snapshot.tag, &target_commit)?;

    let before_size = dir_size(Path::new(".git"))?;
    println!("  Commit: {}", short_hash(&target_commit));
    println!(
        "  Purge backup: {}",
        if create_backup { "bundle" } else { "disabled" }
    );
    println!("  Git storage before purge: {}", format_bytes(before_size));
    println!();
    println!("{}", "[snap] Purge will:".yellow().bold());
    println!("  - delete snapshot tag '{}'", snapshot.tag);
    println!("  - pin metadata used by remaining snapshots");
    if create_backup {
        println!("  - create a targeted Git bundle backup");
    } else {
        println!("  - skip the targeted Git bundle backup");
    }
    println!("  - expire unreachable reflog entries");
    println!("  - run `git gc --prune=now`");

    if !create_backup {
        let proceed = ask_yes_no(
            "[snap] WARNING: Purge backup is disabled. This can permanently remove unreachable Git data. Continue?",
            false,
        )?;
        if !proceed {
            println!("{}", "[snap] Purge cancelled.".yellow());
            return Ok(());
        }
    } else if confirm_command {
        let proceed = ask_yes_no(
            "[snap] WARNING: This will permanently delete the snapshot tag and prune unreachable Git objects. Continue?",
            false,
        )?;
        if !proceed {
            println!("{}", "[snap] Purge cancelled.".yellow());
            return Ok(());
        }
    }

    let target_metadata_hash = metadata_blob_hash_for_snapshot(snapshot);
    let remaining_metadata_hashes = pin_remaining_snapshot_metadata(snapshots, &snapshot.tag)?;

    let backup_path = if create_backup {
        let path = create_purge_bundle(&snapshot.tag)?;
        println!(
            "{}",
            format!("[snap] Purge bundle backup: {}", path.display()).green()
        );
        Some(path)
    } else {
        None
    };

    println!(
        "{}",
        format!("[snap] Deleting tag \"{}\"...", snapshot.tag).cyan()
    );
    run_git_success(&["tag", "-d", &snapshot.tag], None)?;

    if let Some(hash) = target_metadata_hash.as_deref() {
        delete_metadata_ref_if_unused(hash, &remaining_metadata_hashes)?;
    }

    println!("{}", "[snap] Expiring unreachable reflog entries...".cyan());
    run_git_success(
        &["reflog", "expire", "--expire-unreachable=now", "--all"],
        None,
    )?;

    println!("{}", "[snap] Running Git garbage collection...".cyan());
    run_git_success(&["gc", "--prune=now"], None)?;

    let after_size = dir_size(Path::new(".git"))?;
    println!(
        "{}",
        format!(
            "[snap] Purge complete. Git storage: {} -> {}.",
            format_bytes(before_size),
            format_bytes(after_size)
        )
        .green()
    );
    if let Some(path) = backup_path {
        println!("  Backup: {}", path.display());
    }

    let final_report = collect_health_report()?;
    if final_report.has_errors() {
        return Err(anyhow!(
            "Purge completed, but the final health check found errors. Run `snap doctor`."
        ));
    }

    println!("{}", "[snap] Final health check passed.".green());
    println!();
    Ok(())
}

fn ensure_not_active_snapshot(target_commit: &str) -> Result<()> {
    let head = run_git(&["rev-parse", "--verify", "HEAD^{commit}"], None)?;
    if head.success && head.stdout.trim() == target_commit {
        return Err(anyhow!(
            "Cannot purge the active snapshot. Restore or move to another snapshot first."
        ));
    }
    Ok(())
}

fn ensure_not_reachable_from_other_refs(target_tag: &str, target_commit: &str) -> Result<()> {
    let refs = run_git_success(
        &[
            "for-each-ref",
            "--contains",
            target_commit,
            "--format=%(refname)",
            "refs/heads",
            "refs/remotes",
            "refs/tags",
        ],
        None,
    )?;
    let target_ref = format!("refs/tags/{}", target_tag);
    let reachable_refs: Vec<_> = refs
        .lines()
        .map(str::trim)
        .filter(|ref_name| !ref_name.is_empty() && *ref_name != target_ref.as_str())
        .map(ToString::to_string)
        .collect();

    if !reachable_refs.is_empty() {
        let mut message = format!(
            "Cannot purge snapshot '{}' because its commit is still reachable from:",
            target_tag
        );
        for ref_name in reachable_refs.iter().take(10) {
            message.push_str(&format!("\n  - {}", ref_name));
        }
        if reachable_refs.len() > 10 {
            message.push_str(&format!("\n  ... and {} more", reachable_refs.len() - 10));
        }
        message.push_str("\nRestore or move those refs first, then run purge again.");
        return Err(anyhow!(message));
    }

    Ok(())
}

fn pin_remaining_snapshot_metadata(
    snapshots: &[Snapshot],
    deleting_tag: &str,
) -> Result<HashSet<String>> {
    let mut hashes = HashSet::new();
    for snapshot in snapshots {
        if snapshot.tag == deleting_tag {
            continue;
        }
        if let Some(hash) = metadata_blob_hash_for_snapshot(snapshot) {
            pin_metadata_blob(&hash)?;
            hashes.insert(hash);
        }
    }
    Ok(hashes)
}

fn create_purge_bundle(tag: &str) -> Result<PathBuf> {
    let backup_dir = Path::new(".git").join("snap-backups");
    fs::create_dir_all(&backup_dir).with_context(|| {
        format!(
            "Failed to create purge backup directory {}",
            backup_dir.display()
        )
    })?;

    let stamp = Local::now().format("%Y%m%d-%H%M%S").to_string();
    let safe_tag = sanitize_backup_name(tag);
    let path = backup_dir.join(format!("snap-purge-{}-{}.bundle", safe_tag, stamp));
    let path_arg = path.to_string_lossy().to_string();
    run_git_success(&["bundle", "create", &path_arg, tag], None)
        .with_context(|| format!("Failed to create purge bundle for '{}'", tag))?;
    Ok(path)
}

fn sanitize_backup_name(value: &str) -> String {
    value
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '.' {
                c
            } else {
                '-'
            }
        })
        .collect()
}

fn delete_metadata_ref_if_unused(hash: &str, remaining_hashes: &HashSet<String>) -> Result<()> {
    if remaining_hashes.contains(hash) {
        return Ok(());
    }

    let ref_name = metadata_ref_name(hash);
    let existing = run_git(&["show-ref", "--verify", &ref_name], None)?;
    if existing.success {
        run_git_success(&["update-ref", "-d", &ref_name], None)?;
    }
    Ok(())
}

fn dir_size(root: &Path) -> Result<u64> {
    let mut size = 0;
    if !root.exists() {
        return Ok(size);
    }

    for entry in WalkDir::new(root).into_iter().filter_map(Result::ok) {
        if entry.file_type().is_file() {
            size += entry.metadata().map(|metadata| metadata.len()).unwrap_or(0);
        }
    }
    Ok(size)
}

fn format_bytes(bytes: u64) -> String {
    const UNITS: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];
    let mut value = bytes as f64;
    let mut unit = 0;
    while value >= 1024.0 && unit < UNITS.len() - 1 {
        value /= 1024.0;
        unit += 1;
    }

    if unit == 0 {
        format!("{} {}", bytes, UNITS[unit])
    } else {
        format!("{:.1} {}", value, UNITS[unit])
    }
}

fn short_hash(hash: &str) -> &str {
    hash.get(..7).unwrap_or(hash)
}
