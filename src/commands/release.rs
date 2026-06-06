use crate::cli::{ReleaseArgs, ReleaseCommands, ReleaseListArgs, ReleaseUploadArgs};
use crate::git;
use crate::github;
use anyhow::{anyhow, Context, Result};
use colored::*;
use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

const APP_NAME: &str = "snap";
const RELEASE_ROOT: &str = "release-github";
const WINDOWS_TARGET: &str = "windows-x86_64";
const LINUX_TARGET: &str = "linux-x86_64";
const DEFAULT_RELEASE_LIST_LIMIT: usize = 10;

#[derive(Debug, Clone, Copy)]
enum ReleasePlatform {
    Windows,
    Linux,
}

#[derive(Debug, Clone)]
struct ReleaseScript {
    platform: ReleasePlatform,
    runtime: String,
    script: PathBuf,
}

#[derive(Debug, Clone)]
struct ReleaseContext {
    release_version: String,
    release_dir: PathBuf,
    artifacts: Vec<PathBuf>,
}

pub fn execute(args: ReleaseArgs) -> Result<()> {
    match args.command {
        ReleaseCommands::Windows(_) => release_one(ReleasePlatform::Windows),
        ReleaseCommands::Linux(_) => release_one(ReleasePlatform::Linux),
        ReleaseCommands::All(_) => release_all(),
        ReleaseCommands::Upload(args) => upload(args),
        ReleaseCommands::List(args) => list(args),
    }
}

fn release_one(platform: ReleasePlatform) -> Result<()> {
    let script = release_script(platform);
    preflight(&script)?;
    run_script(&script)?;
    print_expected_artifacts(&[platform])
}

fn release_all() -> Result<()> {
    let scripts = [
        release_script(ReleasePlatform::Windows),
        release_script(ReleasePlatform::Linux),
    ];

    for script in &scripts {
        preflight(script).with_context(|| {
            "`snap release all` runs both local scripts only when both runtimes are available. Run `snap release windows` from Windows and `snap release linux` from WSL/Linux if your builds require separate environments."
        })?;
    }

    for script in &scripts {
        run_script(script)?;
    }

    print_expected_artifacts(&[ReleasePlatform::Windows, ReleasePlatform::Linux])
}

fn upload(args: ReleaseUploadArgs) -> Result<()> {
    let context = release_context(ReleasePlatform::all())?;
    ensure_artifacts_exist(&context)?;

    let repo = resolve_repo(args.repo.as_deref())?;
    let target = git::head_commit()?;

    github::ensure_authenticated()?;
    let release_exists = github::release_exists(&repo, &context.release_version)?;

    println!("\n{}", "[snap] GitHub release upload".cyan().bold());
    println!("  Repository: {}", repo);
    println!("  Release: {}", context.release_version);
    println!("  Folder: {}", context.release_dir.display());

    if release_exists && !args.clobber {
        return Err(anyhow!(
            "GitHub Release '{}' already exists for {}. Use `snap release upload --clobber` to upload and overwrite matching assets intentionally.",
            context.release_version,
            repo
        ));
    }

    if release_exists {
        println!(
            "{}",
            "[snap] Existing release found. Uploading assets with --clobber...".cyan()
        );
        github::upload_release_assets(&repo, &context.release_version, &context.artifacts, true)?;
    } else {
        let title = format!("snap {}", context.release_version);
        let notes = format!("snap {} release", context.release_version);
        println!(
            "{}",
            format!(
                "[snap] Creating {} GitHub Release...",
                if args.publish { "published" } else { "draft" }
            )
            .cyan()
        );
        github::create_release(
            &repo,
            &context.release_version,
            &title,
            &notes,
            &target,
            !args.publish,
            &context.artifacts,
        )?;
    }

    println!(
        "{}",
        format!(
            "[snap] Uploaded {} release asset(s) for {}.",
            context.artifacts.len(),
            context.release_version
        )
        .green()
    );
    println!();
    Ok(())
}

fn list(args: ReleaseListArgs) -> Result<()> {
    let limit = parse_release_list_limit(args.limit)?;
    let repo = resolve_repo(args.repo.as_deref())?;

    github::ensure_authenticated()?;
    let releases = github::list_releases(&repo, limit)?;

    println!("\n{}", "[snap] GitHub releases".cyan().bold());
    println!("  Repository: {}", repo);
    println!("  Limit: {}", limit);

    if releases.is_empty() {
        println!("  {}", "No GitHub releases found.".yellow());
        println!();
        return Ok(());
    }

    println!();
    print_release_table(&releases);
    println!();
    Ok(())
}

