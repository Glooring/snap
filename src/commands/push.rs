use crate::cli::PushArgs;
use crate::git;
use crate::git_health::ensure_git_healthy_for_write;
use anyhow::{anyhow, Result};
use colored::*;

#[derive(Debug, Clone)]
pub struct PushSummary {
    pub branch: String,
    pub set_upstream: bool,
    pub metadata_refs: usize,
}

pub fn execute(_args: PushArgs) -> Result<()> {
    let summary = push_snap_aware()?;
    print_summary(&summary);
    Ok(())
}

pub fn push_snap_aware() -> Result<PushSummary> {
    ensure_git_healthy_for_write(false)?;
    git::ensure_no_operation_in_progress()?;
    git::ensure_remote(git::ORIGIN)?;

    if !git::has_head_commit()? {
        return Err(anyhow!(
            "No commits exist yet. Create one with `snap save \"message\"` or `snap new <label>` before pushing."
        ));
    }

    let branch = git::current_branch()?;

    println!("{}", "[snap] Pushing current branch...".cyan());
    let set_upstream = git::push_branch(git::ORIGIN)?;

    println!("{}", "[snap] Pushing snapshot tags...".cyan());
    git::push_tags(git::ORIGIN)?;

    println!("{}", "[snap] Pushing snap metadata refs...".cyan());
    let metadata_refs = git::push_metadata_refs(git::ORIGIN)?;

    Ok(PushSummary {
        branch,
        set_upstream,
        metadata_refs,
    })
}

fn print_summary(summary: &PushSummary) {
    println!("\n{}", "[snap] Push complete.".green().bold());
    println!(
        "  Branch: {}{}",
        summary.branch,
        if summary.set_upstream {
            " (upstream set)"
        } else {
            ""
        }
    );
    println!("  Snapshot tags: pushed");
    println!("  Snap metadata refs: {}", summary.metadata_refs);
    println!();
}
