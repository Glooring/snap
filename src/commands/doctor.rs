use crate::cli::DoctorArgs;
use crate::git_health::collect_health_report;
use anyhow::Result;
use colored::*;

pub fn execute(_args: DoctorArgs) -> Result<()> {
    let report = collect_health_report()?;

    println!("\n{}", "[snap] Git health report".cyan().bold());

    if !report.is_git_repo {
        println!("  {} No .git directory found.", "ERR".red().bold());
        println!("  Run `snap init` to initialize this project.");
        println!();
        return Ok(());
    }

    print_status(
        "Empty object/ref files",
        report.empty_git_files.is_empty(),
        &format!("{} found", report.empty_git_files.len()),
    );
    for path in report.empty_git_files.iter().take(10) {
        println!("    - {}", path.display());
    }
    if report.empty_git_files.len() > 10 {
        println!("    ... and {} more", report.empty_git_files.len() - 10);
    }

    print_optional_error("git status", report.status_error.as_deref());

    match report.head_commit.as_deref() {
        Some(commit) => println!(
            "  {} HEAD commit: {}",
            "OK".green().bold(),
            short_hash(commit)
        ),
        None if report.head_error.is_none() => {
            println!(
                "  {} HEAD has no commit yet (new repository).",
                "OK".green().bold()
            )
        }
        None => print_optional_error("HEAD commit", report.head_error.as_deref()),
    }

    if report.detached_head {
        println!("  {} HEAD is detached.", "ERR".red().bold());
    } else if let Some(branch) = report.current_branch.as_deref() {
        println!("  {} Current branch: {}", "OK".green().bold(), branch);
    } else {
        println!(
            "  {} Current branch: unborn or unavailable.",
            "OK".green().bold()
        );
    }

    print_optional_error("current branch ref", report.branch_error.as_deref());
    print_optional_error("snapshot tag scan", report.snapshots_error.as_deref());

    let invalid_snapshots: Vec<_> = report
        .snapshots
        .iter()
        .filter(|snapshot| snapshot.error.is_some())
        .collect();

    print_status(
        "Snapshot tags",
        report.snapshots_error.is_none() && invalid_snapshots.is_empty(),
        &format!(
            "{} checked, {} invalid",
            report.snapshots.len(),
            invalid_snapshots.len()
        ),
    );
    for snapshot in invalid_snapshots.iter().take(10) {
        println!(
            "    - {}: {}",
            snapshot.tag,
            snapshot.error.as_deref().unwrap_or("invalid")
        );
    }
    if invalid_snapshots.len() > 10 {
        println!("    ... and {} more", invalid_snapshots.len() - 10);
    }

    if let Some(snapshot) = report.latest_valid_snapshot() {
        println!(
            "  {} Latest valid snapshot: {}",
            "OK".green().bold(),
            snapshot.tag
        );
    } else if report.snapshots.is_empty() && report.snapshots_error.is_none() {
        println!("  {} No snapshots found yet.", "OK".green().bold());
    } else {
        println!(
            "  {} No valid snapshot could be identified.",
            "ERR".red().bold()
        );
    }

    if report.has_errors() {
        println!("\n{}", "[snap] Problems were found.".yellow().bold());
        println!("  This command is read-only and did not repair anything.");
        println!("  See `doc/REPAIR_GIT_ERRORS.md` for the manual repair flow.");
    } else {
        println!(
            "\n{}",
            "[snap] Git repository looks healthy.".green().bold()
        );
    }

    println!();
    Ok(())
}

fn print_status(label: &str, ok: bool, detail: &str) {
    let status = if ok {
        "OK".green().bold()
    } else {
        "ERR".red().bold()
    };
    println!("  {} {}: {}", status, label, detail);
}

fn print_optional_error(label: &str, error: Option<&str>) {
    match error {
        Some(error) => println!("  {} {}: {}", "ERR".red().bold(), label, first_line(error)),
        None => println!("  {} {}", "OK".green().bold(), label),
    }
}

fn short_hash(hash: &str) -> &str {
    hash.get(..7).unwrap_or(hash)
}

fn first_line(value: &str) -> &str {
    value.lines().next().unwrap_or(value)
}
