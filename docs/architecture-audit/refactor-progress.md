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

Sprint 10 has completed community feedback readiness. The repo now has a community feedback doc, GitHub About/topics are set, six starter/hardening issues are open, and feedback post drafts invite real critique without artificial engagement asks.

The next phase is Sprint 11: OpenAI Application Package.

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

### Sprint 2 - OSS Hygiene Files

Status: completed
Snapshot: `oss-s2-hygiene`
Description: OSS hygiene files

Completed:

- Created `docs/architecture-audit/refactor-plans/sprint-2-oss-hygiene-files.md`.
- Linked the Sprint 2 plan from `docs/architecture-audit/refactor-plans/README.md`.
- Added root OSS files:
  - `AGENTS.md`
  - `CONTRIBUTING.md`
  - `SECURITY.md`
  - `CHANGELOG.md`
  - `SUPPORT.md`
  - `CODE_OF_CONDUCT.md`
  - `LICENSE`
- Added GitHub templates:
  - `.github/pull_request_template.md`
  - `.github/ISSUE_TEMPLATE/bug_report.yml`
  - `.github/ISSUE_TEMPLATE/feature_request.yml`
  - `.github/ISSUE_TEMPLATE/doctor_report.yml`
- Updated README to link the new community, security, support, changelog, conduct, agent, and license files.
- Kept Sprint 2 documentation/template-only: no Rust source, CLI behavior, metadata, schema, installer, or public API changes.
- Created no GitHub sandbox repositories because Sprint 2 did not touch remote or visibility behavior.

Validation:

| Command | Result |
| --- | --- |
| `git diff --check` | Passed |
| `cargo fmt --check` | Passed |
| `cargo clippy --all-targets --all-features` | Passed with the known 9 warnings |
| `cargo test` | Passed, 96 integration tests |
| `cargo build --release` | Passed |
| `./target/release/snap --help` | Passed |
| `./target/release/snap doctor` | Passed; repo healthy, 8 snapshot tags checked |
| Root file existence checks | Passed |
| `.github` template listing | Passed; 4 template files found |
| `AGENTS.md` safety/binary-rule scan | Passed |
| Contributor validation-command scan | Passed |

Known remaining gaps after Sprint 2:

- CI is still missing; Sprint 3 owns it.
- Strict Clippy still fails with `-D warnings`; Sprint 4 or pre-CI strictness owns cleanup.
- Historical prompt noise and temporary patch comments remain for later cleanup.
- Snapshot/tag ambiguity remains for Sprint 6.
- Linux command-name conflict and packaging metadata remain for Sprint 8.

Checkpoint:

```bash
snap new oss-s2-hygiene "sprint 2: OSS hygiene files"
```

### Sprint 3 - CI on Windows/Linux

Status: completed
Snapshot: `oss-s3-ci`
Description: Windows and Linux CI

Completed:

- Created `docs/architecture-audit/refactor-plans/sprint-3-ci-windows-linux.md`.
- Linked the Sprint 3 plan from `docs/architecture-audit/refactor-plans/README.md`.
- Added `.github/workflows/ci.yml`.
- Configured CI for `ubuntu-latest` and `windows-latest`.
- Configured checkout with `fetch-depth: 0` so source-built `snap doctor` can inspect tags/metadata.
- Configured stable Rust install with `rustfmt` and `clippy`.
- Configured Cargo cache through `actions/cache@v4`.
- Configured CI commands:
  - `cargo fmt --check`
  - `cargo clippy --all-targets --all-features`
  - `cargo test`
  - `cargo build --release`
  - source-built `snap --help`
  - source-built `snap doctor`
- Added README CI badge for `.github/workflows/ci.yml`.
- Kept Sprint 3 CI/documentation-only: no Rust source, CLI behavior, metadata, schema, installer, or public API changes.

Validation:

| Command | Result |
| --- | --- |
| `git diff --check` | Passed |
| `cargo fmt --check` | Passed |
| `cargo clippy --all-targets --all-features` | Passed with the known 9 warnings |
| `cargo test` | Passed, 96 integration tests |
| `cargo build --release` | Passed |
| `./target/release/snap --help` | Passed |
| `./target/release/snap doctor` | Passed; repo healthy, 9 snapshot tags checked |
| Workflow existence/content scan | Passed |
| README badge scan | Passed |
| `actionlint` | Not installed locally |
| Ruby YAML syntax parser | Not available locally |

Known remaining gaps after Sprint 3:

- Strict Clippy still fails with `-D warnings`; Sprint 4 or pre-strict-CI cleanup owns it.
- Temporary patch comments remain for Sprint 4.
- Command-construction audit/hardening remains for Sprint 4.
- Snapshot/tag ambiguity remains for Sprint 6.
- Linux command-name conflict and packaging metadata remain for Sprint 8.

Checkpoint:

```bash
snap new oss-s3-ci "sprint 3: Windows and Linux CI"
```

### Sprint 4 - Source Cleanup and Command Hardening Audit

Status: completed
Snapshot: `oss-s4-cleanup`
Description: Source cleanup and command audit

Completed:

- Created `docs/architecture-audit/refactor-plans/sprint-4-source-cleanup-command-audit.md`.
- Linked the Sprint 4 plan from `docs/architecture-audit/refactor-plans/README.md`.
- Removed temporary patch-marker comments from `Cargo.toml` and `src`.
- Fixed the known Clippy warnings:
  - `clippy::print_literal`
  - `clippy::useless_format`
  - `clippy::useless_vec`
  - `clippy::unnecessary_sort_by`
  - `clippy::derivable_impls`
  - `clippy::nonminimal_bool`
  - `clippy::len_zero`
- Converted formatted Git command strings to explicit argv calls for snapshot tags, metadata refs, blob reads, commits, restore reset, delete tag, update tag, edit tag, and doctor repair tag rewrites.
- Created `docs/architecture-audit/command-construction-audit-2026-06-07.md`.
- Documented remaining dynamic command boundaries as intentional central helpers, GitHub CLI override, and release runtime override.

Validation:

| Command | Result |
| --- | --- |
| `git diff --check` | Passed |
| `cargo fmt --check` | Passed |
| `cargo clippy --all-targets --all-features` | Passed with no warnings |
| `cargo clippy --all-targets --all-features -- -D warnings` | Passed |
| `cargo test` | Passed, 96 integration tests |
| `cargo build --release` | Passed |
| `./target/release/snap --help` | Passed |
| `./target/release/snap doctor` | Passed; repo healthy, 10 snapshot tags checked |
| Patch-marker scan for `Cargo.toml` and `src` | Passed, no matches |
| Formatted Git command scan | Passed, no `run_command(&format!(...))` matches |
| Dynamic command boundary scan | Passed; remaining matches documented in command-construction audit |

Known remaining gaps after Sprint 4:

- Snapshot/tag ambiguity remains for Sprint 6.
- Codex/AI-agent workflow docs remain for Sprint 7.
- Linux command-name conflict and packaging metadata remain for Sprint 8.

Checkpoint:

```bash
snap new oss-s4-cleanup "sprint 4: source cleanup and command audit"
```

### Sprint 5 - Restore, Doctor, and Purge Safety Plan

Status: completed
Snapshot: `oss-s5-safety`
Description: Restore doctor purge safety

Completed:

- Created `docs/architecture-audit/refactor-plans/sprint-5-restore-doctor-purge-safety.md`.
- Linked the Sprint 5 plan from `docs/architecture-audit/refactor-plans/README.md`.
- Added `snap restore --dry-run` so restore previews target, current HEAD, dirty state, and rescue behavior without changing files or tags.
- Added default restore rescue snapshots using `snap-rescue-YYYYMMDD-HHMMSS` labels when the current state is dirty or otherwise unprotected by a snapshot tag.
- Added `snap restore --no-rescue` as the explicit old discard-confirmation path.
- Added `snap doctor --json` for machine-readable read-only health output.
- Added `snap doctor --ci` so automation exits non-zero on doctor warnings or errors.
- Documented doctor exit behavior and the read-only nature of JSON/CI modes.
- Updated README, safety model, known limitations, and doctor docs for restore, doctor, purge, and repair safety.
- Added integration tests for restore dry-run, restore rescue snapshots, doctor JSON, doctor CI success, and doctor JSON+CI warning failure.
- Created no GitHub sandbox repositories because Sprint 5 did not touch remote or visibility behavior.

