use crate::cli::{
    DeleteRepoArgs, RemoteArgs, RemoteCommands, RemoteCreateArgs, RemoteDeleteArgs,
    RemoteSetUrlArgs, RemoteStatusArgs, RemoteVisibilityArgs, RemoteVisibilityCommands,
    VisibilityRepoArgs,
};
use crate::commands::push;
use crate::git;
use crate::github;
use crate::utils::ask_yes_no;
use anyhow::{anyhow, Result};
use colored::*;
use std::io::{self, Write};

#[derive(Debug, Clone, Copy)]
pub enum GitHubVisibility {
    Public,
    Private,
}

impl GitHubVisibility {
    fn as_str(self) -> &'static str {
        match self {
            GitHubVisibility::Public => "public",
            GitHubVisibility::Private => "private",
        }
    }
}

pub fn execute(args: RemoteArgs) -> Result<()> {
    match args.command {
        RemoteCommands::Status(args) => status(args),
        RemoteCommands::Create(args) => create_remote_repo(args),
        RemoteCommands::Visibility(args) => visibility(args),
        RemoteCommands::Delete(args) => delete_remote_repo(args),
        RemoteCommands::SetUrl(args) => set_url(args),
    }
}

fn status(_args: RemoteStatusArgs) -> Result<()> {
    println!("\n{}", "[snap] Remote status".cyan().bold());

    let remotes = git::list_remotes()?;
    if remotes.is_empty() {
        println!("  {} No Git remotes configured.", "WARN".yellow().bold());
    } else {
        for (name, url) in &remotes {
            println!("  {} {}: {}", "OK".green().bold(), name, url);
        }
    }

    match github::auth_status_text() {
        Ok(status) => println!("  {} GitHub auth: {}", "OK".green().bold(), status),
        Err(error) => println!("  {} GitHub auth: {}", "WARN".yellow().bold(), error),
    }

    if let Some(origin_url) = git::remote_url(git::ORIGIN)? {
        if let Some(repo) = git::parse_github_repo_from_url(&origin_url) {
            if let Ok(Some(info)) = github::repo_view(&repo) {
                println!(
                    "  {} GitHub repo: {}",
                    "OK".green().bold(),
                    info.name_with_owner
                );
                if let Some(visibility) = info.visibility {
                    println!("  {} Visibility: {}", "OK".green().bold(), visibility);
                }
                if let Some(url) = info.url {
                    println!("  {} URL: {}", "OK".green().bold(), url);
                }
            }
        }
    }

    println!();
    Ok(())
}

pub fn create_remote_repo(args: RemoteCreateArgs) -> Result<()> {
    validate_owner_repo(&args.repo)?;
    if git::remote_url(git::ORIGIN)?.is_some() {
        return Err(anyhow!(
            "Remote origin is already configured. Use `snap remote set-url <url>` in a repository without origin, or change it manually with Git."
        ));
    }

    github::ensure_authenticated()?;

    let private = !args.public;
    println!(
        "{}",
        format!(
            "[snap] Creating GitHub repository {} as {}...",
            args.repo,
            if private { "private" } else { "public" }
        )
        .cyan()
    );
    github::create_repo(&args.repo, private)?;

    if git::remote_url(git::ORIGIN)?.is_none() {
        let fallback_url = format!("https://github.com/{}.git", args.repo);
        git::add_remote(git::ORIGIN, &fallback_url)?;
    }

    if git::has_head_commit()? {
        println!("{}", "[snap] Running initial snap-aware push...".cyan());
        let summary = push::push_snap_aware()?;
        println!(
            "{}",
            format!(
                "[snap] Remote repository created and branch '{}' pushed.",
                summary.branch
            )
            .green()
        );
    } else {
        println!(
            "{}",
            "[snap] Remote repository created. No commits exist yet, so initial push was skipped."
                .yellow()
        );
    }

    println!();
    Ok(())
}

fn visibility(args: RemoteVisibilityArgs) -> Result<()> {
    match args.command {
        RemoteVisibilityCommands::Public(args) => {
            change_visibility(&args.repo, GitHubVisibility::Public)
        }
        RemoteVisibilityCommands::Private(args) => {
            change_visibility(&args.repo, GitHubVisibility::Private)
        }
    }
}

pub fn make_public(args: VisibilityRepoArgs) -> Result<()> {
    change_visibility(&args.repo, GitHubVisibility::Public)
}

pub fn make_private(args: VisibilityRepoArgs) -> Result<()> {
    change_visibility(&args.repo, GitHubVisibility::Private)
}

