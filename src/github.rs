use anyhow::{anyhow, Context, Result};
use serde_json::Value;
use std::env;
use std::path::PathBuf;
use std::process::{Command, Stdio};

#[derive(Debug, Clone)]
pub struct GhResult {
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
}

#[derive(Debug, Clone)]
pub struct RepoInfo {
    pub name_with_owner: String,
    pub visibility: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ReleaseInfo {
    pub tag_name: String,
    pub name: Option<String>,
    pub is_draft: bool,
    pub is_prerelease: bool,
    pub is_latest: bool,
    pub published_at: Option<String>,
}

pub fn run_gh(args: &[&str]) -> Result<GhResult> {
    let command = env::var("SNAP_GH").unwrap_or_else(|_| "gh".to_string());
    let output = Command::new(&command)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .with_context(|| {
            "GitHub CLI (`gh`) is required for this command. Install it from https://cli.github.com/ and run `gh auth login`."
        })?;

    Ok(GhResult {
        success: output.status.success(),
        stdout: String::from_utf8(output.stdout)?,
        stderr: String::from_utf8_lossy(&output.stderr).to_string(),
    })
}

pub fn run_gh_success(args: &[&str]) -> Result<String> {
    let result = run_gh(args)?;
    if !result.success {
        return Err(anyhow!(
            "Command failed: 'gh {}'\n---\n{}",
            args.join(" "),
            result.stderr.trim()
        ));
    }
    Ok(result.stdout)
}

pub fn run_gh_success_owned(args: Vec<String>) -> Result<String> {
    let refs: Vec<&str> = args.iter().map(String::as_str).collect();
    run_gh_success(&refs)
}

pub fn ensure_authenticated() -> Result<()> {
    let result = run_gh(&["auth", "status"])?;
    if !result.success {
        return Err(anyhow!(
            "GitHub CLI is installed, but you are not authenticated.\nRun `gh auth login`, then retry this snap command."
        ));
    }
    Ok(())
}

pub fn auth_status_text() -> Result<String> {
    let result = run_gh(&["auth", "status"])?;
    if result.success {
        let text = if result.stdout.trim().is_empty() {
            result.stderr.trim()
        } else {
            result.stdout.trim()
        };
        Ok(if text.is_empty() {
            "authenticated".to_string()
        } else {
            text.lines().next().unwrap_or("authenticated").to_string()
        })
    } else {
        Ok("not authenticated".to_string())
    }
}

pub fn create_repo(repo: &str, private: bool) -> Result<()> {
    let visibility = if private { "--private" } else { "--public" };
    run_gh_success(&[
        "repo",
        "create",
        repo,
        visibility,
        "--source=.",
        "--remote=origin",
    ])?;
    Ok(())
}

pub fn edit_repo_visibility(repo: &str, visibility: &str) -> Result<()> {
    run_gh_success(&[
        "repo",
        "edit",
        repo,
        "--visibility",
        visibility,
        "--accept-visibility-change-consequences",
    ])?;
    Ok(())
}

pub fn delete_repo(repo: &str) -> Result<()> {
    run_gh_success(&["repo", "delete", repo, "--yes"]).with_context(|| {
        format!(
            "Failed to delete GitHub repository '{}'. GitHub CLI may require `gh auth refresh -s delete_repo`.",
            repo
        )
    })?;
    Ok(())
}

pub fn release_exists(repo: &str, tag: &str) -> Result<bool> {
    let output = run_gh(&["release", "view", tag, "-R", repo])?;
    Ok(output.success)
}

pub fn create_release(
    repo: &str,
    tag: &str,
    title: &str,
    notes: &str,
    target: &str,
    draft: bool,
    assets: &[PathBuf],
) -> Result<()> {
    let mut args = vec!["release".to_string(), "create".to_string(), tag.to_string()];
    args.extend(assets.iter().map(|path| path.to_string_lossy().to_string()));
    if draft {
        args.push("--draft".to_string());
    }
    args.extend([
        "--title".to_string(),
        title.to_string(),
        "--notes".to_string(),
        notes.to_string(),
        "-R".to_string(),
        repo.to_string(),
        "--target".to_string(),
        target.to_string(),
    ]);

    run_gh_success_owned(args).with_context(|| {
        format!(
            "Failed to create GitHub Release '{}'. Run `snap sync` first if the target commit has not been pushed.",
            tag
        )
    })?;
    Ok(())
}

pub fn upload_release_assets(
    repo: &str,
    tag: &str,
    assets: &[PathBuf],
    clobber: bool,
) -> Result<()> {
    let mut args = vec!["release".to_string(), "upload".to_string(), tag.to_string()];
    args.extend(assets.iter().map(|path| path.to_string_lossy().to_string()));
    if clobber {
        args.push("--clobber".to_string());
    }
    args.extend(["-R".to_string(), repo.to_string()]);

    run_gh_success_owned(args)?;
    Ok(())
}

pub fn list_releases(repo: &str, limit: usize) -> Result<Vec<ReleaseInfo>> {
    let output = run_gh_success(&[
        "release",
        "list",
        "--limit",
        &limit.to_string(),
        "--json",
        "tagName,name,isDraft,isPrerelease,isLatest,publishedAt",
        "-R",
        repo,
    ])?;

    let value: Value = serde_json::from_str(&output)
        .with_context(|| "GitHub CLI returned invalid JSON for release list.")?;
    let Some(items) = value.as_array() else {
        return Err(anyhow!(
            "GitHub CLI returned an unexpected JSON shape for release list."
        ));
    };

    Ok(items
        .iter()
        .filter_map(parse_release_info)
        .collect::<Vec<_>>())
}

fn parse_release_info(value: &Value) -> Option<ReleaseInfo> {
    let tag_name = value.get("tagName")?.as_str()?.to_string();
    let name = value
        .get("name")
        .and_then(Value::as_str)
        .filter(|value| !value.is_empty())
        .map(ToString::to_string);
    let is_draft = value
        .get("isDraft")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let is_prerelease = value
        .get("isPrerelease")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let is_latest = value
        .get("isLatest")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let published_at = value
        .get("publishedAt")
        .and_then(Value::as_str)
        .filter(|value| !value.is_empty())
        .map(ToString::to_string);

    Some(ReleaseInfo {
        tag_name,
        name,
        is_draft,
        is_prerelease,
        is_latest,
        published_at,
    })
}

pub fn repo_view(repo: &str) -> Result<Option<RepoInfo>> {
    let output = run_gh(&[
        "repo",
        "view",
        repo,
        "--json",
        "nameWithOwner,visibility,url",
    ])?;
    if !output.success {
        return Ok(None);
    }

    let value: Value = serde_json::from_str(&output.stdout).unwrap_or(Value::Null);
    if value.is_null() {
        return Ok(None);
    }

    let name_with_owner = value
        .get("nameWithOwner")
        .and_then(Value::as_str)
        .unwrap_or(repo)
        .to_string();
    let visibility = value
        .get("visibility")
        .and_then(Value::as_str)
        .map(ToString::to_string);
    let url = value
        .get("url")
        .and_then(Value::as_str)
        .map(ToString::to_string);

    Ok(Some(RepoInfo {
        name_with_owner,
        visibility,
        url,
    }))
}
