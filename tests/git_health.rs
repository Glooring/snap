use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use std::path::Path;
use std::process::Command as StdCommand;

fn snap_cmd(dir: &Path) -> Command {
    let mut cmd = Command::cargo_bin("snap").expect("snap binary");
    cmd.current_dir(dir);
    cmd
}

fn git(dir: &Path, args: &[&str]) -> String {
    let output = StdCommand::new("git")
        .args(args)
        .current_dir(dir)
        .output()
        .expect("git command");

    assert!(
        output.status.success(),
        "git {:?} failed\nstdout:\n{}\nstderr:\n{}",
        args,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    String::from_utf8(output.stdout).expect("utf8 stdout")
}

fn git_success(dir: &Path, args: &[&str]) -> bool {
    StdCommand::new("git")
        .args(args)
        .current_dir(dir)
        .output()
        .expect("git command")
        .status
        .success()
}

fn init_snap_repo(dir: &Path) {
    snap_cmd(dir).arg("init").assert().success();
    git(dir, &["config", "user.email", "snap-test@example.com"]);
    git(dir, &["config", "user.name", "Snap Test"]);
}

fn create_snapshot(dir: &Path, label: &str, file_name: &str, content: &str) {
    fs::write(dir.join(file_name), content).expect("write fixture");
    snap_cmd(dir)
        .args(["new", label, "test snapshot"])
        .assert()
        .success();
}

#[test]
fn doctor_reports_healthy_repo() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    init_snap_repo(temp.path());
    create_snapshot(temp.path(), "v1", "file.txt", "one");

    snap_cmd(temp.path())
        .arg("doctor")
        .assert()
        .success()
        .stdout(predicate::str::contains("Git repository looks healthy"));
}

#[test]
fn doctor_detects_empty_loose_object() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    init_snap_repo(temp.path());

    let object_dir = temp.path().join(".git").join("objects").join("aa");
    fs::create_dir_all(&object_dir).expect("object dir");
    fs::write(
        object_dir.join("11111111111111111111111111111111111111"),
        "",
    )
    .expect("empty object");

    snap_cmd(temp.path())
        .arg("doctor")
        .assert()
        .success()
        .stdout(predicate::str::contains("Empty object/ref files: 1 found"))
        .stdout(predicate::str::contains("Problems were found"));
}

#[test]
fn doctor_detects_empty_tag_ref() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    init_snap_repo(temp.path());

    let tags_dir = temp.path().join(".git").join("refs").join("tags");
    fs::create_dir_all(&tags_dir).expect("tags dir");
    fs::write(tags_dir.join("broken"), "").expect("empty tag ref");

    snap_cmd(temp.path())
        .arg("doctor")
        .assert()
        .success()
        .stdout(predicate::str::contains("Empty object/ref files: 1 found"))
        .stdout(predicate::str::contains("Problems were found"));
}

#[test]
fn doctor_detects_detached_head() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    init_snap_repo(temp.path());
    create_snapshot(temp.path(), "v1", "file.txt", "one");
    git(temp.path(), &["checkout", "--detach", "HEAD"]);

    snap_cmd(temp.path())
        .arg("doctor")
        .assert()
        .success()
        .stdout(predicate::str::contains("HEAD is detached"))
        .stdout(predicate::str::contains("Problems were found"));
}

#[test]
fn list_reports_git_ref_errors_instead_of_empty_snapshot_list() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    init_snap_repo(temp.path());
    create_snapshot(temp.path(), "v1", "file.txt", "one");

    let tags_dir = temp.path().join(".git").join("refs").join("tags");
    fs::write(
        tags_dir.join("broken"),
        "1111111111111111111111111111111111111111\n",
    )
    .expect("invalid tag ref");

    snap_cmd(temp.path())
        .arg("list")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Failed to inspect snapshot tags"));
}

