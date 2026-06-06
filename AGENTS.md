# Agent Instructions

Snap is a safety-focused Rust CLI over Git. It creates local project checkpoints, restores files, validates Git/Snap metadata with `snap doctor`, and includes branch, remote, and release helpers.

## Binary Rule

This repository is Snap itself. The globally installed `/usr/local/bin/snap` is intentionally older and known-good for refactor checkpoints only.

Use global `snap` only for:

```bash
snap new oss-sN-short-name "sprint N: short description"
snap list
```

Do not replace, reinstall, overwrite, upgrade, or copy over the global Snap binary.

Validate current source behavior only with source-built Snap:

```bash
cargo run -- ...
./target/debug/snap ...
./target/release/snap ...
cargo test
cargo build --release
```

## Standard Checks

For runtime/code changes, run:

```bash
git diff --check
cargo fmt --check
cargo clippy --all-targets --all-features
cargo test
cargo build --release
./target/release/snap doctor
```

Strict Clippy with `-D warnings` is expected to pass after Sprint 4 cleanup.

## Sensitive Areas

Be extra careful in:

- restore and delete flows;
- purge behavior and bundle backups;
- `snap doctor` and Git health repair;
- snapshot metadata and `refs/snap-metadata/*`;
- command execution boundaries;
- branch, remote, GitHub, and release helpers;
- path handling across Windows, Linux, and WSL2.

Do not change destructive behavior without tests and documentation. Prefer disposable local repositories for smoke/integration tests. If GitHub behavior must be tested, use only disposable sandbox repositories with unique names, default private, and clean them up or document leftovers.

## Sprint Workflow

For the OSS-readiness refactor:

```text
plan -> scoped change -> gates -> progress/audit update -> snap checkpoint -> next up
```

Write the sprint plan before implementation, keep changes scoped, update `docs/architecture-audit/refactor-progress.md` and the current audit, then checkpoint with a non-release-looking label such as `oss-s2-hygiene`.
