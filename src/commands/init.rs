use crate::utils::run_command;
use anyhow::Result;
use colored::*;
use std::path::Path;

pub fn execute(_args: crate::cli::InitArgs) -> Result<()> {
    if Path::new(".git").exists() {
        println!("{}", "[snap] This directory is already a Git repository.".yellow());
        return Ok(());
    }

    run_command("git init", None)?;
    let cwd = std::env::current_dir()?.to_string_lossy().to_string();
    println!(
        "{} in {}",
        "[snap] Initialized empty snap repository".green(),
        cwd
    );

    Ok(())
}