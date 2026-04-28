use anyhow::{anyhow, Context, Result};
use chrono::Local;
use std::fs;
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

#[derive(Debug, Clone)]
pub struct RepairPlan {
    pub empty_git_files: Vec<PathBuf>,
    pub target_branch: Option<String>,
    pub target_commit: Option<String>,
    pub needs_head_repair: bool,
    pub needs_branch_repair: bool,
}

#[derive(Debug, Clone)]
pub struct RepairOutcome {
    pub backup_path: PathBuf,
    pub deleted_empty_files: Vec<PathBuf>,
    pub repaired_branch: Option<String>,
    pub repaired_head: bool,
    pub reset_index: bool,
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

pub fn ensure_git_fast_preflight_for_write(allow_unborn_head: bool) -> Result<()> {
    if !Path::new(".git").is_dir() {
        return Err(anyhow!("Not a Git repository. Run `snap init` first."));
    }

    let empty_refs = find_empty_files_under(Path::new(".git/refs"))?;
    if !empty_refs.is_empty() {
        return Err(fast_health_error(
            "Git repository has empty ref files.",
            Some(&format!("First empty ref: {}", empty_refs[0].display())),
        ));
    }

    let status = run_git(&["status", "--porcelain"], None)?;
    if !status.success {
        return Err(fast_health_error(
            &format!("Git status failed: {}", first_line(&status.stderr)),
            None,
        ));
    }

    let head = run_git(&["rev-parse", "--verify", "HEAD^{commit}"], None)?;
    let head_exists = head.success;
    if !head_exists && !(allow_unborn_head && is_unborn_head_error(&head.stderr)) {
        return Err(fast_health_error(
            "Git HEAD does not point to a valid commit.",
            Some(first_line(&head.stderr).as_str()),
        ));
    }

    let branch = run_git(&["symbolic-ref", "--short", "HEAD"], None)?;
    if !branch.success && head_exists {
        return Err(fast_health_error(
            "Git HEAD is detached. `snap` will not write a new snapshot until HEAD is attached to a branch.",
            Some("Run `snap doctor` or `snap doctor --repair` for diagnosis."),
        ));
    }

    if branch.success {
        let branch_name = branch.stdout.trim();
        let ref_name = format!("refs/heads/{}", branch_name);
        let branch_check = run_git(
            &["rev-parse", "--verify", &format!("{}^{{commit}}", ref_name)],
            None,
        )?;
        if !branch_check.success
            && !(allow_unborn_head && is_unborn_head_error(&branch_check.stderr))
        {
            return Err(fast_health_error(
                &format!(
                    "Current branch is invalid: {}",
                    first_line(&branch_check.stderr)
                ),
                None,
            ));
        }
    }

    Ok(())
}

pub fn ensure_git_healthy_for_write(allow_unborn_head: bool) -> Result<()> {
    ensure_git_fast_preflight_for_write(allow_unborn_head)
}

pub fn create_repair_plan(report: &GitHealthReport) -> Result<RepairPlan> {
    if !report.is_git_repo {
        return Err(anyhow!("No .git directory found. Run `snap init` first."));
    }

    let target_branch = infer_target_branch(report)?;
    let needs_head_repair = report.detached_head || report.head_error.is_some();
    let needs_branch_repair = report.branch_error.is_some()
        || (report.status_error.is_some() && report.current_branch.is_some());
    let needs_ref_repair = needs_head_repair || needs_branch_repair;
    let target_commit = if needs_ref_repair {
        Some(
            report
                .latest_valid_snapshot()
                .and_then(|snapshot| snapshot.commit.clone())
                .ok_or_else(|| {
                    anyhow!("Cannot repair refs because no valid snapshot commit was found.")
                })?,
        )
    } else {
        None
    };

    if needs_ref_repair && target_branch.is_none() {
        return Err(anyhow!(
            "Cannot determine the target branch safely. Attach HEAD manually or repair using doc/REPAIR_GIT_ERRORS.md."
        ));
    }

    Ok(RepairPlan {
        empty_git_files: report.empty_git_files.clone(),
        target_branch,
        target_commit,
        needs_head_repair,
        needs_branch_repair,
    })
}

pub fn repair_git_repository(plan: &RepairPlan) -> Result<RepairOutcome> {
    if !Path::new(".git").is_dir() {
        return Err(anyhow!("No .git directory found."));
    }

    let backup_path = backup_git_dir()?;

    let mut deleted_empty_files = Vec::new();
    for path in &plan.empty_git_files {
        if path.is_file() && fs::metadata(path).map(|m| m.len() == 0).unwrap_or(false) {
            fs::remove_file(path)
                .with_context(|| format!("Failed to delete empty Git file {}", path.display()))?;
            deleted_empty_files.push(path.clone());
        }
    }

    let mut repaired_branch = None;
    let mut repaired_head = false;
    if let (Some(branch), Some(commit)) =
        (plan.target_branch.as_deref(), plan.target_commit.as_deref())
    {
        if plan.needs_branch_repair || plan.needs_head_repair {
            let ref_name = format!("refs/heads/{}", branch);
            run_git_success(&["update-ref", &ref_name, commit], None)?;
            repaired_branch = Some(branch.to_string());
        }

        if plan.needs_head_repair {
            fs::write(".git/HEAD", format!("ref: refs/heads/{}\n", branch))
                .context("Failed to normalize .git/HEAD")?;
            repaired_head = true;
        }
    }

    let reset_index = repaired_branch.is_some() || repaired_head || !deleted_empty_files.is_empty();
    if reset_index {
        run_git_success(&["reset", "--mixed", "HEAD"], None)?;
    }

    Ok(RepairOutcome {
        backup_path,
        deleted_empty_files,
        repaired_branch,
        repaired_head,
        reset_index,
    })
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
        files.extend(find_empty_files_under(root)?);
    }
    files.sort();
    Ok(files)
}