Validation:

| Command | Result |
| --- | --- |
| `git diff --check` | Passed |
| `cargo fmt --check` | Passed |
| `cargo clippy --all-targets --all-features` | Passed with no warnings |
| `cargo clippy --all-targets --all-features -- -D warnings` | Passed |
| `cargo test` | Passed, 101 integration tests |
| `cargo build --release` | Passed |
| `./target/release/snap --help` | Passed; top-level help shows restore dry-run and doctor JSON/CI examples |
| `./target/release/snap restore --help` | Passed; shows `--dry-run` and `--no-rescue` |
| `./target/release/snap doctor --help` | Passed; shows `--json` and `--ci` |
| `./target/release/snap doctor` | Passed; repo healthy, 11 snapshot tags checked |

Source-built sandbox smoke test:

- Created disposable repo: `/tmp/snap-agent-smoke-s5-jLGMAo`.
- Used only `/home/glooring/projects/snap/target/release/snap` for product behavior.
- Exercised `init`, `new`, `restore --dry-run`, rescue-backed `restore`, `doctor --json`, `doctor --ci`, `delete --purge`, and final `doctor --ci`.
- Verified dry-run changed no files/tags, rescue restore created `snap-rescue-20260607-011015`, restored `app.txt` to `one`, removed `extra.txt`, produced doctor JSON with `"status": "ok"`, created a purge bundle backup for `s5-three`, and passed final doctor CI.
- Removed the sandbox after the test.
- No external GitHub sandbox repositories were created.

Known remaining gaps after Sprint 5:

- Codex/AI-agent workflow docs remain for Sprint 7.
- Linux command-name conflict and packaging metadata remain for Sprint 8.
- Historical prompt dumps still contain stale placeholders and old snippets; handle in a later docs cleanup if they remain public.

Checkpoint:

```bash
snap new oss-s5-safety "sprint 5: restore doctor purge safety"
```

### Sprint 6 - Snapshot Tags and Refs Decision

Status: completed
Snapshot: `oss-s6-snapshot-model`
Description: Snapshot refs and tag model

Completed:

- Created `docs/architecture-audit/refactor-plans/sprint-6-snapshot-tags-refs-decision.md`.
- Linked the Sprint 6 plan from `docs/architecture-audit/refactor-plans/README.md`.
- Recorded the marker-first decision: new source-built Snap snapshot tags include `Snap-Snapshot: true`.
- Kept existing storage on ordinary Git tags for compatibility; did not move snapshots to `refs/tags/snap/*` or `refs/snapshots/*`.
- Added compatibility filtering so Snap treats tags as snapshots only when they are marked, carry `Snap-Metadata-Ref`, or match the legacy `Snapshot: <tag-label>` commit-subject pattern.
- Updated `snap list`, `status`, `diff`, `restore`, `delete`, `edit`, `update`, metadata loading, and doctor snapshot scanning through shared discovery behavior.
- Updated README, known limitations, and doctor docs to describe marker-first filtering and deferred namespace migration.
- Added integration tests for marked new snapshots, ignored plain release tags, legacy unmarked Snap tags, ignored non-commit release tags, and many plain-tag doctor scanning.
- Created no GitHub sandbox repositories because Sprint 6 did not touch remote or visibility behavior.

Validation:

| Command | Result |
| --- | --- |
| `git diff --check` | Passed |
| `cargo fmt --check` | Passed |
| `cargo clippy --all-targets --all-features` | Passed with no warnings |
| `cargo clippy --all-targets --all-features -- -D warnings` | Passed |
| `cargo test` | Passed, 105 integration tests |
| `cargo build --release` | Passed |
| `./target/release/snap --help` | Passed |
| `./target/release/snap list` | Passed; legacy checkpoints remain visible |
| `./target/release/snap doctor` | Passed; repo healthy, 12 snapshot tags checked |

