use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::{Command as StdCommand, Stdio};

fn snap_cmd(dir: &Path) -> Command {
    let mut cmd = Command::cargo_bin("snap").expect("snap binary");
    cmd.current_dir(dir);
    cmd.env("SNAP_CONFIG_PATH", dir.join(".snapconfig"));
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

fn create_snapshot_with_empty_dir(dir: &Path, label: &str, empty_dir: &str) {
    fs::write(dir.join(format!("{}.txt", label)), label).expect("write fixture");
    fs::create_dir_all(dir.join(empty_dir)).expect("empty dir");
    snap_cmd(dir)
        .args(["new", label, "metadata snapshot"])
        .assert()
        .success();
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
fn new_pins_metadata_blob() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    init_snap_repo(temp.path());
    create_snapshot_with_empty_dir(temp.path(), "v1", "empty-dir");

    let hash = metadata_hash_for_tag(temp.path(), "v1");
    assert!(metadata_blob_exists(temp.path(), &hash));
    assert!(metadata_ref_exists(temp.path(), &hash));
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
fn doctor_handles_many_snapshot_tags_without_windows_command_line_overflow() {
    let temp = assert_fs::TempDir::new().expect("tempdir");
    init_snap_repo(temp.path());
    create_snapshot(temp.path(), "v1", "file.txt", "one");
    create_many_lightweight_tags(temp.path(), 950);

    snap_cmd(temp.path())
        .arg("doctor")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Snapshot tags: 951 checked, 0 invalid",
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
