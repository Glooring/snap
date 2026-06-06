use crate::cli::SyncArgs;
use crate::commands::{pull, push};
use anyhow::Result;
use colored::*;

pub fn execute(_args: SyncArgs) -> Result<()> {
    sync_snap_aware()
}

pub fn sync_snap_aware() -> Result<()> {
    println!("{}", "[snap] Syncing project with origin...".cyan().bold());
    let pull_summary = pull::pull_snap_aware()?;
    let push_summary = push::push_snap_aware()?;

    println!("\n{}", "[snap] Sync complete.".green().bold());
    println!("  Branch: {}", push_summary.branch);
    println!(
        "  Pull: {}",
        if pull_summary.pulled_branch {
            "branch pulled with rebase"
        } else {
            "branch pull skipped (no upstream)"
        }
    );
    println!(
        "  Push: branch pushed{}",
        if push_summary.set_upstream {
            " and upstream set"
        } else {
            ""
        }
    );
    println!("  Snapshot tags: synced");
    println!(
        "  Snap metadata refs pushed: {}",
        push_summary.metadata_refs
    );
    println!();
    Ok(())
}