Source-built sandbox smoke test:

- Created disposable repo: `/tmp/snap-agent-smoke-s6-CAPvOe`.
- Used only `/home/glooring/projects/snap/target/release/snap` for product behavior.
- Exercised `init`, `new`, tag-message inspection, plain release tag creation, legacy unmarked Snap-style tag creation, `list`, `doctor --json`, and `doctor --ci`.
- Verified `s6-one` contained `Snap-Snapshot: true`, `snap list` showed `s6-one` and `legacy-one`, `snap list` did not show `release-1`, doctor JSON reported `"snapshot_count": 2`, and doctor CI passed.
- Removed the sandbox after the test.
- No external GitHub sandbox repositories were created.

Known remaining gaps after Sprint 6:

- Namespaced snapshot refs remain deferred; a future migration command/design should own `refs/tags/snap/*` or `refs/snapshots/*` if the project moves beyond marker-first tags.
- Push/pull/sync still use the current all-tags behavior; remote refspec changes belong with a future namespace migration.
- Linux command-name conflict and packaging metadata remain for Sprint 8.
- Historical prompt dumps still contain stale placeholders and old snippets; handle in a later docs cleanup if they remain public.

Checkpoint:

```bash
snap new oss-s6-snapshot-model "sprint 6: snapshot refs and tag model"
```

### Sprint 7 - Codex and AI-Agent Workflow Docs

Status: completed
Snapshot: `oss-s7-codex`
Description: Codex and AI-agent workflow docs

Completed:

- Created `docs/architecture-audit/refactor-plans/sprint-7-codex-ai-agent-workflow-docs.md`.
- Linked the Sprint 7 plan from `docs/architecture-audit/refactor-plans/README.md`.
- Added `doc/CODEX_WORKFLOW.md` with before/during/after Codex workflow, rollback, and non-interactive boundaries.
- Added `doc/CODEX_TASKS.md` with recipes for refactors, dependency upgrades, formatter sweeps, generated-code migrations, release prep, docs-only edits, and CI/doctor triage.
- Refreshed `doc/AI_AGENT_WORKFLOW.md` to point to the Codex docs and use `doctor --json --ci`, `restore --dry-run`, rescue snapshots, and `Snap-Snapshot: true` tag markers.
- Updated README quick workflow and public-doc links for Codex.
- Updated `AGENTS.md` with strict Clippy in the standard checks and current AI-agent/safety-sensitive surfaces.
- Documented that read-only automation is encouraged, while destructive prompt bypasses such as broad `--yes` behavior need a separate safeguarded design.
- Created no GitHub sandbox repositories because Sprint 7 was documentation-only and did not touch remote or visibility behavior.

Validation:

| Command | Result |
| --- | --- |
| `git diff --check` | Passed |
| `cargo fmt --check` | Passed |
| `cargo test` | Passed, 105 integration tests |
| `cargo build --release` | Passed |
| `./target/release/snap --help` | Passed |
| `./target/release/snap doctor` | Passed; repo healthy, 13 snapshot tags checked |
| Codex/AI-agent docs scan | Passed; expected `Codex`, `AI-agent`, `doctor --json --ci`, `restore --dry-run`, `Snap-Snapshot`, and `--yes` safeguard text found |

Known remaining gaps after Sprint 7:

- Linux command-name conflict and packaging metadata remain for Sprint 8.
- Namespaced snapshot refs remain deferred; a future migration command/design should own `refs/tags/snap/*` or `refs/snapshots/*` if the project moves beyond marker-first tags.
- Push/pull/sync still use the current all-tags behavior; remote refspec changes belong with a future namespace migration.
- Historical prompt dumps still contain stale placeholders and old snippets; handle in a later docs cleanup if they remain public.

Checkpoint:

