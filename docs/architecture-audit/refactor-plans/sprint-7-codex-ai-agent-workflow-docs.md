# Sprint 7 - Codex And AI-Agent Workflow Docs

Status: planned at sprint start
Date started: 2026-06-07
Starting checkpoint: `oss-s6-snapshot-model`
Checkpoint label: `oss-s7-codex`

## Purpose

Sprint 7 makes Snap's AI-assisted development story concrete and safe. The docs should show how to use Snap around Codex, editor agents, migrations, formatters, and other tools that can change many files quickly.

This sprint is documentation-focused. It should not add bypass flags, destructive automation, Rust behavior changes, schema changes, or global Snap binary changes.

## Scope

In scope:

- Add `doc/CODEX_WORKFLOW.md` with a practical before/during/after agent workflow.
- Add `doc/CODEX_TASKS.md` with task recipes for risky refactors, dependency upgrades, formatter sweeps, generated-code edits, release prep, docs-only edits, and CI/doctor triage.
- Refresh `doc/AI_AGENT_WORKFLOW.md` so it points to the Codex docs and uses current safety features:
  - `restore --dry-run`;
  - rescue snapshots;
  - `doctor --json --ci`;
  - marker-first snapshot tags.
- Add compact README examples/links for Codex and AI-agent workflows.
- Refresh `AGENTS.md` with current Sprint 5-6 safety surfaces and source-built validation expectations.
- Document the non-interactive roadmap: read-only automation is available through `doctor --json --ci` and `restore --dry-run`; destructive automation remains intentionally prompt-bound unless a future design specifies safeguards and tests.

Out of scope:

- Adding `--yes`, force, or prompt-bypass behavior.
- Changing restore, delete, purge, doctor, tag/ref storage, or remote behavior.
- Creating GitHub sandbox repositories.
- Packaging/name-conflict work. Sprint 8 owns that.

## Validation Plan

Run after edits:

```bash
git diff --check
cargo fmt --check
cargo test
cargo build --release
./target/release/snap --help
./target/release/snap doctor
rg -n "Codex|AI-agent|doctor --json --ci|restore --dry-run|Snap-Snapshot|--yes" README.md AGENTS.md doc/CODEX_WORKFLOW.md doc/CODEX_TASKS.md doc/AI_AGENT_WORKFLOW.md
```

Because this sprint is docs-focused, Clippy is optional unless Rust files change. If any Rust source changes unexpectedly, run the full runtime/code gate including normal and strict Clippy.

## Acceptance Criteria

Sprint 7 is complete when:

- Codex workflow and task recipe docs exist and are linked.
- AI-agent docs use the current Sprint 5-6 safety features.
- Docs clearly state that destructive automation does not get casual prompt bypasses.
- AGENTS.md reflects source-built validation and current safety-sensitive surfaces.
- Progress and audit docs mark Sprint 7 complete and document only Sprint 8 as next up.
- The final checkpoint exists:

```bash
snap new oss-s7-codex "sprint 7: Codex and AI-agent workflow docs"
snap list
git status --short
```
