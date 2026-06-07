use crate::cli::ListArgs;
use crate::config::{load_config, SortOrder};
use crate::git;
use crate::utils::{
    format_timestamp, get_active_commit_full, get_snapshots, get_snapshots_with_limit, Snapshot,
};
use anyhow::{anyhow, Result};
use colored::*;
use std::cmp::max;
use std::collections::HashMap;
use std::env;

#[derive(Debug, Clone)]
struct ListedSnapshot {
    snapshot: Snapshot,
    branch_display: Option<String>,
}

#[derive(Debug, Clone)]
enum BranchListMode {
    Default,
    Branch(String),
    AllBranches,
}

pub fn execute(args: ListArgs) -> Result<()> {
    let config = load_config()?;
    let branch_mode = branch_mode(&args);
    let branch_aware = !matches!(branch_mode, BranchListMode::Default);

    // Use limit from CLI arg if present, otherwise from config.
    let limit_str = args.limit.as_deref().unwrap_or(&config.options.list_limit);
    let numeric_limit = limit_str
        .parse::<usize>()
        .ok()
        .filter(|limit| *limit > 0 && !limit_str.eq_ignore_ascii_case("all"));
    let query_limit = if !branch_aware && config.options.order_by == SortOrder::Timestamp {
        numeric_limit.map(|limit| limit.saturating_add(1))
    } else {
        None
    };

    let mut snapshots = match query_limit {
        Some(limit) => get_snapshots_with_limit(Some(limit))?,
        None => get_snapshots()?,
    };
    let active_commit = get_active_commit_full()?.unwrap_or_default();

    if config.options.order_by == SortOrder::Label {
        snapshots.sort_by(|a, b| b.tag.cmp(&a.tag));
    }

    let mut snapshots = apply_branch_mode(snapshots, &branch_mode)?;

    let cwd_path = env::current_dir()?;
    let project_name = cwd_path
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("current project");

    println!("\n{} \"{}\":", "[snap] Snapshots for".cyan(), project_name);
    if let BranchListMode::Branch(branch) = &branch_mode {
        println!("{} {}", "[snap] Branch:".cyan(), branch);
    }

    if snapshots.is_empty() {
        println!(
            "\n  {}",
            "No snapshots found. Use \"snap new <label>\" to create one.".yellow()
        );
        return Ok(());
    }

    let mut truncated = false;

    if let Some(limit) = numeric_limit {
        if snapshots.len() > limit {
            snapshots.truncate(limit);
            truncated = true;
        }
    }

    const HEADER_ID: &str = "ID";
    const HEADER_LABEL: &str = "Label";
    const HEADER_BRANCH: &str = "Branch";
    const HEADER_DESC: &str = "Description";
    const HEADER_TIME: &str = "Timestamp";
    const MAX_DESC_WIDTH: usize = 50;
    const COL_PADDING: usize = 2;
    const FORMATTED_TIME_LEN: usize = 16;
    const SHORT_ID_LEN: usize = 7; // The length of the short commit hash

    let show_ids = config.options.show_ids;
    let show_branch_column = matches!(branch_mode, BranchListMode::AllBranches);

    let max_label_len = snapshots
        .iter()
        .map(|s| s.snapshot.tag.len())
        .max()
        .unwrap_or(0);
    let max_branch_len = snapshots
        .iter()
        .filter_map(|s| s.branch_display.as_deref())
        .map(str::len)
        .max()
        .unwrap_or(0);
    let max_desc_len = snapshots
        .iter()
        .map(|s| s.snapshot.description.len())
        .max()
        .unwrap_or(0);

    let id_w = if show_ids {
        max(HEADER_ID.len(), SHORT_ID_LEN)
    } else {
        0
    };
    let label_w = max(HEADER_LABEL.len(), max_label_len);
    let branch_w = if show_branch_column {
        max(HEADER_BRANCH.len(), max_branch_len)
    } else {
        0
    };
    let desc_w = max(HEADER_DESC.len(), max_desc_len).min(MAX_DESC_WIDTH);
    let time_w = max(HEADER_TIME.len(), FORMATTED_TIME_LEN);

    let id_print_w = if show_ids { id_w + COL_PADDING } else { 0 };
    let label_print_w = label_w + COL_PADDING;
    let branch_print_w = if show_branch_column {
        branch_w + COL_PADDING
    } else {
        0
    };
    let desc_print_w = desc_w + COL_PADDING;
    let time_print_w = time_w;

    let mut header = "  ".to_string();
    let mut separator = "  ".to_string();

    if show_ids {
        header.push_str(&format!("{:<width$}", HEADER_ID, width = id_print_w));
        separator.push_str(&format!("{:<width$}", "-".repeat(id_w), width = id_print_w));
    }
    header.push_str(&format!("{:<width$}", HEADER_LABEL, width = label_print_w));
    separator.push_str(&format!(
        "{:<width$}",
        "-".repeat(label_w),
        width = label_print_w
    ));

    if show_branch_column {
        header.push_str(&format!(
            "{:<width$}",
            HEADER_BRANCH,
            width = branch_print_w
        ));
        separator.push_str(&format!(
            "{:<width$}",
            "-".repeat(branch_w),
            width = branch_print_w
        ));
    }

    header.push_str(&format!("{:<width$}", HEADER_DESC, width = desc_print_w));
    separator.push_str(&format!(
        "{:<width$}",
        "-".repeat(desc_w),
        width = desc_print_w
    ));

    header.push_str(HEADER_TIME);
    separator.push_str(&"-".repeat(time_w));

    println!("\n{}", header.bold());
    println!("{}", separator.bold());

    for listed in &snapshots {
        let snap = &listed.snapshot;
        let is_active = !active_commit.is_empty() && snap.full_id == active_commit;
        let mut line = "  ".to_string();

        if show_ids {
            line.push_str(&format!("{:<width$}", snap.id, width = id_print_w));
        }
        line.push_str(&format!("{:<width$}", &snap.tag, width = label_print_w));

        if let Some(branch) = listed.branch_display.as_deref() {
            line.push_str(&format!("{:<width$}", branch, width = branch_print_w));
        }

        let desc_trunc = if snap.description.len() > desc_w {
            format!("{}..", &snap.description[..desc_w - 2])
        } else {
            snap.description.clone()
        };
        line.push_str(&format!("{:<width$}", desc_trunc, width = desc_print_w));

        line.push_str(&format!(
            "{:<width$}",
            format_timestamp(&snap.timestamp),
            width = time_print_w
        ));

        if is_active {
            line.push_str(&format!("   {}", "(active)".green().bold()));
        }
        println!("{}", line);
    }
    if truncated {
        println!("  {}", "...".dimmed());
    }
    if show_branch_column {
        println!(
            "  {}",
            "Branch legend: branch, <current> (shared), shared, unattached".dimmed()
        );
    }

    println!();
    Ok(())
}

