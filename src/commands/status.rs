use crate::cli::StatusArgs;
use crate::git;
use crate::utils::{get_active_commit_full, get_snapshots_pointing_at, get_snapshots_with_limit};
use anyhow::Result;
use colored::*;

pub fn execute(_args: StatusArgs) -> Result<()> {
    println!("\n{}", "[snap] Project status".cyan().bold());

    match git::current_branch() {
        Ok(branch) => println!("  {} Branch: {}", "OK".green().bold(), branch),
        Err(error) => println!("  {} Branch: {}", "ERR".red().bold(), error),
    }

    match git::remote_url(git::ORIGIN)? {
        Some(url) => println!("  {} Remote origin: {}", "OK".green().bold(), url),
        None => println!("  {} Remote origin: not configured", "WARN".yellow().bold()),
    }

    match git::upstream_branch()? {
        Some(upstream) => {
            let relation = match git::ahead_behind()? {
                Some((0, 0)) => "up to date".to_string(),
                Some((ahead, 0)) => format!("ahead {}", ahead),
                Some((0, behind)) => format!("behind {}", behind),
                Some((ahead, behind)) => format!("ahead {}, behind {}", ahead, behind),
                None => "unknown".to_string(),
            };
            println!(
                "  {} Upstream: {} ({})",
                "OK".green().bold(),
                upstream,
                relation
            );
        }
        None => println!("  {} Upstream: not configured", "WARN".yellow().bold()),
    }

    let summary = git::worktree_summary()?;
    let status_label = if summary.is_clean() {
        "OK".green().bold()
    } else {
        "WARN".yellow().bold()
    };
    println!("  {} Working tree: {}", status_label, summary.describe());

    let latest_snapshot = get_snapshots_with_limit(Some(1))?.into_iter().next();
    let active_snapshot = match get_active_commit_full()? {
        Some(commit) => get_snapshots_pointing_at(&commit)?.into_iter().next(),
        None => None,
    };

    if let Some(snapshot) = active_snapshot {
        println!(
            "  {} Active snapshot: {}",
            "OK".green().bold(),
            snapshot.tag
        );
    } else {
        println!("  {} Active snapshot: none", "WARN".yellow().bold());
    }

    if let Some(snapshot) = latest_snapshot {
        println!(
            "  {} Latest snapshot: {}",
            "OK".green().bold(),
            snapshot.tag
        );
    } else {
        println!("  {} Latest snapshot: none", "WARN".yellow().bold());
    }

    println!();
    Ok(())
}
