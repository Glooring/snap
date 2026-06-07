use assert_cmd::Command;
use predicates::prelude::*;
use std::env;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command as StdCommand, Stdio};

fn snap_cmd(dir: &Path) -> Command {
    let mut cmd = Command::cargo_bin("snap").expect("snap binary");
    cmd.current_dir(dir);
    cmd.env("SNAP_CONFIG_PATH", dir.join(".snapconfig"));
    cmd
}

fn snap_cmd_with_path(dir: &Path, path_prefix: &Path) -> Command {
    let mut cmd = snap_cmd(dir);
    let current_path = env::var_os("PATH").unwrap_or_default();
    let mut paths = vec![path_prefix.to_path_buf()];
    paths.extend(env::split_paths(&current_path));
    let joined = env::join_paths(paths).expect("join PATH");
    cmd.env("PATH", joined);
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

fn current_branch(dir: &Path) -> String {
    git(dir, &["symbolic-ref", "--short", "HEAD"])
        .trim()
        .to_string()
}

fn init_bare_repo(dir: &Path) {
    git(dir, &["init", "--bare"]);
}

fn init_snap_repo(dir: &Path) {
    snap_cmd(dir).arg("init").assert().success();
    git(dir, &["config", "user.email", "snap-test@example.com"]);
    git(dir, &["config", "user.name", "Snap Test"]);
}

fn create_fake_gh(bin_dir: &Path) -> PathBuf {
    fs::create_dir_all(bin_dir).expect("fake gh bin dir");

    #[cfg(windows)]
    {
        let path = bin_dir.join("gh.bat");
        fs::write(
            &path,
            r#"@echo off
echo %*>> "%GH_FAKE_LOG%"
if "%1"=="auth" if "%2"=="status" if "%GH_FAKE_AUTH_FAIL%"=="1" exit /b 1
if "%1"=="auth" if "%2"=="status" exit /b 0
if "%1"=="repo" if "%2"=="create" (
  git remote add origin "%GH_FAKE_REMOTE_URL%"
  exit /b 0
)
if "%1"=="repo" if "%2"=="view" (
  echo {"nameWithOwner":"owner/repo","visibility":"PRIVATE","url":"https://github.com/owner/repo"}
  exit /b 0
)
if "%1"=="repo" if "%2"=="delete" exit /b 0
if "%1"=="release" if "%2"=="view" (
  if "%GH_FAKE_RELEASE_EXISTS%"=="1" exit /b 0
  exit /b 1
)
if "%1"=="release" if "%2"=="create" exit /b 0
if "%1"=="release" if "%2"=="upload" exit /b 0
if "%1"=="release" if "%2"=="list" (
  if "%GH_FAKE_RELEASE_LIST_EMPTY%"=="1" (
    echo []
    exit /b 0
  )
  echo [{"tagName":"v7.2.0","name":"snap v7.2.0","isDraft":true,"isPrerelease":false,"isLatest":false,"publishedAt":null},{"tagName":"v7.1.0","name":"snap v7.1.0","isDraft":false,"isPrerelease":true,"isLatest":false,"publishedAt":"2026-05-10T10:00:00Z"},{"tagName":"v7.0.0","name":"snap v7.0.0","isDraft":false,"isPrerelease":false,"isLatest":true,"publishedAt":"2026-05-01T08:30:00Z"}]
  exit /b 0
)
exit /b 0
"#,
        )
        .expect("write fake gh");
        path
    }

    #[cfg(not(windows))]
    {
        use std::os::unix::fs::PermissionsExt;

        let path = bin_dir.join("gh");
        fs::write(
            &path,
            r#"#!/usr/bin/env bash
echo "$*" >> "$GH_FAKE_LOG"
if [[ "$1" == "auth" && "$2" == "status" ]]; then
  if [[ "$GH_FAKE_AUTH_FAIL" == "1" ]]; then
    exit 1
  fi
  exit 0
fi
if [[ "$1" == "repo" && "$2" == "create" ]]; then
  git remote add origin "$GH_FAKE_REMOTE_URL"
  exit 0
fi
if [[ "$1" == "repo" && "$2" == "view" ]]; then
  echo '{"nameWithOwner":"owner/repo","visibility":"PRIVATE","url":"https://github.com/owner/repo"}'
  exit 0
fi
if [[ "$1" == "repo" && "$2" == "delete" ]]; then
  exit 0
fi
if [[ "$1" == "release" && "$2" == "view" ]]; then
  if [[ "$GH_FAKE_RELEASE_EXISTS" == "1" ]]; then
    exit 0
  fi
  exit 1
fi
if [[ "$1" == "release" && "$2" == "create" ]]; then
  exit 0
fi
if [[ "$1" == "release" && "$2" == "upload" ]]; then
  exit 0
fi
if [[ "$1" == "release" && "$2" == "list" ]]; then
  if [[ "$GH_FAKE_RELEASE_LIST_EMPTY" == "1" ]]; then
    echo '[]'
    exit 0
  fi
  echo '[{"tagName":"v7.2.0","name":"snap v7.2.0","isDraft":true,"isPrerelease":false,"isLatest":false,"publishedAt":null},{"tagName":"v7.1.0","name":"snap v7.1.0","isDraft":false,"isPrerelease":true,"isLatest":false,"publishedAt":"2026-05-10T10:00:00Z"},{"tagName":"v7.0.0","name":"snap v7.0.0","isDraft":false,"isPrerelease":false,"isLatest":true,"publishedAt":"2026-05-01T08:30:00Z"}]'
  exit 0
fi
exit 0
"#,
        )
        .expect("write fake gh");
        let mut permissions = fs::metadata(&path).expect("fake gh metadata").permissions();
        permissions.set_mode(0o755);
        fs::set_permissions(&path, permissions).expect("chmod fake gh");
        path
    }
}

fn create_fake_release_runtime(bin_dir: &Path, name: &str) -> PathBuf {
    fs::create_dir_all(bin_dir).expect("fake release bin dir");

    #[cfg(windows)]
    {
        let path = bin_dir.join(format!("{}.bat", name));
        fs::write(
            &path,
            r#"@echo off
echo %~nx0 %*>> "%SNAP_RELEASE_FAKE_LOG%"
exit /b 0
"#,
        )
        .expect("write fake release runtime");
        path
    }

    #[cfg(not(windows))]
    {
        use std::os::unix::fs::PermissionsExt;

        let path = bin_dir.join(name);
        fs::write(
            &path,
            r#"#!/usr/bin/env bash
echo "$(basename "$0") $*" >> "$SNAP_RELEASE_FAKE_LOG"
exit 0
"#,
        )
        .expect("write fake release runtime");
        let mut permissions = fs::metadata(&path)
            .expect("fake runtime metadata")
            .permissions();
        permissions.set_mode(0o755);
        fs::set_permissions(&path, permissions).expect("chmod fake runtime");
        path
    }
}

fn init_release_fixture(dir: &Path) {
    init_snap_repo(dir);
    fs::create_dir_all(dir.join("src")).expect("release fixture src");
    fs::write(
        dir.join("Cargo.toml"),
        r#"[package]
name = "snap"
version = "7.2.0"
edition = "2021"
"#,
    )
    .expect("release fixture Cargo.toml");
    fs::write(
        dir.join("Cargo.lock"),
        r#"# This file is automatically @generated by Cargo.
version = 4

[[package]]
name = "snap"
version = "7.2.0"
"#,
    )
    .expect("release fixture Cargo.lock");
    fs::write(dir.join("src").join("main.rs"), "fn main() {}\n").expect("release fixture main");
}

fn create_release_artifacts(dir: &Path) -> PathBuf {
    let release_dir = dir.join("release-github").join("v7.2.0");
    fs::create_dir_all(&release_dir).expect("release artifact dir");
    for artifact in [
        "snap-v7.2.0-windows-x86_64.exe",
        "snap-v7.2.0-windows-x86_64-setup.exe",
        "snap-v7.2.0-windows-x86_64.msi",
        "snap-v7.2.0-linux-x86_64",
        "snap-v7.2.0-linux-x86_64.tar.gz",
        "snap-v7.2.0-macos-aarch64",
        "snap-v7.2.0-macos-aarch64.tar.gz",
        "snap-v7.2.0-macos-x86_64",
        "snap-v7.2.0-macos-x86_64.tar.gz",
    ] {
        fs::write(release_dir.join(artifact), artifact).expect("release artifact");
    }
    release_dir
}

fn init_release_upload_fixture(dir: &Path) {
    init_release_fixture(dir);
    create_snapshot(dir, "v-release", "release.txt", "release");
    create_release_artifacts(dir);
}

fn create_snapshot(dir: &Path, label: &str, file_name: &str, content: &str) {
    fs::write(dir.join(file_name), content).expect("write fixture");
    snap_cmd(dir)
        .args(["new", label, "test snapshot"])
        .assert()
        .success();
}

fn create_snapshot_with_empty_dir(dir: &Path, label: &str, empty_dir: &str) {
    fs::write(dir.join(format!("{}.txt", label)), label).expect("write fixture");
    fs::create_dir_all(dir.join(empty_dir)).expect("empty dir");
    snap_cmd(dir)
        .args(["new", label, "metadata snapshot"])
        .assert()
        .success();
}

fn metadata_json_for_tag(dir: &Path, tag: &str) -> String {
    let hash = metadata_hash_for_tag(dir, tag);
    git(dir, &["cat-file", "-p", &hash])
}

fn set_readonly_path(path: &Path, readonly: bool) {
    let mut permissions = fs::metadata(path)
        .expect("metadata for readonly path")
        .permissions();
    permissions.set_readonly(readonly);
    fs::set_permissions(path, permissions).expect("set readonly path");
}

fn is_readonly_path(path: &Path) -> bool {
    fs::metadata(path)
        .expect("metadata for readonly assertion")
        .permissions()
        .readonly()
}

#[cfg(windows)]
fn hidden_fixture_path(dir: &Path) -> PathBuf {
    dir.join("hidden metadata.txt")
}

#[cfg(not(windows))]
fn hidden_fixture_path(dir: &Path) -> PathBuf {
    dir.join(".hidden metadata")
}

#[cfg(windows)]
fn set_hidden_path(path: &Path, hidden: bool) {
    let flag = if hidden { "+H" } else { "-H" };
    let output = StdCommand::new("attrib")
        .arg(flag)
        .arg(path)
        .output()
        .expect("run attrib");
    assert!(
        output.status.success(),
        "attrib {} failed\nstdout:\n{}\nstderr:\n{}",
        flag,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

#[cfg(not(windows))]
fn set_hidden_path(_path: &Path, _hidden: bool) {}

#[cfg(windows)]
fn is_hidden_path(path: &Path) -> bool {
    use std::os::windows::fs::MetadataExt;

    const FILE_ATTRIBUTE_HIDDEN: u32 = 0x2;
    fs::metadata(path)
        .expect("metadata for hidden assertion")
        .file_attributes()
        & FILE_ATTRIBUTE_HIDDEN
        != 0
}

#[cfg(not(windows))]
fn is_hidden_path(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(|name| name.starts_with('.'))
        .unwrap_or(false)
}

fn create_many_lightweight_tags(dir: &Path, count: usize) {
    let head = git(dir, &["rev-parse", "HEAD"]).trim().to_string();
    let mut child = StdCommand::new("git")
        .args(["update-ref", "--stdin"])
        .current_dir(dir)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn git update-ref");

    {
        let stdin = child.stdin.as_mut().expect("git stdin");
        for index in 0..count {
            writeln!(stdin, "create refs/tags/many-{index:04} {head}").expect("write update-ref");
        }
    }

    let output = child.wait_with_output().expect("wait git update-ref");
    assert!(
        output.status.success(),
        "git update-ref --stdin failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

fn metadata_hash_for_tag(dir: &Path, tag: &str) -> String {
    let ref_name = format!("refs/tags/{}", tag);
    let contents = git(dir, &["for-each-ref", "--format=%(contents)", &ref_name]);
    contents
        .lines()
        .find_map(|line| line.strip_prefix("Snap-Metadata-Ref:"))
        .map(str::trim)
        .map(ToString::to_string)
        .unwrap_or_else(|| panic!("metadata hash for tag {}", tag))
}

fn metadata_hash_for_tag_optional(dir: &Path, tag: &str) -> Option<String> {
    let ref_name = format!("refs/tags/{}", tag);
    let contents = git(dir, &["for-each-ref", "--format=%(contents)", &ref_name]);
    contents
        .lines()
        .find_map(|line| line.strip_prefix("Snap-Metadata-Ref:"))
        .map(str::trim)
        .map(ToString::to_string)
}

fn tag_message_for_tag(dir: &Path, tag: &str) -> String {
    let ref_name = format!("refs/tags/{}", tag);
    git(dir, &["for-each-ref", "--format=%(contents)", &ref_name])
}

fn create_legacy_snapshot_tag(dir: &Path, label: &str, file_name: &str, content: &str) {
    fs::write(dir.join(file_name), content).expect("write legacy fixture");
    git(dir, &["add", "-A"]);
    git(dir, &["commit", "-m", &format!("Snapshot: {}", label)]);
    git(dir, &["tag", "-a", label, "-m", "legacy snap snapshot"]);
}

fn write_snap_config(dir: &Path, track_metadata_only_changes: bool) {
    fs::write(
        dir.join(".snapconfig"),
        format!(
            r#"{{
  "options": {{
    "show_ids": false,
    "confirm_command": true,
    "order_by": "Timestamp",
    "edit_updates_timestamp": false,
    "track_metadata_only_changes": {},
    "list_limit": "all"
  }}
}}"#,
            track_metadata_only_changes
        ),
    )
    .expect("write snap config");
}

fn write_legacy_snap_config_without_metadata_only_option(dir: &Path) {
    fs::write(
        dir.join(".snapconfig"),
        r#"{
  "options": {
    "show_ids": false,
    "confirm_command": true,
    "order_by": "Timestamp",
    "edit_updates_timestamp": false,
    "list_limit": "all"
  }
}"#,
    )
    .expect("write legacy snap config");
}

fn metadata_ref_exists(dir: &Path, hash: &str) -> bool {
    let ref_name = format!("refs/snap-metadata/{}", hash);
    git_success(dir, &["show-ref", "--verify", &ref_name])
}

fn delete_metadata_ref(dir: &Path, hash: &str) {
    let ref_name = format!("refs/snap-metadata/{}", hash);
    git(dir, &["update-ref", "-d", &ref_name]);
}

fn prune_unreachable_now(dir: &Path) {
    git(
        dir,
        &["reflog", "expire", "--expire-unreachable=now", "--all"],
    );
    git(dir, &["gc", "--prune=now"]);
}

fn metadata_blob_exists(dir: &Path, hash: &str) -> bool {
    git_success(dir, &["cat-file", "-e", &format!("{}^{{blob}}", hash)])
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
fn doctor_json_reports_healthy_repo() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    init_snap_repo(temp.path());
    create_snapshot(temp.path(), "v1", "file.txt", "one");

    let assert = snap_cmd(temp.path())
        .args(["doctor", "--json"])
        .assert()
        .success();
    let stdout = String::from_utf8(assert.get_output().stdout.clone()).expect("utf8 stdout");
    let value: serde_json::Value = serde_json::from_str(&stdout).expect("doctor json");

    assert_eq!(value["schema_version"], 1);
    assert_eq!(value["status"], "ok");
    assert_eq!(value["summary"]["has_errors"], false);
    assert_eq!(value["summary"]["has_warnings"], false);
    assert_eq!(value["report"]["is_git_repo"], true);
}

#[test]
fn doctor_ci_succeeds_for_clean_repo() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    init_snap_repo(temp.path());
    create_snapshot(temp.path(), "v1", "file.txt", "one");

    snap_cmd(temp.path())
        .args(["doctor", "--ci"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Git repository looks healthy"));
}

#[test]
fn doctor_json_ci_fails_on_warnings_after_printing_json() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    init_snap_repo(temp.path());
    create_snapshot_with_empty_dir(temp.path(), "v1", "empty-dir");
    let hash = metadata_hash_for_tag(temp.path(), "v1");
    delete_metadata_ref(temp.path(), &hash);
    assert!(metadata_blob_exists(temp.path(), &hash));

    let assert = snap_cmd(temp.path())
        .args(["doctor", "--json", "--ci"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("health warnings or errors"));
    let stdout = String::from_utf8(assert.get_output().stdout.clone()).expect("utf8 stdout");
    let value: serde_json::Value = serde_json::from_str(&stdout).expect("doctor json");

    assert_eq!(value["status"], "warning");
    assert_eq!(value["summary"]["has_errors"], false);
    assert_eq!(value["summary"]["has_warnings"], true);
    assert_eq!(value["summary"]["unpinned_metadata_count"], 1);
}

#[test]
fn new_pins_metadata_blob() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    init_snap_repo(temp.path());
    create_snapshot_with_empty_dir(temp.path(), "v1", "empty-dir");

    let hash = metadata_hash_for_tag(temp.path(), "v1");
    assert!(metadata_blob_exists(temp.path(), &hash));
    assert!(metadata_ref_exists(temp.path(), &hash));
}

#[test]
fn new_marks_snapshot_tags_and_hides_marker_from_description() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    init_snap_repo(temp.path());
    create_snapshot(temp.path(), "v1", "file.txt", "one");

    let tag_message = tag_message_for_tag(temp.path(), "v1");
    assert!(tag_message.contains("Snap-Snapshot: true"));

    snap_cmd(temp.path())
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("v1"))
        .stdout(predicate::str::contains("test snapshot"))
        .stdout(predicate::str::contains("Snap-Snapshot").not());
}

