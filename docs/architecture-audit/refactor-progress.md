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

Agents are encouraged to test deeply with disposable local projects and, when relevant, disposable GitHub sandbox repositories. Use source-built Snap for those tests, use unique names, clean up external repos, and record commands/results in the sprint notes.

## Current State

Sprint 1 has completed the first public-facing docs pass. The README now leads with Snap as a Git-powered local checkpoint workflow, and focused public docs cover AI-agent use, beginner use, why not just Git, safety, doctor, and limitations.

The next phase is Sprint 2: OSS Hygiene Files.

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

### Foundation - Sandbox Test Policy

Status: completed
Snapshot: `oss-sandbox-test-policy`
Description: Document sandbox testing permission

Completed:

- Documented that agents should test as much real Snap behavior as practical.
- Allowed disposable local projects and Git repositories for source-built Snap smoke/integration testing.
- Allowed disposable GitHub sandbox repositories for remote/visibility tests when relevant, with unique names, cleanup, and progress-log documentation.
- Repeated that global `snap` remains checkpoint-only and source-built Snap is required for product behavior tests.

Validation:

| Command | Result |
| --- | --- |
| `git diff --check` | Passed |
| Sandbox/GitHub/source-built wording scan | Passed |

Checkpoint:

```bash
snap new oss-sandbox-test-policy "Document sandbox testing permission"
```

### Sprint 0 - Baseline Audit and Safety Rails

Status: completed
Snapshot: `oss-s0-baseline`
Description: Baseline audit and source-built validation safety rails

Completed:

- Created `docs/architecture-audit/refactor-plans/sprint-0-baseline-safety-rails.md`.
- Linked the Sprint 0 plan from `docs/architecture-audit/refactor-plans/README.md`.
- Refreshed the current OSS readiness audit with the 2026-06-07 local baseline.
- Confirmed Sprint 0 stayed documentation-only: no Rust source, CLI behavior, metadata, schema, or public API changes.
- Validated current Snap behavior only with source-built commands.
- Recorded strict Clippy as deferred baseline debt for Sprint 4 or before strict CI enforcement.
- Confirmed no GitHub sandbox repositories were created because Sprint 0 did not touch remote or visibility behavior.

Baseline environment:

| Fact | Value |
| --- | --- |
| Run timestamp | `2026-06-07T00:08:49+03:00` |
| Branch | `main` |
| Starting commit | `093fc05b44c90f0d69ffa43927f6ead01f960f75` |
| Initial `git status --short` | Clean |
| `rustc --version` | `rustc 1.95.0 (59807616e 2026-04-14)` |
| `cargo --version` | `cargo 1.95.0 (f2d3ce0bd 2026-03-21)` |

Validation:

| Command | Result |
| --- | --- |
| `git diff --check` | Passed |
| `cargo fmt --check` | Passed |
| `cargo clippy --all-targets --all-features` | Passed with 9 warnings |
| `cargo clippy --all-targets --all-features -- -D warnings` | Failed with 9 warnings-as-errors; deferred intentionally |
| `cargo test` | Passed, 96 integration tests |
| `cargo run -- --help` | Passed; source-built debug help shows modern command surface |
| `cargo run -- doctor` | Passed; repo healthy, 6 snapshot tags checked |
| `cargo build --release` | Passed |
| `./target/release/snap --help` | Passed; source-built release help shows modern command surface |
| `./target/release/snap doctor` | Passed; repo healthy, 6 snapshot tags checked |
| `cargo audit` | Not installed: `error: no such command: audit` |

Baseline metrics:

| Area | Result |
| --- | ---: |
| `src/**/*.rs` files | 33 |
| `tests/**/*.rs` files | 1 |
| `doc/*.md` files | 8 |
| `docs/**/*.md` tracked audit files before Sprint 0 plan | 4 |
| `src` Rust LOC | 6,445 |
| `tests` Rust LOC | 2,698 |
| `doc` Markdown LOC | 4,925 |
| `docs` audit Markdown LOC before Sprint 0 plan | 1,582 |
| `.github` tracked files | 0 |
| Standard OSS root files found | 0 |

