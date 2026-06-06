# Sprint 11 - OpenAI Application Package

Status: planned at sprint start
Sprint checkpoint: `oss-s11-application`
Date: 2026-06-07
Scope: final evidence package, application answers, closure decision, and honest remaining-gap disclosure

## Purpose

Sprint 11 turns the completed OSS-readiness work into a concise, evidence-backed application package.

The goal is not to invent stronger claims. The goal is to make the public repository evidence easy to inspect, answer the OpenAI/Codex application questions truthfully, and decide whether the remaining closure gaps are blockers or normal follow-up issues.

## Scope

In scope:

- create `doc/OPENAI_APPLICATION_PACKAGE.md`;
- assemble ready-to-paste application answers for maintainer role, project qualification, and API credit usage;
- cite visible repository evidence: README, docs, CI workflow, safety features, cross-platform/performance notes, packaging docs, community feedback doc, GitHub topics, and issues;
- record current gaps honestly: no GitHub release yet, no external community feedback yet, checksum automation still manual, namespaced refs deferred, and Linux `snap` name conflict documented rather than fully solved;
- run read-only GitHub evidence checks against `Glooring/snap`;
- decide whether release creation is blocked on maintainer approval or can be handled by normal issue-driven follow-up;
- update `docs/architecture-audit/refactor-progress.md`;
- update `docs/architecture-audit/agent-efficiency-oss-readiness-audit-2026-06-06.md`.

Out of scope:

- submitting the application;
- asking for artificial stars, upvotes, follows, or vanity feedback;
- creating fake usage or feedback signals;
- creating disposable GitHub sandbox repositories;
- changing Rust runtime behavior, CLI output, schema, or metadata;
- creating a public GitHub release without an explicit maintainer decision and validated release assets/checksums;
- replacing, reinstalling, or upgrading the global `/usr/local/bin/snap`.

## Evidence Contract

Run read-only repository checks:

```bash
git status --short
git rev-parse --abbrev-ref HEAD
git rev-parse HEAD
gh repo view Glooring/snap --json nameWithOwner,description,repositoryTopics,isPrivate,url
gh issue list --repo Glooring/snap --state open --limit 20 --json number,title,labels,url
gh release list --repo Glooring/snap --limit 5 --json tagName,name,isDraft,isPrerelease,publishedAt,isLatest
```

Run local/source-built validation:

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

Run application-package scans:

```bash
rg -n "OpenAI|Codex|API credits|application|release|feedback|stars|upvotes|blazing-fast|guaranteed|production-ready" README.md doc docs/architecture-audit .github Cargo.toml
rg -n "your-username|TODO|FIXME|FINAL|START: THE FIX|END: THE FIX|CORRECTED LINE" Cargo.toml README.md doc docs src .github -g '!docs/architecture-audit/reference-inputs/**'
```

## Application Answer Requirements

The package should include:

- role: `Primary maintainer`;
- short qualification answer grounded in Snap's current public evidence;
- API credits usage answer focused on OSS maintenance, issue triage, tests, docs, release automation, and safety review;
- evidence checklist with file links and GitHub issue links;
- honest gap disclosure;
- "do not claim" list for unsupported adoption, external feedback, release availability, automated checksums, completed namespace migration, or universal performance claims.

## Release Decision

Current evidence before Sprint 11 shows no GitHub releases:

```text
[]
```

Sprint 11 should not create a release merely to satisfy an audit checkbox. If release creation is needed before application submission, record it as an explicit maintainer decision with prerequisites:

- release tag/version decision;
- validated release artifacts;
- `SHA256SUMS.txt`;
- checked release notes;
- no conflict with the global checkpoint Snap workflow.

If those prerequisites are not satisfied in this sprint, the package should mark "current release exists" as the remaining blocker/follow-up rather than overclaiming.

## Acceptance Criteria

- Sprint 11 plan exists and is linked from the refactor-plan index.
- `doc/OPENAI_APPLICATION_PACKAGE.md` exists.
- Application answers are concise, truthful, and backed by visible repository evidence.
- Current release status is checked and recorded.
- Unsupported claims are explicitly excluded.
- Audit/progress docs reflect Sprint 11 status and remaining closure decision.
- Validation gates pass with source-built Snap.
- Final checkpoint is created with:

```bash
snap new oss-s11-application "sprint 11: OpenAI application package"
snap list
git status --short
```
