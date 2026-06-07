use crate::cli::ExamplesArgs;
use anyhow::Result;
use colored::*;

pub fn execute(_args: ExamplesArgs) -> Result<()> {
    println!("\n{}", "Snap examples".cyan().bold());
    println!();
    println!("{}", "Project already has a GitHub remote:".bold());
    println!("  snap status");
    println!("  snap history");
    println!("  snap update-repo \"implemented login screen\"");
    println!();
    println!(
        "{}",
        "Create and connect a private GitHub repository:".bold()
    );
    println!("  snap init");
    println!("  gh auth login");
    println!("  snap setup-repo owner/repo --private");
    println!("  snap update-repo \"initial project state\"");
    println!();
    println!(
        "{}",
        "Use the structured commands instead of aliases:".bold()
    );
    println!("  snap save \"fixed restore flow\"");
    println!("  snap sync");
    println!("  snap remote status");
    println!();
    println!(
        "{}",
        "Create an important snapshot before risky work:".bold()
    );
    println!("  snap new v42 \"before branch migration\"");
    println!("  snap list");
    println!("  snap list --branch main");
    println!("  snap list --all-branches");
    println!("  snap restore v42");
    println!();
    println!("{}", "Understand snapshots across branches:".bold());
    println!("  snap list --all-branches");
    println!("  snap list --branch master");
    println!("  snap history --all-branches");
    println!();
    println!("{}", "Work on a local branch:".bold());
    println!("  snap branch new feature-login");
    println!("  snap save \"start login screen\"");
    println!("  snap sync");
    println!("  snap branch switch main");
    println!("  snap branch merge feature-login");
    println!();
    println!("{}", "Prepare release assets:".bold());
    println!("  snap release windows");
    println!("  snap release linux");
    println!("  snap release macos");
    println!("  snap release upload");
    println!("  snap release list");
    println!("  # Assets are written to release-github/vX.Y.Z");
    println!();
    println!("{}", "Delete a disposable GitHub repository:".bold());
    println!("  snap delete-repo owner/disposable-test-repo");
    println!("  # You must type owner/disposable-test-repo exactly.");
    println!();
    println!("{}", "More help:".bold());
    println!("  snap --help");
    println!("  snap remote --help");
    println!();
    Ok(())
}