#[test]
fn plain_release_tags_are_not_snapshots() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    init_snap_repo(temp.path());
    create_snapshot(temp.path(), "v1", "file.txt", "one");
    git(
        temp.path(),
        &["tag", "-a", "release-1", "-m", "release tag"],
    );

    snap_cmd(temp.path())
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("v1"))
        .stdout(predicate::str::contains("release-1").not());

    let assert = snap_cmd(temp.path())
        .args(["doctor", "--json"])
        .assert()
        .success();
    let stdout = String::from_utf8(assert.get_output().stdout.clone()).expect("utf8 stdout");
    let value: serde_json::Value = serde_json::from_str(&stdout).expect("doctor json");
    assert_eq!(value["summary"]["snapshot_count"], 1);
}

#[test]
fn legacy_unmarked_snap_tags_remain_visible() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    init_snap_repo(temp.path());
    create_legacy_snapshot_tag(temp.path(), "legacy-one", "legacy.txt", "one");

    let tag_message = tag_message_for_tag(temp.path(), "legacy-one");
    assert!(!tag_message.contains("Snap-Snapshot: true"));

    snap_cmd(temp.path())
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("legacy-one"))
        .stdout(predicate::str::contains("legacy snap snapshot"));
}

#[test]
fn doctor_ignores_plain_release_tag_that_points_to_blob() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    init_snap_repo(temp.path());
    create_snapshot(temp.path(), "v1", "file.txt", "one");
    fs::write(temp.path().join("release.txt"), "release").expect("release fixture");
    let blob = git(temp.path(), &["hash-object", "-w", "release.txt"]);
    git(
        temp.path(),
        &[
            "tag",
            "-a",
            "release-blob",
            "-m",
            "release blob",
            blob.trim(),
        ],
    );

    snap_cmd(temp.path())
        .arg("doctor")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Snapshot tags: 1 checked, 0 invalid",
        ))
        .stdout(predicate::str::contains("release-blob").not());
}

#[test]
fn new_ignores_metadata_only_changes_by_default() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    init_snap_repo(temp.path());
    create_snapshot(temp.path(), "v1", "file.txt", "one");
    fs::create_dir_all(temp.path().join("empty-dir")).expect("empty dir");

    snap_cmd(temp.path())
        .args(["new", "v2", "metadata only"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Only snap metadata changed"))
        .stdout(predicate::str::contains(
            "Metadata-only snapshots are disabled",
        ));

    assert!(!git_success(temp.path(), &["rev-parse", "--verify", "v2"]));
}

#[test]
fn new_include_metadata_only_flag_creates_metadata_only_snapshot() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    init_snap_repo(temp.path());
    create_snapshot(temp.path(), "v1", "file.txt", "one");
    fs::create_dir_all(temp.path().join("empty-dir")).expect("empty dir");

    snap_cmd(temp.path())
        .args(["new", "v2", "--include-metadata-only", "metadata only"])
        .assert()
        .success()
        .stdout(predicate::str::contains("New snapshot created"));

    let hash = metadata_hash_for_tag(temp.path(), "v2");
    assert!(metadata_ref_exists(temp.path(), &hash));
}

#[test]
fn new_tracks_metadata_only_changes_when_config_enabled() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    init_snap_repo(temp.path());
    write_snap_config(temp.path(), true);
    create_snapshot(temp.path(), "v1", "file.txt", "one");
    fs::create_dir_all(temp.path().join("empty-dir")).expect("empty dir");

    snap_cmd(temp.path())
        .args(["new", "v2", "metadata only"])
        .assert()
        .success()
        .stdout(predicate::str::contains("New snapshot created"));

    let hash = metadata_hash_for_tag(temp.path(), "v2");
    assert!(metadata_ref_exists(temp.path(), &hash));
}

#[test]
fn legacy_config_without_metadata_only_option_defaults_to_ignoring_metadata_only_changes() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    init_snap_repo(temp.path());
    write_legacy_snap_config_without_metadata_only_option(temp.path());
    create_snapshot(temp.path(), "v1", "file.txt", "one");
    fs::create_dir_all(temp.path().join("empty-dir")).expect("empty dir");

    snap_cmd(temp.path())
        .args(["new", "v2", "metadata only"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Metadata-only snapshots are disabled",
        ));

    assert!(!git_success(temp.path(), &["rev-parse", "--verify", "v2"]));
}

#[test]
fn file_changes_still_create_snapshot_and_record_current_metadata() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    init_snap_repo(temp.path());
    create_snapshot(temp.path(), "v1", "file.txt", "one");
    fs::create_dir_all(temp.path().join("empty-dir")).expect("empty dir");
    fs::write(temp.path().join("file.txt"), "two").expect("modify file");

    snap_cmd(temp.path())
        .args(["new", "v2", "file and metadata"])
        .assert()
        .success()
        .stdout(predicate::str::contains("New snapshot created"));

    let hash = metadata_hash_for_tag(temp.path(), "v2");
    assert!(metadata_ref_exists(temp.path(), &hash));
}

#[test]
fn snapshot_restore_handles_spaces_unicode_and_nested_empty_dirs() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    init_snap_repo(temp.path());

    let space_file = temp.path().join("dir with spaces").join("nested file.txt");
    let unicode_file = temp.path().join("unicodé λ").join("naïve Δ.txt");
    let empty_dir = temp
        .path()
        .join("empty dirs")
        .join("nested empty")
        .join("leaf");

    fs::create_dir_all(space_file.parent().expect("space parent")).expect("space dirs");
    fs::create_dir_all(unicode_file.parent().expect("unicode parent")).expect("unicode dirs");
    fs::create_dir_all(&empty_dir).expect("nested empty dir");
    fs::write(&space_file, "space path v1").expect("space file");
    fs::write(&unicode_file, "unicode path v1").expect("unicode file");

    snap_cmd(temp.path())
        .args(["new", "edge-one", "space unicode empty dir snapshot"])
        .assert()
        .success();

    let metadata = metadata_json_for_tag(temp.path(), "edge-one");
    assert!(metadata.contains("empty dirs/nested empty/leaf"));

    fs::remove_dir_all(temp.path().join("dir with spaces")).expect("remove space dir");
    fs::remove_dir_all(temp.path().join("unicodé λ")).expect("remove unicode dir");
    fs::remove_dir_all(temp.path().join("empty dirs")).expect("remove empty dirs");
    fs::write(temp.path().join("replacement.txt"), "replacement").expect("replacement");

    snap_cmd(temp.path())
        .args(["new", "edge-two", "changed edge paths"])
        .assert()
        .success();

    snap_cmd(temp.path())
        .args(["restore", "edge-one"])
        .assert()
        .success();

    assert_eq!(
        fs::read_to_string(&space_file).expect("restored space file"),
        "space path v1"
    );
    assert_eq!(
        fs::read_to_string(&unicode_file).expect("restored unicode file"),
        "unicode path v1"
    );
    assert!(
        empty_dir.exists(),
        "nested empty directory should be restored"
    );
    assert!(
        !temp.path().join("replacement.txt").exists(),
        "restore should remove files added after the target snapshot"
    );
}

#[test]
fn snapshot_restore_preserves_hidden_and_readonly_metadata() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    init_snap_repo(temp.path());

    let hidden_path = hidden_fixture_path(temp.path());
    let readonly_path = temp.path().join("readonly metadata.txt");
    fs::write(&hidden_path, "hidden").expect("hidden fixture");
    fs::write(&readonly_path, "readonly").expect("readonly fixture");
    set_hidden_path(&hidden_path, true);
    set_readonly_path(&readonly_path, true);

    snap_cmd(temp.path())
        .args(["new", "meta-one", "hidden and readonly metadata"])
        .assert()
        .success();

    let metadata = metadata_json_for_tag(temp.path(), "meta-one");
    let hidden_name = hidden_path
        .file_name()
        .expect("hidden file name")
        .to_string_lossy();
    assert!(metadata.contains(hidden_name.as_ref()));
    assert!(metadata.contains("readonly metadata.txt"));

    set_hidden_path(&hidden_path, false);
    set_readonly_path(&readonly_path, false);

    snap_cmd(temp.path())
        .args([
            "new",
            "meta-two",
            "--include-metadata-only",
            "metadata cleared",
        ])
        .assert()
        .success();

    snap_cmd(temp.path())
        .args(["restore", "meta-one"])
        .assert()
        .success();

    let restored_hidden = is_hidden_path(&hidden_path);
    let restored_readonly = is_readonly_path(&readonly_path);
    set_readonly_path(&readonly_path, false);

    assert!(restored_hidden, "hidden metadata should be restored");
    assert!(restored_readonly, "read-only metadata should be restored");
}