Strict Clippy decision:

- Do not fix strict Clippy in Sprint 0.
- Record the 9 mechanical warnings-as-errors as baseline debt.
- Clean them in Sprint 4 or before enabling `-D warnings` in CI.

Source-built sandbox smoke test:

- Created disposable repo: `/tmp/snap-agent-smoke-s0-hwmvPg`.
- Used only `/home/glooring/projects/snap/target/release/snap` for product behavior.
- Exercised `init`, `new`, `list`, `diff`, `doctor`, and `restore`.
- Verified `restore s0-first` restored `app.txt` to `first` and removed `extra.txt`.
- Removed the sandbox after the test.
- No external GitHub sandbox repositories were created.

Checkpoint:

```bash
snap new oss-s0-baseline "sprint 0: baseline audit and safety rails"
```

### Sprint 1 - README Positioning and Public Docs

Status: completed
Snapshot: `oss-s1-readme`
Description: README positioning and public docs

Completed:

- Created `docs/architecture-audit/refactor-plans/sprint-1-readme-positioning-public-docs.md`.
- Linked the Sprint 1 plan from `docs/architecture-audit/refactor-plans/README.md`.
- Rewrote README so it leads with Git-powered local checkpoints, risky refactors, AI-agent edits, release work, and beginner-friendly workflows.
- Removed old README-first-positioning around `Snap (The Rust Edition)`, `blazing-fast`, and project-folder backup framing.
- Added focused public docs:
  - `doc/AI_AGENT_WORKFLOW.md`
  - `doc/BEGINNER_WORKFLOW.md`
  - `doc/WHY_NOT_GIT.md`
  - `doc/SAFETY_MODEL.md`
  - `doc/SNAP_DOCTOR.md`
  - `doc/KNOWN_LIMITATIONS.md`
- Documented truthful constraints: Snap uses Git and does not replace Git; snapshots are currently Git tags; Linux `snap` command-name conflict remains open; strict Clippy is still known debt.
- Kept Sprint 1 documentation-only: no Rust source, CLI behavior, metadata, schema, installer, or public API changes.
- Created no GitHub sandbox repositories because Sprint 1 did not touch remote or visibility behavior.

Validation:

| Command | Result |
| --- | --- |
| `git diff --check` | Passed |
| `cargo fmt --check` | Passed |
| `cargo clippy --all-targets --all-features` | Passed with the known 9 warnings |
| `cargo test` | Passed, 96 integration tests |
| `cargo build --release` | Passed |
| `./target/release/snap --help` | Passed |
| `./target/release/snap doctor` | Passed; repo healthy, 7 snapshot tags checked |
| Scoped stale README/new-doc scan | Passed; no `Snap (The Rust Edition)`, `blazing-fast`, `your-username`, or stale private path matches in README/new docs |
| Required topic scan | Passed; README/new docs include `Git is the engine`, AI-agent workflow, beginner workflow, why-not-Git, known limitations, and `snap doctor` coverage |

Known remaining gaps after Sprint 1:

- `doc/prompt-2.txt` still contains historical `your-username` placeholders and old README snippets; this remains tracked for Sprint 2 or a later cleanup sprint.
- Standard OSS files and `.github` templates are still missing; Sprint 2 owns those.
- CI is still missing; Sprint 3 owns it.
- Strict Clippy still fails with `-D warnings`; Sprint 4 or pre-CI strictness owns cleanup.

Checkpoint:

```bash
snap new oss-s1-readme "sprint 1: README positioning and public docs"
```

## Next Up

Sprint 2 - OSS Hygiene Files:

- Add standard maintainer/community files and GitHub templates so contributors and future agents have clear project instructions.
