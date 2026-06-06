# Sprint 4 - Source Cleanup And Command Hardening Audit

Status: planned at sprint start
Date started: 2026-06-07
Starting checkpoint: `oss-s3-ci`
Checkpoint label: `oss-s4-cleanup`

## Purpose

Sprint 4 cleans obvious source/doc professionalism issues and makes strict Clippy credible. It also records the command-construction risk surface so future hardening work is explicit instead of buried in the audit.

This sprint may change Rust source only for mechanical cleanup, strict Clippy fixes, and narrow command-construction safety improvements that preserve behavior.

## Scope

In scope:

- Remove temporary patch-marker comments such as `START`, `END`, `THE FIX`, and `CORRECTED LINE` from `Cargo.toml` and `src`.
- Fix the known normal/strict Clippy warnings:
  - `clippy::print_literal`
  - `clippy::useless_format`
  - `clippy::useless_vec`
  - `clippy::unnecessary_sort_by`
  - `clippy::derivable_impls`
  - `clippy::nonminimal_bool`
  - `clippy::len_zero`
- Run strict Clippy with `-D warnings` and make it pass.
- Create `docs/architecture-audit/command-construction-audit-2026-06-07.md`.
- For command-construction sites, either harden obvious low-risk cases or document why they remain intentional/follow-up work.
- Update README/progress/audit only as needed to reflect the new strict Clippy and command-audit status.

Out of scope:

- Large command-runner rewrites.
- Snapshot tag/ref model changes.
- New restore/doctor/purge features such as dry-run, JSON, CI mode, or rescue snapshots.
- Remote/GitHub sandbox testing unless code changes touch remote behavior.
- Packaging/name-conflict work.
- Any global Snap reinstall, overwrite, replacement, or upgrade.

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
./target/release/snap doctor
rg -n "FINAL|CORRECT|START|END|THE FIX|CORRECTED LINE" Cargo.toml src
rg -n "format!\\(\"git |format!\\(\"gh |run_command\\(&format!|Command::new\\(&command|Command::new\\(command" src
```

If command-construction patterns remain, the Sprint 4 command audit must classify them and state the follow-up.

## Acceptance Criteria

Sprint 4 is complete when:

- Strict Clippy passes.
- Patch-marker comments are gone from `Cargo.toml` and `src`.
- Command-construction audit doc exists and is linked from the current audit/progress.
- Runtime tests and source-built doctor pass.
- Progress and audit docs mark Sprint 4 complete and document only Sprint 5 as next up.
- The final checkpoint exists:

```bash
snap new oss-s4-cleanup "sprint 4: source cleanup and command audit"
snap list
git status --short
```
