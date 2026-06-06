# Sprint 3 - CI On Windows And Linux

Status: planned at sprint start
Date started: 2026-06-07
Starting checkpoint: `oss-s2-hygiene`
Checkpoint label: `oss-s3-ci`

## Purpose

Sprint 3 adds the first public CI baseline so Snap's formatting, Clippy, tests, release build, and source-built binary checks are visible on Ubuntu and Windows.

This sprint is CI/documentation-only. It must not change Rust source, CLI behavior, snapshot metadata, schemas, installer behavior, or the global Snap binary.

## Scope

In scope:

- Add `.github/workflows/ci.yml`.
- Run CI on `ubuntu-latest` and `windows-latest`.
- Use stable Rust with `rustfmt` and `clippy`.
- Run:
  - `cargo fmt --check`
  - `cargo clippy --all-targets --all-features`
  - `cargo test`
  - `cargo build --release`
  - source-built `snap --help`
  - source-built `snap doctor`
- Use full checkout history/tags so `snap doctor` can inspect tags/metadata in CI.
- Add a README CI badge after the workflow path is stable.
- Update audit and progress docs.

Out of scope:

- Strict Clippy with `-D warnings`; that is known baseline debt for Sprint 4 or pre-strict-CI cleanup.
- Release packaging, checksums, upload automation, or installer builds.
- Rust source cleanup.
- GitHub sandbox repositories.
- Any global Snap reinstall, overwrite, replacement, or upgrade.

## CI Design

The workflow should:

- trigger on pushes and pull requests to `main`;
- use `actions/checkout` with `fetch-depth: 0`;
- install stable Rust through `dtolnay/rust-toolchain`;
- add `rustfmt` and `clippy`;
- cache Cargo dependencies with `Swatinem/rust-cache`;
- use OS-specific binary paths for source-built checks:
  - Linux: `./target/release/snap --help` and `./target/release/snap doctor`
  - Windows: `.\target\release\snap.exe --help` and `.\target\release\snap.exe doctor`

## Validation Plan

Run locally after edits:

```bash
git diff --check
cargo fmt --check
cargo clippy --all-targets --all-features
cargo test
cargo build --release
./target/release/snap --help
./target/release/snap doctor
test -f .github/workflows/ci.yml
rg -n "ubuntu-latest|windows-latest|cargo fmt --check|cargo clippy --all-targets --all-features|cargo test|cargo build --release|snap doctor" .github/workflows/ci.yml README.md
```

If `actionlint` is available locally, run it. If not, document that it was unavailable.

## Acceptance Criteria

Sprint 3 is complete when:

- `.github/workflows/ci.yml` exists.
- Workflow covers Ubuntu and Windows.
- Workflow runs fmt, normal Clippy, tests, release build, source-built help, and source-built doctor.
- README has a CI badge for the workflow.
- Progress and audit docs mark Sprint 3 complete and document only Sprint 4 as next up.
- The final checkpoint exists:

```bash
snap new oss-s3-ci "sprint 3: Windows and Linux CI"
snap list
git status --short
```