#[test]
fn update_pins_replacement_metadata_blob() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    init_snap_repo(temp.path());
    create_snapshot_with_empty_dir(temp.path(), "v1", "empty-a");

    fs::remove_dir_all(temp.path().join("empty-a")).expect("remove old empty dir");
    fs::create_dir_all(temp.path().join("empty-b")).expect("new empty dir");

    snap_cmd(temp.path())
        .arg("update")
        .write_stdin("y\n")
        .assert()
        .success();

    let hash = metadata_hash_for_tag(temp.path(), "v1");
    assert!(metadata_blob_exists(temp.path(), &hash));
    assert!(metadata_ref_exists(temp.path(), &hash));
}

#[test]
fn update_ignores_metadata_only_changes_by_default() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    init_snap_repo(temp.path());
    create_snapshot(temp.path(), "v1", "file.txt", "one");
    let tag_before = git(temp.path(), &["rev-parse", "v1^{tag}"]);
    fs::create_dir_all(temp.path().join("empty-dir")).expect("empty dir");

    snap_cmd(temp.path())
        .arg("update")
        .assert()
        .success()
        .stdout(predicate::str::contains("Only snap metadata changed"))
        .stdout(predicate::str::contains(
            "Metadata-only updates are disabled",
        ));

    let tag_after = git(temp.path(), &["rev-parse", "v1^{tag}"]);
    assert_eq!(tag_before, tag_after);
    assert!(metadata_hash_for_tag_optional(temp.path(), "v1").is_none());
}

#[test]
fn update_include_metadata_only_flag_amends_metadata_only_changes() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    init_snap_repo(temp.path());
    create_snapshot(temp.path(), "v1", "file.txt", "one");
    fs::create_dir_all(temp.path().join("empty-dir")).expect("empty dir");

    snap_cmd(temp.path())
        .args(["update", "--include-metadata-only"])
        .write_stdin("y\n")
        .assert()
        .success()
        .stdout(predicate::str::contains("Update complete"));

    let hash = metadata_hash_for_tag(temp.path(), "v1");
    assert!(metadata_ref_exists(temp.path(), &hash));
}

#[test]
fn edit_keeps_metadata_valid_and_pinned_after_retagging() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    init_snap_repo(temp.path());
    create_snapshot_with_empty_dir(temp.path(), "v1", "empty-dir");
    let before_hash = metadata_hash_for_tag(temp.path(), "v1");

    snap_cmd(temp.path())
        .args(["edit", "v1"])
        .write_stdin("v1-renamed\nrenamed snapshot\n")
        .assert()
        .success();

    let after_hash = metadata_hash_for_tag(temp.path(), "v1-renamed");
    assert_eq!(before_hash, after_hash);
    assert!(metadata_ref_exists(temp.path(), &after_hash));
}

#[test]
fn doctor_detects_missing_metadata_blob() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    init_snap_repo(temp.path());
    create_snapshot_with_empty_dir(temp.path(), "v1", "empty-dir");
    let hash = metadata_hash_for_tag(temp.path(), "v1");

    delete_metadata_ref(temp.path(), &hash);
    prune_unreachable_now(temp.path());
    assert!(!metadata_blob_exists(temp.path(), &hash));

    snap_cmd(temp.path())
        .arg("doctor")
        .assert()
        .success()
        .stdout(predicate::str::contains("Snapshot metadata"))
        .stdout(predicate::str::contains("Missing metadata blob"))
        .stdout(predicate::str::contains("Problems were found"));
}

#[test]
fn doctor_repair_pins_unpinned_existing_metadata() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    init_snap_repo(temp.path());
    create_snapshot_with_empty_dir(temp.path(), "v1", "empty-dir");
    let hash = metadata_hash_for_tag(temp.path(), "v1");

    delete_metadata_ref(temp.path(), &hash);
    assert!(metadata_blob_exists(temp.path(), &hash));
    assert!(!metadata_ref_exists(temp.path(), &hash));

    snap_cmd(temp.path())
        .args(["doctor", "--repair"])
        .write_stdin("y\n")
        .assert()
        .success()
        .stdout(predicate::str::contains("Pinned metadata refs"));

    assert!(metadata_ref_exists(temp.path(), &hash));
}

#[test]
fn doctor_repair_regenerates_missing_active_metadata() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    init_snap_repo(temp.path());
    create_snapshot_with_empty_dir(temp.path(), "v1", "empty-dir");
    let hash = metadata_hash_for_tag(temp.path(), "v1");

    delete_metadata_ref(temp.path(), &hash);
    prune_unreachable_now(temp.path());
    assert!(!metadata_blob_exists(temp.path(), &hash));

    snap_cmd(temp.path())
        .args(["doctor", "--repair"])
        .write_stdin("y\n")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Repaired active snapshot metadata",
        ));

    let repaired_hash = metadata_hash_for_tag(temp.path(), "v1");
    assert_eq!(hash, repaired_hash);
    assert!(metadata_blob_exists(temp.path(), &hash));
    assert!(metadata_ref_exists(temp.path(), &hash));
}

#[test]
fn doctor_repair_reports_non_active_missing_metadata_without_guessing() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    init_snap_repo(temp.path());
    create_snapshot_with_empty_dir(temp.path(), "v1", "empty-dir");
    let hash = metadata_hash_for_tag(temp.path(), "v1");
    fs::remove_dir_all(temp.path().join("empty-dir")).expect("remove metadata source");
    create_snapshot(temp.path(), "v2", "file.txt", "two");

    delete_metadata_ref(temp.path(), &hash);
    prune_unreachable_now(temp.path());

    snap_cmd(temp.path())
        .args(["doctor", "--repair"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "No safe automatic repair remains for historical metadata loss",
        ));

    assert!(!metadata_blob_exists(temp.path(), &hash));
}

#[test]
fn doctor_reports_historical_missing_metadata_as_unrepairable_warning() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    init_snap_repo(temp.path());
    create_snapshot_with_empty_dir(temp.path(), "v1", "empty-dir");
    let hash = metadata_hash_for_tag(temp.path(), "v1");
    fs::remove_dir_all(temp.path().join("empty-dir")).expect("remove metadata source");
    create_snapshot(temp.path(), "v2", "file.txt", "two");

    delete_metadata_ref(temp.path(), &hash);
    prune_unreachable_now(temp.path());

    snap_cmd(temp.path())
        .arg("doctor")
        .assert()
        .success()
        .stdout(predicate::str::contains("Warnings were found"))
        .stdout(predicate::str::contains("historical invalid"))
        .stdout(predicate::str::contains(
            "cannot be reconstructed automatically",
        ))
        .stdout(predicate::str::contains("Run `snap doctor --repair`").not());

    snap_cmd(temp.path())
        .args(["doctor", "--repair"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "No safe automatic repair remains for historical metadata loss",
        ));
}

