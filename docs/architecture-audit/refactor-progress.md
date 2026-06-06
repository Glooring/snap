# Snap OSS Refactor Progress

Status: active  
Started: 2026-06-06  
Master plan: [`CODEX_OSS_REFACTOR_PLAN.md`](CODEX_OSS_REFACTOR_PLAN.md)  
Current audit: [`agent-efficiency-oss-readiness-audit-2026-06-06.md`](agent-efficiency-oss-readiness-audit-2026-06-06.md)

## Workflow

This refactor follows the same sprint discipline proven in the Synthedu refactor:

```text
plan -> scoped change -> gates -> progress/audit update -> snap checkpoint -> next up
```

Because this is the `snap` repo itself, checkpoint labels should avoid release-looking names such as `v7.3`. Use labels like `oss-s0-baseline`, `oss-s1-readme`, and `oss-plan-foundation` until the snapshot/tag model is safer.

The global `snap` command is intentionally older and is used only as the stable checkpoint tool for this refactor. Do not replace or reinstall it from this repo during the refactor. Validate current source behavior with `cargo run -- ...`, `./target/debug/snap ...`, or `./target/release/snap ...`.

## Current State

The refactor has not started runtime/code changes yet.

The current phase is foundation setup: make the plan, audit, progress log, and reference-input structure self-contained inside the Snap repo.

## Entries

### Foundation - Codex OSS Master Plan

Status: completed  
Snapshot: `oss-plan-codex`  
Description: Codex OSS refactor master plan

Completed:

- Created the first master plan for Snap OSS readiness.
- Recorded the Synthedu sprint workflow as the process model.
- Recorded current Snap baseline strengths and risks.
- Validated the first plan with docs checks plus Rust consistency checks.

Validation:

| Command | Result |
| --- | --- |
| `git diff --check` | Passed |
| `rg "CODEX_OSS_REFACTOR_PLAN\|refactor-workflow\|ROADMAP_SNAP_CODEX_OSS\|Codex for Open Source" ...` | Passed |
| `cargo fmt --check` | Passed |
| `cargo test` | Passed, 96 tests |
| `snap new oss-plan-codex "Codex OSS refactor master plan"` | Passed |

### Foundation - Local Audit and Reference Inputs

Status: completed  
Snapshot: `oss-plan-foundation`  
Description: Audit and local reference foundations

Completed:

- Moved the master plan into `docs/architecture-audit/`.
- Created the initial architecture audit for Snap.
- Created this progress log.
- Created `docs/architecture-audit/refactor-plans/` for historical sprint plans.
- Created `docs/architecture-audit/reference-inputs/` for local ignored copies of imported roadmap/workflow source material.
- Removed committed-doc dependence on absolute Synthedu workspace paths.

Validation:

| Command | Result |
| --- | --- |
| `git diff --check` | Passed |
| Stale path/reference scan | Passed, no stale references |
| Git ignored-reference check | Passed, reference source copies are ignored |

Checkpoint:

```bash
snap new oss-plan-foundation "Audit and local reference foundations"
```

### Foundation - Detailed OSS Readiness Audit

Status: completed
Snapshot: `oss-audit-detailed`
Description: Detailed OSS readiness audit

Completed:

- Renamed the audit from the earlier Synthedu-style name to `agent-efficiency-oss-readiness-audit-2026-06-06.md`, because Snap is a CLI/OSS project rather than a platform.
- Expanded the audit to match the detailed style used in the Synthedu refactor program: metadata, executive verdict, baseline evidence, metrics, architecture map, agent-efficiency assessment, CLI/OSS health risks, hotspots, validation contract, snapshot discipline, roadmap, and closure criteria.
- Updated links in the master plan and refactor plan index.

Validation:

| Command | Result |
| --- | --- |
| `git diff --check` | Passed |
| Old audit-name scan | Passed, no stale names |
| New audit-name reference scan | Passed, current audit is linked from plan/progress/index |

Checkpoint:

```bash
snap new oss-audit-detailed "Detailed OSS readiness audit"
```

### Foundation - Complete Starting Audit Baseline

Status: completed
Snapshot: `oss-audit-baseline`
Description: Complete OSS readiness baseline audit

Completed:

- Re-audited the current Snap repository as the real starting baseline for the upcoming refactor.
- Recorded exact gate results, metrics, hotspots, OSS hygiene gaps, command-safety findings, packaging/docs risks, and snapshot/tag risks.
- Clarified the binary rule: global `/usr/local/bin/snap` is intentionally older and should be used only for refactor checkpoints; product behavior must be validated with source-built Snap from this repo, such as `./target/release/snap`.

Validation so far:

| Command | Result |
| --- | --- |
| `git diff --check` | Passed before the final audit rewrite |
| `cargo fmt --check` | Passed |
| `cargo clippy --all-targets --all-features` | Passed with warnings |
| `cargo clippy --all-targets --all-features -- -D warnings` | Failed on 9 baseline warnings |
| `cargo test` | Passed, 96 tests |
| `cargo build --release` | Passed |
| `./target/release/snap doctor` | Passed |
| `cargo audit` | Not installed |

Final validation:

| Command | Result |
| --- | --- |
| `git diff --check` | Passed |
| Source-built/global Snap wording scan | Passed, audit distinguishes checkpoint binary from product-under-test binary |
| Stale path/name scan | Passed |
| `git status --short --ignored docs/architecture-audit` | Passed, only intended docs changes plus ignored reference inputs |

Checkpoint:

```bash
snap new oss-audit-baseline "Complete OSS readiness baseline audit"
```

### Foundation - Global Snap Binary Rule

Status: completed
Snapshot: `oss-global-binary-rule`
Description: Clarify global Snap checkpoint rule

Completed:

- Strengthened the master plan, audit, and progress log so future agents know the global `snap` binary is intentionally older and must be used only for workflow checkpoints.
- Documented that the refactor must not replace, reinstall, overwrite, or upgrade `/usr/local/bin/snap`.
- Documented that current project behavior must be validated with source-built Snap through `cargo run -- ...`, `./target/debug/snap ...`, or `./target/release/snap ...`.
- Documented the per-sprint checkpoint pattern for this repo: `snap new oss-sN-short-name "sprint N: short description"`.

Validation:

| Command | Result |
| --- | --- |
| `git diff --check` | Passed |
| Global/source-built binary wording scan | Passed |

Checkpoint:

```bash
snap new oss-global-binary-rule "Clarify global Snap checkpoint rule"
```

## Next Up

Sprint 0 - Baseline Audit and Safety Rails:

- write a dedicated Sprint 0 plan under `docs/architecture-audit/refactor-plans/`;
- run the full Rust gate baseline where feasible;
- update the architecture audit with exact metrics and warnings;
- mark this foundation entry complete;
- create checkpoint `oss-s0-baseline`.