fn release_script(platform: ReleasePlatform) -> ReleaseScript {
    match platform {
        ReleasePlatform::Windows => ReleaseScript {
            platform,
            runtime: env::var("SNAP_RELEASE_POWERSHELL").unwrap_or_else(|_| "powershell".into()),
            script: env::var_os("SNAP_RELEASE_WINDOWS_SCRIPT")
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from("scripts").join("release-windows.ps1")),
        },
        ReleasePlatform::Linux => ReleaseScript {
            platform,
            runtime: env::var("SNAP_RELEASE_BASH").unwrap_or_else(|_| "bash".into()),
            script: env::var_os("SNAP_RELEASE_LINUX_SCRIPT")
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from("scripts").join("release-linux.sh")),
        },
    }
}

fn preflight(script: &ReleaseScript) -> Result<()> {
    if !script.script.exists() {
        return Err(anyhow!(
            "Release script was not found: {}",
            script.script.display()
        ));
    }

    let status = match script.platform {
        ReleasePlatform::Windows => Command::new(&script.runtime)
            .args(["-NoProfile", "-Command", "$PSVersionTable.PSVersion"])
            .status(),
        ReleasePlatform::Linux => Command::new(&script.runtime).arg("--version").status(),
    };

    match status {
        Ok(_) => Ok(()),
        Err(error) => Err(anyhow!(
            "Required release runtime '{}' was not found or could not be started: {}",
            script.runtime,
            error
        )),
    }
}

fn run_script(script: &ReleaseScript) -> Result<()> {
    println!(
        "\n{}",
        format!(
            "[snap] Running {} release script...",
            script.platform.label()
        )
        .cyan()
        .bold()
    );

    let status = match script.platform {
        ReleasePlatform::Windows => Command::new(&script.runtime)
            .args(["-ExecutionPolicy", "Bypass", "-File"])
            .arg(&script.script)
            .status()
            .with_context(|| {
                format!(
                    "Failed to spawn Windows release runtime '{}'.",
                    script.runtime
                )
            })?,
        ReleasePlatform::Linux => Command::new(&script.runtime)
            .arg(&script.script)
            .status()
            .with_context(|| {
                format!(
                    "Failed to spawn Linux release runtime '{}'.",
                    script.runtime
                )
            })?,
    };

    if !status.success() {
        return Err(anyhow!(
            "{} release script failed with status {}.",
            script.platform.label(),
            status
        ));
    }

    Ok(())
}

fn print_expected_artifacts(platforms: &[ReleasePlatform]) -> Result<()> {
    let context = release_context(platforms)?;

    println!(
        "\n{}",
        "[snap] Expected GitHub release assets:".green().bold()
    );
    println!("  Folder: {}", context.release_dir.display());

    for artifact in context.artifacts {
        println!("  {}", artifact.display());
    }

    println!();
    Ok(())
}

fn release_context(platforms: &[ReleasePlatform]) -> Result<ReleaseContext> {
    let version = cargo_version()?;
    let release_version = format!("v{}", version);
    let release_dir = Path::new(RELEASE_ROOT).join(&release_version);
    let artifacts = platforms
        .iter()
        .flat_map(|platform| platform.artifacts(&release_version))
        .map(|artifact| release_dir.join(artifact))
        .collect();

    Ok(ReleaseContext {
        release_version,
        release_dir,
        artifacts,
    })
}

fn ensure_artifacts_exist(context: &ReleaseContext) -> Result<()> {
    let missing: Vec<_> = context
        .artifacts
        .iter()
        .filter(|path| !path.is_file())
        .collect();

    if missing.is_empty() {
        return Ok(());
    }

    let mut message = format!(
        "Missing release artifact(s) in {}:",
        context.release_dir.display()
    );
    for path in missing {
        message.push_str(&format!("\n  {}", path.display()));
    }
    message.push_str("\nRun `snap release windows` and `snap release linux` before uploading.");

    Err(anyhow!(message))
}

