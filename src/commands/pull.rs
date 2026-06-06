use crate::cli::PullArgs;
use crate::git;
use crate::git_health::ensure_git_healthy_for_write;
use anyhow::{anyhow, Result};
use colored::*;

#[derive(Debug, Clone)]
pub struct PullSummary {
    pub branch: String,
    pub pulled_branch: bool,
    pub fetched_metadata: bool,
}

pub fn execute(_args: PullArgs) -> Result<()> {
    let summary = pull_snap_aware()?;
    print_summary(&summary);
    Ok(())
}

pub fn pull_snap_aware() -> Result<PullSummary> {
    ensure_git_healthy_for_write(false)?;
    git::ensure_no_operation_in_progress()?;
    git::ensure_remote(git::ORIGIN)?;

    if git::has_worktree_changes()? {
        return Err(anyhow!(
            "Your working tree has local changes.\nSave them first with `snap save \"message\"`, then retry `snap pull`."
        ));
    }

    let branch = git::current_branch()?;

    println!("{}", "[snap] Fetching branch updates...".cyan());
    git::fetch(git::ORIGIN)?;

    println!("{}", "[snap] Fetching snapshot tags...".cyan());
    git::fetch_tags(git::ORIGIN)?;

    println!("{}", "[snap] Fetching snap metadata refs...".cyan());
    let fetched_metadata = git::fetch_metadata_refs(git::ORIGIN)?;

    let pulled_branch = if git::upstream_branch()?.is_some() {
        println!("{}", "[snap] Pulling current branch with rebase...".cyan());
        git::pull_rebase()?;
        true
    } else {
        println!(
            "{}",
            "[snap] Current branch has no upstream yet. Branch pull skipped.".yellow()
        );
        false
    };

    Ok(PullSummary {
        branch,
        pulled_branch,
        fetched_metadata,
    })
}

fn print_summary(summary: &PullSummary) {
    println!("\n{}", "[snap] Pull complete.".green().bold());
    println!(
        "  Branch: {} ({})",
        summary.branch,
        if summary.pulled_branch {
            "pulled with rebase"
        } else {
            "no upstream"
        }
    );
    println!("  Snapshot tags: fetched");
    println!(
        "  Snap metadata refs: {}",
        if summary.fetched_metadata {
            "fetched"
        } else {
            "none on remote"
        }
    );
    println!();
}
