# Snap Agent Efficiency and Platform Health Audit

Date: 2026-06-06  
Repository: `Glooring/snap`  
Local path: `/home/glooring/projects/snap`  
Status: initial OSS-readiness baseline

## Purpose

This audit is the current operational source of truth for the Snap OSS-readiness refactor.

It exists before Sprint 0 so future agents do not need to infer the project state from Synthedu paths, private roadmap notes, or old prompt files. The master plan is [`CODEX_OSS_REFACTOR_PLAN.md`](CODEX_OSS_REFACTOR_PLAN.md), while this audit tracks current state, risks, validation, and the next recommended sprint.

## Executive Summary

Snap is already a substantial Rust CLI, not just a small experiment. It has a broad command surface, Git-backed checkpoint behavior, Git health diagnostics, metadata handling, tests, and release/installer scripts.

The main gap is presentation and OSS maintainability: the repo needs clearer positioning, standard OSS files, CI, cleaned patch comments, stronger safety docs, and a clear plan for Git tag/snapshot separation before it looks ready for serious public promotion or a Codex for Open Source application.

Recommended next step: Sprint 0 should create the refactor progress log, capture exact baseline metrics/gates, and convert this audit into a living status document.

## Current Baseline

Confirmed from local inspection:

- Package: `snap`
- Current Cargo version: `7.2.0`
- Main language: Rust
- Current branch: `main`
- Latest checkpoint before this audit setup: `oss-plan-codex`
- CLI includes daily workflow commands, snapshot commands, branch/remote/GitHub commands, release commands, diagnostics, and examples.
- Existing tests include a large Git health integration suite.
- Existing docs live mostly under `doc/`, while this refactor now uses `docs/architecture-audit/` for operational audit/planning docs.

## Important Existing Strengths

- Native Rust CLI with no Node/runtime dependency.
- Clear underlying user problem: local checkpoints without copying entire project folders.
- Git-powered model with snapshot, diff, restore, delete, edit, update, status, save, sync, branch, remote, release, and doctor workflows.
- `snap doctor` is a strong differentiator because it frames the tool as safety-aware, not just convenience-oriented.
- Existing tests cover many Git health, metadata, purge, remote, release, branch, save, restore, and update flows.
- Release and installer scripts already exist for Windows/Linux-oriented workflows.

## Key Risks

| Risk | Status | Why It Matters | Recommended Handling |
| --- | --- | --- | --- |
| OSS positioning is not yet sharp enough | Open | A reviewer should understand Snap in 10 seconds. | Sprint 1 README/docs rewrite. |
| Standard OSS files are missing | Open | `AGENTS.md`, contributing/security/changelog/templates signal maintainership. | Sprint 2 OSS hygiene. |
| CI is not visible in repo baseline | Open | Cross-platform claims need automated proof. | Sprint 3 Windows/Linux CI. |
| Temporary patch comments remain | Open | Comments like "START: THE FIX" look unprofessional. | Sprint 4 cleanup. |
| Snapshot tags can be confused with release tags | Open | Current snapshot model scans Git tags and can mix with normal release tags. | Sprint 6 marker/namespace decision. |
| Command string construction needs audit | Open | Git/GitHub commands can involve user input. | Sprint 4 command hardening audit. |
| Linux command name conflict | Open | Canonical `snap` exists on many Linux systems. | Sprint 8 packaging/name conflict docs. |
| Safety docs need consolidation | Open | Restore, purge, and doctor repair are sensitive. | Sprint 5 safety docs/features plan. |

## Current Documentation Layout

Committed operational docs:

- `docs/architecture-audit/CODEX_OSS_REFACTOR_PLAN.md`
- `docs/architecture-audit/agent-efficiency-platform-health-audit-2026-06-06.md`
- `docs/architecture-audit/refactor-progress.md`
- `docs/architecture-audit/refactor-plans/README.md`
- `docs/architecture-audit/reference-inputs/README.md`

Ignored local reference inputs:

- `docs/architecture-audit/reference-inputs/ROADMAP_SNAP_CODEX_OSS.md`
- `docs/architecture-audit/reference-inputs/refactor-workflow.md`

These ignored inputs are useful locally, but committed docs must remain self-contained.

## Validation Baseline

Already run after creating the first master plan:

| Command | Result |
| --- | --- |
| `git diff --check` | Passed |
| `rg "CODEX_OSS_REFACTOR_PLAN\|refactor-workflow\|ROADMAP_SNAP_CODEX_OSS\|Codex for Open Source" ...` | Passed |
| `cargo fmt --check` | Passed |
| `cargo test` | Passed, 96 tests |
| `snap new oss-plan-codex "Codex OSS refactor master plan"` | Passed |
| Stale path/reference scan | Passed, no stale references |
| Git ignored-reference check | Passed, source reference copies are ignored |

The next Sprint 0 should rerun and record a fuller baseline, including clippy and release build if feasible.

## Snapshot Discipline

Because Snap itself currently uses Git tags for checkpoints, use non-release-looking checkpoint labels in this repo.

Recommended labels:

- `oss-plan-codex`
- `oss-plan-foundation`
- `oss-s0-baseline`
- `oss-s1-readme`

Avoid labels like `v7.3` or `v8.0` until snapshot refs/tags are separated from release tags.

## Next Recommendation

Proceed with Sprint 0: Baseline Audit and Safety Rails.

Sprint 0 should:

- update `docs/architecture-audit/refactor-progress.md`;
- write a dedicated Sprint 0 plan under `docs/architecture-audit/refactor-plans/`;
- run full Rust gates where feasible;
- record exact command output summaries and baseline warnings;
- update this audit with current metrics;
- create checkpoint `oss-s0-baseline`.

After Sprint 0, Sprint 1 should handle README positioning and public-facing docs.
