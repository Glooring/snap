mod cli;
mod commands;
mod config;
mod os;
mod utils;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};
use colored::Colorize;

fn main() -> Result<()> {
    std::panic::set_hook(Box::new(|panic_info| {
        eprintln!("\n{}", "[snap] A critical error occurred and the program had to stop.".red().bold());
        if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
            eprintln!("Error details: {}", s);
        } else {
            eprintln!("An unknown error occurred.");
        }
        if let Some(location) = panic_info.location() {
            eprintln!("Occurred in file '{}' at line {}", location.file(), location.line());
        }
        eprintln!("{}", "Please report this issue if it persists.".yellow());
    }));
    
    let cli = Cli::parse();

    // The init and options commands can run anywhere. Other commands require a repo.
    if !matches!(cli.command, Commands::Init(_) | Commands::Options(_)) {
        if let Err(e) = config::ensure_repo_exists() {
             eprintln!("\n{} {}", "[snap] Error:".red().bold(), e);
             std::process::exit(1);
        }
    }

    let result = match cli.command {
        Commands::Init(args) => commands::init::execute(args),
        Commands::New(args) => commands::new::execute(args),
        Commands::List(args) => commands::list::execute(args),
        Commands::Restore(args) => commands::restore::execute(args),
        Commands::Delete(args) => commands::delete::execute(args),
        Commands::Edit(args) => commands::edit::execute(args),
        Commands::Update(args) => commands::update::execute(args),
        Commands::Diff(args) => commands::diff::execute(args),
        Commands::Options(args) => commands::options::execute(args),
    };

    if let Err(e) = result {
        eprintln!("\n{} {}", "[snap] Error:".red().bold(), e);
        let mut source = e.source();
        while let Some(s) = source {
            eprintln!("  {} {}", "Caused by:".dimmed(), s);
            source = s.source();
        }
        std::process::exit(1);
    }
    
    Ok(())
}