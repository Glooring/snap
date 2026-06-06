# OpenAI Application Package

Status: prepared locally, not submitted
Last refreshed: 2026-06-07T02:16:38+03:00
Repository: <https://github.com/Glooring/snap>

This package intentionally does not rely on a hard-coded local commit SHA or checkpoint label. Before submission, run `git rev-parse HEAD` and verify that the public GitHub repository shows the same content after publication.

## Submission Readiness

Do not submit this package as public evidence until the local OSS-readiness commits are pushed to `origin/main`.

Current state:

- local `main` is ahead of `origin/main`;
- GitHub About description and topics are live;
- GitHub issues #1-#6 are live;
- local README/docs/CI/community package changes are not fully visible on GitHub until pushed;
- `gh workflow list --repo Glooring/snap` returned no workflows, because the CI workflow is still local;
- `gh release list --repo Glooring/snap --limit 5 --json tagName,name,isDraft,isPrerelease,publishedAt,isLatest` returned `[]`.

Recommended submission decision:

- Ready to submit after pushing the local OSS-readiness commits if a public release is not required.
- If the application requires a current release, create one only after a maintainer release decision, validated artifacts, checked release notes, and `SHA256SUMS.txt`.

## Ready-To-Paste Answers

### Role

```text
Primary maintainer
```

### Short Qualification

```text
Snap is an MIT-licensed Rust CLI for Git-powered local checkpoints. It helps beginners stop copying whole project folders manually and helps developers save a local checkpoint before risky refactors, AI-agent edits, dependency upgrades, or release work. The current OSS-readiness branch documents the safety model, known limitations, Codex/AI-agent workflow, packaging, cross-platform notes, benchmark methodology, and community feedback paths. I am the primary maintainer.
```

### API Credits Usage

```text
I will use API credits for open-source maintenance: issue triage, documentation, tests, release automation, and review of safety-sensitive changes. For Snap, I will use Codex/OpenAI tools to improve restore and doctor safety, release checksum automation, namespaced snapshot-ref design, Windows/Linux/WSL2 validation, contributor docs, and bug fixes. All AI-assisted changes will be manually reviewed, tested locally with source-built Snap, and kept behind ordinary GitHub review and issue tracking.
```

### Project Summary

```text
Snap is a Rust command-line tool that uses Git as its storage engine and adds a smaller workflow for local checkpoints: snap new, snap list, snap diff, snap restore --dry-run, and snap doctor. It is meant for developers who want a quick safety point before broad edits, and for beginners who need a bridge from folder-copy backups toward Git-backed history. The project is MIT-licensed and maintained publicly at https://github.com/Glooring/snap.
```

## Evidence Checklist

Local evidence prepared in this branch:

| Evidence | Location |
| --- | --- |
| Product positioning | `README.md` |
| AI-agent workflow | `doc/AI_AGENT_WORKFLOW.md` |
| Codex workflow | `doc/CODEX_WORKFLOW.md` |
| Codex task recipes | `doc/CODEX_TASKS.md` |
| Beginner workflow | `doc/BEGINNER_WORKFLOW.md` |
| Why Snap is not a Git replacement | `doc/WHY_NOT_GIT.md` |
| Safety model | `doc/SAFETY_MODEL.md` |
| Doctor docs | `doc/SNAP_DOCTOR.md` |
| Known limitations | `doc/KNOWN_LIMITATIONS.md` |
| Installation and packaging | `doc/INSTALLATION.md` and `doc/BUILD_INSTALLERS_WINDOWS_WSL.md` |
| Performance methodology | `doc/PERFORMANCE.md` |
| Cross-platform notes | `doc/CROSS_PLATFORM.md` |
| Community feedback readiness | `doc/COMMUNITY_FEEDBACK.md` |
| Contributor and maintainer files | `AGENTS.md`, `CONTRIBUTING.md`, `SECURITY.md`, `SUPPORT.md`, `CODE_OF_CONDUCT.md`, `CHANGELOG.md`, `LICENSE` |
| GitHub templates and CI | `.github/ISSUE_TEMPLATE/*`, `.github/pull_request_template.md`, `.github/workflows/ci.yml` |
| Current validation history | `docs/architecture-audit/refactor-progress.md` |
| Current audit | `docs/architecture-audit/agent-efficiency-oss-readiness-audit-2026-06-06.md` |

Live GitHub evidence already visible:

| Evidence | Status |
| --- | --- |
| Repository visibility | Public |
| About description | `Git-powered local checkpoint CLI for risky refactors, AI-agent edits, experiments, and beginner-friendly project history.` |
| Topics | `ai-assisted-coding`, `cli`, `codex`, `developer-tools`, `git`, `linux`, `rust`, `snapshot`, `version-control`, `windows`, `wsl2` |
| Good-first issue #1 | <https://github.com/Glooring/snap/issues/1> |
| Good-first issue #2 | <https://github.com/Glooring/snap/issues/2> |
| Good-first issue #3 | <https://github.com/Glooring/snap/issues/3> |
| Safety/hardening issue #4 | <https://github.com/Glooring/snap/issues/4> |
| Safety/hardening issue #5 | <https://github.com/Glooring/snap/issues/5> |
| Safety/hardening issue #6 | <https://github.com/Glooring/snap/issues/6> |

## Current Validation Evidence

Sprint 11 validation completed with:

```text
git diff --check: passed
cargo fmt --check: passed
cargo clippy --all-targets --all-features: passed with no warnings
cargo clippy --all-targets --all-features -- -D warnings: passed
cargo test: passed, 107 integration tests
cargo build --release: passed
./target/release/snap --help: passed
./target/release/snap doctor: passed
```

Before the Sprint 11 checkpoint, source-built doctor reported:

```text
Snapshot tags: 17 checked, 0 invalid
Snapshot metadata: 17 checked, 0 active invalid, 0 historical invalid, 0 unpinned
Latest valid snapshot: oss-s10-community
```

## Honest Gaps To Disclose

- Local OSS-readiness commits are not yet pushed to `origin/main`.
- No GitHub release exists yet.
- No external community feedback is available yet beyond the maintainer-created issues.
- Release checksum automation is documented but not implemented.
- Snapshot tags use a marker-first filter, but a namespaced snapshot-ref migration is deferred to issue #4.
- Snap still uses the `snap` binary name; Linux/WSL2 docs explain the Canonical Snapcraft conflict and suggest a local `gitsnap` filename when needed.
- `cargo audit` is not installed locally.

## Do Not Claim

- Do not claim broad adoption or third-party production usage.
- Do not claim a current release exists until one is published.
- Do not claim live GitHub CI until the workflow is pushed and runs.
- Do not claim external feedback has arrived.
- Do not claim universal or platform-wide speed improvements.
- Do not use `blazing-fast`, `guaranteed`, or similar unsupported language.
- Do not claim snapshot refs have been fully migrated away from ordinary Git tags.
- Do not claim checksum automation exists.

## Final Pre-Submission Checklist

- Push local OSS-readiness commits to `origin/main` only after maintainer approval.
- Confirm GitHub shows the updated README, docs, workflow, templates, and package doc.
- Confirm `gh workflow list --repo Glooring/snap` shows the CI workflow.
- Decide whether a public release is required before submission.
- If creating a release, build artifacts, generate `SHA256SUMS.txt`, check release notes, and publish deliberately.
- Rerun source-built validation after any release/package changes.
- Keep application answers aligned with the actual public repo state on the day of submission.

## Publication Decision Runbook

This section is a maintainer runbook. It records commands to run only after deciding to publish the local OSS-readiness work.

### Option A: Publish Docs And CI Without A Release

Use this if the application does not require a current GitHub release:

```bash
git status --branch --short
git rev-parse HEAD
git push origin main
gh workflow list --repo Glooring/snap
gh repo view Glooring/snap --json nameWithOwner,description,repositoryTopics,isPrivate,url
gh issue list --repo Glooring/snap --state open --limit 20
```

Do not push all local Snap checkpoint tags by default. They are useful local refactor checkpoints but can clutter public GitHub tags and are not required for README/docs/CI evidence.

After pushing, verify in a browser or with `gh` that GitHub shows:

- updated README positioning;
- `doc/OPENAI_APPLICATION_PACKAGE.md`;
- `.github/workflows/ci.yml`;
- issue templates and PR template;
- existing About description and topics;
- issues #1-#6.

### Option B: Publish A Current Release

Use this only if the maintainer decides a current release is required before submission.

Prerequisites:

- release version/tag decision;
- release notes checked against current behavior;
- artifacts built for the supported platforms;
- `SHA256SUMS.txt` generated and reviewed;
- source-built validation rerun after release-prep changes;
- no overwrite of the global `/usr/local/bin/snap`.

Do not claim a release exists until `gh release list --repo Glooring/snap` shows it.
