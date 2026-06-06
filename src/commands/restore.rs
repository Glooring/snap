use crate::cli::RestoreArgs;
use crate::config::{load_config, SortOrder};
use crate::git_health::{ensure_git_healthy_for_write, resolve_snapshot_commit, run_git};
use crate::utils::{
    ask_yes_no, check_dirty, create_tag_message, find_snapshot, format_snapshot_line,
    gather_metadata, get_active_commit_full, get_snapshots, get_snapshots_pointing_at,
    hash_metadata_blob, load_metadata_for_snapshot, pin_metadata_blob, run_command,
    run_command_args,
};
use anyhow::{anyhow, Context, Result};
use chrono::Local;
use colored::*;
use inquire::Select;
use std::cmp::Reverse;
use std::collections::HashSet;
use std::env;
use std::fs;

pub fn execute(args: RestoreArgs) -> Result<()> {
    ensure_git_healthy_for_write(false)?;
    let RestoreArgs {
        id_or_label,
        dry_run,
        no_rescue,
    } = args;

    let config = load_config()?;
    let mut snapshots = get_snapshots()?;
    if snapshots.is_empty() {
        return Err(anyhow!("No snapshots found to restore."));
    }

    if config.options.order_by == SortOrder::Label {
        snapshots.sort_by(|a, b| b.tag.cmp(&a.tag));
    }

    let snapshot_to_restore = match id_or_label {
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
    let snapshot_commit = resolve_snapshot_commit(&snapshot_to_restore.tag).with_context(|| {
        format!(
            "Snapshot \"{}\" does not point to a valid commit.",
            snapshot_to_restore.tag
        )
    })?;
    let dirty = check_dirty()?;
    let current_head = get_active_commit_full()?;

    if dry_run {
        print_dry_run(
            &snapshot_to_restore.tag,
            &snapshot_commit,
            current_head.as_deref(),
            dirty,
            no_rescue,
        )?;
        return Ok(());
    }

    if dirty {
        println!(
            "\n{}",
            "[snap] WARNING: Your project has uncommitted changes.".yellow()
        );
        let question = if no_rescue {
            "To restore a snapshot, all local changes must be discarded. Continue?"
        } else {
            "Snap will create a rescue snapshot before restoring. Continue?"
        };
        if !ask_yes_no(question, false)? {
            println!("{}", "[snap] Restore cancelled.".yellow());
            return Ok(());
        }
    }

    if no_rescue {
        if dirty {
            println!("{}", "[snap] Discarding all local changes...".cyan());
            run_command("git reset --hard HEAD", None)?;
            run_command("git clean -fd", None)?;
            println!(
                "{}\n",
                "[snap] Workspace is now clean. Proceeding with restore.".green()
            );
        }
    } else if let Some(label) =
        create_rescue_snapshot_if_needed(&snapshot_to_restore.tag, &snapshot_commit, dirty)?
    {
        println!(
            "{}",
            format!("[snap] Rescue snapshot created: {}", label).green()
        );
    }

    println!(
        "\n{} \"{}\"...",
        "[snap] Restoring project files for snapshot".cyan(),
        snapshot_to_restore.tag
    );
    run_command_args("git", &["reset", "--hard", &snapshot_commit], None)?;

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

    let mut dirs_to_remove: Vec<_> = source_empty.difference(&target_empty).collect();
    dirs_to_remove.sort_by_key(|path| Reverse(path.len()));
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
            let _ = crate::os::set_readonly(&full_path, false);
        }
    }
    // Paths to make READ-ONLY: are read-only in target but NOT in source.
    for path_str in target_readonly.difference(&source_readonly) {
        let full_path = cwd.join(path_str);
        if full_path.exists() {
            let _ = crate::os::set_readonly(&full_path, true);
        }
    }

    println!(
        "\n{}",
        "[snap] Restore complete. Your project is now at the state of this snapshot.".green()
    );
    println!();
    Ok(())
}