```bash
snap new oss-s7-codex "sprint 7: Codex and AI-agent workflow docs"
```

### Sprint 8 - Packaging and Name Conflict

Status: completed
Snapshot: `oss-s8-packaging`
Description: Packaging and name conflict docs

Completed:

- Created `docs/architecture-audit/refactor-plans/sprint-8-packaging-name-conflict.md`.
- Linked the Sprint 8 plan from `docs/architecture-audit/refactor-plans/README.md`.
- Added `doc/INSTALLATION.md` with release asset names, checksum expectations, Windows install paths, Linux install paths, WSL2 notes, and binary verification commands.
- Updated README Installation/Public Docs links to point to the central install/release-assets doc.
- Updated `doc/BUILD_INSTALLERS_WINDOWS_WSL.md` so maintainer release docs include `SHA256SUMS.txt` expectations and conflict-safe Linux/WSL install examples using `gitsnap`.
- Updated `doc/KNOWN_LIMITATIONS.md`, `doc/SAFETY_MODEL.md`, and `doc/GIT_HEALTH_STABILIZATION.md` to reflect the documented Linux command-name conflict policy.
- Replaced the placeholder `Packager.toml` identifier with `io.github.glooring.snap`.
- Replaced Cargo's unmeasured `blazing fast` package description with a conservative local-checkpoint description.
- Reduced `README_INSTALLER.md` to a compatibility pointer so stale installer scaffolding no longer carries placeholder metadata.
- Kept the project and built binary named `Snap`/`snap`; `gitsnap` is documented only as a local conflict-safe filename workaround.
- Created no GitHub sandbox repositories because Sprint 8 did not touch remote or visibility behavior.

Validation:

| Command | Result |
| --- | --- |
| `git diff --check` | Passed |
| `cargo fmt --check` | Passed |
| `cargo clippy --all-targets --all-features` | Passed with no warnings |
| `cargo clippy --all-targets --all-features -- -D warnings` | Passed |
| `cargo test` | Passed, 105 integration tests |
| `cargo build --release` | Passed |
| `./target/release/snap --help` | Passed |
| `./target/release/snap doctor` | Passed; repo healthy, 14 snapshot tags checked |
| Public packaging placeholder scan | Passed; no `com.yourname`, `Your Name`, `you@example`, `blazing fast`, stale planned-conflict wording, or dangerous `sudo cp target/release/snap` examples in the active packaging docs |
| Packaging policy scan | Passed; README/install/build docs include Canonical Snapcraft warning, `command -v snap`, `gitsnap`, `SHA256SUMS.txt`, release asset names, Windows Program Files path, and WSL2 guidance |

Metrics after Sprint 8 docs:

| Area | Result |
| --- | ---: |
| `src/**/*.rs` files | 33 |
| `tests/**/*.rs` files | 1 |
| `doc/*.md` files | 17 |
| `docs/**/*.md` tracked audit files | 14 |
| `src` Rust LOC | 6,786 |
| `tests` Rust LOC | 2,932 |
| `doc` Markdown LOC | 5,604 |
| `docs` audit Markdown LOC | 3,241 |

Known remaining gaps after Sprint 8:

- Checksum generation/upload is documented but not automated in release scripts or `snap release upload`.
- The official Linux binary name remains `snap`; `gitsnap` is only a documented local workaround. Any official alternate Linux binary name needs a separate decision record.
- Namespaced snapshot refs remain deferred; a future migration command/design should own `refs/tags/snap/*` or `refs/snapshots/*` if the project moves beyond marker-first tags.
- Historical prompt/performance docs still contain old wording and placeholders; handle in a later docs cleanup if they remain public.

Checkpoint:

```bash
snap new oss-s8-packaging "sprint 8: packaging and name conflict docs"
```

### Sprint 9 - Cross-Platform and Performance Proof

Status: completed
Snapshot: `oss-s9-platform`
Description: Cross-platform and performance proof

Completed:

- Created `docs/architecture-audit/refactor-plans/sprint-9-cross-platform-performance-proof.md`.
- Linked the Sprint 9 plan from `docs/architecture-audit/refactor-plans/README.md`.
- Added `doc/PERFORMANCE.md` with benchmark methodology, interpretation rules, and a clear warning against broad speed claims.
- Added `doc/CROSS_PLATFORM.md` with current Linux/Windows/WSL2 evidence and edge-case expectations.
- Added optional benchmark helpers:
  - `scripts/benchmark.sh`
  - `scripts/benchmark.ps1`
- Updated README public-doc links for performance and cross-platform docs.
- Replaced the remaining source help `blazing fast` wording with `A Git-powered local checkpoint workflow tool.`
- Rewrote `doc/documentation-solution-emptydir-speed.md` as a current historical note that points to the benchmark methodology.
- Retired obsolete tracked prompt dumps:
  - `doc/prompt-1.txt`
  - `doc/prompt-2 - Copy.txt`
  - `doc/prompt-2.txt`
- Added integration coverage for:
  - paths with spaces;
  - Unicode paths;
  - nested empty-directory restore;
  - hidden metadata;
  - read-only metadata restore.
- Created no GitHub sandbox repositories because Sprint 9 did not touch remote or visibility behavior.

Validation:

| Command | Result |
| --- | --- |
| `git diff --check` | Passed |
| `cargo fmt --check` | Passed |
| `cargo clippy --all-targets --all-features` | Passed with no warnings |
| `cargo clippy --all-targets --all-features -- -D warnings` | Passed |
| `cargo test` | Passed, 107 integration tests |
| `cargo build --release` | Passed |
| `./target/release/snap --help` | Passed |
| `./target/release/snap -h` | Passed; short help shows `A Git-powered local checkpoint workflow tool.` |
| `./target/release/snap doctor` | Passed; repo healthy, 15 snapshot tags checked |
| `bash scripts/benchmark.sh --help` | Passed |
| `bash scripts/benchmark.sh` | Passed; disposable benchmark repo under `target/snap-benchmarks` was removed after the run |
| `pwsh -File scripts/benchmark.ps1 -Help` | Not run; `pwsh` is unavailable locally |
| Performance/public-noise scan | Passed for active docs/source; remaining speed-claim hits are negative examples or historical audit/plan text |
| Targeted proof scan | Passed; found benchmark docs/scripts, cross-platform docs, new edge-case tests, and conservative CLI wording |

Local benchmark smoke output:

| Scenario | Result |
| --- | ---: |
| `snap init` | 2 ms |
| `snap new bench-baseline` | 36 ms |
| `snap new bench-changed` | 53 ms |
| `snap list` | 4 ms |
| `snap diff bench-baseline bench-changed` | 8 ms |
| `snap doctor` | 21 ms |
| `snap restore bench-baseline --dry-run` | 19 ms |

These numbers are local observations only, from the disposable Sprint 9 script run on this workspace. They are not universal benchmark claims.

Metrics after Sprint 9:

| Area | Result |
| --- | ---: |
| `src/**/*.rs` files | 33 |
| `tests/**/*.rs` files | 1 |
| `doc/*.md` files | 19 |
| `docs/**/*.md` tracked audit files | 15 |
| `src` Rust LOC | 6,790 |
| `tests` Rust LOC | 3,113 |
| `doc` Markdown LOC | 5,709 |
| `docs` audit Markdown LOC | 3,598 |

Known remaining gaps after Sprint 9:

- Checksum generation/upload is documented but not automated in release scripts or `snap release upload`.
- The official Linux binary name remains `snap`; `gitsnap` is only a documented local workaround. Any official alternate Linux binary name needs a separate decision record.
- Namespaced snapshot refs remain deferred; a future migration command/design should own `refs/tags/snap/*` or `refs/snapshots/*` if the project moves beyond marker-first tags.
- Windows and WSL2 benchmark/script runs should be recorded from those platforms before making release-specific cross-platform performance claims.

Checkpoint:

```bash
snap new oss-s9-platform "sprint 9: cross-platform and performance proof"
```

### Sprint 10 - Community Feedback

