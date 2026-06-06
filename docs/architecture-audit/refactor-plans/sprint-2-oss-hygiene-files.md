# Sprint 2 - OSS Hygiene Files

Status: planned at sprint start
Date started: 2026-06-07
Starting checkpoint: `oss-s1-readme`
Checkpoint label: `oss-s2-hygiene`

## Purpose

Sprint 2 gives Snap the standard public project files a new user, contributor, security reporter, or coding agent expects in an OSS Rust CLI repository.

This sprint is documentation and repository-template only. It must not change Rust source, CLI behavior, snapshot metadata, schemas, installer behavior, or the global Snap binary.

## Scope

In scope:

- Add root maintainer/community files:
  - `AGENTS.md`
  - `CONTRIBUTING.md`
  - `SECURITY.md`
  - `CHANGELOG.md`
  - `SUPPORT.md`
  - `CODE_OF_CONDUCT.md`
  - `LICENSE`
- Add GitHub contribution templates:
  - `.github/pull_request_template.md`
  - `.github/ISSUE_TEMPLATE/bug_report.yml`
  - `.github/ISSUE_TEMPLATE/feature_request.yml`
  - `.github/ISSUE_TEMPLATE/doctor_report.yml`
- Update README links now that the files exist.
- Update the current audit and progress log.

Out of scope:

- GitHub Actions CI. Sprint 3 owns CI.
- Rust source cleanup, strict Clippy cleanup, command-hardening changes, or packaging fixes.
- Snapshot tag/ref model changes.
- GitHub sandbox repositories.
- Any global Snap reinstall, overwrite, replacement, or upgrade.

## File Requirements

`AGENTS.md` must tell future agents:

- Snap is a safety-focused Rust CLI over Git.
- Global `snap` is checkpoint-only in this repo.
- Current behavior must be validated with source-built Snap.
- Sensitive areas include restore, delete, doctor, Git health, metadata, command execution, remotes, release helpers, and path handling.
- Destructive behavior changes need tests and documentation.

The community files must:

- Be concise and maintainable.
- Avoid pretending the project already has CI or mature public processes that are still planned.
- Point security reports to a clear private contact path.
- Keep support expectations realistic.
- Use the MIT license text consistent with `Cargo.toml`.

GitHub templates must:

- Ask for source-built validation when relevant.
- Ask doctor reports to include `snap doctor` output, OS, Git version, and whether `--repair` was used.
- Remind contributors not to test against existing user repositories for remote/GitHub behavior.

## Validation Plan

Run after edits:

```bash
git diff --check
cargo fmt --check
cargo clippy --all-targets --all-features
cargo test
cargo build --release
./target/release/snap --help
./target/release/snap doctor
test -f AGENTS.md
test -f CONTRIBUTING.md
test -f SECURITY.md
test -f CHANGELOG.md
test -f SUPPORT.md
test -f CODE_OF_CONDUCT.md
test -f LICENSE
find .github -maxdepth 3 -type f -print | sort
rg -n "global `snap`|source-built Snap|restore|delete|doctor|Git health|metadata|command execution" AGENTS.md
rg -n "cargo fmt --check|cargo clippy --all-targets --all-features|cargo test|./target/release/snap doctor" CONTRIBUTING.md .github/pull_request_template.md
```

Do not run strict Clippy as a Sprint 2 gate; it remains known baseline debt.

## Acceptance Criteria

Sprint 2 is complete when:

- All scoped root OSS files exist.
- GitHub issue templates and PR template exist.
- README links the new community files.
- Progress and audit docs mark Sprint 2 complete and document only Sprint 3 as next up.
- The final checkpoint exists:

```bash
snap new oss-s2-hygiene "sprint 2: OSS hygiene files"
snap list
git status --short
```