#[test]
fn doctor_accept_metadata_loss_rewrites_historical_tags_cleanly() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    init_snap_repo(temp.path());
    create_snapshot_with_empty_dir(temp.path(), "v1", "empty-dir");
    let hash = metadata_hash_for_tag(temp.path(), "v1");
    fs::remove_dir_all(temp.path().join("empty-dir")).expect("remove metadata source");
    create_snapshot(temp.path(), "v2", "file.txt", "two");

    delete_metadata_ref(temp.path(), &hash);
    prune_unreachable_now(temp.path());

    snap_cmd(temp.path())
        .args(["doctor", "--repair", "--accept-metadata-loss"])
        .write_stdin("y\n")
        .assert()
        .success()
        .stdout(predicate::str::contains("Repair applied"))
        .stdout(predicate::str::contains(
            "Accepted historical metadata loss for: 1 snapshot tag(s)",
        ));

    assert!(metadata_hash_for_tag_optional(temp.path(), "v1").is_none());

    snap_cmd(temp.path())
        .arg("doctor")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Snapshot metadata: 0 checked, 0 active invalid, 0 historical invalid, 0 unpinned",
        ))
        .stdout(predicate::str::contains("Git repository looks healthy"));

    snap_cmd(temp.path())
        .args(["restore", "v1"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Restore complete"));
}

#[test]
fn new_reports_actionable_error_when_active_metadata_is_missing() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    init_snap_repo(temp.path());
    create_snapshot_with_empty_dir(temp.path(), "v1", "empty-dir");
    let hash = metadata_hash_for_tag(temp.path(), "v1");

    delete_metadata_ref(temp.path(), &hash);
    prune_unreachable_now(temp.path());
    fs::write(temp.path().join("next.txt"), "next").expect("write next");

    snap_cmd(temp.path())
        .args(["new", "v2", "should fail"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Run `snap doctor --repair`"));
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
fn doctor_handles_many_plain_tags_without_windows_command_line_overflow() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    init_snap_repo(temp.path());
    create_snapshot(temp.path(), "v1", "file.txt", "one");
    create_many_lightweight_tags(temp.path(), 950);

    snap_cmd(temp.path())
        .arg("doctor")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Snapshot tags: 1 checked, 0 invalid",
        ));
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
fn help_lists_friendly_git_workflow_commands() {
    let temp = assert_fs::TempDir::new().expect("tempdir");

    snap_cmd(temp.path())
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("status"))
        .stdout(predicate::str::contains("save"))
        .stdout(predicate::str::contains("sync"))
        .stdout(predicate::str::contains("history"))
        .stdout(predicate::str::contains("branch"))
        .stdout(predicate::str::contains("remote"))
        .stdout(predicate::str::contains("release"))
        .stdout(predicate::str::contains("update-repo"))
        .stdout(predicate::str::contains("setup-repo"))
        .stdout(predicate::str::contains("make-public"))
        .stdout(predicate::str::contains("make-private"))
        .stdout(predicate::str::contains("delete-repo"))
        .stdout(predicate::str::contains("examples"))
        .stdout(predicate::str::contains("Daily workflow"))
        .stdout(predicate::str::contains("Remote/GitHub"));

    snap_cmd(temp.path())
        .args(["history", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("snap history 50"))
        .stdout(predicate::str::contains("snap history all"))
        .stdout(predicate::str::contains("Defaults to 20"));

    snap_cmd(temp.path())
        .args(["remote", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("create"))
        .stdout(predicate::str::contains("visibility"))
        .stdout(predicate::str::contains("delete"))
        .stdout(predicate::str::contains("set-url"))
        .stdout(predicate::str::contains("gh auth login"));

    snap_cmd(temp.path())
        .args(["remote", "visibility", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("public"))
        .stdout(predicate::str::contains("private"));

    snap_cmd(temp.path())
        .args(["branch", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("new"))
        .stdout(predicate::str::contains("switch"))
        .stdout(predicate::str::contains("delete"))
        .stdout(predicate::str::contains("merge"));

    snap_cmd(temp.path())
        .args(["branch", "delete", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("--force"));

    snap_cmd(temp.path())
        .args(["release", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("windows"))
        .stdout(predicate::str::contains("linux"))
        .stdout(predicate::str::contains("macos"))
        .stdout(predicate::str::contains("all"))
        .stdout(predicate::str::contains("upload"))
        .stdout(predicate::str::contains("list"));

    snap_cmd(temp.path())
        .args(["release", "upload", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("--repo"))
        .stdout(predicate::str::contains("--publish"))
        .stdout(predicate::str::contains("--clobber"));

    snap_cmd(temp.path())
        .args(["release", "list", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("--repo"))
        .stdout(predicate::str::contains("Defaults to 10"));
}

#[test]
fn examples_prints_practical_workflows_without_repo() {
    let temp = assert_fs::TempDir::new().expect("tempdir");

    snap_cmd(temp.path())
        .arg("examples")
        .assert()
        .success()
        .stdout(predicate::str::contains("Snap examples"))
        .stdout(predicate::str::contains("snap history"))
        .stdout(predicate::str::contains("snap update-repo"))
        .stdout(predicate::str::contains("snap setup-repo"))
        .stdout(predicate::str::contains("snap branch new"))
        .stdout(predicate::str::contains("snap list --branch main"))
        .stdout(predicate::str::contains("snap list --all-branches"))
        .stdout(predicate::str::contains("snap release windows"))
        .stdout(predicate::str::contains("snap release linux"))
        .stdout(predicate::str::contains("snap release macos"))
        .stdout(predicate::str::contains("snap release upload"))
        .stdout(predicate::str::contains("snap release list"))
        .stdout(predicate::str::contains("snap delete-repo"))
        .stdout(predicate::str::contains("release-github"));
}

#[test]
fn history_shows_snapshot_decorations_and_respects_limit() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    init_snap_repo(temp.path());
    create_snapshot(temp.path(), "v1", "file.txt", "one");
    create_snapshot(temp.path(), "v2", "file.txt", "two");

    snap_cmd(temp.path())
        .arg("history")
        .assert()
        .success()
        .stdout(predicate::str::contains("[snap] Project history"))
        .stdout(predicate::str::contains("Snapshot: v2"))
        .stdout(predicate::str::contains("tag: v2"));

    snap_cmd(temp.path())
        .args(["history", "1"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Snapshot: v2"))
        .stdout(predicate::str::contains("Snapshot: v1").not());
}

#[test]
fn history_all_invalid_empty_and_dirty_tree_behaviors() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    init_snap_repo(temp.path());

    snap_cmd(temp.path())
        .arg("history")
        .assert()
        .success()
        .stdout(predicate::str::contains("No commits found yet"));

    create_snapshot(temp.path(), "v1", "file.txt", "one");
    fs::write(temp.path().join("dirty.txt"), "dirty").expect("dirty file");

    snap_cmd(temp.path())
        .args(["history", "all"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Snapshot: v1"));

    snap_cmd(temp.path())
        .args(["history", "abc"])
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "History limit must be a positive number or `all`",
        ));
}

#[test]
fn release_windows_uses_script_and_prints_expected_artifacts() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    let fake_bin = assert_fs::TempDir::new().expect("fake release bin");
    init_release_fixture(temp.path());
    let fake_powershell = create_fake_release_runtime(fake_bin.path(), "fake-powershell");
    let windows_script = temp.path().join("fake-release-windows.ps1");
    fs::write(&windows_script, "# fake windows release\n").expect("fake windows script");
    let log_path = temp.path().join("release-windows.log");

    snap_cmd(temp.path())
        .args(["release", "windows"])
        .env("SNAP_RELEASE_POWERSHELL", &fake_powershell)
        .env("SNAP_RELEASE_WINDOWS_SCRIPT", &windows_script)
        .env("SNAP_RELEASE_FAKE_LOG", &log_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("snap-v7.2.0-windows-x86_64.exe"))
        .stdout(predicate::str::contains(
            "snap-v7.2.0-windows-x86_64-setup.exe",
        ))
        .stdout(predicate::str::contains("snap-v7.2.0-windows-x86_64.msi"));

    let log = fs::read_to_string(log_path).expect("read release windows log");
    assert!(log.contains("-ExecutionPolicy Bypass -File"));
    assert!(log.contains(&windows_script.to_string_lossy().to_string()));
}

#[test]
fn release_linux_uses_script_and_prints_expected_artifacts() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    let fake_bin = assert_fs::TempDir::new().expect("fake release bin");
    init_release_fixture(temp.path());
    let fake_bash = create_fake_release_runtime(fake_bin.path(), "fake-bash");
    let linux_script = temp.path().join("fake-release-linux.sh");
    fs::write(&linux_script, "#!/usr/bin/env bash\n").expect("fake linux script");
    let log_path = temp.path().join("release-linux.log");

    snap_cmd(temp.path())
        .args(["release", "linux"])
        .env("SNAP_RELEASE_BASH", &fake_bash)
        .env("SNAP_RELEASE_LINUX_SCRIPT", &linux_script)
        .env("SNAP_RELEASE_FAKE_LOG", &log_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("snap-v7.2.0-linux-x86_64"))
        .stdout(predicate::str::contains("snap-v7.2.0-linux-x86_64.tar.gz"));

    let log = fs::read_to_string(log_path).expect("read release linux log");
    assert!(log.contains(&linux_script.to_string_lossy().to_string()));
}

#[test]
fn release_macos_uses_script_and_prints_expected_artifacts() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    let fake_bin = assert_fs::TempDir::new().expect("fake release bin");
    init_release_fixture(temp.path());
    let fake_bash = create_fake_release_runtime(fake_bin.path(), "fake-bash");
    let macos_script = temp.path().join("fake-release-macos.sh");
    fs::write(&macos_script, "#!/usr/bin/env bash\n").expect("fake macos script");
    let log_path = temp.path().join("release-macos.log");

    snap_cmd(temp.path())
        .args(["release", "macos"])
        .env("SNAP_RELEASE_BASH", &fake_bash)
        .env("SNAP_RELEASE_MACOS_SCRIPT", &macos_script)
        .env("SNAP_RELEASE_MACOS_TARGET", "macos-aarch64")
        .env("SNAP_RELEASE_FAKE_LOG", &log_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("snap-v7.2.0-macos-aarch64"))
        .stdout(predicate::str::contains("snap-v7.2.0-macos-aarch64.tar.gz"));

    let log = fs::read_to_string(log_path).expect("read release macos log");
    assert!(log.contains(&macos_script.to_string_lossy().to_string()));
}

#[test]
fn release_all_preflights_then_runs_windows_and_linux() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    let fake_bin = assert_fs::TempDir::new().expect("fake release bin");
    init_release_fixture(temp.path());
    let fake_powershell = create_fake_release_runtime(fake_bin.path(), "fake-powershell");
    let fake_bash = create_fake_release_runtime(fake_bin.path(), "fake-bash");
    let windows_script = temp.path().join("fake-release-windows.ps1");
    let linux_script = temp.path().join("fake-release-linux.sh");
    fs::write(&windows_script, "# fake windows release\n").expect("fake windows script");
    fs::write(&linux_script, "#!/usr/bin/env bash\n").expect("fake linux script");
    let log_path = temp.path().join("release-all.log");

    snap_cmd(temp.path())
        .args(["release", "all"])
        .env("SNAP_RELEASE_POWERSHELL", &fake_powershell)
        .env("SNAP_RELEASE_BASH", &fake_bash)
        .env("SNAP_RELEASE_WINDOWS_SCRIPT", &windows_script)
        .env("SNAP_RELEASE_LINUX_SCRIPT", &linux_script)
        .env("SNAP_RELEASE_FAKE_LOG", &log_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("snap-v7.2.0-windows-x86_64.exe"))
        .stdout(predicate::str::contains("snap-v7.2.0-linux-x86_64"))
        .stdout(predicate::str::contains("snap-v7.2.0-linux-x86_64.tar.gz"));

    let log = fs::read_to_string(log_path).expect("read release all log");
    let windows_index = log
        .find(&windows_script.to_string_lossy().to_string())
        .expect("windows script in log");
    let linux_index = log
        .find(&linux_script.to_string_lossy().to_string())
        .expect("linux script in log");
    assert!(windows_index < linux_index);
}

#[test]
fn release_all_missing_runtime_fails_before_running_scripts() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    let fake_bin = assert_fs::TempDir::new().expect("fake release bin");
    init_release_fixture(temp.path());
    let fake_bash = create_fake_release_runtime(fake_bin.path(), "fake-bash");
    let windows_script = temp.path().join("fake-release-windows.ps1");
    let linux_script = temp.path().join("fake-release-linux.sh");
    fs::write(&windows_script, "# fake windows release\n").expect("fake windows script");
    fs::write(&linux_script, "#!/usr/bin/env bash\n").expect("fake linux script");
    let log_path = temp.path().join("release-missing.log");

    snap_cmd(temp.path())
        .args(["release", "all"])
        .env(
            "SNAP_RELEASE_POWERSHELL",
            temp.path().join("missing-powershell"),
        )
        .env("SNAP_RELEASE_BASH", &fake_bash)
        .env("SNAP_RELEASE_WINDOWS_SCRIPT", &windows_script)
        .env("SNAP_RELEASE_LINUX_SCRIPT", &linux_script)
        .env("SNAP_RELEASE_FAKE_LOG", &log_path)
        .assert()
        .failure()
        .stderr(predicate::str::contains("Required release runtime"))
        .stderr(predicate::str::contains("snap release all"));

    assert!(!log_path.exists());
}

#[test]
fn release_upload_infers_origin_and_creates_draft_release() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    let fake_bin = assert_fs::TempDir::new().expect("fake gh bin");
    init_release_upload_fixture(temp.path());
    let fake_gh = create_fake_gh(fake_bin.path());
    let log_path = temp.path().join("release-upload-create.log");
    git(
        temp.path(),
        &[
            "remote",
            "add",
            "origin",
            "https://github.com/owner/repo.git",
        ],
    );
    let head = git(temp.path(), &["rev-parse", "HEAD"]).trim().to_string();

    snap_cmd_with_path(temp.path(), fake_bin.path())
        .args(["release", "upload"])
        .env("SNAP_GH", &fake_gh)
        .env("GH_FAKE_LOG", &log_path)
        .env("GH_FAKE_REMOTE_URL", "unused")
        .assert()
        .success()
        .stdout(predicate::str::contains("GitHub release upload"))
        .stdout(predicate::str::contains("Uploaded 9 release asset"));

    let log = fs::read_to_string(log_path).expect("read release upload log");
    assert!(log.contains("auth status"));
    assert!(log.contains("release view v7.2.0 -R owner/repo"));
    assert!(log.contains("release create v7.2.0"));
    assert!(log.contains("snap-v7.2.0-windows-x86_64.exe"));
    assert!(log.contains("snap-v7.2.0-windows-x86_64-setup.exe"));
    assert!(log.contains("snap-v7.2.0-windows-x86_64.msi"));
    assert!(log.contains("snap-v7.2.0-linux-x86_64"));
    assert!(log.contains("snap-v7.2.0-linux-x86_64.tar.gz"));
    assert!(log.contains("snap-v7.2.0-macos-aarch64"));
    assert!(log.contains("snap-v7.2.0-macos-aarch64.tar.gz"));
    assert!(log.contains("snap-v7.2.0-macos-x86_64"));
    assert!(log.contains("snap-v7.2.0-macos-x86_64.tar.gz"));
    assert!(log.contains("--draft"));
    assert!(log.contains("--title"));
    assert!(log.contains("snap v7.2.0"));
    assert!(log.contains("--notes"));
    assert!(log.contains("snap v7.2.0 release"));
    assert!(log.contains("-R owner/repo"));
    assert!(log.contains(&format!("--target {}", head)));
}

#[test]
fn release_upload_publish_uses_explicit_repo_without_draft() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    let fake_bin = assert_fs::TempDir::new().expect("fake gh bin");
    init_release_upload_fixture(temp.path());
    let fake_gh = create_fake_gh(fake_bin.path());
    let log_path = temp.path().join("release-upload-publish.log");

    snap_cmd_with_path(temp.path(), fake_bin.path())
        .args(["release", "upload", "--repo", "owner/other", "--publish"])
        .env("SNAP_GH", &fake_gh)
        .env("GH_FAKE_LOG", &log_path)
        .env("GH_FAKE_REMOTE_URL", "unused")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Creating published GitHub Release",
        ));

    let log = fs::read_to_string(log_path).expect("read release publish log");
    assert!(log.contains("release create v7.2.0"));
    assert!(log.contains("-R owner/other"));
    assert!(!log.contains("--draft"));
}

#[test]
fn release_upload_existing_release_requires_clobber() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    let fake_bin = assert_fs::TempDir::new().expect("fake gh bin");
    init_release_upload_fixture(temp.path());
    let fake_gh = create_fake_gh(fake_bin.path());
    let log_path = temp.path().join("release-upload-existing.log");

    snap_cmd_with_path(temp.path(), fake_bin.path())
        .args(["release", "upload", "--repo", "owner/repo"])
        .env("SNAP_GH", &fake_gh)
        .env("GH_FAKE_LOG", &log_path)
        .env("GH_FAKE_REMOTE_URL", "unused")
        .env("GH_FAKE_RELEASE_EXISTS", "1")
        .assert()
        .failure()
        .stderr(predicate::str::contains("already exists"))
        .stderr(predicate::str::contains("--clobber"));

    let log = fs::read_to_string(log_path).expect("read release existing log");
    assert!(log.contains("auth status"));
    assert!(log.contains("release view v7.2.0 -R owner/repo"));
    assert!(!log.contains("release create"));
    assert!(!log.contains("release upload"));
}

#[test]
fn release_upload_clobber_uploads_to_existing_release() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    let fake_bin = assert_fs::TempDir::new().expect("fake gh bin");
    init_release_upload_fixture(temp.path());
    let fake_gh = create_fake_gh(fake_bin.path());
    let log_path = temp.path().join("release-upload-clobber.log");

    snap_cmd_with_path(temp.path(), fake_bin.path())
        .args(["release", "upload", "--repo", "owner/repo", "--clobber"])
        .env("SNAP_GH", &fake_gh)
        .env("GH_FAKE_LOG", &log_path)
        .env("GH_FAKE_REMOTE_URL", "unused")
        .env("GH_FAKE_RELEASE_EXISTS", "1")
        .assert()
        .success()
        .stdout(predicate::str::contains("Existing release found"))
        .stdout(predicate::str::contains("Uploaded 9 release asset"));

    let log = fs::read_to_string(log_path).expect("read release clobber log");
    assert!(log.contains("release upload v7.2.0"));
    assert!(log.contains("--clobber"));
    assert!(log.contains("-R owner/repo"));
    assert!(!log.contains("release create"));
}

