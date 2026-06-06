# Sprint 6 - Snapshot Tags And Refs Decision

Status: planned at sprint start
Date started: 2026-06-07
Starting checkpoint: `oss-s5-safety`
Checkpoint label: `oss-s6-snapshot-model`

## Purpose

Sprint 6 resolves the immediate release-tag confusion risk without attempting a risky storage migration. Snap should stop treating arbitrary Git tags as snapshots, while preserving existing Snap-created tags and the current global-checkpoint workflow.

This sprint may change snapshot tag messages, snapshot discovery, doctor tag scanning, tests, and public docs for the snapshot/tag model only.

## Decision

Use an explicit marker in Snap-created annotated tag messages now:

```text
Snap-Snapshot: true
```

Do not move snapshots to `refs/tags/snap/<label>` or `refs/snapshots/<label>` in Sprint 6. Those models remain possible later, but they need a migration command, remote-sync story, and compatibility window.

## Compatibility Contract

Snap should treat a Git tag as a snapshot when any of these is true:

- the annotated tag message contains `Snap-Snapshot: true`;
- the tag message contains `Snap-Metadata-Ref: <hash>`;
- the peeled commit subject is exactly `Snapshot: <tag-label>`, which preserves legacy Snap-created tags including the global checkpoint tags used during this refactor.

Snap should ignore ordinary release tags that do not satisfy that contract, even if they are annotated tags or point to non-commit objects.

## Scope

In scope:

- Add the snapshot marker to new tag messages created by `new`, `update`, `edit`, restore rescue snapshots, and doctor repair tag rewrites through the shared tag-message helper.
- Update snapshot discovery so `list`, `status`, `diff`, `restore`, `delete`, `edit`, `update`, branch filters, and metadata loading use the compatibility contract.
- Update `doctor` snapshot scanning so ordinary release tags are ignored instead of reported as invalid Snap snapshots.
- Add integration tests for marked new snapshots, legacy unmarked Snap tags, ignored plain release tags, and ignored non-commit release tags.
- Update README/known limitations/safety docs and the audit/progress logs to describe the marker-first decision.

Out of scope:

- Moving snapshots to `refs/tags/snap/*` or `refs/snapshots/*`.
- Changing push/pull/sync refspec behavior. Remote sync still follows the current all-tags behavior until a later namespace migration plan owns it.
- Migrating existing tags in-place.
- Changing the global `/usr/local/bin/snap` checkpoint binary.
- GitHub sandbox testing; Sprint 6 does not touch remote visibility behavior.

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
./target/release/snap list
./target/release/snap doctor
```

Run a disposable local smoke test in `/tmp/snap-agent-smoke-s6-XXXXXX` using only `./target/release/snap` for product behavior:

- initialize a Git repo;
- create a Snap snapshot and verify the tag message contains `Snap-Snapshot: true`;
- create a plain annotated release tag and verify `snap list` ignores it;
- create a legacy unmarked Snap-style tag and verify `snap list` still shows it;
- run `doctor --json` and verify only Snap-compatible tags are counted;
- remove the sandbox and record cleanup status.

## Acceptance Criteria

Sprint 6 is complete when:

- New source-built Snap snapshots include `Snap-Snapshot: true`.
- Ordinary release tags no longer appear in `snap list` or doctor snapshot counts.
- Legacy Snap tags remain visible and restorable by compatibility rules.
- Tests cover marker, legacy, and ignored-release behavior.
- Docs clearly state the marker-first decision and deferred namespace migration.
- Progress and audit docs mark Sprint 6 complete and document only Sprint 7 as next up.
- The final checkpoint exists:

```bash
snap new oss-s6-snapshot-model "sprint 6: snapshot refs and tag model"
snap list
git status --short
```