pub fn delete_repo_alias(args: DeleteRepoArgs) -> Result<()> {
    println!(
        "{}",
        "[snap] delete-repo is a shortcut for `snap remote delete`."
            .cyan()
            .bold()
    );

    delete_github_repo(&args.repo)
}

pub fn delete_remote_repo(args: RemoteDeleteArgs) -> Result<()> {
    delete_github_repo(&args.repo)
}

pub fn change_visibility(repo: &str, visibility: GitHubVisibility) -> Result<()> {
    validate_owner_repo(repo)?;

    if !confirm_visibility_change(repo, visibility)? {
        println!("{}", "[snap] Visibility change cancelled.".yellow());
        println!();
        return Ok(());
    }

    github::ensure_authenticated()?;

    println!(
        "{}",
        format!(
            "[snap] Changing GitHub repository {} visibility to {}...",
            repo,
            visibility.as_str()
        )
        .cyan()
    );
    github::edit_repo_visibility(repo, visibility.as_str())?;

    println!(
        "{}",
        format!(
            "[snap] GitHub repository {} is now {}.",
            repo,
            visibility.as_str()
        )
        .green()
    );
    println!();
    Ok(())
}

pub fn delete_github_repo(repo: &str) -> Result<()> {
    validate_owner_repo(repo)?;

    if !confirm_repo_delete(repo)? {
        println!("{}", "[snap] Repository deletion cancelled.".yellow());
        println!();
        return Ok(());
    }

    github::ensure_authenticated()?;

    println!(
        "{}",
        format!("[snap] Deleting GitHub repository {}...", repo).red()
    );
    github::delete_repo(repo)?;

    println!(
        "{}",
        format!(
            "[snap] GitHub repository {} was deleted. Local files were not changed.",
            repo
        )
        .green()
    );
    println!();
    Ok(())
}

fn confirm_visibility_change(repo: &str, visibility: GitHubVisibility) -> Result<bool> {
    match visibility {
        GitHubVisibility::Public => confirm_public_visibility(repo),
        GitHubVisibility::Private => confirm_private_visibility(repo),
    }
}

fn confirm_public_visibility(repo: &str) -> Result<bool> {
    println!("\n{}", "[snap] Public visibility change".red().bold());
    println!("  Repository: {}", repo.bold());
    println!("  This changes the GitHub repository, not your local files.");
    println!(
        "{}",
        "  Warning: the repository and its history may become visible to anyone.".yellow()
    );
    print!("Type the repository name to confirm: ");
    io::stdout().flush()?;

    let mut confirmation = String::new();
    io::stdin().read_line(&mut confirmation)?;
    Ok(confirmation.trim() == repo)
}

fn confirm_private_visibility(repo: &str) -> Result<bool> {
    println!("\n{}", "[snap] Private visibility change".cyan().bold());
    println!("  Repository: {}", repo.bold());
    println!("  This changes the GitHub repository, not your local files.");
    ask_yes_no(
        "[snap] Continue changing this GitHub repository to private?",
        false,
    )
}

fn confirm_repo_delete(repo: &str) -> Result<bool> {
    println!("\n{}", "[snap] Delete GitHub repository".red().bold());
    println!("  Repository: {}", repo.bold());
    println!("  This deletes the GitHub repository, not your local files.");
    println!(
        "{}",
        "  Warning: this can remove remote branches, tags, releases, issues, and settings."
            .yellow()
    );
    println!("{}", "  This action cannot be undone on GitHub.".yellow());
    print!("Type the repository name to confirm deletion: ");
    io::stdout().flush()?;

    let mut confirmation = String::new();
    io::stdin().read_line(&mut confirmation)?;
    Ok(confirmation.trim() == repo)
}

fn set_url(args: RemoteSetUrlArgs) -> Result<()> {
    if git::remote_url(git::ORIGIN)?.is_some() {
        return Err(anyhow!(
            "Remote origin is already configured. V1 refuses to replace it automatically; use Git manually if you intentionally want to change it."
        ));
    }

    git::add_remote(git::ORIGIN, &args.url)?;
    println!(
        "\n{} {}",
        "[snap] Remote origin added:".green(),
        args.url.bold()
    );
    println!();
    Ok(())
}

fn validate_owner_repo(repo: &str) -> Result<()> {
    let mut parts = repo.split('/');
    let owner = parts.next().unwrap_or("");
    let name = parts.next().unwrap_or("");
    if owner.is_empty() || name.is_empty() || parts.next().is_some() {
        return Err(anyhow!(
            "Repository must be in owner/repo format, for example `glooring/my-project`."
        ));
    }
    Ok(())
}