#[test]
fn release_upload_fails_before_github_when_artifacts_are_missing() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    let fake_bin = assert_fs::TempDir::new().expect("fake gh bin");
    init_release_fixture(temp.path());
    create_snapshot(temp.path(), "v-release", "release.txt", "release");
    let fake_gh = create_fake_gh(fake_bin.path());
    let log_path = temp.path().join("release-upload-missing-artifact.log");

    snap_cmd_with_path(temp.path(), fake_bin.path())
        .args(["release", "upload", "--repo", "owner/repo"])
        .env("SNAP_GH", &fake_gh)
        .env("GH_FAKE_LOG", &log_path)
        .env("GH_FAKE_REMOTE_URL", "unused")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Missing release artifact"));

    assert!(!log_path.exists());
}

#[test]
fn release_upload_requires_repo_when_origin_is_not_github() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    let fake_bin = assert_fs::TempDir::new().expect("fake gh bin");
    init_release_upload_fixture(temp.path());
    let fake_gh = create_fake_gh(fake_bin.path());
    let log_path = temp.path().join("release-upload-no-origin.log");

    snap_cmd_with_path(temp.path(), fake_bin.path())
        .args(["release", "upload"])
        .env("SNAP_GH", &fake_gh)
        .env("GH_FAKE_LOG", &log_path)
        .env("GH_FAKE_REMOTE_URL", "unused")
        .assert()
        .failure()
        .stderr(predicate::str::contains("--repo owner/repo"));

    assert!(!log_path.exists());
}

#[test]
fn release_upload_rejects_invalid_repo_before_github() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    let fake_bin = assert_fs::TempDir::new().expect("fake gh bin");
    init_release_upload_fixture(temp.path());
    let fake_gh = create_fake_gh(fake_bin.path());
    let log_path = temp.path().join("release-upload-invalid-repo.log");

    snap_cmd_with_path(temp.path(), fake_bin.path())
        .args(["release", "upload", "--repo", "owner-only"])
        .env("SNAP_GH", &fake_gh)
        .env("GH_FAKE_LOG", &log_path)
        .env("GH_FAKE_REMOTE_URL", "unused")
        .assert()
        .failure()
        .stderr(predicate::str::contains("owner/repo format"));

    assert!(!log_path.exists());
}

#[test]
fn release_upload_requires_head_commit_before_github() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    let fake_bin = assert_fs::TempDir::new().expect("fake gh bin");
    init_release_fixture(temp.path());
    create_release_artifacts(temp.path());
    let fake_gh = create_fake_gh(fake_bin.path());
    let log_path = temp.path().join("release-upload-no-head.log");

    snap_cmd_with_path(temp.path(), fake_bin.path())
        .args(["release", "upload", "--repo", "owner/repo"])
        .env("SNAP_GH", &fake_gh)
        .env("GH_FAKE_LOG", &log_path)
        .env("GH_FAKE_REMOTE_URL", "unused")
        .assert()
        .failure()
        .stderr(predicate::str::contains("No HEAD commit found"));

    assert!(!log_path.exists());
}

#[test]
fn release_upload_reports_github_auth_failure() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    let fake_bin = assert_fs::TempDir::new().expect("fake gh bin");
    init_release_upload_fixture(temp.path());
    let fake_gh = create_fake_gh(fake_bin.path());
    let log_path = temp.path().join("release-upload-auth.log");

    snap_cmd_with_path(temp.path(), fake_bin.path())
        .args(["release", "upload", "--repo", "owner/repo"])
        .env("SNAP_GH", &fake_gh)
        .env("GH_FAKE_LOG", &log_path)
        .env("GH_FAKE_REMOTE_URL", "unused")
        .env("GH_FAKE_AUTH_FAIL", "1")
        .assert()
        .failure()
        .stderr(predicate::str::contains("gh auth login"));

    let log = fs::read_to_string(log_path).expect("read release auth log");
    assert!(log.contains("auth status"));
    assert!(!log.contains("release view"));
}

#[test]
fn release_list_infers_origin_and_prints_github_releases() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    let fake_bin = assert_fs::TempDir::new().expect("fake gh bin");
    init_snap_repo(temp.path());
    let fake_gh = create_fake_gh(fake_bin.path());
    let log_path = temp.path().join("release-list.log");
    git(
        temp.path(),
        &[
            "remote",
            "add",
            "origin",
            "https://github.com/owner/repo.git",
        ],
    );

    snap_cmd_with_path(temp.path(), fake_bin.path())
        .args(["release", "list"])
        .env("SNAP_GH", &fake_gh)
        .env("GH_FAKE_LOG", &log_path)
        .env("GH_FAKE_REMOTE_URL", "unused")
        .assert()
        .success()
        .stdout(predicate::str::contains("[snap] GitHub releases"))
        .stdout(predicate::str::contains("Repository: owner/repo"))
        .stdout(predicate::str::contains("Limit: 10"))
        .stdout(predicate::str::contains("v7.2.0"))
        .stdout(predicate::str::contains("draft"))
        .stdout(predicate::str::contains("prerelease"))
        .stdout(predicate::str::contains("published"))
        .stdout(predicate::str::contains("yes"));

    let log = fs::read_to_string(log_path).expect("read release list log");
    assert!(log.contains("auth status"));
    assert!(log.contains("release list --limit 10"));
    assert!(log.contains("tagName,name,isDraft,isPrerelease,isLatest,publishedAt"));
    assert!(log.contains("-R owner/repo"));
}

#[test]
fn release_list_uses_explicit_repo_and_limit() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    let fake_bin = assert_fs::TempDir::new().expect("fake gh bin");
    init_snap_repo(temp.path());
    let fake_gh = create_fake_gh(fake_bin.path());
    let log_path = temp.path().join("release-list-explicit.log");

    snap_cmd_with_path(temp.path(), fake_bin.path())
        .args(["release", "list", "20", "--repo", "owner/other"])
        .env("SNAP_GH", &fake_gh)
        .env("GH_FAKE_LOG", &log_path)
        .env("GH_FAKE_REMOTE_URL", "unused")
        .assert()
        .success()
        .stdout(predicate::str::contains("Repository: owner/other"))
        .stdout(predicate::str::contains("Limit: 20"));

    let log = fs::read_to_string(log_path).expect("read release list explicit log");
    assert!(log.contains("release list --limit 20"));
    assert!(log.contains("-R owner/other"));
}

#[test]
fn release_list_empty_output_succeeds_with_clear_message() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    let fake_bin = assert_fs::TempDir::new().expect("fake gh bin");
    init_snap_repo(temp.path());
    let fake_gh = create_fake_gh(fake_bin.path());
    let log_path = temp.path().join("release-list-empty.log");

    snap_cmd_with_path(temp.path(), fake_bin.path())
        .args(["release", "list", "--repo", "owner/repo"])
        .env("SNAP_GH", &fake_gh)
        .env("GH_FAKE_LOG", &log_path)
        .env("GH_FAKE_REMOTE_URL", "unused")
        .env("GH_FAKE_RELEASE_LIST_EMPTY", "1")
        .assert()
        .success()
        .stdout(predicate::str::contains("No GitHub releases found"));

    let log = fs::read_to_string(log_path).expect("read empty release list log");
    assert!(log.contains("release list --limit 10"));
}

#[test]
fn release_list_rejects_invalid_limits_and_repo_before_github() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    let fake_bin = assert_fs::TempDir::new().expect("fake gh bin");
    init_snap_repo(temp.path());
    let fake_gh = create_fake_gh(fake_bin.path());
    let log_path = temp.path().join("release-list-invalid.log");

    snap_cmd_with_path(temp.path(), fake_bin.path())
        .args(["release", "list", "0", "--repo", "owner/repo"])
        .env("SNAP_GH", &fake_gh)
        .env("GH_FAKE_LOG", &log_path)
        .env("GH_FAKE_REMOTE_URL", "unused")
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "Release list limit must be a positive number",
        ));

    snap_cmd_with_path(temp.path(), fake_bin.path())
        .args(["release", "list", "abc", "--repo", "owner/repo"])
        .env("SNAP_GH", &fake_gh)
        .env("GH_FAKE_LOG", &log_path)
        .env("GH_FAKE_REMOTE_URL", "unused")
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "Release list limit must be a positive number",
        ));

    snap_cmd_with_path(temp.path(), fake_bin.path())
        .args(["release", "list", "--repo", "owner-only"])
        .env("SNAP_GH", &fake_gh)
        .env("GH_FAKE_LOG", &log_path)
        .env("GH_FAKE_REMOTE_URL", "unused")
        .assert()
        .failure()
        .stderr(predicate::str::contains("owner/repo format"));

    assert!(!log_path.exists());
}

#[test]
fn release_list_requires_repo_when_origin_is_not_github() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    let fake_bin = assert_fs::TempDir::new().expect("fake gh bin");
    init_snap_repo(temp.path());
    let fake_gh = create_fake_gh(fake_bin.path());
    let log_path = temp.path().join("release-list-no-origin.log");

    snap_cmd_with_path(temp.path(), fake_bin.path())
        .args(["release", "list"])
        .env("SNAP_GH", &fake_gh)
        .env("GH_FAKE_LOG", &log_path)
        .env("GH_FAKE_REMOTE_URL", "unused")
        .assert()
        .failure()
        .stderr(predicate::str::contains("--repo owner/repo"));

    assert!(!log_path.exists());
}

#[test]
fn release_list_reports_github_auth_failure() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    let fake_bin = assert_fs::TempDir::new().expect("fake gh bin");
    init_snap_repo(temp.path());
    let fake_gh = create_fake_gh(fake_bin.path());
    let log_path = temp.path().join("release-list-auth.log");

    snap_cmd_with_path(temp.path(), fake_bin.path())
        .args(["release", "list", "--repo", "owner/repo"])
        .env("SNAP_GH", &fake_gh)
        .env("GH_FAKE_LOG", &log_path)
        .env("GH_FAKE_REMOTE_URL", "unused")
        .env("GH_FAKE_AUTH_FAIL", "1")
        .assert()
        .failure()
        .stderr(predicate::str::contains("gh auth login"));

    let log = fs::read_to_string(log_path).expect("read release list auth log");
    assert!(log.contains("auth status"));
    assert!(!log.contains("release list"));
}

#[test]
fn top_help_includes_branch_snapshot_discovery() {
    let temp = assert_fs::TempDir::new().expect("tempdir");

    snap_cmd(temp.path())
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("snap list --all-branches"))
        .stdout(predicate::str::contains("snap list --branch main"))
        .stdout(predicate::str::contains("snap history --all-branches"))
        .stdout(predicate::str::contains("snap list --help"))
        .stdout(predicate::str::contains("snap history --help"));
}

