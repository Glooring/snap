use crate::cli::HistoryArgs;
use crate::git;
use anyhow::{anyhow, Result};
use colored::*;

const DEFAULT_HISTORY_LIMIT: usize = 20;

enum HistoryLimit {
    Limited(usize),
    All,
}

enum HistoryScope {
    Current,
    Branch(String),
    AllBranches,
}

pub fn execute(args: HistoryArgs) -> Result<()> {
    let limit = parse_limit(args.limit.as_deref())?;
    let scope = history_scope(&args)?;

    println!("\n{}", "[snap] Project history".cyan().bold());
    match &scope {
        HistoryScope::Branch(branch) => println!("{} {}", "[snap] Branch:".cyan(), branch),
        HistoryScope::AllBranches => println!("{}", "[snap] Scope: all branches".cyan()),
        HistoryScope::Current => {}
    }

    if !git::has_head_commit()? {
        println!("  {}", "No commits found yet.".yellow());
        println!();
        return Ok(());
    }

    let output = match (&scope, limit) {
        (HistoryScope::Current, HistoryLimit::Limited(limit)) => git::log_history(Some(limit))?,
        (HistoryScope::Current, HistoryLimit::All) => git::log_history(None)?,
        (HistoryScope::Branch(branch), HistoryLimit::Limited(limit)) => {
            git::log_history_for_branch(Some(limit), branch)?
        }
        (HistoryScope::Branch(branch), HistoryLimit::All) => {
            git::log_history_for_branch(None, branch)?
        }
        (HistoryScope::AllBranches, HistoryLimit::Limited(limit)) => {
            git::log_history_all(Some(limit))?
        }
        (HistoryScope::AllBranches, HistoryLimit::All) => git::log_history_all(None)?,
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

fn parse_limit(value: Option<&str>) -> Result<HistoryLimit> {
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

fn history_scope(args: &HistoryArgs) -> Result<HistoryScope> {
    if let Some(branch) = args.branch.as_deref() {
        if !git::branch_exists(branch)? {
            return Err(anyhow!(
                "Local branch '{}' does not exist. Run `snap branch list` to see available branches.",
                branch
            ));
        }
        Ok(HistoryScope::Branch(branch.to_string()))
    } else if args.all_branches {
        Ok(HistoryScope::AllBranches)
    } else {
        Ok(HistoryScope::Current)
    }
}
