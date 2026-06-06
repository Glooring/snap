# Sprint 0 - Baseline Audit And Safety Rails

Status: completed during Sprint 0 implementation
Date run: 2026-06-07
Starting branch: `main`
Starting commit: `093fc05b44c90f0d69ffa43927f6ead01f960f75`
Checkpoint label: `oss-s0-baseline`

## Purpose

Sprint 0 turns the existing OSS-readiness foundation into a repeatable baseline for the rest of the refactor. It captures exact validation results, records known gaps, and confirms that current Snap behavior is validated only with source-built binaries.

This sprint is documentation-only. It must not change Rust source, CLI behavior, snapshot metadata, schemas, public APIs, or install behavior.

## Scope

In scope:

- Add this Sprint 0 plan and link it from the refactor-plan index.
- Refresh the progress log with Sprint 0 results.
- Refresh the current OSS readiness audit with the latest local baseline.
- Record strict Clippy as deferred baseline debt.
- Run source-built validation and one disposable local smoke test.
- Create the final checkpoint with the global `snap` binary.

Out of scope:

- Rust source cleanup.
- README repositioning.
- OSS maintainer files.
- CI.
- GitHub sandbox repositories.
- Any global Snap reinstall, overwrite, or upgrade.

## Validation Contract

Use source-built Snap for product behavior:

```bash
cargo run -- --help
cargo run -- doctor
./target/release/snap --help
./target/release/snap doctor
```

Use the global `snap` command only for the final refactor checkpoint and `snap list`.

Do not use global `snap --help`, global `snap doctor`, or global `snap --version` as source behavior evidence.

## Baseline Results

| Command | Result |
| --- | --- |
| `git status --short` | Clean before Sprint 0 edits |
| `git rev-parse --abbrev-ref HEAD` | `main` |
| `git rev-parse HEAD` | `093fc05b44c90f0d69ffa43927f6ead01f960f75` |
| `rustc --version` | `rustc 1.95.0 (59807616e 2026-04-14)` |
| `cargo --version` | `cargo 1.95.0 (f2d3ce0bd 2026-03-21)` |
| `git diff --check` | Passed |
| `cargo fmt --check` | Passed |
| `cargo clippy --all-targets --all-features` | Passed with 9 warnings |
| `cargo clippy --all-targets --all-features -- -D warnings` | Failed with 9 warnings-as-errors |
| `cargo test` | Passed, 96 integration tests |
| `cargo run -- --help` | Passed; modern command surface shown |
| `cargo run -- doctor` | Passed; 6 snapshot tags and 6 metadata entries checked |
| `cargo build --release` | Passed |
| `./target/release/snap --help` | Passed; modern command surface shown |
| `./target/release/snap doctor` | Passed; 6 snapshot tags and 6 metadata entries checked |
| `cargo audit` | Unavailable: `error: no such command: audit` |

## Metrics Snapshot

| Area | Count / LOC |
| --- | ---: |
| `src/**/*.rs` files | 33 |
| `tests/**/*.rs` files | 1 |
| `doc/*.md` files | 8 |
| `docs/**/*.md` tracked audit files before this Sprint 0 plan | 4 |
| `src` Rust LOC | 6,445 |
| `tests` Rust LOC | 2,698 |
| `doc` Markdown LOC | 4,925 |
| `docs` audit Markdown LOC before this Sprint 0 plan | 1,582 |
| `.github` tracked files | 0 |
| Standard OSS root files found | 0 |

Largest Rust files at baseline:

| File | LOC |
| --- | ---: |
| `tests/git_health.rs` | 2,698 |
| `src/git_health.rs` | 1,017 |
| `src/git.rs` | 487 |
| `src/commands/release.rs` | 471 |
| `src/commands/doctor.rs` | 434 |
| `src/cli.rs` | 426 |
| `src/utils.rs` | 420 |
| `src/commands/delete.rs` | 380 |
| `src/commands/list.rs` | 318 |
| `src/commands/remote.rs` | 304 |

## Strict Clippy Decision

Strict Clippy is deferred from Sprint 0. The failure is accepted as baseline debt because the findings are mechanical and do not block the documentation-only safety-rails sprint.

The 9 warnings-as-errors are:

- `src/commands/branch.rs`: `clippy::print_literal`
- `src/commands/list.rs`: `clippy::useless_format`
- `src/commands/options.rs`: `clippy::useless_format`, `clippy::useless_vec`
- `src/commands/restore.rs`: `clippy::unnecessary_sort_by`
- `src/config.rs`: `clippy::derivable_impls`
- `src/git_health.rs`: `clippy::nonminimal_bool`, `clippy::len_zero`

Cleanup should happen in Sprint 4 or before enforcing strict Clippy in CI.

## OSS Readiness Gaps Recorded

- Standard OSS files are missing: `AGENTS.md`, `CONTRIBUTING.md`, `SECURITY.md`, `CHANGELOG.md`, `SUPPORT.md`, `CODE_OF_CONDUCT.md`, and root `LICENSE` or `LICENSE.md`.
- `.github` is absent; there are no GitHub Actions workflows, issue templates, or PR template.
- README positioning still leads with older Rust/snapshot framing instead of the stronger Git-powered checkpoint workflow.
- `doc/prompt-2.txt` still contains placeholder `your-username` links.
- Patch-comment artifacts remain in `Cargo.toml`, `src/commands/new.rs`, `src/commands/diff.rs`, and `src/commands/update.rs`.
- Command construction and dynamic command boundaries still need audit in Sprint 4.
- Snapshot discovery still scans `refs/tags`; snapshot metadata uses `Snap-Metadata-Ref` and `refs/snap-metadata`.
- `Packager.toml` still contains placeholder identifier `com.yourname.snap`.
- The Linux command-name conflict with Canonical `snap` is still unresolved.
- `cargo audit` is not installed locally.
- Strict Clippy is not yet clean enough for `-D warnings`.

## Smoke Test

A disposable local project was created at `/tmp/snap-agent-smoke-s0-hwmvPg` and removed after the test.

Source-built release binary used:

```bash
/home/glooring/projects/snap/target/release/snap
```

Commands exercised:

- `git init`
- initial Git commit
- `snap init`
- `snap new s0-first "first source-built smoke snapshot"`
- `snap new s0-second "second source-built smoke snapshot"`
- `snap list`
- `snap diff s0-first s0-second`
- `snap doctor`
- `snap restore s0-first`

Result:

- Both snapshots were created.
- `list` displayed both snapshots and marked `s0-second` active.
- `diff` reported `app.txt` modified and `extra.txt` added.
- `doctor` reported a healthy repo with 2 snapshot tags.
- `restore s0-first` restored `app.txt` to `first` and removed `extra.txt`.
- Sandbox cleanup completed.

## Completion Criteria

Sprint 0 is complete when:

- This plan exists and is linked from the refactor-plan index.
- The progress log records Sprint 0 as completed.
- The current OSS readiness audit records refreshed baseline results.
- Source-built Snap validation is recorded.
- Strict Clippy is documented as deferred baseline debt.
- The final checkpoint exists:

```bash
snap new oss-s0-baseline "sprint 0: baseline audit and safety rails"
snap list
git status --short
```