#[test]
fn list_help_includes_branch_filters_and_rejects_conflicting_flags() {
    let temp = assert_fs::TempDir::new().expect("tempdir");

    snap_cmd(temp.path())
        .args(["list", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("--branch"))
        .stdout(predicate::str::contains("--all-branches"))
        .stdout(predicate::str::contains("reachability"))
        .stdout(predicate::str::contains(
            "-                   snapshot tag exists",
        ));

    snap_cmd(temp.path())
        .args(["list", "--branch", "main", "--all-branches"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("cannot be used with"));
}

#[test]
fn history_help_includes_branch_graph_options_and_rejects_conflicting_flags() {
    let temp = assert_fs::TempDir::new().expect("tempdir");

    snap_cmd(temp.path())
        .args(["history", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("--branch"))
        .stdout(predicate::str::contains("--all-branches"))
        .stdout(predicate::str::contains("read-only"))
        .stdout(predicate::str::contains("snap history --branch main"))
        .stdout(predicate::str::contains("snap history --all-branches"));

    snap_cmd(temp.path())
        .args(["history", "--branch", "main", "--all-branches"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("cannot be used with"));
}

#[test]
fn examples_include_branch_snapshot_discovery_commands() {
    let temp = assert_fs::TempDir::new().expect("tempdir");

    snap_cmd(temp.path())
        .arg("examples")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Understand snapshots across branches",
        ))
        .stdout(predicate::str::contains("snap list --all-branches"))
        .stdout(predicate::str::contains("snap list --branch master"))
        .stdout(predicate::str::contains("snap history --all-branches"));
}

#[test]
fn list_branch_filters_snapshots_by_local_branch_reachability() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    init_snap_repo(temp.path());
    create_snapshot(temp.path(), "v-main", "main.txt", "main");
    let base = current_branch(temp.path());
    snap_cmd(temp.path())
        .args(["branch", "new", "feature-x"])
        .assert()
        .success();
    create_snapshot(temp.path(), "v-feature", "feature.txt", "feature");

    snap_cmd(temp.path())
        .args(["list", "--branch", &base])
        .assert()
        .success()
        .stdout(predicate::str::contains(format!("Branch: {}", base)))
        .stdout(predicate::str::contains("v-main"))
        .stdout(predicate::str::contains("v-feature").not());

    snap_cmd(temp.path())
        .args(["list", "--branch", "feature-x"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Branch: feature-x"))
        .stdout(predicate::str::contains("v-main"))
        .stdout(predicate::str::contains("v-feature"));
}

#[test]
fn list_all_branches_adds_branch_column_and_marks_shared_snapshots() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    init_snap_repo(temp.path());
    create_snapshot(temp.path(), "v-main", "main.txt", "main");
    snap_cmd(temp.path())
        .args(["branch", "new", "feature-x"])
        .assert()
        .success();
    create_snapshot(temp.path(), "v-feature", "feature.txt", "feature");

    snap_cmd(temp.path())
        .args(["list", "--all-branches"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Branch"))
        .stdout(predicate::str::contains("v-main"))
        .stdout(predicate::str::contains("feature-x (shared)"))
        .stdout(predicate::str::contains("v-feature"))
        .stdout(predicate::str::contains("feature-x"));
}

#[test]
fn list_all_branches_marks_unreachable_tag_with_placeholder() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    init_snap_repo(temp.path());
    create_snapshot(temp.path(), "v-main", "main.txt", "main");
    let base = current_branch(temp.path());
    snap_cmd(temp.path())
        .args(["branch", "new", "orphan-branch"])
        .assert()
        .success();
    create_snapshot(temp.path(), "v-orphan", "orphan.txt", "orphan");
    snap_cmd(temp.path())
        .args(["branch", "switch", &base])
        .assert()
        .success();
    git(temp.path(), &["branch", "-D", "orphan-branch"]);

    snap_cmd(temp.path())
        .args(["list", "--all-branches"])
        .assert()
        .success()
        .stdout(predicate::str::contains("v-orphan"))
        .stdout(predicate::str::contains(
            "- = no local branch reaches this snapshot",
        ));
}

#[test]
fn list_branch_missing_fails_clearly_and_limit_applies_after_filtering() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    init_snap_repo(temp.path());
    create_snapshot(temp.path(), "v-main", "main.txt", "main");
    snap_cmd(temp.path())
        .args(["branch", "new", "feature-x"])
        .assert()
        .success();
    create_snapshot(temp.path(), "v-feature", "feature.txt", "feature");

    snap_cmd(temp.path())
        .args(["list", "--branch", "missing"])
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "Local branch 'missing' does not exist",
        ));

    snap_cmd(temp.path())
        .args(["list", "1", "--branch", "feature-x"])
        .assert()
        .success()
        .stdout(predicate::str::contains("v-feature"))
        .stdout(predicate::str::contains("v-main").not())
        .stdout(predicate::str::contains("..."));
}

#[test]
fn history_branch_missing_fails_clearly() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    init_snap_repo(temp.path());
    create_snapshot(temp.path(), "v-main", "main.txt", "main");

    snap_cmd(temp.path())
        .args(["history", "--branch", "missing"])
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "Local branch 'missing' does not exist",
        ));
}

#[test]
fn history_branch_filters_and_all_branches_graphs_snapshots() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    init_snap_repo(temp.path());
    create_snapshot(temp.path(), "v-base", "base.txt", "base");
    let base = current_branch(temp.path());
    snap_cmd(temp.path())
        .args(["branch", "new", "feature-x"])
        .assert()
        .success();
    create_snapshot(temp.path(), "v-feature", "feature.txt", "feature");
    snap_cmd(temp.path())
        .args(["branch", "switch", &base])
        .assert()
        .success();
    create_snapshot(temp.path(), "v-main", "main.txt", "main");

    snap_cmd(temp.path())
        .args(["history", "--branch", &base])
        .assert()
        .success()
        .stdout(predicate::str::contains(format!("Branch: {}", base)))
        .stdout(predicate::str::contains("v-base"))
        .stdout(predicate::str::contains("v-main"))
        .stdout(predicate::str::contains("v-feature").not());

    snap_cmd(temp.path())
        .args(["history", "--all-branches"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Scope: all branches"))
        .stdout(predicate::str::contains("v-base"))
        .stdout(predicate::str::contains("v-main"))
        .stdout(predicate::str::contains("v-feature"));
}

#[test]
fn remote_create_rejects_public_and_private_together() {
    let temp = assert_fs::TempDir::new().expect("tempdir");

    snap_cmd(temp.path())
        .args(["remote", "create", "owner/repo", "--public", "--private"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("cannot be used with"));
}

#[test]
fn status_reports_branch_remote_and_worktree_summary() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    init_snap_repo(temp.path());
    create_snapshot(temp.path(), "v1", "file.txt", "one");
    fs::write(temp.path().join("dirty.txt"), "dirty").expect("dirty file");

    snap_cmd(temp.path())
        .arg("status")
        .assert()
        .success()
        .stdout(predicate::str::contains("Project status"))
        .stdout(predicate::str::contains("Branch:"))
        .stdout(predicate::str::contains("Remote origin: not configured"))
        .stdout(predicate::str::contains("untracked"))
        .stdout(predicate::str::contains("Active snapshot: v1"));
}

#[test]
fn branch_new_list_and_switch_workflow() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    init_snap_repo(temp.path());
    create_snapshot(temp.path(), "v1", "file.txt", "one");
    let base = current_branch(temp.path());

    snap_cmd(temp.path())
        .args(["branch", "new", "feature-x"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Now on branch 'feature-x'"));
    assert_eq!(current_branch(temp.path()), "feature-x");

    snap_cmd(temp.path())
        .args(["branch", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Branches"))
        .stdout(predicate::str::contains("*"))
        .stdout(predicate::str::contains("feature-x"))
        .stdout(predicate::str::contains(&base));

    snap_cmd(temp.path())
        .args(["branch", "switch", &base])
        .assert()
        .success()
        .stdout(predicate::str::contains(format!(
            "Now on branch '{}'",
            base
        )));
    assert_eq!(current_branch(temp.path()), base);

    snap_cmd(temp.path())
        .args(["branch", "switch", &base])
        .assert()
        .success()
        .stdout(predicate::str::contains("Already on branch"));
}

#[test]
fn branch_switch_missing_and_invalid_new_fail_clearly() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    init_snap_repo(temp.path());
    create_snapshot(temp.path(), "v1", "file.txt", "one");

    snap_cmd(temp.path())
        .args(["branch", "switch", "missing"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("does not exist"));

    snap_cmd(temp.path())
        .args(["branch", "new", "invalid/name.."])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Invalid branch name"));
}

#[test]
fn branch_mutating_commands_refuse_dirty_worktree() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    init_snap_repo(temp.path());
    create_snapshot(temp.path(), "v1", "file.txt", "one");
    let base = current_branch(temp.path());
    git(temp.path(), &["switch", "-c", "feature-dirty"]);
    git(temp.path(), &["switch", &base]);
    fs::write(temp.path().join("dirty.txt"), "dirty").expect("dirty file");

    for args in [
        vec!["branch", "new", "other"],
        vec!["branch", "switch", "feature-dirty"],
        vec!["branch", "delete", "feature-dirty"],
        vec!["branch", "merge", "feature-dirty"],
    ] {
        snap_cmd(temp.path())
            .args(args)
            .assert()
            .failure()
            .stderr(predicate::str::contains("working tree has local changes"));
    }
}

#[test]
fn branch_switch_refuses_git_operation_in_progress() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    init_snap_repo(temp.path());
    create_snapshot(temp.path(), "v1", "file.txt", "one");
    let base = current_branch(temp.path());
    git(temp.path(), &["switch", "-c", "feature-op"]);
    git(temp.path(), &["switch", &base]);
    fs::write(temp.path().join(".git").join("MERGE_HEAD"), "merge").expect("merge marker");

    snap_cmd(temp.path())
        .args(["branch", "switch", "feature-op"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("merge in progress"));
}

#[test]
fn branch_delete_refuses_current_and_deletes_merged_branch() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    init_snap_repo(temp.path());
    create_snapshot(temp.path(), "v1", "file.txt", "one");
    let base = current_branch(temp.path());
    snap_cmd(temp.path())
        .args(["branch", "new", "feature-delete"])
        .assert()
        .success();
    snap_cmd(temp.path())
        .args(["branch", "switch", &base])
        .assert()
        .success();

    snap_cmd(temp.path())
        .args(["branch", "delete", &base])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Cannot delete the current branch"));

    snap_cmd(temp.path())
        .args(["branch", "delete", "feature-delete"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Branch 'feature-delete' deleted"));
    assert!(git(temp.path(), &["branch", "--list", "feature-delete"])
        .trim()
        .is_empty());
}

#[test]
fn branch_delete_force_requires_exact_confirmation() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    init_snap_repo(temp.path());
    create_snapshot(temp.path(), "v1", "file.txt", "one");
    let base = current_branch(temp.path());
    git(temp.path(), &["switch", "-c", "feature-force"]);
    fs::write(temp.path().join("force.txt"), "force").expect("write force file");
    git(temp.path(), &["add", "force.txt"]);
    git(temp.path(), &["commit", "-m", "force branch commit"]);
    git(temp.path(), &["switch", &base]);

    snap_cmd(temp.path())
        .args(["branch", "delete", "feature-force", "--force"])
        .write_stdin("wrong\n")
        .assert()
        .success()
        .stdout(predicate::str::contains("Branch deletion cancelled"));
    assert!(!git(temp.path(), &["branch", "--list", "feature-force"])
        .trim()
        .is_empty());

    snap_cmd(temp.path())
        .args(["branch", "delete", "feature-force", "--force"])
        .write_stdin("feature-force\n")
        .assert()
        .success()
        .stdout(predicate::str::contains("with force"))
        .stdout(predicate::str::contains("Branch 'feature-force' deleted"));
    assert!(git(temp.path(), &["branch", "--list", "feature-force"])
        .trim()
        .is_empty());
}

#[test]
fn branch_merge_merges_clean_branch() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    init_snap_repo(temp.path());
    create_snapshot(temp.path(), "v1", "file.txt", "one");
    let base = current_branch(temp.path());
    git(temp.path(), &["switch", "-c", "feature-merge"]);
    fs::write(temp.path().join("merged.txt"), "merged").expect("write merged file");
    git(temp.path(), &["add", "merged.txt"]);
    git(temp.path(), &["commit", "-m", "feature merge commit"]);
    git(temp.path(), &["switch", &base]);

    snap_cmd(temp.path())
        .args(["branch", "merge", "feature-merge"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Branch 'feature-merge' merged"));
    assert!(temp.path().join("merged.txt").exists());
}

#[test]
fn branch_merge_conflict_reports_resolution_hint() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    init_snap_repo(temp.path());
    create_snapshot(temp.path(), "v1", "file.txt", "one");
    let base = current_branch(temp.path());
    git(temp.path(), &["switch", "-c", "feature-conflict"]);
    fs::write(temp.path().join("file.txt"), "feature").expect("feature change");
    git(temp.path(), &["add", "file.txt"]);
    git(temp.path(), &["commit", "-m", "feature conflict commit"]);
    git(temp.path(), &["switch", &base]);
    fs::write(temp.path().join("file.txt"), "base").expect("base change");
    git(temp.path(), &["add", "file.txt"]);
    git(temp.path(), &["commit", "-m", "base conflict commit"]);

    snap_cmd(temp.path())
        .args(["branch", "merge", "feature-conflict"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Resolve conflicts with Git"));
}

#[test]
fn save_creates_normal_git_commit_without_snapshot_tag() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    init_snap_repo(temp.path());
    fs::write(temp.path().join("file.txt"), "one").expect("write file");

    snap_cmd(temp.path())
        .args(["save", "normal", "commit"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Changes saved"));

    let message = git(temp.path(), &["log", "-1", "--pretty=%B"]);
    assert!(message.contains("normal commit"));
    let tags = git(temp.path(), &["tag", "--list"]);
    assert!(tags.trim().is_empty());
}

#[test]
fn save_without_changes_succeeds_with_clear_message() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    init_snap_repo(temp.path());
    create_snapshot(temp.path(), "v1", "file.txt", "one");

    snap_cmd(temp.path())
        .args(["save", "nothing", "changed"])
        .assert()
        .success()
        .stdout(predicate::str::contains("No changes to commit"));
}

#[test]
fn save_refuses_detached_head() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    init_snap_repo(temp.path());
    create_snapshot(temp.path(), "v1", "file.txt", "one");
    git(temp.path(), &["checkout", "--detach", "HEAD"]);
    fs::write(temp.path().join("file.txt"), "two").expect("modify file");

    snap_cmd(temp.path())
        .args(["save", "should", "fail"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("HEAD is detached"));
}

#[test]
fn save_refuses_git_operation_in_progress() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    init_snap_repo(temp.path());
    create_snapshot(temp.path(), "v1", "file.txt", "one");
    fs::write(temp.path().join(".git").join("MERGE_HEAD"), "merge").expect("merge marker");
    fs::write(temp.path().join("file.txt"), "two").expect("modify file");

    snap_cmd(temp.path())
        .args(["save", "should", "fail"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("merge in progress"));
}

#[test]
fn push_syncs_branch_tags_and_snap_metadata_refs_to_bare_remote() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    let remote = assert_fs::TempDir::new().expect("remote");
    init_snap_repo(temp.path());
    create_snapshot_with_empty_dir(temp.path(), "v1", "empty-dir");
    let metadata_hash = metadata_hash_for_tag(temp.path(), "v1");
    init_bare_repo(remote.path());
    let remote_url = remote.path().to_string_lossy().to_string();
    git(temp.path(), &["remote", "add", "origin", &remote_url]);

    snap_cmd(temp.path())
        .arg("push")
        .assert()
        .success()
        .stdout(predicate::str::contains("Push complete"))
        .stdout(predicate::str::contains("Snap metadata refs: 1"));

    let branch = git(temp.path(), &["symbolic-ref", "--short", "HEAD"])
        .trim()
        .to_string();
    assert!(git_success(
        remote.path(),
        &["show-ref", "--verify", &format!("refs/heads/{}", branch)]
    ));
    assert!(git_success(
        remote.path(),
        &["show-ref", "--verify", "refs/tags/v1"]
    ));
    assert!(metadata_ref_exists(remote.path(), &metadata_hash));
}

#[test]
fn pull_fetches_snap_metadata_refs_from_bare_remote() {
    let source = assert_fs::TempDir::new().expect("source");
    let remote = assert_fs::TempDir::new().expect("remote");
    let parent = assert_fs::TempDir::new().expect("parent");
    init_snap_repo(source.path());
    create_snapshot_with_empty_dir(source.path(), "v1", "empty-dir");
    let metadata_hash = metadata_hash_for_tag(source.path(), "v1");
    init_bare_repo(remote.path());
    let remote_url = remote.path().to_string_lossy().to_string();
    git(source.path(), &["remote", "add", "origin", &remote_url]);
    snap_cmd(source.path()).arg("push").assert().success();

    let clone_path = parent.path().join("clone");
    let clone_arg = clone_path.to_string_lossy().to_string();
    git(parent.path(), &["clone", &remote_url, &clone_arg]);
    git(
        &clone_path,
        &["config", "user.email", "snap-test@example.com"],
    );
    git(&clone_path, &["config", "user.name", "Snap Test"]);
    assert!(!metadata_ref_exists(&clone_path, &metadata_hash));

    snap_cmd(&clone_path)
        .arg("pull")
        .assert()
        .success()
        .stdout(predicate::str::contains("Pull complete"))
        .stdout(predicate::str::contains("Snap metadata refs: fetched"));

    assert!(metadata_ref_exists(&clone_path, &metadata_hash));
}

#[test]
fn sync_sets_upstream_when_branch_has_none() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    let remote = assert_fs::TempDir::new().expect("remote");
    init_snap_repo(temp.path());
    create_snapshot(temp.path(), "v1", "file.txt", "one");
    init_bare_repo(remote.path());
    let remote_url = remote.path().to_string_lossy().to_string();
    git(temp.path(), &["remote", "add", "origin", &remote_url]);

    snap_cmd(temp.path())
        .arg("sync")
        .assert()
        .success()
        .stdout(predicate::str::contains("Sync complete"))
        .stdout(predicate::str::contains("upstream set"));

    let upstream = git(
        temp.path(),
        &["rev-parse", "--abbrev-ref", "--symbolic-full-name", "@{u}"],
    );
    assert!(upstream.trim().starts_with("origin/"));
}

#[test]
fn update_repo_saves_changes_and_runs_snap_aware_sync() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    let remote = assert_fs::TempDir::new().expect("remote");
    init_snap_repo(temp.path());
    create_snapshot_with_empty_dir(temp.path(), "v1", "empty-dir");
    let metadata_hash = metadata_hash_for_tag(temp.path(), "v1");
    init_bare_repo(remote.path());
    let remote_url = remote.path().to_string_lossy().to_string();
    git(temp.path(), &["remote", "add", "origin", &remote_url]);
    fs::write(temp.path().join("feature.txt"), "feature").expect("write feature");

    snap_cmd(temp.path())
        .args(["update-repo", "sync", "feature"])
        .assert()
        .success()
        .stdout(predicate::str::contains("update-repo is a shortcut"))
        .stdout(predicate::str::contains("Changes saved"))
        .stdout(predicate::str::contains("Sync complete"))
        .stdout(predicate::str::contains("upstream set"));

    let message = git(temp.path(), &["log", "-1", "--pretty=%B"]);
    assert!(message.contains("sync feature"));
    let branch = git(temp.path(), &["symbolic-ref", "--short", "HEAD"])
        .trim()
        .to_string();
    assert!(git_success(
        remote.path(),
        &["show-ref", "--verify", &format!("refs/heads/{}", branch)]
    ));
    assert!(git_success(
        remote.path(),
        &["show-ref", "--verify", "refs/tags/v1"]
    ));
    assert!(metadata_ref_exists(remote.path(), &metadata_hash));
}

#[test]
fn update_repo_syncs_existing_commits_when_there_are_no_changes() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    let remote = assert_fs::TempDir::new().expect("remote");
    init_snap_repo(temp.path());
    create_snapshot(temp.path(), "v1", "file.txt", "one");
    init_bare_repo(remote.path());
    let remote_url = remote.path().to_string_lossy().to_string();
    git(temp.path(), &["remote", "add", "origin", &remote_url]);

    snap_cmd(temp.path())
        .arg("update-repo")
        .assert()
        .success()
        .stdout(predicate::str::contains("No changes to commit"))
        .stdout(predicate::str::contains("Sync complete"))
        .stdout(predicate::str::contains("upstream set"));

    let upstream = git(
        temp.path(),
        &["rev-parse", "--abbrev-ref", "--symbolic-full-name", "@{u}"],
    );
    assert!(upstream.trim().starts_with("origin/"));
}

#[test]
fn setup_repo_alias_uses_remote_create_logic() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    let remote = assert_fs::TempDir::new().expect("remote");
    let fake_bin = assert_fs::TempDir::new().expect("fake gh bin");
    init_snap_repo(temp.path());
    create_snapshot_with_empty_dir(temp.path(), "v1", "empty-dir");
    let metadata_hash = metadata_hash_for_tag(temp.path(), "v1");
    init_bare_repo(remote.path());
    let fake_gh = create_fake_gh(fake_bin.path());
    let log_path = temp.path().join("setup-gh.log");
    let remote_url = remote.path().to_string_lossy().to_string();

    snap_cmd_with_path(temp.path(), fake_bin.path())
        .args(["setup-repo", "owner/repo", "--private"])
        .env("SNAP_GH", &fake_gh)
        .env("GH_FAKE_LOG", &log_path)
        .env("GH_FAKE_REMOTE_URL", &remote_url)
        .assert()
        .success()
        .stdout(predicate::str::contains("setup-repo is a shortcut"))
        .stdout(predicate::str::contains("Remote repository created"));

    let log = fs::read_to_string(log_path).expect("read gh log");
    assert!(log.contains("auth status"));
    assert!(log.contains("repo create owner/repo --private"));
    assert!(log.contains("--source=."));
    assert!(log.contains("--remote=origin"));
    assert!(git_success(
        remote.path(),
        &["show-ref", "--verify", "refs/tags/v1"]
    ));
    assert!(metadata_ref_exists(remote.path(), &metadata_hash));
}

