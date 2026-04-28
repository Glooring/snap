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

    let object_dir = temp.path().join(".git").join("objects").join("bb");
    fs::create_dir_all(&object_dir).expect("object dir");
    fs::write(
        object_dir.join("22222222222222222222222222222222222222"),
        "",
    )
    .expect("empty object");

    snap_cmd(temp.path())
        .args(["new", "v2", "should fail"])
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "Git repository has empty object/ref files",
        ));

    let tags = git(temp.path(), &["tag", "--list"]);
    assert!(!tags.lines().any(|tag| tag == "v2"));
}
