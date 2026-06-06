use crate::cli::UpdateRepoArgs;
use crate::commands::{save, sync};
use anyhow::Result;
use colored::*;

pub fn execute(args: UpdateRepoArgs) -> Result<()> {
    println!(
        "{}",
        "[snap] update-repo is a shortcut for `snap save` followed by `snap sync`."
            .cyan()
            .bold()
    );

    let committed = save::save_changes(args.message)?;
    if !committed {
        println!(
            "{}",
            "[snap] No new commit was created; syncing existing commits, tags, and metadata refs."
                .yellow()
        );
        println!();
    }

    sync::sync_snap_aware()
}