#[test]
fn remote_visibility_public_requires_exact_confirmation_and_uses_gh_edit() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    let fake_bin = assert_fs::TempDir::new().expect("fake gh bin");
    init_snap_repo(temp.path());
    let fake_gh = create_fake_gh(fake_bin.path());
    let log_path = temp.path().join("visibility-public-gh.log");

    snap_cmd_with_path(temp.path(), fake_bin.path())
        .args(["remote", "visibility", "public", "owner/repo"])
        .env("SNAP_GH", &fake_gh)
        .env("GH_FAKE_LOG", &log_path)
        .env("GH_FAKE_REMOTE_URL", "unused")
        .write_stdin("owner/repo\n")
        .assert()
        .success()
        .stdout(predicate::str::contains("Public visibility change"))
        .stdout(predicate::str::contains("history may become visible"))
        .stdout(predicate::str::contains("is now public"));

    let log = fs::read_to_string(log_path).expect("read gh log");
    assert!(log.contains("auth status"));
    assert!(log.contains("repo edit owner/repo --visibility public"));
    assert!(log.contains("--accept-visibility-change-consequences"));
}

#[test]
fn remote_visibility_public_wrong_confirmation_cancels_without_gh() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    let fake_bin = assert_fs::TempDir::new().expect("fake gh bin");
    init_snap_repo(temp.path());
    let fake_gh = create_fake_gh(fake_bin.path());
    let log_path = temp.path().join("visibility-cancel-gh.log");

    snap_cmd_with_path(temp.path(), fake_bin.path())
        .args(["remote", "visibility", "public", "owner/repo"])
        .env("SNAP_GH", &fake_gh)
        .env("GH_FAKE_LOG", &log_path)
        .env("GH_FAKE_REMOTE_URL", "unused")
        .write_stdin("wrong/repo\n")
        .assert()
        .success()
        .stdout(predicate::str::contains("Visibility change cancelled"));

    assert!(!log_path.exists());
}

#[test]
fn remote_visibility_private_uses_gh_edit_after_confirmation() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    let fake_bin = assert_fs::TempDir::new().expect("fake gh bin");
    init_snap_repo(temp.path());
    let fake_gh = create_fake_gh(fake_bin.path());
    let log_path = temp.path().join("visibility-private-gh.log");

    snap_cmd_with_path(temp.path(), fake_bin.path())
        .args(["remote", "visibility", "private", "owner/repo"])
        .env("SNAP_GH", &fake_gh)
        .env("GH_FAKE_LOG", &log_path)
        .env("GH_FAKE_REMOTE_URL", "unused")
        .write_stdin("y\n")
        .assert()
        .success()
        .stdout(predicate::str::contains("Private visibility change"))
        .stdout(predicate::str::contains("is now private"));

    let log = fs::read_to_string(log_path).expect("read gh log");
    assert!(log.contains("auth status"));
    assert!(log.contains("repo edit owner/repo --visibility private"));
    assert!(log.contains("--accept-visibility-change-consequences"));
}

#[test]
fn make_public_alias_uses_same_visibility_logic() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    let fake_bin = assert_fs::TempDir::new().expect("fake gh bin");
    init_snap_repo(temp.path());
    let fake_gh = create_fake_gh(fake_bin.path());
    let log_path = temp.path().join("make-public-gh.log");

    snap_cmd_with_path(temp.path(), fake_bin.path())
        .args(["make-public", "owner/repo"])
        .env("SNAP_GH", &fake_gh)
        .env("GH_FAKE_LOG", &log_path)
        .env("GH_FAKE_REMOTE_URL", "unused")
        .write_stdin("owner/repo\n")
        .assert()
        .success()
        .stdout(predicate::str::contains("Public visibility change"))
        .stdout(predicate::str::contains("is now public"));

    let log = fs::read_to_string(log_path).expect("read gh log");
    assert!(log.contains("repo edit owner/repo --visibility public"));
}

#[test]
fn make_private_alias_uses_same_visibility_logic() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    let fake_bin = assert_fs::TempDir::new().expect("fake gh bin");
    init_snap_repo(temp.path());
    let fake_gh = create_fake_gh(fake_bin.path());
    let log_path = temp.path().join("make-private-gh.log");

    snap_cmd_with_path(temp.path(), fake_bin.path())
        .args(["make-private", "owner/repo"])
        .env("SNAP_GH", &fake_gh)
        .env("GH_FAKE_LOG", &log_path)
        .env("GH_FAKE_REMOTE_URL", "unused")
        .write_stdin("y\n")
        .assert()
        .success()
        .stdout(predicate::str::contains("Private visibility change"))
        .stdout(predicate::str::contains("is now private"));

    let log = fs::read_to_string(log_path).expect("read gh log");
    assert!(log.contains("repo edit owner/repo --visibility private"));
}

#[test]
fn remote_visibility_rejects_invalid_repo_before_confirmation() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    init_snap_repo(temp.path());

    snap_cmd(temp.path())
        .args(["remote", "visibility", "public", "owner-only"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("owner/repo format"));
}

#[test]
fn remote_visibility_reports_missing_gh_after_confirmation() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    init_snap_repo(temp.path());

    snap_cmd(temp.path())
        .args(["remote", "visibility", "public", "owner/repo"])
        .env("SNAP_GH", temp.path().join("missing-gh.exe"))
        .write_stdin("owner/repo\n")
        .assert()
        .failure()
        .stderr(predicate::str::contains("GitHub CLI (`gh`) is required"));
}

