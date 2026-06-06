use crate::cli::HistoryArgs;
use crate::git;
use anyhow::{anyhow, Result};
use colored::*;

const DEFAULT_HISTORY_LIMIT: usize = 20;

enum HistoryLimit {
    Limited(usize),
    All,
}

pub fn execute(args: HistoryArgs) -> Result<()> {
    let limit = parse_limit(args.limit)?;

    println!("\n{}", "[snap] Project history".cyan().bold());

    if !git::has_head_commit()? {
        println!("  {}", "No commits found yet.".yellow());
        println!();
        return Ok(());
    }

    let output = match limit {
        HistoryLimit::Limited(limit) => git::log_history(Some(limit))?,
        HistoryLimit::All => git::log_history(None)?,
    };

    if output.trim().is_empty() {
        println!("  {}", "No commits found yet.".yellow());
    } else {
        println!();
        print!("{}", output);
        if !output.ends_with('\n') {
            println!();
        }
    }

    println!();
    Ok(())
}

fn parse_limit(value: Option<String>) -> Result<HistoryLimit> {
    let Some(value) = value else {
        return Ok(HistoryLimit::Limited(DEFAULT_HISTORY_LIMIT));
    };

    if value.eq_ignore_ascii_case("all") {
        return Ok(HistoryLimit::All);
    }

    let parsed = value.parse::<usize>().map_err(|_| {
        anyhow!(
            "History limit must be a positive number or `all`, got '{}'.",
            value
        )
    })?;
    if parsed == 0 {
        return Err(anyhow!(
            "History limit must be a positive number or `all`, got '{}'.",
            value
        ));
    }

    Ok(HistoryLimit::Limited(parsed))
}
