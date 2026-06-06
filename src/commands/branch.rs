use crate::cli::{BranchArgs, BranchCommands, BranchDeleteArgs, BranchNameArgs};
use crate::git;
use crate::git_health::ensure_git_healthy_for_write;
use anyhow::{anyhow, Result};
use colored::*;
use std::io::{self, Write};

pub fn execute(args: BranchArgs) -> Result<()> {
    match args.command {
        BranchCommands::List(_) => list(),
        BranchCommands::New(args) => new(args),
        BranchCommands::Switch(args) => switch(args),
        BranchCommands::Delete(args) => delete(args),
        BranchCommands::Merge(args) => merge(args),
    }
}

fn list() -> Result<()> {
    let branches = git::list_branches()?;

    println!("\n{}", "[snap] Branches".cyan().bold());
    if branches.is_empty() {
        println!("  {} No local branches found.", "WARN".yellow().bold());
        println!();
        return Ok(());
    }

    println!();
    println!("  {:<2} {:<28} {:<28} Status", "", "Branch", "Upstream");
    println!(
        "  {:<2} {:<28} {:<28} {}",
        "", "------", "--------", "------"
    );
    for branch in branches {
        let marker = if branch.current { "*" } else { " " };
        let upstream = branch.upstream.as_deref().unwrap_or("-");
        println!(
            "  {:<2} {:<28} {:<28} {}",
            marker,
            branch.name,
            upstream,
            branch.status()
        );
    }
    println!();
    Ok(())
}

fn new(args: BranchNameArgs) -> Result<()> {
    ensure_branch_write_ready()?;
    git::validate_branch_name(&args.name)?;
    if git::branch_exists(&args.name)? {
        return Err(anyhow!(
            "Local branch '{}' already exists. Use `snap branch switch {}` to switch to it.",
            args.name,
            args.name
        ));
    }

    println!(
        "{}",
        format!("[snap] Creating and switching to branch '{}'...", args.name).cyan()
    );
    git::create_branch_and_switch(&args.name)?;
    println!(
        "{}",
        format!("[snap] Now on branch '{}'.", args.name).green()
    );
    println!("  Next: run `snap sync` when you want to publish this branch.");
    println!();
    Ok(())
}

fn switch(args: BranchNameArgs) -> Result<()> {
    ensure_branch_preflight()?;
    git::validate_branch_name(&args.name)?;
    if !git::branch_exists(&args.name)? {
        return Err(anyhow!(
            "Local branch '{}' does not exist. Run `snap branch list` to see available branches.",
            args.name
        ));
    }

    if git::current_branch()? == args.name {
        println!(
            "{}",
            format!("[snap] Already on branch '{}'.", args.name).yellow()
        );
        println!();
        return Ok(());
    }

    ensure_clean_worktree()?;

    println!(
        "{}",
        format!("[snap] Switching to branch '{}'...", args.name).cyan()
    );
    git::switch_branch(&args.name)?;
    println!(
        "{}",
        format!("[snap] Now on branch '{}'.", args.name).green()
    );
    println!();
    Ok(())
}

fn delete(args: BranchDeleteArgs) -> Result<()> {
    ensure_branch_write_ready()?;
    git::validate_branch_name(&args.name)?;
    if !git::branch_exists(&args.name)? {
        return Err(anyhow!(
            "Local branch '{}' does not exist. Run `snap branch list` to see available branches.",
            args.name
        ));
    }

    if git::current_branch()? == args.name {
        return Err(anyhow!(
            "Cannot delete the current branch '{}'. Switch to another branch first.",
            args.name
        ));
    }

    if args.force && !confirm_force_delete(&args.name)? {
        println!("{}", "[snap] Branch deletion cancelled.".yellow());
        println!();
        return Ok(());
    }

    println!(
        "{}",
        format!(
            "[snap] Deleting branch '{}'{}...",
            args.name,
            if args.force { " with force" } else { "" }
        )
        .cyan()
    );
    git::delete_branch(&args.name, args.force)?;
    println!(
        "{}",
        format!("[snap] Branch '{}' deleted.", args.name).green()
    );
    println!();
    Ok(())
}

fn merge(args: BranchNameArgs) -> Result<()> {
    ensure_branch_write_ready()?;
    git::validate_branch_name(&args.name)?;
    if !git::branch_exists(&args.name)? {
        return Err(anyhow!(
            "Local branch '{}' does not exist. Run `snap branch list` to see available branches.",
            args.name
        ));
    }

    println!(
        "{}",
        format!(
            "[snap] Merging branch '{}' into current branch...",
            args.name
        )
        .cyan()
    );
    git::merge_branch(&args.name)?;
    println!(
        "{}",
        format!("[snap] Branch '{}' merged.", args.name).green()
    );
    println!();
    Ok(())
}

fn ensure_branch_write_ready() -> Result<()> {
    ensure_branch_preflight()?;
    ensure_clean_worktree()
}

fn ensure_branch_preflight() -> Result<()> {
    ensure_git_healthy_for_write(false)?;
    git::ensure_no_operation_in_progress()
}

fn ensure_clean_worktree() -> Result<()> {
    let summary = git::worktree_summary()?;
    if !summary.is_clean() {
        return Err(anyhow!(
            "Your working tree has local changes ({}).\nSave them first with `snap save \"message\"`, or clean them before running this branch command.",
            summary.describe()
        ));
    }
    Ok(())
}

fn confirm_force_delete(branch: &str) -> Result<bool> {
    println!("\n{}", "[snap] Force branch delete".red().bold());
    println!("  Branch: {}", branch.bold());
    println!("  This can delete an unmerged local branch.");
    print!("Type the branch name to confirm: ");
    io::stdout().flush()?;

    let mut confirmation = String::new();
    io::stdin().read_line(&mut confirmation)?;
    Ok(confirmation.trim() == branch)
}