fn print_dry_run(
    target_tag: &str,
    target_commit: &str,
    current_head: Option<&str>,
    dirty: bool,
    no_rescue: bool,
) -> Result<()> {
    println!("\n{}", "[snap] Restore dry run".cyan().bold());
    println!("  Target snapshot: {}", target_tag.bold());
    println!("  Target commit: {}", short_hash(target_commit));
    match current_head {
        Some(head) => println!("  Current HEAD: {}", short_hash(head)),
        None => println!("  Current HEAD: unavailable"),
    }
    println!(
        "  Workspace: {}",
        if dirty {
            "dirty".yellow()
        } else {
            "clean".green()
        }
    );

    if no_rescue {
        println!("  Rescue snapshot: disabled by --no-rescue");
        if dirty {
            println!("  Local changes: would be discarded after confirmation");
        }
    } else if dirty {
        println!("  Rescue snapshot: would be created before restore");
    } else if current_head == Some(target_commit) {
        println!("  Rescue snapshot: not needed; target is already current HEAD");
    } else if let Some(head) = current_head {
        let current_snapshots = get_snapshots_pointing_at(head)?;
        if current_snapshots.is_empty() {
            println!("  Rescue snapshot: would tag the current HEAD before restore");
        } else {
            let labels: Vec<_> = current_snapshots
                .iter()
                .map(|snapshot| snapshot.tag.as_str())
                .collect();
            println!(
                "  Rescue snapshot: not needed; current HEAD already has snapshot tag(s): {}",
                labels.join(", ")
            );
        }
    } else {
        println!("  Rescue snapshot: unavailable; current HEAD could not be read");
    }

    println!("  Files changed: no");
    println!("  Tags created: no");
    println!();
    Ok(())
}

fn create_rescue_snapshot_if_needed(
    target_tag: &str,
    target_commit: &str,
    dirty: bool,
) -> Result<Option<String>> {
    let Some(current_head) = get_active_commit_full()? else {
        return Ok(None);
    };

    if !dirty && current_head == target_commit {
        return Ok(None);
    }

    if !dirty && !get_snapshots_pointing_at(&current_head)?.is_empty() {
        return Ok(None);
    }

    let label = unique_rescue_label()?;
    println!(
        "{}",
        format!(
            "[snap] Creating rescue snapshot '{}' before restore...",
            label
        )
        .cyan()
    );

    let metadata = gather_metadata()?;
    let metadata_blob_hash = hash_metadata_blob(&metadata)?;
    if let Some(hash) = metadata_blob_hash.as_deref() {
        pin_metadata_blob(hash)?;
    }

    if dirty {
        run_command("git add -A", None)?;
        run_command_args(
            "git",
            &[
                "commit",
                "--allow-empty",
                "-m",
                &format!("Snapshot: {}", label),
            ],
            None,
        )?;
    }

    let description = format!("Rescue snapshot before restoring '{}'.", target_tag);
    let tag_message = create_tag_message(&description, metadata_blob_hash.as_deref());
    if dirty {
        run_command_args("git", &["tag", "-a", &label, "-F", "-"], Some(&tag_message))?;
    } else {
        run_command_args(
            "git",
            &["tag", "-a", &label, "-F", "-", &current_head],
            Some(&tag_message),
        )?;
    }

    Ok(Some(label))
}

fn unique_rescue_label() -> Result<String> {
    let base = format!("snap-rescue-{}", Local::now().format("%Y%m%d-%H%M%S"));
    for suffix in 0..100 {
        let label = if suffix == 0 {
            base.clone()
        } else {
            format!("{}-{}", base, suffix)
        };
        let tag_ref = format!("refs/tags/{}", label);
        if !run_git(&["show-ref", "--verify", "--quiet", &tag_ref], None)?.success {
            return Ok(label);
        }
    }

    Err(anyhow!("Could not create a unique rescue snapshot label."))
}

fn short_hash(hash: &str) -> &str {
    hash.get(..7).unwrap_or(hash)
}