#[test]
fn restore_keeps_head_attached_to_branch() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    init_snap_repo(temp.path());
    create_snapshot(temp.path(), "v1", "file.txt", "one");
    create_snapshot(temp.path(), "v2", "file.txt", "two");

    let branch_before = git(temp.path(), &["symbolic-ref", "--short", "HEAD"]);

    snap_cmd(temp.path())
        .args(["restore", "v1"])
        .write_stdin("y\n")
        .assert()
        .success();

    let branch_after = git(temp.path(), &["symbolic-ref", "--short", "HEAD"]);
    assert_eq!(branch_before, branch_after);

    let head = git(temp.path(), &["rev-parse", "HEAD"]);
    let v1 = git(temp.path(), &["rev-parse", "v1^{commit}"]);
    assert_eq!(head, v1);
}

#[test]
fn new_stops_before_writing_when_health_check_fails() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    init_snap_repo(temp.path());
    create_snapshot(temp.path(), "v1", "file.txt", "one");

    let tags_dir = temp.path().join(".git").join("refs").join("tags");
    fs::write(tags_dir.join("broken"), "").expect("empty tag ref");

    snap_cmd(temp.path())
        .args(["new", "v2", "should fail"])
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "Git repository has empty ref files",
        ));

    let tags = git(temp.path(), &["tag", "--list"]);
    assert!(!tags.lines().any(|tag| tag == "v2"));
}

#[test]
fn doctor_repair_deletes_empty_git_files_and_creates_backup() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    init_snap_repo(temp.path());
    create_snapshot(temp.path(), "v1", "file.txt", "one");

    let object_dir = temp.path().join(".git").join("objects").join("cc");
    fs::create_dir_all(&object_dir).expect("object dir");
    let empty_object = object_dir.join("33333333333333333333333333333333333333");
    fs::write(&empty_object, "").expect("empty object");

    let empty_tag = temp
        .path()
        .join(".git")
        .join("refs")
        .join("tags")
        .join("broken");
    fs::write(&empty_tag, "").expect("empty tag");

    snap_cmd(temp.path())
        .args(["doctor", "--repair"])
        .write_stdin("y\n")
        .assert()
        .success()
        .stdout(predicate::str::contains("Repair applied"))
        .stdout(predicate::str::contains("Backup:"));

    assert!(!empty_object.exists());
    assert!(!empty_tag.exists());

    let has_backup = fs::read_dir(temp.path())
        .expect("read temp")
        .filter_map(Result::ok)
        .any(|entry| {
            entry
                .file_name()
                .to_string_lossy()
                .starts_with(".git.backup.")
        });
    assert!(has_backup);
}

#[test]
fn doctor_repair_repairs_invalid_branch_to_latest_snapshot() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    init_snap_repo(temp.path());
    create_snapshot(temp.path(), "v1", "file.txt", "one");
    create_snapshot(temp.path(), "v2", "file.txt", "two");

    let branch = git(temp.path(), &["symbolic-ref", "--short", "HEAD"])
        .trim()
        .to_string();
    fs::write(
        temp.path()
            .join(".git")
            .join("refs")
            .join("heads")
            .join(&branch),
        "1111111111111111111111111111111111111111\n",
    )
    .expect("invalid branch ref");

    assert!(!git_success(temp.path(), &["status", "--porcelain"]));

    snap_cmd(temp.path())
        .args(["doctor", "--repair"])
        .write_stdin("y\n")
        .assert()
        .success()
        .stdout(predicate::str::contains("Repair applied"))
        .stdout(predicate::str::contains("Repaired branch ref"));

    assert!(git_success(temp.path(), &["status", "--porcelain"]));
    let head = git(temp.path(), &["rev-parse", "HEAD"]);
    let v2 = git(temp.path(), &["rev-parse", "v2^{commit}"]);
    assert_eq!(head, v2);
}

#[test]
fn doctor_repair_normalizes_detached_head_when_single_branch_exists() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    init_snap_repo(temp.path());
    create_snapshot(temp.path(), "v1", "file.txt", "one");
    let head = git(temp.path(), &["rev-parse", "HEAD"]);
    fs::write(temp.path().join(".git").join("HEAD"), head).expect("raw head");

    snap_cmd(temp.path())
        .args(["doctor", "--repair"])
        .write_stdin("y\n")
        .assert()
        .success()
        .stdout(predicate::str::contains("Normalized .git/HEAD"));

    let branch = git(temp.path(), &["symbolic-ref", "--short", "HEAD"]);
    assert!(!branch.trim().is_empty());
}