fn branch_mode(args: &ListArgs) -> BranchListMode {
    if let Some(branch) = args.branch.as_deref() {
        BranchListMode::Branch(branch.to_string())
    } else if args.all_branches {
        BranchListMode::AllBranches
    } else {
        BranchListMode::Default
    }
}

fn apply_branch_mode(
    snapshots: Vec<Snapshot>,
    mode: &BranchListMode,
) -> Result<Vec<ListedSnapshot>> {
    match mode {
        BranchListMode::Default => Ok(snapshots
            .into_iter()
            .map(|snapshot| ListedSnapshot {
                snapshot,
                branch_display: None,
            })
            .collect()),
        BranchListMode::Branch(branch) => filter_branch_snapshots(snapshots, branch),
        BranchListMode::AllBranches => annotate_all_branch_snapshots(snapshots),
    }
}

fn filter_branch_snapshots(snapshots: Vec<Snapshot>, branch: &str) -> Result<Vec<ListedSnapshot>> {
    if !git::branch_exists(branch)? {
        return Err(anyhow!(
            "Local branch '{}' does not exist. Run `snap branch list` to see available branches.",
            branch
        ));
    }

    let mut listed = Vec::new();
    for snapshot in snapshots {
        if git::is_commit_reachable_from_ref(&snapshot.full_id, branch)? {
            listed.push(ListedSnapshot {
                snapshot,
                branch_display: None,
            });
        }
    }
    Ok(listed)
}

fn annotate_all_branch_snapshots(snapshots: Vec<Snapshot>) -> Result<Vec<ListedSnapshot>> {
    let current_branch = git::current_branch().ok();
    let mut cache = HashMap::new();
    let mut listed = Vec::new();

    for snapshot in snapshots {
        let branches = branches_for_commit(&snapshot.full_id, &mut cache)?;
        let branch_display = Some(format_branch_display(branches, current_branch.as_deref()));
        listed.push(ListedSnapshot {
            snapshot,
            branch_display,
        });
    }

    Ok(listed)
}

fn branches_for_commit<'a>(
    commit: &str,
    cache: &'a mut HashMap<String, Vec<String>>,
) -> Result<&'a Vec<String>> {
    if !cache.contains_key(commit) {
        cache.insert(commit.to_string(), git::branches_containing_commit(commit)?);
    }
    Ok(cache.get(commit).expect("branch cache entry"))
}

fn format_branch_display(branches: &[String], current_branch: Option<&str>) -> String {
    if branches.is_empty() {
        return "unattached".to_string();
    }

    if branches.len() == 1 {
        return branches[0].clone();
    }

    if let Some(current) = current_branch {
        if branches.iter().any(|branch| branch == current) {
            return format!("{} (shared)", current);
        }
    }

    "shared".to_string()
}
