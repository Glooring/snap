use crate::cli::DiffArgs;
use crate::utils::{find_snapshot, get_snapshots, load_metadata_for_snapshot};
use anyhow::{anyhow, Result};
use colored::*;
use std::collections::{HashMap, HashSet};

pub fn execute(args: DiffArgs) -> Result<()> {
    let snapshots = get_snapshots()?;
    let snapshot_a = find_snapshot(&snapshots, &args.snapshot_a)
        .ok_or_else(|| anyhow!("Could not find snapshot '{}'", args.snapshot_a))?;
    let snapshot_b = find_snapshot(&snapshots, &args.snapshot_b)
        .ok_or_else(|| anyhow!("Could not find snapshot '{}'", args.snapshot_b))?;

    println!(
        "\n{} {} (\"{}\") ➜ {} (\"{}\"):\n",
        "[snap] Comparing snapshots".cyan(),
        snapshot_a.id.yellow(),
        snapshot_a.tag,
        snapshot_b.id.yellow(),
        snapshot_b.tag
    );

    let metadata_a = load_metadata_for_snapshot(snapshot_a)?;
    let metadata_b = load_metadata_for_snapshot(snapshot_b)?;
    
    let hidden_a: HashSet<_> = metadata_a.hidden_paths.into_iter().collect();
    let readonly_a: HashSet<_> = metadata_a.readonly_paths.into_iter().collect();
    let hidden_b: HashSet<_> = metadata_b.hidden_paths.into_iter().collect();
    let readonly_b: HashSet<_> = metadata_b.readonly_paths.into_iter().collect();
    let empty_a: HashSet<_> = metadata_a.empty_dirs.into_iter().collect();
    let empty_b: HashSet<_> = metadata_b.empty_dirs.into_iter().collect();
    
    let diff_output = crate::utils::run_command(
        &format!("git diff --name-status {} {}", snapshot_a.full_id, snapshot_b.full_id),
        None,
    )?;

    let mut changes = HashMap::new();
    let mut files_in_diff = HashSet::new();

    for line in diff_output.lines() {
        let parts: Vec<&str> = line.split('\t').collect();
        let status = parts[0].chars().next().unwrap_or('?');
        let file_path = parts.get(1).copied().unwrap_or("");
        
        let (prefix, color, label) = match status {
            'A' => ("+", "green", "added"),
            'D' => ("-", "red", "deleted"),
            'M' => ("~", "yellow", "modified"),
            'R' => (">>", "cyan", "renamed"),
            _ => ("?", "white", "unknown"),
        };

        println!("  {}", format!("{} {}", prefix, file_path).color(color));
        *changes.entry(label).or_insert(0) += 1;
        files_in_diff.insert(file_path.to_string());
    }

    // Combine all paths that have any attribute in either snapshot to iterate over.
    let mut all_attr_paths: HashSet<_> = hidden_a.union(&hidden_b).cloned().collect();
    all_attr_paths.extend(readonly_a.union(&readonly_b).cloned());

    for path in &all_attr_paths {
        // --- START: THE FIX ---
        // We pass `path` (which is a &String) directly, not `&path` (which is a &&String).
        if files_in_diff.contains(path) { continue; }
        // --- END: THE FIX ---

        let hidden_changed = hidden_a.contains(path) != hidden_b.contains(path);
        let readonly_changed = readonly_a.contains(path) != readonly_b.contains(path);

        if hidden_changed || readonly_changed {
            let mut attr_changes = Vec::new();
            if hidden_changed {
                attr_changes.push("visibility");
            }
            if readonly_changed {
                attr_changes.push("read-only");
            }
            println!("  {}", format!("! {} ({} changed)", path, attr_changes.join(", ")).magenta());
            *changes.entry("attribute change").or_insert(0) += 1;
        }
    }

    for dir in empty_b.difference(&empty_a) {
        println!("  {}", format!("+ {}/ (empty directory)", dir).green());
        *changes.entry("empty dir added").or_insert(0) += 1;
    }
    for dir in empty_a.difference(&empty_b) {
        println!("  {}", format!("- {}/ (empty directory)", dir).red());
        *changes.entry("empty dir removed").or_insert(0) += 1;
    }

    let summary_parts: Vec<String> = changes.iter()
        .filter(|(_, &count)| count > 0)
        .map(|(&label, &count)| format!("{} {}", count, label))
        .collect();

    if summary_parts.is_empty() {
        println!("\n{}", "[snap] The two snapshots are identical. No differences found.".green());
    } else {
        println!("\n{} {}", "[snap] Summary:".cyan(), summary_parts.join(", "));
    }

    Ok(())
}