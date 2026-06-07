# Sprint 12 - Public CI And Contributor Triage

Status: planned at sprint start
Sprint checkpoint: `oss-s12-public-ci`
Date: 2026-06-07
Scope: public CI repair after first push, contributor PR triage, and publication evidence refresh

## Purpose

Sprint 12 completes the immediate post-publication loop after pushing the OSS-readiness branch to GitHub.

The first public CI run proved that Windows works, but Ubuntu failed before project checks because the workflow installed Rust components with ambiguous `rustup` syntax. This sprint fixes the workflow, proves the public CI signal, and responds to the first contributor activity.

## Scope

In scope:

- fix `.github/workflows/ci.yml` so Rust components install correctly on Ubuntu and Windows;
- verify no local Snap checkpoint tags were pushed;
- rerun local gates with source-built Snap;
- push the CI fix to `origin/main`;
- wait for the public CI run and record results;
- triage PR #7 and PR #8 against the now-published README/docs baseline;
- comment on issue #2 with maintainer guidance;
- update progress/audit/application package docs.

Out of scope:

- publishing local Snap checkpoint tags;
- merging contributor PRs before they are rebased or adapted to the new docs;
- creating a GitHub release;
- changing Rust source, CLI behavior, snapshot schema, release artifacts, or global `/usr/local/bin/snap`;
- replacing, reinstalling, or upgrading the global Snap binary.

## Validation Plan

Run locally:

```bash
git diff --check
cargo fmt --check
cargo clippy --all-targets --all-features
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo build --release
./target/release/snap --help
./target/release/snap doctor
```

Run GitHub checks:

```bash
git push origin main
gh run list --repo Glooring/snap --limit 10 --json databaseId,headSha,headBranch,status,conclusion,workflowName,url
gh run watch <run-id> --repo Glooring/snap --exit-status
gh pr view 7 --repo Glooring/snap --json number,title,state,mergeable,reviewDecision,headRefName,baseRefName,url
gh pr view 8 --repo Glooring/snap --json number,title,state,mergeable,reviewDecision,headRefName,baseRefName,url
```

## Contributor Triage Policy

- Treat the new forks/PRs as a positive early signal, not as broad adoption.
- Thank contributors promptly.
- Prefer PR #7 as the better direction because it creates a dedicated demo doc and covers `doctor`/`restore --dry-run`.
- Ask PR #7 to rebase on the updated `main` and avoid release-looking labels such as `v1`/`v2`.
- Treat PR #8 as duplicate or superseded by #7 unless the author wants to take another good-first issue.
- Do not merge either PR until the public CI baseline is green and the PR is adapted to the current docs.

## Acceptance Criteria

- Sprint 12 plan exists and is linked from the refactor-plan index.
- Public `main` contains the CI fix.
- Public CI passes on Ubuntu and Windows, or any remaining failure is exact and documented.
- PR #7/#8 and issue #2 receive clear maintainer guidance.
- Audit/progress/package docs record public push, CI results, and contributor triage.
- Final checkpoint is created with:

```bash
snap new oss-s12-public-ci "sprint 12: public CI and contributor triage"
snap list
git status --short
```
