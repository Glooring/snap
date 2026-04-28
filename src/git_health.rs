use anyhow::{anyhow, Context, Result};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use walkdir::WalkDir;

#[derive(Debug, Clone)]
pub struct GitCommandResult {
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
}

#[derive(Debug, Clone)]
pub struct SnapshotCheck {
    pub tag: String,
    pub commit: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Clone)]
pub struct GitHealthReport {
    pub is_git_repo: bool,
    pub empty_git_files: Vec<PathBuf>,
    pub status_error: Option<String>,
    pub head_commit: Option<String>,
    pub head_error: Option<String>,
    pub current_branch: Option<String>,
    pub detached_head: bool,
    pub branch_error: Option<String>,
    pub snapshots_error: Option<String>,
    pub snapshots: Vec<SnapshotCheck>,
}

impl GitHealthReport {
    pub fn has_errors(&self) -> bool {
        !self.is_git_repo
            || !self.empty_git_files.is_empty()
            || self.status_error.is_some()
            || self.head_error.is_some()
            || self.detached_head
            || self.branch_error.is_some()
            || self.snapshots_error.is_some()
            || self.snapshots.iter().any(|s| s.error.is_some())
    }

    pub fn latest_valid_snapshot(&self) -> Option<&SnapshotCheck> {
        self.snapshots
            .iter()
            .find(|snapshot| snapshot.commit.is_some() && snapshot.error.is_none())
    }
}

pub fn run_git(args: &[&str], input: Option<&str>) -> Result<GitCommandResult> {
    let mut cmd = Command::new("git");
    cmd.args(args).stdout(Stdio::piped()).stderr(Stdio::piped());

    if input.is_some() {
        cmd.stdin(Stdio::piped());
    }

    let mut child = cmd
        .spawn()
        .with_context(|| format!("Failed to spawn git {}", args.join(" ")))?;

    if let (Some(stdin), Some(input_data)) = (child.stdin.as_mut(), input) {
        use std::io::Write;
        stdin.write_all(input_data.as_bytes())?;
    }

    let output = child.wait_with_output()?;

    Ok(GitCommandResult {
        success: output.status.success(),
        stdout: String::from_utf8(output.stdout)?,
        stderr: String::from_utf8_lossy(&output.stderr).to_string(),
    })
}

pub fn run_git_success(args: &[&str], input: Option<&str>) -> Result<String> {
    let result = run_git(args, input)?;
    if !result.success {
        return Err(anyhow!(
            "Command failed: 'git {}'\n---\n{}",
            args.join(" "),
            result.stderr.trim()
        ));
    }
    Ok(result.stdout)
}

pub fn collect_health_report() -> Result<GitHealthReport> {
    let is_git_repo = Path::new(".git").is_dir();
    if !is_git_repo {
        return Ok(GitHealthReport {
            is_git_repo,
            empty_git_files: Vec::new(),
            status_error: None,
            head_commit: None,
            head_error: Some("No .git directory found.".to_string()),
            current_branch: None,
            detached_head: false,
            branch_error: None,
            snapshots_error: None,
            snapshots: Vec::new(),
        });
    }

    let empty_git_files = find_empty_git_files()?;

    let status = run_git(&["status", "--porcelain"], None)?;
    let status_error = if status.success {
        None
    } else {
        Some(status.stderr.trim().to_string())
    };

    let head = run_git(&["rev-parse", "--verify", "HEAD^{commit}"], None)?;
    let (head_commit, head_error) = if head.success {
        (Some(head.stdout.trim().to_string()), None)
    } else if is_unborn_head_error(&head.stderr) {
        (None, None)
    } else {
        (None, Some(head.stderr.trim().to_string()))
    };

    let branch = run_git(&["symbolic-ref", "--short", "HEAD"], None)?;
    let (current_branch, detached_head) = if branch.success {
        (Some(branch.stdout.trim().to_string()), false)
    } else if head_commit.is_some() {
        (None, true)
    } else {
        (None, false)
    };

    let branch_error = if let Some(branch_name) = current_branch.as_deref() {
        let ref_name = format!("refs/heads/{}", branch_name);
        let branch_check = run_git(
            &["rev-parse", "--verify", &format!("{}^{{commit}}", ref_name)],
            None,
        )?;
        if branch_check.success || is_unborn_head_error(&branch_check.stderr) {
            None
        } else {
            Some(branch_check.stderr.trim().to_string())
        }
    } else {
        None
    };

    let (snapshots, snapshots_error) = collect_snapshot_checks()?;

    Ok(GitHealthReport {
        is_git_repo,
        empty_git_files,
        status_error,
        head_commit,
        head_error,
        current_branch,
        detached_head,
        branch_error,
        snapshots_error,
        snapshots,
    })
}