#[test]
fn remote_delete_requires_exact_confirmation_and_uses_gh_delete() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    let fake_bin = assert_fs::TempDir::new().expect("fake gh bin");
    init_snap_repo(temp.path());
    let fake_gh = create_fake_gh(fake_bin.path());
    let log_path = temp.path().join("remote-delete-gh.log");

    snap_cmd_with_path(temp.path(), fake_bin.path())
        .args(["remote", "delete", "owner/repo"])
        .env("SNAP_GH", &fake_gh)
        .env("GH_FAKE_LOG", &log_path)
        .write_stdin("owner/repo\n")
        .assert()
        .success()
        .stdout(predicate::str::contains("Delete GitHub repository"))
        .stdout(predicate::str::contains("not your local files"))
        .stdout(predicate::str::contains("was deleted"));

    let log = fs::read_to_string(log_path).expect("read gh log");
    assert!(log.contains("auth status"));
    assert!(log.contains("repo delete owner/repo --yes"));
}

#[test]
fn remote_delete_wrong_confirmation_cancels_without_gh() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    let fake_bin = assert_fs::TempDir::new().expect("fake gh bin");
    init_snap_repo(temp.path());
    let fake_gh = create_fake_gh(fake_bin.path());
    let log_path = temp.path().join("remote-delete-cancel-gh.log");

    snap_cmd_with_path(temp.path(), fake_bin.path())
        .args(["remote", "delete", "owner/repo"])
        .env("SNAP_GH", &fake_gh)
        .env("GH_FAKE_LOG", &log_path)
        .write_stdin("wrong/repo\n")
        .assert()
        .success()
        .stdout(predicate::str::contains("Repository deletion cancelled"));

    assert!(!log_path.exists());
}

#[test]
fn delete_repo_alias_uses_same_remote_delete_logic() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    let fake_bin = assert_fs::TempDir::new().expect("fake gh bin");
    init_snap_repo(temp.path());
    let fake_gh = create_fake_gh(fake_bin.path());
    let log_path = temp.path().join("delete-repo-gh.log");

    snap_cmd_with_path(temp.path(), fake_bin.path())
        .args(["delete-repo", "owner/repo"])
        .env("SNAP_GH", &fake_gh)
        .env("GH_FAKE_LOG", &log_path)
        .write_stdin("owner/repo\n")
        .assert()
        .success()
        .stdout(predicate::str::contains("delete-repo is a shortcut"))
        .stdout(predicate::str::contains("was deleted"));

    let log = fs::read_to_string(log_path).expect("read gh log");
    assert!(log.contains("repo delete owner/repo --yes"));
}

#[test]
fn remote_delete_rejects_invalid_repo_before_confirmation() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    init_snap_repo(temp.path());

    snap_cmd(temp.path())
        .args(["remote", "delete", "owner-only"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("owner/repo format"));
}

#[test]
fn remote_delete_reports_missing_gh_after_confirmation() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    init_snap_repo(temp.path());

    snap_cmd(temp.path())
        .args(["remote", "delete", "owner/repo"])
        .env("SNAP_GH", temp.path().join("missing-gh.exe"))
        .write_stdin("owner/repo\n")
        .assert()
        .failure()
        .stderr(predicate::str::contains("GitHub CLI (`gh`) is required"));
}

#[test]
fn remote_delete_reports_auth_failure_before_delete() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    let fake_bin = assert_fs::TempDir::new().expect("fake gh bin");
    init_snap_repo(temp.path());
    let fake_gh = create_fake_gh(fake_bin.path());
    let log_path = temp.path().join("remote-delete-auth-fail-gh.log");

    snap_cmd_with_path(temp.path(), fake_bin.path())
        .args(["remote", "delete", "owner/repo"])
        .env("SNAP_GH", &fake_gh)
        .env("GH_FAKE_LOG", &log_path)
        .env("GH_FAKE_AUTH_FAIL", "1")
        .write_stdin("owner/repo\n")
        .assert()
        .failure()
        .stderr(predicate::str::contains("gh auth login"));

    let log = fs::read_to_string(log_path).expect("read gh log");
    assert!(log.contains("auth status"));
    assert!(!log.contains("repo delete"));
}

#[test]
fn remote_create_uses_gh_and_runs_initial_snap_aware_push() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    let remote = assert_fs::TempDir::new().expect("remote");
    let fake_bin = assert_fs::TempDir::new().expect("fake gh bin");
    init_snap_repo(temp.path());
    create_snapshot_with_empty_dir(temp.path(), "v1", "empty-dir");
    let metadata_hash = metadata_hash_for_tag(temp.path(), "v1");
    init_bare_repo(remote.path());
    let fake_gh = create_fake_gh(fake_bin.path());
    assert!(fake_gh.exists());
    let log_path = temp.path().join("gh.log");
    let remote_url = remote.path().to_string_lossy().to_string();

    snap_cmd_with_path(temp.path(), fake_bin.path())
        .args(["remote", "create", "owner/repo", "--public"])
        .env("SNAP_GH", &fake_gh)
        .env("GH_FAKE_LOG", &log_path)
        .env("GH_FAKE_REMOTE_URL", &remote_url)
        .assert()
        .success()
        .stdout(predicate::str::contains("Remote repository created"));

    let log = fs::read_to_string(log_path).expect("read gh log");
    assert!(log.contains("auth status"));
    assert!(log.contains("repo create owner/repo --public"));
    assert!(log.contains("--source=."));
    assert!(log.contains("--remote=origin"));
    assert!(git_success(
        remote.path(),
        &["show-ref", "--verify", "refs/tags/v1"]
    ));
    assert!(metadata_ref_exists(remote.path(), &metadata_hash));
}

#[test]
fn remote_status_uses_gh_view_for_github_origin() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    let fake_bin = assert_fs::TempDir::new().expect("fake gh bin");
    init_snap_repo(temp.path());
    let fake_gh = create_fake_gh(fake_bin.path());
    let log_path = temp.path().join("gh-status.log");
    git(
        temp.path(),
        &[
            "remote",
            "add",
            "origin",
            "https://github.com/owner/repo.git",
        ],
    );

    snap_cmd_with_path(temp.path(), fake_bin.path())
        .args(["remote", "status"])
        .env("SNAP_GH", &fake_gh)
        .env("GH_FAKE_LOG", &log_path)
        .env("GH_FAKE_REMOTE_URL", "unused")
        .assert()
        .success()
        .stdout(predicate::str::contains("GitHub repo: owner/repo"))
        .stdout(predicate::str::contains("Visibility: PRIVATE"));

    let log = fs::read_to_string(log_path).expect("read gh status log");
    assert!(log.contains("auth status"));
    assert!(log.contains("repo view owner/repo"));
    assert!(log.contains("nameWithOwner,visibility,url"));
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
fn restore_dry_run_does_not_change_files_head_or_tags() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    init_snap_repo(temp.path());
    create_snapshot(temp.path(), "v1", "file.txt", "one");
    create_snapshot(temp.path(), "v2", "file.txt", "two");
    fs::write(temp.path().join("file.txt"), "dirty").expect("dirty file");
    fs::write(temp.path().join("extra.txt"), "extra").expect("extra file");

    let head_before = git(temp.path(), &["rev-parse", "HEAD"]);
    let tags_before = git(temp.path(), &["tag", "--list"]);

    snap_cmd(temp.path())
        .args(["restore", "v1", "--dry-run"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Restore dry run"))
        .stdout(predicate::str::contains("would be created before restore"))
        .stdout(predicate::str::contains("Files changed: no"))
        .stdout(predicate::str::contains("Tags created: no"));

    let head_after = git(temp.path(), &["rev-parse", "HEAD"]);
    let tags_after = git(temp.path(), &["tag", "--list"]);
    assert_eq!(head_before, head_after);
    assert_eq!(tags_before, tags_after);
    assert_eq!(
        fs::read_to_string(temp.path().join("file.txt")).expect("file"),
        "dirty"
    );
    assert_eq!(
        fs::read_to_string(temp.path().join("extra.txt")).expect("extra"),
        "extra"
    );
}

#[test]
fn restore_creates_rescue_snapshot_for_dirty_worktree() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    init_snap_repo(temp.path());
    create_snapshot(temp.path(), "v1", "file.txt", "one");
    create_snapshot(temp.path(), "v2", "file.txt", "two");
    fs::write(temp.path().join("file.txt"), "dirty").expect("dirty file");
    fs::write(temp.path().join("extra.txt"), "extra").expect("extra file");

    snap_cmd(temp.path())
        .args(["restore", "v1"])
        .write_stdin("y\n")
        .assert()
        .success()
        .stdout(predicate::str::contains("Rescue snapshot created"))
        .stdout(predicate::str::contains("Restore complete"));

    let rescue_tags = git(temp.path(), &["tag", "--list", "snap-rescue-*"]);
    let rescue_tag = rescue_tags.lines().next().expect("rescue tag").to_string();
    assert!(!rescue_tag.is_empty());
    assert_eq!(
        git(temp.path(), &["show", &format!("{}:file.txt", rescue_tag)]),
        "dirty"
    );
    assert_eq!(
        git(temp.path(), &["show", &format!("{}:extra.txt", rescue_tag)]),
        "extra"
    );
    assert_eq!(
        fs::read_to_string(temp.path().join("file.txt")).expect("file"),
        "one"
    );
    assert!(!temp.path().join("extra.txt").exists());

    let branch = git(temp.path(), &["symbolic-ref", "--short", "HEAD"]);
    assert!(!branch.trim().is_empty());
    let head = git(temp.path(), &["rev-parse", "HEAD"]);
    let v1 = git(temp.path(), &["rev-parse", "v1^{commit}"]);
    assert_eq!(head, v1);
}

#[test]
fn delete_without_purge_does_not_gc_and_prints_hint() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    init_snap_repo(temp.path());
    create_snapshot(temp.path(), "v1", "file.txt", "one");
    create_snapshot(temp.path(), "v2", "file.txt", "two");

    snap_cmd(temp.path())
        .args(["delete", "v1"])
        .write_stdin("y\n")
        .assert()
        .success()
        .stdout(predicate::str::contains("Disk space was not reclaimed"));

    assert!(!git_success(temp.path(), &["rev-parse", "--verify", "v1"]));
}

#[test]
fn delete_purge_creates_bundle_backup_by_default() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    init_snap_repo(temp.path());
    create_snapshot(temp.path(), "v1", "file.txt", "one");
    create_snapshot(temp.path(), "v2", "file.txt", "two");
    snap_cmd(temp.path())
        .args(["restore", "v1"])
        .write_stdin("y\n")
        .assert()
        .success();

    snap_cmd(temp.path())
        .args(["delete", "v2", "--purge"])
        .write_stdin("y\n")
        .assert()
        .success()
        .stdout(predicate::str::contains("Purge bundle backup"))
        .stdout(predicate::str::contains("Final health check passed"));

    assert!(!git_success(temp.path(), &["rev-parse", "--verify", "v2"]));
    let backup_dir = temp.path().join(".git").join("snap-backups");
    let has_bundle = fs::read_dir(backup_dir)
        .expect("backup dir")
        .filter_map(Result::ok)
        .any(|entry| entry.path().extension().and_then(|ext| ext.to_str()) == Some("bundle"));
    assert!(has_bundle);
}

#[test]
fn delete_purge_no_backup_skips_bundle_and_keeps_metadata_healthy() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    init_snap_repo(temp.path());
    create_snapshot_with_empty_dir(temp.path(), "v1", "empty-dir");
    let v1_hash = metadata_hash_for_tag(temp.path(), "v1");
    create_snapshot(temp.path(), "v2", "file.txt", "two");
    snap_cmd(temp.path())
        .args(["restore", "v1"])
        .write_stdin("y\n")
        .assert()
        .success();

    snap_cmd(temp.path())
        .args(["delete", "v2", "--purge", "--no-backup"])
        .write_stdin("y\n")
        .assert()
        .success()
        .stdout(predicate::str::contains("Purge backup is disabled"))
        .stdout(predicate::str::contains("Final health check passed"));

    assert!(!temp.path().join(".git").join("snap-backups").exists());
    assert!(metadata_ref_exists(temp.path(), &v1_hash));

    fs::write(temp.path().join("v3.txt"), "three").expect("write v3");
    snap_cmd(temp.path())
        .args(["new", "v3", "after purge"])
        .assert()
        .success();
}

#[test]
fn delete_purge_refuses_active_snapshot() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    init_snap_repo(temp.path());
    create_snapshot(temp.path(), "v1", "file.txt", "one");

    snap_cmd(temp.path())
        .args(["delete", "v1", "--purge"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Cannot purge the active snapshot"));
}

#[test]
fn delete_purge_refuses_branch_reachable_snapshot() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    init_snap_repo(temp.path());
    create_snapshot(temp.path(), "v1", "file.txt", "one");
    create_snapshot(temp.path(), "v2", "file.txt", "two");

    snap_cmd(temp.path())
        .args(["delete", "v1", "--purge"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("still reachable"));
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
