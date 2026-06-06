use crate::cli::DoctorArgs;
use crate::git_health::{
    collect_health_report, create_repair_plan, repair_git_repository, GitHealthReport, RepairPlan,
};
use anyhow::{anyhow, Result};
use colored::*;
use std::io::{self, Write};

pub fn execute(args: DoctorArgs) -> Result<()> {
    if args.accept_metadata_loss && !args.repair {
        return Err(anyhow!(
            "`--accept-metadata-loss` must be used with `snap doctor --repair`."
        ));
    }

    let report = collect_health_report()?;
    print_report(&report);

    if args.repair {
        repair(report, args.accept_metadata_loss)?;
    }

    Ok(())
}

fn print_report(report: &GitHealthReport) {
    println!("\n{}", "[snap] Git health report".cyan().bold());

    if !report.is_git_repo {
        println!("  {} No .git directory found.", "ERR".red().bold());
        println!("  Run `snap init` to initialize this project.");
        println!();
        return;
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

    print_metadata_report(report);

    if report.has_errors() {
        println!("\n{}", "[snap] Problems were found.".yellow().bold());
        println!("  Run `snap doctor --repair` to repair safe cases with a backup.");
        println!("  See `doc/REPAIR_GIT_ERRORS.md` for the manual repair flow.");
    } else if report.has_warnings() {
        println!("\n{}", "[snap] Warnings were found.".yellow().bold());
        if report.has_repairable_warnings() {
            println!("  Run `snap doctor --repair` to repair safe metadata pinning cases.");
        }
        if report.has_historical_metadata_loss() {
            println!(
                "  Some historical snapshot metadata is missing and cannot be reconstructed automatically."
            );
            println!(
                "  Snapshot file contents remain available, but empty-dir/hidden/read-only metadata is degraded for those old snapshots."
            );
            println!(
                "  To accept that loss and make future doctor checks clean, run `snap doctor --repair --accept-metadata-loss`."
            );
        }
    } else {
        println!(
            "\n{}",
            "[snap] Git repository looks healthy.".green().bold()
        );
    }

    println!();
}

fn repair(report: GitHealthReport, accept_metadata_loss: bool) -> Result<()> {
    if !report.has_problems() {
        println!("{}", "[snap] No repair needed.".green());
        return Ok(());
    }

    let plan = create_repair_plan(&report, accept_metadata_loss)?;
    print_repair_plan(&plan);

    if !has_repair_actions(&plan) {
        if report.has_historical_metadata_loss() {
            println!(
                "{}",
                "[snap] No safe automatic repair remains for historical metadata loss.".yellow()
            );
            println!("  The missing metadata blobs were already pruned by Git.");
            println!("  Snapshot file contents remain available, but empty-dir/hidden/read-only metadata cannot be reconstructed automatically.");
            println!(
                "  Run `snap doctor --repair --accept-metadata-loss` to rewrite historical tags without those broken metadata refs."
            );
        } else {
            println!(
                "{}",
                "[snap] No safe automatic repair is available for the detected problem.".yellow()
            );
            println!("  See `doc/REPAIR_GIT_ERRORS.md` for the manual repair flow.");
        }
        return Ok(());
    }

    if !confirm_repair("[snap] Create a .git backup and apply this repair plan?")? {
        println!("{}", "[snap] Repair cancelled.".yellow());
        return Ok(());
    }

    let outcome = repair_git_repository(&plan)?;
    println!("\n{}", "[snap] Repair applied.".green().bold());
    println!("  Backup: {}", outcome.backup_path.display());
    println!(
        "  Deleted empty Git files: {}",
        outcome.deleted_empty_files.len()
    );
    if let Some(branch) = outcome.repaired_branch.as_deref() {
        println!("  Repaired branch ref: {}", branch);
    }
    if outcome.repaired_head {
        println!("  Normalized .git/HEAD");
    }
    if outcome.reset_index {
        println!("  Rebuilt Git index");
    }
    if !outcome.pinned_metadata_refs.is_empty() {
        println!(
            "  Pinned metadata refs: {}",
            outcome.pinned_metadata_refs.len()
        );
    }
    for tag in &outcome.repaired_active_metadata_tags {
        println!("  Repaired active snapshot metadata: {}", tag);
    }
    if !outcome.forgotten_metadata_tags.is_empty() {
        println!(
            "  Accepted historical metadata loss for: {} snapshot tag(s)",
            outcome.forgotten_metadata_tags.len()
        );
    }

    println!("\n{}", "[snap] Rechecking repository...".cyan());
    let final_report = collect_health_report()?;
    print_report(&final_report);

    Ok(())
}

fn print_repair_plan(plan: &RepairPlan) {
    println!("{}", "[snap] Repair plan:".cyan().bold());

    if plan.empty_git_files.is_empty() {
        println!("  - No empty Git object/ref files to delete.");
    } else {
        println!(
            "  - Delete {} empty Git object/ref file(s).",
            plan.empty_git_files.len()
        );
        for path in plan.empty_git_files.iter().take(10) {
            println!("    - {}", path.display());
        }
        if plan.empty_git_files.len() > 10 {
            println!("    ... and {} more", plan.empty_git_files.len() - 10);
        }
    }

    if let (Some(branch), Some(commit)) =
        (plan.target_branch.as_deref(), plan.target_commit.as_deref())
    {
        println!(
            "  - Repair branch '{}' to commit {}.",
            branch,
            short_hash(commit)
        );
    }

    if plan.needs_head_repair {
        if let Some(branch) = plan.target_branch.as_deref() {
            println!("  - Normalize .git/HEAD to refs/heads/{}.", branch);
        }
    }

    if !plan.metadata_refs_to_pin.is_empty() {
        println!(
            "  - Pin {} existing snapshot metadata blob(s).",
            plan.metadata_refs_to_pin.len()
        );
    }

    for tag in &plan.active_metadata_repairs {
        println!(
            "  - Regenerate missing metadata for active snapshot '{}'.",
            tag
        );
    }

    if !plan.metadata_tags_to_forget.is_empty() {
        println!(
            "  - Rewrite {} historical snapshot tag(s) without broken metadata refs.",
            plan.metadata_tags_to_forget.len()
        );
        for tag in plan.metadata_tags_to_forget.iter().take(10) {
            println!("    - {}", tag);
        }
        if plan.metadata_tags_to_forget.len() > 10 {
            println!(
                "    ... and {} more",
                plan.metadata_tags_to_forget.len() - 10
            );
        }
    }

    if has_repair_actions(plan) {
        println!("  - Create a full .git backup before modifying anything.");
        if !plan.empty_git_files.is_empty() || plan.needs_branch_repair || plan.needs_head_repair {
            println!("  - Rebuild the Git index with `git reset --mixed HEAD`.");
        }
    }
}

fn has_repair_actions(plan: &RepairPlan) -> bool {
    !plan.empty_git_files.is_empty()
        || plan.needs_branch_repair
        || plan.needs_head_repair
        || !plan.metadata_refs_to_pin.is_empty()
        || !plan.active_metadata_repairs.is_empty()
        || !plan.metadata_tags_to_forget.is_empty()
}

fn confirm_repair(question: &str) -> Result<bool> {
    print!("{} [y/N] ", question.yellow());
    io::stdout().flush()?;

    let mut answer = String::new();
    io::stdin().read_line(&mut answer)?;
    Ok(answer.trim().to_lowercase().starts_with('y'))
}

fn print_status(label: &str, ok: bool, detail: &str) {
    let status = if ok {
        "OK".green().bold()
    } else {
        "ERR".red().bold()
    };
    println!("  {} {}: {}", status, label, detail);
}

fn print_metadata_report(report: &GitHealthReport) {
    print_optional_error("snapshot metadata scan", report.metadata_error.as_deref());
    if report.metadata_error.is_some() {
        return;
    }

    let invalid_metadata: Vec<_> = report
        .metadata_blobs
        .iter()
        .filter(|metadata| metadata.error.is_some())
        .collect();
    let unpinned_metadata: Vec<_> = report
        .metadata_blobs
        .iter()
        .filter(|metadata| metadata.error.is_none() && !metadata.pinned)
        .collect();

    let active_invalid_count = invalid_metadata
        .iter()
        .filter(|metadata| metadata.snapshot_commit.as_deref() == report.head_commit.as_deref())
        .count();
    let historical_invalid_count = invalid_metadata.len().saturating_sub(active_invalid_count);

    let status = if active_invalid_count > 0 {
        "ERR".red().bold()
    } else if !invalid_metadata.is_empty()
        || !unpinned_metadata.is_empty()
        || !report.unused_metadata_refs.is_empty()
    {
        "WARN".yellow().bold()
    } else {
        "OK".green().bold()
    };

    println!(
        "  {} Snapshot metadata: {} checked, {} active invalid, {} historical invalid, {} unpinned",
        status,
        report.metadata_blobs.len(),
        active_invalid_count,
        historical_invalid_count,
        unpinned_metadata.len()
    );

    for metadata in invalid_metadata.iter().take(10) {
        println!(
            "    - {}: {} [{}] ({})",
            metadata.snapshot_tag,
            short_hash(&metadata.blob_hash),
            metadata.object_type.as_deref().unwrap_or("missing"),
            metadata.error.as_deref().unwrap_or("invalid metadata")
        );
    }
    if invalid_metadata.len() > 10 {
        println!("    ... and {} more", invalid_metadata.len() - 10);
    }

    for metadata in unpinned_metadata.iter().take(10) {
        match metadata.pin_target.as_deref() {
            Some(target) => println!(
                "    - {}: metadata ref points to {}, expected {}",
                metadata.snapshot_tag,
                short_hash(target),
                short_hash(&metadata.blob_hash)
            ),
            None => println!(
                "    - {}: metadata blob {} is not protected from git gc",
                metadata.snapshot_tag,
                short_hash(&metadata.blob_hash)
            ),
        }
    }
    if unpinned_metadata.len() > 10 {
        println!(
            "    ... and {} more unpinned metadata blobs",
            unpinned_metadata.len() - 10
        );
    }

    if !report.unused_metadata_refs.is_empty() {
        println!(
            "  {} Unused metadata refs: {}",
            "WARN".yellow().bold(),
            report.unused_metadata_refs.len()
        );
        for ref_name in report.unused_metadata_refs.iter().take(10) {
            println!("    - {}", ref_name);
        }
        if report.unused_metadata_refs.len() > 10 {
            println!(
                "    ... and {} more",
                report.unused_metadata_refs.len() - 10
            );
        }
    }
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
