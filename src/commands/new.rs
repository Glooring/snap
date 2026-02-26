use crate::cli::NewArgs;
use crate::utils::{
    check_dirty, create_tag_message, find_snapshot, gather_metadata, get_active_commit_full,
    get_snapshots, hash_metadata_blob, load_metadata_for_snapshot, run_command,
};
use anyhow::{anyhow, Context, Result};
use colored::*;

fn sanitize_tag_name(label: &str) -> String {
    label
        .trim()
        .replace(char::is_whitespace, "-")
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '-' || *c == '.' || *c == '_')
        .collect()
}

pub fn execute(args: NewArgs) -> Result<()> {
    let tag_name = sanitize_tag_name(&args.label);
    let all_snapshots = get_snapshots()?;

    if all_snapshots.iter().any(|s| s.tag == tag_name) {
        return Err(anyhow!(
            "A snapshot with the label \"{}\" already exists.",
            tag_name
        ));
    }

    // Check for both file changes (Git) and metadata changes.
    let git_has_changes = check_dirty()?;
    let current_metadata = gather_metadata()?;

    let old_metadata = match get_active_commit_full()? {
        Some(id) => {
            if let Some(active_snapshot) = find_snapshot(&all_snapshots, &id) {
                load_metadata_for_snapshot(active_snapshot)?
            } else {
                // Not a snapshot commit, so treat as having no prior metadata for comparison.
                Default::default()
            }
        }
        None => {
            // No commits yet, so no prior metadata.
            Default::default()
        }
    };

    let metadata_has_changes = current_metadata != old_metadata;

    // Only exit if there are absolutely no changes.
    if !git_has_changes && !metadata_has_changes {
        println!(
            "{}",
            "[snap] No changes to commit. Working tree is clean.\n".yellow()
        );
        return Ok(());
    }

    let description = args.description.join(" ");

    println!("\n{}", "[snap] Step 1/4: Scanning for metadata (hidden files, empty dirs)...".cyan());
    // --- START: CORRECTED LINE ---
    // Reuse the metadata we already gathered.
    let metadata_blob_hash = hash_metadata_blob(&current_metadata)?;
    // --- END: CORRECTED LINE ---

    println!("{}", "[snap] Step 2/4: Staging all files...".cyan());
    run_command("git add -A", None)?;

    println!("{}", "[snap] Step 3/4: Creating the commit...".cyan());
    let commit_msg = format!("Snapshot: {}", tag_name);
    // Use --allow-empty to create a commit even if only metadata changed.
    run_command(&format!("git commit --allow-empty -m \"{}\"", commit_msg), None)?;

    let full_id = get_active_commit_full()?.context("Failed to get new commit ID")?;
    
    println!(
        "{}",
        "[snap] Step 4/4: Creating the annotated snapshot tag...".cyan()
    );
    let tag_message = create_tag_message(&description, metadata_blob_hash.as_deref());
    let tag_cmd = format!("git tag -a {} -F -", tag_name);
    run_command(&tag_cmd, Some(&tag_message))?;

    let short_id = &full_id[..7];
    println!(
        "\n{} [{}] {}",
        "[snap] New snapshot created:".green(),
        short_id,
        tag_name.bold()
    );

    println!();
    Ok(())
}