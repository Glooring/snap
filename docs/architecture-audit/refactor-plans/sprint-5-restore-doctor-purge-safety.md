# Sprint 5 - Restore, Doctor, And Purge Safety Plan

Status: planned at sprint start
Date started: 2026-06-07
Starting checkpoint: `oss-s4-cleanup`
Checkpoint label: `oss-s5-safety`

## Purpose

Sprint 5 strengthens trust around Snap's destructive and repair-oriented commands. It turns the roadmap safety items into implemented behavior where the scope is reasonable, and documents any remaining limits explicitly.

This sprint may change Rust source, CLI flags, tests, and public docs for restore, doctor, purge, and safety documentation only.

## Scope

In scope:

- Add `snap restore --dry-run` so users can preview the target snapshot, dirty-worktree state, and rescue behavior without changing files, commits, tags, or metadata refs.
- Add default rescue snapshot protection before `snap restore` changes the working tree, with `--no-rescue` as an explicit escape hatch.
- Preserve existing restore behavior for branch attachment and metadata reconciliation.
- Add `snap doctor --json` for machine-readable read-only health output.
- Add `snap doctor --ci` so automation can fail on health errors or warnings without attempting repair.
- Document doctor exit-code behavior for normal, JSON, and CI modes.
- Update safety docs for restore, delete, purge, and repair.
- Add integration tests for the new restore and doctor behavior.
- Run one disposable local source-built smoke test that covers restore dry-run, rescue restore, doctor JSON, doctor CI, and purge safety.

Out of scope:

- Snapshot tag/ref namespace redesign. Sprint 6 owns that decision.
- Large Git health model rewrites unrelated to JSON/CI output.
- Remote or GitHub behavior. No GitHub sandbox repositories should be created in Sprint 5.
- Packaging, binary naming, installer metadata, or release distribution changes.
- Changing global `/usr/local/bin/snap` or validating current behavior with global Snap.

## Behavior Decisions

Restore safety:

- `snap restore <snapshot> --dry-run` is read-only.
- By default, `snap restore` creates a rescue snapshot before moving the working tree when the current state is not already protected by a snapshot tag or when the working tree is dirty.
- Rescue labels use a non-release-looking prefix such as `snap-rescue-YYYYMMDD-HHMMSS`.
- Dirty worktrees are committed into the rescue snapshot before restore. This is intentional so uncommitted and untracked files can be recovered through the rescue tag.
- `snap restore --no-rescue` keeps the old discard-confirmation path and remains explicitly destructive.

Doctor safety:

- `snap doctor` remains human-readable and read-only by default.
- `snap doctor --json` prints only JSON and does not repair.
- `snap doctor --ci` exits successfully only when the report is clean. Warnings and errors produce a non-zero exit through the existing CLI error path.
- `snap doctor --json --ci` prints JSON first, then exits non-zero when automation should fail.
- `--json` and `--ci` are read-only modes and should not be combined with `--repair`.

Purge safety:

- Existing purge bundle backups, branch-reachability refusal, active-snapshot refusal, and final health check remain the Sprint 5 purge baseline.
- Sprint 5 updates docs and tests around those guarantees instead of redesigning purge internals.

## Validation Plan

Run after edits:

```bash
git diff --check
cargo fmt --check
cargo clippy --all-targets --all-features
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo build --release
./target/release/snap --help
./target/release/snap restore --help
./target/release/snap doctor --help
./target/release/snap doctor
```

Run a disposable local smoke test in `/tmp/snap-agent-smoke-s5-XXXXXX` using only `./target/release/snap` for product behavior:

- initialize a Git repo;
- create two snapshots;
- create dirty/untracked changes;
- run `restore --dry-run`;
- run `restore` and verify a rescue snapshot exists;
- run `doctor --json`;
- run `doctor --ci`;
- run a purge path with source-built Snap;
- remove the sandbox and record cleanup status.

## Acceptance Criteria

Sprint 5 is complete when:

- The Sprint 5 plan exists and is linked from the refactor-plan index.
- Restore dry-run is implemented, tested, and documented.
- Restore rescue snapshot behavior is implemented, tested, and documented.
- Doctor JSON and CI modes are implemented, tested, and documented.
- Purge safety docs reflect the existing backup/refusal/final-check behavior.
- Progress and audit docs mark Sprint 5 complete and document only Sprint 6 as next up.
- The final checkpoint exists:

```bash
snap new oss-s5-safety "sprint 5: restore doctor purge safety"
snap list
git status --short
```