fn resolve_repo(repo: Option<&str>) -> Result<String> {
    if let Some(repo) = repo {
        validate_owner_repo(repo)?;
        return Ok(repo.to_string());
    }

    let Some(origin_url) = git::remote_url(git::ORIGIN)? else {
        return Err(anyhow!(
            "Remote origin is not configured. Use `snap release upload --repo owner/repo` or `snap release list --repo owner/repo`."
        ));
    };

    git::parse_github_repo_from_url(&origin_url).ok_or_else(|| {
        anyhow!(
            "Could not determine a GitHub repository from origin '{}'. Use `snap release upload --repo owner/repo` or `snap release list --repo owner/repo`.",
            origin_url
        )
    })
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

fn parse_release_list_limit(value: Option<String>) -> Result<usize> {
    let Some(value) = value else {
        return Ok(DEFAULT_RELEASE_LIST_LIMIT);
    };

    let parsed = value.parse::<usize>().map_err(|_| {
        anyhow!(
            "Release list limit must be a positive number, got '{}'.",
            value
        )
    })?;
    if parsed == 0 {
        return Err(anyhow!(
            "Release list limit must be a positive number, got '{}'.",
            value
        ));
    }

    Ok(parsed)
}

fn print_release_table(releases: &[github::ReleaseInfo]) {
    let tag_w = releases
        .iter()
        .map(|release| release.tag_name.len())
        .max()
        .unwrap_or(0)
        .max("Tag".len());
    let name_w = releases
        .iter()
        .map(|release| release.name.as_deref().unwrap_or("-").len())
        .max()
        .unwrap_or(0)
        .max("Name".len());
    let state_w = releases
        .iter()
        .map(|release| release_state(release).len())
        .max()
        .unwrap_or(0)
        .max("State".len());
    let published_w = releases
        .iter()
        .map(|release| release.published_at.as_deref().unwrap_or("-").len())
        .max()
        .unwrap_or(0)
        .max("Published".len());

    println!(
        "  {:<tag_w$}  {:<name_w$}  {:<state_w$}  {:<published_w$}  Latest",
        "Tag", "Name", "State", "Published"
    );
    println!(
        "  {:<tag_w$}  {:<name_w$}  {:<state_w$}  {:<published_w$}  ------",
        "-".repeat(tag_w),
        "-".repeat(name_w),
        "-".repeat(state_w),
        "-".repeat(published_w)
    );

    for release in releases {
        println!(
            "  {:<tag_w$}  {:<name_w$}  {:<state_w$}  {:<published_w$}  {}",
            release.tag_name,
            release.name.as_deref().unwrap_or("-"),
            release_state(release),
            release.published_at.as_deref().unwrap_or("-"),
            if release.is_latest { "yes" } else { "-" }
        );
    }
}

fn release_state(release: &github::ReleaseInfo) -> &'static str {
    if release.is_draft {
        "draft"
    } else if release.is_prerelease {
        "prerelease"
    } else {
        "published"
    }
}

fn cargo_version() -> Result<String> {
    let output = Command::new("cargo")
        .arg("pkgid")
        .output()
        .context("Failed to run `cargo pkgid` while reading release version.")?;

    if !output.status.success() {
        return Err(anyhow!(
            "Failed to read package version with `cargo pkgid`: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }

    let pkgid = String::from_utf8(output.stdout)?.trim().to_string();
    let id_part = pkgid
        .rsplit('#')
        .next()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| anyhow!("Failed to parse package version from `cargo pkgid`."))?;
    let version = id_part.rsplit('@').next().unwrap_or(id_part).trim();
    if version.is_empty() {
        return Err(anyhow!(
            "Failed to parse package version from `cargo pkgid`."
        ));
    }
    Ok(version.to_string())
}

impl ReleasePlatform {
    fn all() -> &'static [ReleasePlatform] {
        &[ReleasePlatform::Windows, ReleasePlatform::Linux]
    }

    fn label(self) -> &'static str {
        match self {
            ReleasePlatform::Windows => "Windows",
            ReleasePlatform::Linux => "Linux",
        }
    }

    fn artifacts(self, release_version: &str) -> Vec<String> {
        match self {
            ReleasePlatform::Windows => vec![
                format!("{APP_NAME}-{release_version}-{WINDOWS_TARGET}.exe"),
                format!("{APP_NAME}-{release_version}-{WINDOWS_TARGET}-setup.exe"),
                format!("{APP_NAME}-{release_version}-{WINDOWS_TARGET}.msi"),
            ],
            ReleasePlatform::Linux => vec![
                format!("{APP_NAME}-{release_version}-{LINUX_TARGET}"),
                format!("{APP_NAME}-{release_version}-{LINUX_TARGET}.tar.gz"),
            ],
        }
    }
}
