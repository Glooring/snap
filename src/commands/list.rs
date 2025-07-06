use crate::cli::ListArgs;
use crate::config::{load_config, SortOrder};
use crate::utils::{format_timestamp, get_active_commit_full, get_snapshots};
use anyhow::Result;
use colored::*;
use std::cmp::max;
use std::env;

pub fn execute(_args: ListArgs) -> Result<()> {
    let config = load_config()?;
    let mut snapshots = get_snapshots()?;
    let active_commit = get_active_commit_full()?.unwrap_or_default();

    if config.options.order_by == SortOrder::Label {
        snapshots.sort_by(|a, b| b.tag.cmp(&a.tag));
    }

    let cwd_path = env::current_dir()?;
    let project_name = cwd_path
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("current project");

    println!("\n{} \"{}\":", "[snap] Snapshots for".cyan(), project_name);

    if snapshots.is_empty() {
        println!("\n  {}", "No snapshots found. Use \"snap new <label>\" to create one.".yellow());
        return Ok(());
    }

    // --- START: DYNAMIC COLUMN WIDTH CALCULATION ---

    const HEADER_ID: &str = "ID";
    const HEADER_LABEL: &str = "Label";
    const HEADER_DESC: &str = "Description";
    const HEADER_TIME: &str = "Timestamp";
    const MAX_DESC_WIDTH: usize = 50;
    const COL_PADDING: usize = 2;
    const FORMATTED_TIME_LEN: usize = 16; 
    const SHORT_ID_LEN: usize = 7; // The length of the short commit hash

    let show_ids = config.options.show_ids;

    // 1. Find the maximum length of the data in each column.
    let max_label_len = snapshots.iter().map(|s| s.tag.len()).max().unwrap_or(0);
    let max_desc_len = snapshots.iter().map(|s| s.description.len()).max().unwrap_or(0);

    // 2. Determine the actual column width (the data width without padding).
    // --- FIX: Apply dynamic sizing to the ID column as well. ---
    let id_w = if show_ids { max(HEADER_ID.len(), SHORT_ID_LEN) } else { 0 };
    let label_w = max(HEADER_LABEL.len(), max_label_len);
    let desc_w = max(HEADER_DESC.len(), max_desc_len).min(MAX_DESC_WIDTH);
    let time_w = max(HEADER_TIME.len(), FORMATTED_TIME_LEN);

    // 3. Determine the printing width, which includes padding.
    let id_print_w = if show_ids { id_w + COL_PADDING } else { 0 };
    let label_print_w = label_w + COL_PADDING;
    let desc_print_w = desc_w + COL_PADDING;
    let time_print_w = time_w;

    // --- END: DYNAMIC COLUMN WIDTH CALCULATION ---


    // Build the header string using the new dynamic widths.
    let mut header = "  ".to_string();
    let mut separator = "  ".to_string();

    if show_ids {
        header.push_str(&format!("{:<width$}", HEADER_ID, width = id_print_w));
        separator.push_str(&format!("{:<width$}", "-".repeat(id_w), width = id_print_w));
    }
    header.push_str(&format!("{:<width$}", HEADER_LABEL, width = label_print_w));
    separator.push_str(&format!("{:<width$}", "-".repeat(label_w), width = label_print_w));
    
    header.push_str(&format!("{:<width$}", HEADER_DESC, width = desc_print_w));
    separator.push_str(&format!("{:<width$}", "-".repeat(desc_w), width = desc_print_w));

    header.push_str(HEADER_TIME);
    separator.push_str(&"-".repeat(time_w));
    
    println!("\n{}", header.bold());
    println!("{}", separator.bold());

    for snap in snapshots {
        let is_active = !active_commit.is_empty() && snap.full_id == active_commit;
        let mut line = "  ".to_string();

        if show_ids {
            line.push_str(&format!("{:<width$}", snap.id, width = id_print_w));
        }
        line.push_str(&format!("{:<width$}", &snap.tag, width = label_print_w));
        
        let desc_trunc = if snap.description.len() > desc_w {
            format!("{}..", &snap.description[..desc_w - 2])
        } else {
            snap.description.clone()
        };
        line.push_str(&format!("{:<width$}", desc_trunc, width = desc_print_w));

        line.push_str(&format!("{:<width$}", format_timestamp(&snap.timestamp), width = time_print_w));
        
        if is_active {
            line.push_str(&format!("   {}", "(active)".green().bold()));
        }
        println!("{}", line);
    }
    
    Ok(())
}