Status: completed
Snapshot: `oss-s10-community`
Description: Community feedback readiness

Completed:

- Created `docs/architecture-audit/refactor-plans/sprint-10-community-feedback.md`.
- Linked the Sprint 10 plan from `docs/architecture-audit/refactor-plans/README.md`.
- Added `doc/COMMUNITY_FEEDBACK.md` with:
  - GitHub About description;
  - recommended topics;
  - good-first issue candidates;
  - safety/hardening issue candidates;
  - feedback post drafts;
  - real feedback signals to seek and artificial signals to avoid.
- Updated README public-doc links for community feedback readiness.
- Updated the real GitHub `Glooring/snap` About description to:
  - `Git-powered local checkpoint CLI for risky refactors, AI-agent edits, experiments, and beginner-friendly project history.`
- Set GitHub topics:
  - `ai-assisted-coding`
  - `cli`
  - `codex`
  - `developer-tools`
  - `git`
  - `linux`
  - `rust`
  - `snapshot`
  - `version-control`
  - `windows`
  - `wsl2`
- Created good-first issues:
  - <https://github.com/Glooring/snap/issues/1> - Document Windows and WSL2 benchmark results
  - <https://github.com/Glooring/snap/issues/2> - Add a tiny demo fixture for beginner docs
  - <https://github.com/Glooring/snap/issues/3> - Improve release checksum instructions
- Created safety/release hardening issues:
  - <https://github.com/Glooring/snap/issues/4> - Design namespaced snapshot refs migration
  - <https://github.com/Glooring/snap/issues/5> - Automate release checksums
  - <https://github.com/Glooring/snap/issues/6> - Audit snap-aware push/pull tag refspecs
- Created no disposable GitHub sandbox repositories.

Validation:

| Command | Result |
| --- | --- |
| `gh auth status` | Passed; authenticated as `Glooring` |
| `gh repo view Glooring/snap --json nameWithOwner,description,repositoryTopics,isPrivate,url` | Passed; verified public repo, updated description, and 11 topics |
| `gh issue list --repo Glooring/snap --state open --limit 20 --json number,title,labels,url` | Passed; verified issues #1-#6 |
| `git diff --check` | Passed |
| `cargo fmt --check` | Passed |
| `cargo clippy --all-targets --all-features` | Passed with no warnings |
| `cargo clippy --all-targets --all-features -- -D warnings` | Passed |
| `cargo test` | Passed, 107 integration tests |
| `cargo build --release` | Passed |
| `./target/release/snap --help` | Passed; source-built help shows the current command surface |
| `./target/release/snap doctor` | Passed; repo healthy, 16 snapshot tags checked before the Sprint 10 checkpoint |
| Community feedback/readiness scan | Passed; artificial engagement terms are framed as things to avoid |
| `gh release list --repo Glooring/snap --limit 5 --json tagName,name,isDraft,isPrerelease,publishedAt,isLatest` | Passed; returned `[]`, so release creation remains a Sprint 11 input |

Metrics after Sprint 10:

| Area | Result |
| --- | ---: |
| `src/**/*.rs` files | 33 |
| `tests/**/*.rs` files | 1 |
| `doc/*.md` files | 20 |
| `docs/**/*.md` tracked audit files | 16 |
| `src` Rust LOC | 6,790 |
| `tests` Rust LOC | 3,113 |
| `doc` Markdown LOC | 5,911 |
| `docs` audit Markdown LOC | 3,775 |

Known remaining gaps after Sprint 10:

- A current release still needs to exist and be referenced before closure.
- OpenAI/Codex application answers need a final evidence-backed package.
- Real feedback is now possible through issues, but external community responses are not yet available.
- Checksum automation, alternate Linux binary naming, and namespaced snapshot refs remain normal issue-tracked follow-ups.

Checkpoint:

```bash
snap new oss-s10-community "sprint 10: community feedback readiness"
```

## Next Up

Sprint 11 - OpenAI Application Package:

- Prepare final application answers and closure evidence from the now-polished public repo.
