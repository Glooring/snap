use crate::cli::UpdateArgs;
use crate::config::load_config;
use crate::utils::{
    ask_yes_no, check_dirty, create_tag_message, find_snapshot, gather_metadata,
    get_active_commit_full, get_snapshots, hash_metadata_blob, load_metadata_for_snapshot,
    run_command,
};
use anyhow::{Context, Result};
use colored::*;

pub fn execute(_args: UpdateArgs) -> Result<()> {
    let config = load_config()?;
    let all_snapshots = get_snapshots()?;

    let active_commit_full = get_active_commit_full()?
        .context("Cannot get active commit. Are you in a git repository?")?;

    let active_snapshot = find_snapshot(&all_snapshots, &active_commit_full)
        .cloned()
        .context("The current state (HEAD) does not correspond to a known snapshot.\nHint: Run `snap restore` to an existing snapshot first.")?;

    // Check for both file changes (Git) and metadata changes before proceeding.
    let git_has_changes = check_dirty()?;
    let current_metadata = gather_metadata()?;
    let old_metadata = load_metadata_for_snapshot(&active_snapshot)?;
    let metadata_has_changes = current_metadata != old_metadata;

    if !git_has_changes && !metadata_has_changes {
        println!("{}", "[snap] No changes to update. Working tree is clean.".yellow());
        return Ok(());
    }

    println!("\n{}", "[snap] This command will replace the active snapshot with the current project state.".cyan());
    println!("{}", "  Target Snapshot:".yellow());
    println!("    Label:       {}", active_snapshot.tag);
    println!("    Description: {}", active_snapshot.description);

    if config.options.confirm_command {
        let proceed = ask_yes_no(
            &format!("[snap] This will amend the commit for snapshot \"{}\". This action is hard to undo.", active_snapshot.tag),
            false
        )?;
        if !proceed {
            println!("{}", "[snap] Update cancelled.".yellow());
            return Ok(());
        }
    }
    
    println!("\n{}", "[snap] Step 1/3: Scanning for new metadata...".cyan());
    // --- START: CORRECTED LINE ---
    // Reuse the metadata we already gathered.
    let new_metadata_blob_hash = hash_metadata_blob(&current_metadata)?;
    // --- END: CORRECTED LINE ---

    println!("{}", "[snap] Step 2/3: Staging changes and amending commit...".cyan());
    run_command("git add -A", None)?;
    // Use --allow-empty for robustness, though --amend usually implies it's not needed.
    run_command("git commit --amend --no-edit --allow-empty", None)?;
    
    let new_commit_id = get_active_commit_full()?
        .context("Failed to get new commit ID after amend.")?;
    let new_short_id = &new_commit_id[..7];

    println!("{}", format!("[snap] Step 3/3: Moving tag \"{}\" to new commit {}...", active_snapshot.tag, new_short_id).cyan());
    
    let new_tag_message = create_tag_message(&active_snapshot.description, new_metadata_blob_hash.as_deref());
    let tag_cmd = format!("git tag -a -f {} -F -", active_snapshot.tag);
    run_command(&tag_cmd, Some(&new_tag_message))?;

    println!("\n{}", format!("[snap] Update complete. Snapshot \"{}\" now points to new commit [{}].", active_snapshot.tag, new_short_id).green());

    Ok(())
}