fn find_empty_files_under(root: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    if !root.exists() {
        return Ok(files);
    }

    for entry in WalkDir::new(root).into_iter().filter_map(Result::ok) {
        if !entry.file_type().is_file() {
            continue;
        }

        if entry.metadata().map(|m| m.len() == 0).unwrap_or(false) {
            files.push(entry.path().to_path_buf());
        }
    }

    files.sort();
    Ok(files)
}

fn infer_target_branch(report: &GitHealthReport) -> Result<Option<String>> {
    if let Some(branch) = report
        .current_branch
        .as_deref()
        .filter(|branch| !branch.is_empty())
    {
        return Ok(Some(branch.to_string()));
    }

    if let Some(branch) = parse_head_branch()? {
        return Ok(Some(branch));
    }

    let branches = list_local_branches()?;
    if branches.len() == 1 {
        return Ok(branches.into_iter().next());
    }

    Ok(None)
}

fn parse_head_branch() -> Result<Option<String>> {
    let head = match fs::read_to_string(".git/HEAD") {
        Ok(head) => head,
        Err(_) => return Ok(None),
    };
    let head = head.trim();
    let Some(rest) = head.strip_prefix("ref: refs/heads/") else {
        return Ok(None);
    };
    if rest.is_empty() {
        Ok(None)
    } else {
        Ok(Some(rest.to_string()))
    }
}

fn list_local_branches() -> Result<Vec<String>> {
    let refs = run_git(
        &["for-each-ref", "refs/heads", "--format=%(refname:short)"],
        None,
    )?;
    if !refs.success {
        return Ok(Vec::new());
    }

    let mut branches: Vec<_> = refs
        .stdout
        .lines()
        .map(str::trim)
        .filter(|branch| !branch.is_empty())
        .map(ToString::to_string)
        .collect();
    branches.sort();
    branches.dedup();
    Ok(branches)
}

fn backup_git_dir() -> Result<PathBuf> {
    let stamp = Local::now().format("%Y%m%d-%H%M%S").to_string();
    let mut backup_path = PathBuf::from(format!(".git.backup.{}", stamp));
    let mut counter = 1;
    while backup_path.exists() {
        backup_path = PathBuf::from(format!(".git.backup.{}-{}", stamp, counter));
        counter += 1;
    }
    copy_dir_recursive(Path::new(".git"), &backup_path)?;
    Ok(backup_path)
}

fn copy_dir_recursive(source: &Path, destination: &Path) -> Result<()> {
    fs::create_dir_all(destination).with_context(|| {
        format!(
            "Failed to create backup directory {}",
            destination.display()
        )
    })?;

    for entry in WalkDir::new(source).into_iter().filter_map(Result::ok) {
        let relative = entry.path().strip_prefix(source)?;
        let target = destination.join(relative);
        if entry.file_type().is_dir() {
            fs::create_dir_all(&target)?;
        } else if entry.file_type().is_file() {
            if let Some(parent) = target.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(entry.path(), &target).with_context(|| {
                format!(
                    "Failed to copy {} to {}",
                    entry.path().display(),
                    target.display()
                )
            })?;
        }
    }

    Ok(())
}

fn fast_health_error(message: &str, detail: Option<&str>) -> anyhow::Error {
    let mut details = vec![
        message.to_string(),
        "Run `snap doctor` for a full diagnosis.".to_string(),
        "Run `snap doctor --repair` to repair safe cases with a backup.".to_string(),
    ];

    if let Some(detail) = detail {
        details.push(detail.to_string());
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
