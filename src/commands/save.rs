use crate::cli::SaveArgs;
use crate::git;
use crate::git_health::{ensure_git_healthy_for_write, run_git_success};
use anyhow::{anyhow, Result};
use colored::*;
use inquire::Text;

pub fn execute(args: SaveArgs) -> Result<()> {
    save_changes(args.message).map(|_| ())
}

pub fn save_changes(message_parts: Vec<String>) -> Result<bool> {
    ensure_git_healthy_for_write(true)?;
    git::ensure_no_operation_in_progress()?;
    git::current_branch()?;

    if !git::has_worktree_changes()? {
        println!(
            "{}",
            "[snap] No changes to commit. Working tree is clean.".yellow()
        );
        println!();
        return Ok(false);
    }

    let message = commit_message(message_parts)?;

    println!("{}", "[snap] Staging all changes...".cyan());
    run_git_success(&["add", "-A"], None)?;

    println!("{}", "[snap] Creating Git commit...".cyan());
    run_git_success(&["commit", "-m", &message], None)?;

    println!(
        "\n{} {}",
        "[snap] Changes saved with commit message:".green(),
        message.bold()
    );
    println!();
    Ok(true)
}

fn commit_message(parts: Vec<String>) -> Result<String> {
    let joined = parts.join(" ").trim().to_string();
    let message = if joined.is_empty() {
        Text::new("Commit message:").prompt()?.trim().to_string()
    } else {
        joined
    };

    if message.is_empty() {
        return Err(anyhow!("Commit message cannot be empty."));
    }

    Ok(message)
}