pub fn ensure_git_healthy_for_write(allow_unborn_head: bool) -> Result<()> {
    let report = collect_health_report()?;

    if !report.is_git_repo {
        return Err(anyhow!("Not a Git repository. Run `snap init` first."));
    }

    if !report.empty_git_files.is_empty() {
        return Err(health_error(
            "Git repository has empty object/ref files.",
            &report,
        ));
    }

    if let Some(error) = report.status_error.as_deref() {
        return Err(health_error(
            &format!("Git status failed: {}", first_line(error)),
            &report,
        ));
    }

    if report.detached_head {
        return Err(health_error(
            "Git HEAD is detached. `snap` will not write a new snapshot until HEAD is attached to a branch.",
            &report,
        ));
    }

    if report.head_error.is_some() && !(allow_unborn_head && report.head_commit.is_none()) {
        return Err(health_error(
            "Git HEAD does not point to a valid commit.",
            &report,
        ));
    }

    if let Some(error) = report.branch_error.as_deref() {
        return Err(health_error(
            &format!("Current branch is invalid: {}", first_line(error)),
            &report,
        ));
    }

    if let Some(error) = report.snapshots_error.as_deref() {
        return Err(health_error(
            &format!("Could not inspect snapshot tags: {}", first_line(error)),
            &report,
        ));
    }

    if let Some(broken) = report
        .snapshots
        .iter()
        .find(|snapshot| snapshot.error.is_some())
    {
        return Err(health_error(
            &format!(
                "Snapshot tag \"{}\" does not point to a valid commit.",
                broken.tag
            ),
            &report,
        ));
    }

    Ok(())
}

pub fn resolve_snapshot_commit(tag: &str) -> Result<String> {
    Ok(run_git_success(
        &["rev-parse", "--verify", &format!("{}^{{commit}}", tag)],
        None,
    )?
    .trim()
    .to_string())
}

fn collect_snapshot_checks() -> Result<(Vec<SnapshotCheck>, Option<String>)> {
    let tags = run_git(
        &[
            "for-each-ref",
            "refs/tags",
            "--sort=-taggerdate",
            "--format=%(refname:short)",
        ],
        None,
    )?;

    if !tags.success {
        return Ok((Vec::new(), Some(tags.stderr.trim().to_string())));
    }

    let mut snapshots = Vec::new();
    for tag in tags
        .stdout
        .lines()
        .map(str::trim)
        .filter(|tag| !tag.is_empty())
    {
        let commit = run_git(
            &["rev-parse", "--verify", &format!("{}^{{commit}}", tag)],
            None,
        )?;
        if commit.success {
            snapshots.push(SnapshotCheck {
                tag: tag.to_string(),
                commit: Some(commit.stdout.trim().to_string()),
                error: None,
            });
        } else {
            snapshots.push(SnapshotCheck {
                tag: tag.to_string(),
                commit: None,
                error: Some(commit.stderr.trim().to_string()),
            });
        }
    }

    Ok((snapshots, None))
}

fn find_empty_git_files() -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    for root in [Path::new(".git/objects"), Path::new(".git/refs")] {
        if !root.exists() {
            continue;
        }

        for entry in WalkDir::new(root).into_iter().filter_map(Result::ok) {
            if !entry.file_type().is_file() {
                continue;
            }

            if entry.metadata().map(|m| m.len() == 0).unwrap_or(false) {
                files.push(entry.path().to_path_buf());
            }
        }
    }
    files.sort();
    Ok(files)
}

fn health_error(message: &str, report: &GitHealthReport) -> anyhow::Error {
    let mut details = vec![
        message.to_string(),
        "Run `snap doctor` for a read-only diagnosis.".to_string(),
        "See `doc/REPAIR_GIT_ERRORS.md` for the manual repair flow.".to_string(),
    ];

    if let Some(snapshot) = report.latest_valid_snapshot() {
        details.push(format!("Latest valid snapshot found: {}", snapshot.tag));
    }

    anyhow!("{}", details.join("\n"))
}

fn first_line(value: &str) -> String {
    value.lines().next().unwrap_or(value).to_string()
}

fn is_unborn_head_error(stderr: &str) -> bool {
    stderr.contains("Needed a single revision")
        || stderr.contains("ambiguous argument 'HEAD")
        || stderr.contains("unknown revision or path not in the working tree")
}
