# Sprint 1 - README Positioning And Public Docs

Status: planned at sprint start
Date started: 2026-06-07
Starting checkpoint: `oss-s0-baseline`
Checkpoint label: `oss-s1-readme`

## Purpose

Sprint 1 makes the public repository understandable in the first few seconds. The README should present Snap as a Git-powered local checkpoint workflow for risky refactors, AI-agent edits, experiments, release work, and beginners who might otherwise copy whole project folders.

This sprint is documentation-only. It must not change Rust source, CLI behavior, snapshot metadata, schemas, public command contracts, installer behavior, or the global Snap binary.

## Scope

In scope:

- Rewrite the README opening around the product identity: "Git is the engine. Snap is the workflow."
- Make the source command surface visible: snapshots, daily Git workflow, branch helpers, GitHub/remote helpers, release helpers, examples, doctor, and options.
- Add public docs for the Sprint 1 topics named in the master plan:
  - `doc/AI_AGENT_WORKFLOW.md`
  - `doc/BEGINNER_WORKFLOW.md`
  - `doc/WHY_NOT_GIT.md`
  - `doc/SAFETY_MODEL.md`
  - `doc/SNAP_DOCTOR.md`
  - `doc/KNOWN_LIMITATIONS.md`
- Link the new docs from README and keep them concise enough to maintain.
- Record truthful limitations: Snap uses Git, creates local commits and annotated snapshot tags, currently scans snapshot tags under `refs/tags`, has a Linux command-name conflict risk, lacks CI/OSS hygiene files today, and strict Clippy is known deferred debt.
- Update the refactor progress log, current audit, and plan index after validation.

Out of scope:

- Adding OSS maintainer files such as `AGENTS.md`, `CONTRIBUTING.md`, `SECURITY.md`, `CHANGELOG.md`, templates, or CI. Those belong to Sprint 2 and Sprint 3.
- Cleaning Rust source warnings, patch comments, command-construction risks, or packaging identifiers.
- Implementing new safety flags such as `restore --dry-run` or `doctor --json`.
- Creating GitHub sandbox repositories.
- Reinstalling, overwriting, replacing, or upgrading global `/usr/local/bin/snap`.

## Content Requirements

README must include:

- What Snap is.
- Why Snap exists.
- AI-agent and risky-refactor workflow.
- Beginner workflow.
- Why not just Git.
- Why CLI-first.
- Safety model.
- Known limitations.
- How Snap stores snapshots.
- `snap doctor` overview.
- Current source command surface.

The new docs must:

- Avoid overstated adoption, benchmark, or safety claims.
- Say clearly that Snap uses Git and does not replace Git.
- Explain destructive operations at a user-facing level.
- Point deeper repair details to existing Git health docs instead of duplicating all internals.

## Validation Plan

Run after edits:

```bash
git diff --check
cargo fmt --check
cargo test
cargo build --release
./target/release/snap --help
./target/release/snap doctor
rg -n "Snap \\(The Rust Edition\\)|blazing-fast|your-username|stale-path-or-private-absolute-path" README.md doc/AI_AGENT_WORKFLOW.md doc/BEGINNER_WORKFLOW.md doc/WHY_NOT_GIT.md doc/SAFETY_MODEL.md doc/SNAP_DOCTOR.md doc/KNOWN_LIMITATIONS.md
rg -n "Git is the engine|AI-agent|beginner|Why not just Git|Known limitations|snap doctor" README.md doc/AI_AGENT_WORKFLOW.md doc/BEGINNER_WORKFLOW.md doc/WHY_NOT_GIT.md doc/SAFETY_MODEL.md doc/SNAP_DOCTOR.md doc/KNOWN_LIMITATIONS.md
```

Do not run strict Clippy as a Sprint 1 gate; it is known Sprint 0 baseline debt.

The broader repository still contains known historical prompt/audit references such as `doc/prompt-2.txt`; Sprint 1 does not clean those.

## Acceptance Criteria

Sprint 1 is complete when:

- README no longer leads with Rust rewrite or backup framing.
- README explains the core value in under 10 seconds.
- README links all six new public docs.
- Source-built help and doctor are still green.
- Progress and audit docs mark Sprint 1 complete and document only the Sprint 2 title/objective as next up.
- The final checkpoint exists:

```bash
snap new oss-s1-readme "sprint 1: README positioning and public docs"
snap list
git status --short
```
