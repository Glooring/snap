use crate::git_health::{run_git, run_git_success};
use anyhow::{anyhow, Context, Result};
use std::path::PathBuf;

pub const ORIGIN: &str = "origin";
pub const SNAP_METADATA_NAMESPACE: &str = "refs/snap-metadata";
pub const SNAP_METADATA_REFSPEC: &str = "refs/snap-metadata/*:refs/snap-metadata/*";

#[derive(Debug, Clone)]
pub struct BranchInfo {
    pub name: String,
    pub current: bool,
    pub upstream: Option<String>,
    pub tracking: Option<String>,
}

impl BranchInfo {
    pub fn status(&self) -> String {
        match (&self.upstream, &self.tracking) {
            (Some(_), Some(tracking)) if !tracking.trim().is_empty() => tracking
                .trim()
                .trim_start_matches('[')
                .trim_end_matches(']')
                .to_string(),
            (Some(_), _) => "up to date".to_string(),
            (None, _) => "local only".to_string(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct WorktreeSummary {
    pub added: usize,
    pub modified: usize,
    pub deleted: usize,
    pub renamed: usize,
    pub untracked: usize,
}

impl WorktreeSummary {
    pub fn is_clean(&self) -> bool {
        self.added == 0
            && self.modified == 0
            && self.deleted == 0
            && self.renamed == 0
            && self.untracked == 0
    }

    pub fn describe(&self) -> String {
        if self.is_clean() {
            return "clean".to_string();
        }

        let mut parts = Vec::new();
        push_count(&mut parts, self.added, "added");
        push_count(&mut parts, self.modified, "modified");
        push_count(&mut parts, self.deleted, "deleted");
        push_count(&mut parts, self.renamed, "renamed");
        push_count(&mut parts, self.untracked, "untracked");
        parts.join(", ")
    }
}

fn push_count(parts: &mut Vec<String>, count: usize, label: &str) {
    if count > 0 {
        parts.push(format!("{} {}", count, label));
    }
}

pub fn ensure_no_operation_in_progress() -> Result<()> {
    let git_dir = git_dir()?;
    let operations = [
        ("merge", "MERGE_HEAD"),
        ("rebase", "rebase-merge"),
        ("rebase", "rebase-apply"),
        ("cherry-pick", "CHERRY_PICK_HEAD"),
        ("revert", "REVERT_HEAD"),
    ];

    for (operation, marker) in operations {
        if git_dir.join(marker).exists() {
            return Err(anyhow!(
                "Git has a {} in progress. Finish it with Git, then rerun this snap command.",
                operation
            ));
        }
    }

    Ok(())
}

pub fn git_dir() -> Result<PathBuf> {
    let output = run_git_success(&["rev-parse", "--git-dir"], None)?;
    Ok(PathBuf::from(output.trim()))
}

pub fn has_head_commit() -> Result<bool> {
    Ok(run_git(&["rev-parse", "--verify", "HEAD^{commit}"], None)?.success)
}

pub fn head_commit() -> Result<String> {
    let output = run_git(&["rev-parse", "--verify", "HEAD^{commit}"], None)?;
    if !output.success {
        return Err(anyhow!(
            "No HEAD commit found. Create a commit before uploading release assets."
        ));
    }
    Ok(output.stdout.trim().to_string())
}

pub fn current_branch() -> Result<String> {
    let branch = run_git(&["symbolic-ref", "--short", "HEAD"], None)?;
    if !branch.success {
        return Err(anyhow!(
            "Git HEAD is detached. Attach HEAD to a branch before running this command."
        ));
    }
    Ok(branch.stdout.trim().to_string())
}

pub fn list_branches() -> Result<Vec<BranchInfo>> {
    let current = current_branch().ok();
    let output = run_git_success(
        &[
            "for-each-ref",
            "refs/heads",
            "--sort=refname",
            "--format=%(refname:short)%09%(upstream:short)%09%(upstream:track)",
        ],
        None,
    )?;

    Ok(output
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let mut parts = line.split('\t');
            let name = parts.next().unwrap_or("").trim().to_string();
            let upstream = parts
                .next()
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .map(ToString::to_string);
            let tracking = parts
                .next()
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .map(ToString::to_string);
            let current = current.as_deref() == Some(name.as_str());
            BranchInfo {
                name,
                current,
                upstream,
                tracking,
            }
        })
        .collect())
}

pub fn validate_branch_name(name: &str) -> Result<()> {
    let trimmed = name.trim();
    if trimmed.is_empty() || trimmed != name {
        return Err(anyhow!(
            "Invalid branch name '{}'. Branch names cannot be empty or padded with spaces.",
            name
        ));
    }
    if name.starts_with('-') {
        return Err(anyhow!(
            "Invalid branch name '{}'. Branch names cannot start with '-'.",
            name
        ));
    }

    let result = run_git(&["check-ref-format", "--branch", name], None)?;
    if !result.success {
        let detail = result
            .stderr
            .lines()
            .next()
            .unwrap_or("Git rejected this name.");
        return Err(anyhow!("Invalid branch name '{}': {}", name, detail));
    }
    Ok(())
}

pub fn branch_exists(name: &str) -> Result<bool> {
    let ref_name = format!("refs/heads/{}", name);
    Ok(run_git(&["show-ref", "--verify", "--quiet", &ref_name], None)?.success)
}

pub fn is_commit_reachable_from_ref(commit: &str, branch: &str) -> Result<bool> {
    let ref_name = format!("refs/heads/{}", branch);
    Ok(run_git(&["merge-base", "--is-ancestor", commit, &ref_name], None)?.success)
}

pub fn branches_containing_commit(commit: &str) -> Result<Vec<String>> {
    let output = run_git_success(
        &["branch", "--format=%(refname:short)", "--contains", commit],
        None,
    )
    .with_context(|| format!("Failed to inspect branches containing commit '{}'.", commit))?;
    Ok(output
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(ToString::to_string)
        .collect())
}

pub fn create_branch_and_switch(name: &str) -> Result<()> {
    run_git_success(&["switch", "-c", name], None)
        .with_context(|| format!("Failed to create and switch to branch '{}'.", name))?;
    Ok(())
}

pub fn switch_branch(name: &str) -> Result<()> {
    run_git_success(&["switch", name], None)
        .with_context(|| format!("Failed to switch to branch '{}'.", name))?;
    Ok(())
}

pub fn delete_branch(name: &str, force: bool) -> Result<()> {
    let flag = if force { "-D" } else { "-d" };
    run_git_success(&["branch", flag, name], None)
        .with_context(|| format!("Failed to delete branch '{}'.", name))?;
    Ok(())
}

pub fn merge_branch(name: &str) -> Result<()> {
    run_git_success(&["merge", name], None).with_context(|| {
        format!(
            "Failed to merge branch '{}'. Resolve conflicts with Git, then run `snap status`.",
            name
        )
    })?;
    Ok(())
}

pub fn upstream_branch() -> Result<Option<String>> {
    let upstream = run_git(
        &["rev-parse", "--abbrev-ref", "--symbolic-full-name", "@{u}"],
        None,
    )?;
    if upstream.success {
        let value = upstream.stdout.trim();
        if value.is_empty() {
            Ok(None)
        } else {
            Ok(Some(value.to_string()))
        }
    } else {
        Ok(None)
    }
}

pub fn ahead_behind() -> Result<Option<(usize, usize)>> {
    if upstream_branch()?.is_none() {
        return Ok(None);
    }

    let output = run_git(
        &["rev-list", "--left-right", "--count", "@{u}...HEAD"],
        None,
    )?;
    if !output.success {
        return Ok(None);
    }

    let mut parts = output.stdout.split_whitespace();
    let behind = parts
        .next()
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or(0);
    let ahead = parts
        .next()
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or(0);
    Ok(Some((ahead, behind)))
}

pub fn worktree_summary() -> Result<WorktreeSummary> {
    let output = run_git_success(&["status", "--porcelain"], None)?;
    Ok(parse_worktree_summary(&output))
}

fn parse_worktree_summary(output: &str) -> WorktreeSummary {
    let mut summary = WorktreeSummary::default();

    for line in output.lines().filter(|line| !line.trim().is_empty()) {
        let status = line.get(..2).unwrap_or(line);
        if status == "??" {
            summary.untracked += 1;
            continue;
        }

        if status.contains('R') {
            summary.renamed += 1;
        } else if status.contains('D') {
            summary.deleted += 1;
        } else if status.contains('A') {
            summary.added += 1;
        } else {
            summary.modified += 1;
        }
    }

    summary
}

pub fn has_worktree_changes() -> Result<bool> {
    Ok(!worktree_summary()?.is_clean())
}

pub fn log_history(limit: Option<usize>) -> Result<String> {
    log_history_with_refs(limit, &[])
}

pub fn log_history_all(limit: Option<usize>) -> Result<String> {
    log_history_with_refs(limit, &["--all"])
}

pub fn log_history_for_branch(limit: Option<usize>, branch: &str) -> Result<String> {
    let ref_name = format!("refs/heads/{}", branch);
    log_history_with_refs(limit, &[&ref_name])
}

fn log_history_with_refs(limit: Option<usize>, refs: &[&str]) -> Result<String> {
    let mut args = vec![
        "log".to_string(),
        "--graph".to_string(),
        "--decorate".to_string(),
        "--oneline".to_string(),
    ];
    if let Some(limit) = limit {
        args.push(format!("-n{}", limit));
    }
    args.extend(refs.iter().map(|value| value.to_string()));

    let args: Vec<&str> = args.iter().map(String::as_str).collect();
    run_git_success(&args, None).context("Failed to read Git history.")
}

pub fn remote_url(remote: &str) -> Result<Option<String>> {
    let output = run_git(&["remote", "get-url", remote], None)?;
    if output.success {
        let url = output.stdout.trim();
        if url.is_empty() {
            Ok(None)
        } else {
            Ok(Some(url.to_string()))
        }
    } else {
        Ok(None)
    }
}

pub fn ensure_remote(remote: &str) -> Result<String> {
    remote_url(remote)?.ok_or_else(|| {
        anyhow!(
            "No remote named '{}' is configured.\nCreate one with `snap remote create owner/repo --private` or connect one with `snap remote set-url <url>`.",
            remote
        )
    })
}

pub fn add_remote(remote: &str, url: &str) -> Result<()> {
    run_git_success(&["remote", "add", remote, url], None)
        .with_context(|| format!("Failed to add remote '{}'.", remote))?;
    Ok(())
}

pub fn list_remotes() -> Result<Vec<(String, String)>> {
    let output = run_git_success(&["remote", "-v"], None)?;
    let mut remotes = Vec::new();

    for line in output.lines() {
        let mut parts = line.split_whitespace();
        let Some(name) = parts.next() else {
            continue;
        };
        let Some(url) = parts.next() else {
            continue;
        };
        let direction = parts.next().unwrap_or("");
        if direction == "(fetch)" {
            remotes.push((name.to_string(), url.to_string()));
        }
    }

    Ok(remotes)
}

pub fn local_refs(namespace: &str) -> Result<Vec<String>> {
    let output = run_git_success(&["for-each-ref", namespace, "--format=%(refname)"], None)?;
    Ok(output
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(ToString::to_string)
        .collect())
}

pub fn has_remote_refs(remote: &str, pattern: &str) -> Result<bool> {
    let output = run_git(&["ls-remote", remote, pattern], None)?;
    if !output.success {
        return Err(anyhow!(
            "Failed to inspect remote refs '{} {}': {}",
            remote,
            pattern,
            output.stderr.trim()
        ));
    }
    Ok(!output.stdout.trim().is_empty())
}

pub fn fetch(remote: &str) -> Result<()> {
    run_git_success(&["fetch", remote], None)
        .with_context(|| format!("Failed to fetch from '{}'.", remote))?;
    Ok(())
}

pub fn fetch_tags(remote: &str) -> Result<()> {
    run_git_success(&["fetch", remote, "--tags"], None)
        .with_context(|| format!("Failed to fetch tags from '{}'.", remote))?;
    Ok(())
}

pub fn fetch_metadata_refs(remote: &str) -> Result<bool> {
    if !has_remote_refs(remote, "refs/snap-metadata/*")? {
        return Ok(false);
    }

    run_git_success(&["fetch", remote, SNAP_METADATA_REFSPEC], None)
        .with_context(|| format!("Failed to fetch snap metadata refs from '{}'.", remote))?;
    Ok(true)
}

pub fn pull_rebase() -> Result<()> {
    run_git_success(&["pull", "--rebase"], None).context("Failed to pull with rebase.")?;
    Ok(())
}

pub fn push_branch(remote: &str) -> Result<bool> {
    let has_upstream = upstream_branch()?.is_some();
    if has_upstream {
        run_git_success(&["push", remote, "HEAD"], None)
            .with_context(|| format!("Failed to push branch to '{}'.", remote))?;
        Ok(false)
    } else {
        run_git_success(&["push", "-u", remote, "HEAD"], None)
            .with_context(|| format!("Failed to push branch to '{}'.", remote))?;
        Ok(true)
    }
}

pub fn push_tags(remote: &str) -> Result<()> {
    run_git_success(&["push", remote, "--tags"], None)
        .with_context(|| format!("Failed to push tags to '{}'.", remote))?;
    Ok(())
}

pub fn push_metadata_refs(remote: &str) -> Result<usize> {
    let refs = local_refs(SNAP_METADATA_NAMESPACE)?;
    if refs.is_empty() {
        return Ok(0);
    }

    run_git_success(&["push", remote, SNAP_METADATA_REFSPEC], None)
        .with_context(|| format!("Failed to push snap metadata refs to '{}'.", remote))?;
    Ok(refs.len())
}

pub fn parse_github_repo_from_url(url: &str) -> Option<String> {
    let trimmed = url.trim().trim_end_matches(".git");

    if let Some(rest) = trimmed.strip_prefix("git@github.com:") {
        return normalize_owner_repo(rest);
    }

    for prefix in [
        "https://github.com/",
        "http://github.com/",
        "ssh://git@github.com/",
    ] {
        if let Some(rest) = trimmed.strip_prefix(prefix) {
            return normalize_owner_repo(rest);
        }
    }

    None
}

fn normalize_owner_repo(value: &str) -> Option<String> {
    let parts: Vec<_> = value.split('/').filter(|part| !part.is_empty()).collect();
    if parts.len() == 2 {
        Some(format!("{}/{}", parts[0], parts[1]))
    } else {
        None
    }
}
