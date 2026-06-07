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

Sprint 13 has published the first current OSS-readiness release, `v7.2.0`, with Linux and Windows assets plus `SHA256SUMS.txt`. Public CI is green on Ubuntu and Windows at commit `17fc054d318a7a78e7388aff1343e761d486d0fe`, the release tag `v7.2.0` points to `108c20397f13a8e37a684e9a6e3d7f0ce6d083a0`, and Release Smoke run `27086213709` passed against the public assets on Ubuntu and Windows. PR #7 remains the active beginner-demo direction and needs a rebase/update, PR #8 was closed as a duplicate, and issue #2 has maintainer guidance.

The next phase is normal issue-driven maintenance plus OpenAI/Codex application submission using the now-current public evidence. Do not continue local-only refactoring for aesthetics.

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

### Sprint 11 - OpenAI Application Package

Status: completed locally; closure blocked on maintainer publication/release decision
Snapshot: `oss-s11-application`
Description: OpenAI application package

Completed:

- Created `docs/architecture-audit/refactor-plans/sprint-11-openai-application-package.md`.
- Linked the Sprint 11 plan from `docs/architecture-audit/refactor-plans/README.md`.
- Added `doc/OPENAI_APPLICATION_PACKAGE.md` with:
  - ready-to-paste role, qualification, API-credit usage, and project-summary answers;
  - local evidence checklist;
  - live GitHub evidence checklist;
  - current validation summary;
  - honest gap disclosure;
  - unsupported-claim list;
  - final pre-submission checklist.
- Linked the application package from README.
- Confirmed local `main` is ahead of `origin/main`; the application package warns not to submit until the local OSS-readiness commits are pushed.
- Confirmed `gh workflow list --repo Glooring/snap` returns no workflows because the CI workflow is still local/unpushed.
- Confirmed no GitHub releases exist yet.
- Did not push `main` and did not create a GitHub release, because those are explicit maintainer decisions.

Validation:

| Command | Result |
| --- | --- |
| `git status --branch --short` | Passed; local `main` was ahead of `origin/main` by 17 commits before the Sprint 11 checkpoint |
| `gh repo view Glooring/snap --json nameWithOwner,description,repositoryTopics,isPrivate,url` | Passed; verified public repo, updated description, and 11 topics |
| `gh issue list --repo Glooring/snap --state open --limit 20 --json number,title,labels,url` | Passed; verified issues #1-#6 |
| `gh release list --repo Glooring/snap --limit 5 --json tagName,name,isDraft,isPrerelease,publishedAt,isLatest` | Passed; returned `[]` |
| `gh workflow list --repo Glooring/snap` | Passed with no output; live GitHub workflow is not present until the local CI commit is pushed |
| `git diff --check` | Passed |
| Application/release wording scan | Passed; unsupported terms appear in the application package as limitations or do-not-claim language |
| Placeholder/patch-marker scan | Passed; matches are historical audit/plan references only |
| `cargo fmt --check` | Passed |
| `cargo clippy --all-targets --all-features` | Passed with no warnings |
| `cargo clippy --all-targets --all-features -- -D warnings` | Passed |
| `cargo test` | Passed, 107 integration tests |
| `cargo build --release` | Passed |
| `./target/release/snap --help` | Passed; source-built help shows the current command surface |
| `./target/release/snap doctor` | Passed; repo healthy, 17 snapshot tags checked before the Sprint 11 checkpoint |

Metrics after Sprint 11:

| Area | Result |
| --- | ---: |
| `src/**/*.rs` files | 33 |
| `tests/**/*.rs` files | 1 |
| `doc/*.md` files | 21 |
| `docs/**/*.md` tracked audit files | 17 |
| `src` Rust LOC | 6,790 |
| `tests` Rust LOC | 3,113 |
| `doc` Markdown LOC | 6,053 |
| `docs` audit Markdown LOC | 3,958 |

Known remaining gaps after Sprint 11:

- Local OSS-readiness commits are not pushed to `origin/main`; public GitHub README/docs/CI are stale until a maintainer approves pushing.
- No current GitHub release exists.
- Live GitHub CI workflow is not visible until the local `.github/workflows/ci.yml` commit is pushed.
- External community feedback is not available yet beyond maintainer-created issues.
- Checksum automation, alternate Linux binary naming, namespaced snapshot refs, and `cargo audit` remain follow-up work.

Checkpoint:

```bash
snap new oss-s11-application "sprint 11: OpenAI application package"
```

### Closure Follow-up - Publication Decision Runbook

Status: completed locally; closure still blocked on maintainer publication/release decision
Snapshot: `oss-s11-publish-runbook`
Description: Publication decision runbook

Completed:

- Refreshed `doc/OPENAI_APPLICATION_PACKAGE.md` so it no longer hard-codes a local commit SHA or checkpoint label.
- Added an explicit publication decision runbook:
  - Option A: push local docs/CI/package evidence without a release;
  - Option B: create a release only after version, artifacts, checksums, notes, and validation are ready.
- Documented that local Snap checkpoint tags should not be pushed by default because they are workflow checkpoints, not release tags.
- Kept this follow-up documentation-only: no Rust source, CLI behavior, metadata, schema, package metadata, public GitHub state, or release state changed.
- Did not push `main` and did not create a GitHub release.

Validation:

| Command | Result |
| --- | --- |
| `git diff --check` | Passed |
| Application package scan | Passed; found hard-coded-SHA warning, `git push origin main`, no-default-tag-push warning, `gh release list`, and `SHA256SUMS.txt` release prerequisites |
| `cargo fmt --check` | Passed |
| `cargo clippy --all-targets --all-features` | Passed with no warnings |
| `cargo clippy --all-targets --all-features -- -D warnings` | Passed |
| `cargo test` | Passed, 107 integration tests |
| `cargo build --release` | Passed |
| `./target/release/snap --help` | Passed; source-built help shows the current command surface |
| `./target/release/snap doctor` | Passed; repo healthy, 18 snapshot tags checked before the follow-up checkpoint |

Metrics after publication runbook follow-up:

| Area | Result |
| --- | ---: |
| `doc` Markdown LOC | 6,097 |
| `docs` audit Markdown LOC | 4,150 |

Checkpoint:

```bash
snap new oss-s11-publish-runbook "sprint 11 follow-up: publication decision runbook"
```

### Sprint 12 - Public CI and Contributor Triage

Status: completed
Snapshot: `oss-s12-public-ci`
Description: Public CI repair and contributor triage

Completed:

- Created `docs/architecture-audit/refactor-plans/sprint-12-public-ci-contributor-triage.md`.
- Linked the Sprint 12 plan from `docs/architecture-audit/refactor-plans/README.md`.
- Pushed local `main` to `origin/main` without pushing local Snap checkpoint tags.
- Verified public GitHub now shows the OSS-readiness README/docs/templates/workflow content.
- Observed the first public CI run fail on Ubuntu during Rust component installation while Windows passed.
- Fixed `.github/workflows/ci.yml` by using repeated `--component` flags for `rustfmt` and `clippy`.
- Pushed the CI fix to `origin/main` without pushing local Snap checkpoint tags.
- Verified public CI run `27084650490` passed on both Ubuntu and Windows for commit `0b3918b035c68bb2a4db0767780d625233ae1e0f`.
- Triaged the first external contributor activity:
  - PR #7 remains open as the preferred beginner-demo direction and was asked to rebase/update against current `main`.
  - PR #8 was closed as a duplicate of #7.
  - Issue #2 received maintainer guidance that #7 is the active direction and #1/#3 remain good first issues.
- Recorded the early public signal conservatively: 4 forks and external issue/PR activity are positive interest, not broad adoption.
- Confirmed no disposable GitHub sandbox repositories were created.
- Confirmed no GitHub release was created.

Validation:

| Command / check | Result |
| --- | --- |
| `git push origin main` | Passed; pushed local OSS-readiness commits and then the CI fix to public `main` |
| `git ls-remote --tags origin \| rg 'oss-'` | Passed; returned no matches, so local Snap checkpoint tags were not pushed |
| First public CI run `27084524436` | Failed on Ubuntu at `Install Rust stable`; Windows passed |
| First failure cause | `rustup toolchain install stable --profile minimal --component rustfmt clippy` parsed `clippy` as a toolchain on Ubuntu |
| `git diff --check` | Passed before the CI-fix commit |
| CI workflow scan | Passed; workflow now uses `--component rustfmt --component clippy` |
| `cargo fmt --check` | Passed |
| `cargo clippy --all-targets --all-features` | Passed with no warnings |
| `cargo clippy --all-targets --all-features -- -D warnings` | Passed |
| `cargo test` | Passed, 107 integration tests |
| `cargo build --release` | Passed |
| `./target/release/snap --help` | Passed; source-built help shows the current command surface |
| `./target/release/snap doctor` | Passed; repo healthy, 19 snapshot tags checked before the Sprint 12 checkpoint |
| Public CI run `27084650490` | Passed; Ubuntu job completed in 1m25s and Windows job completed in 3m32s |
| Public CI annotation | Non-blocking GitHub Actions warning about Node.js 20 deprecation for `actions/cache@v4` and `actions/checkout@v4` |
| `gh workflow list --repo Glooring/snap` | Passed; `CI` is active |
| `gh release list --repo Glooring/snap --limit 5 --json tagName,name,isDraft,isPrerelease,publishedAt,isLatest` | Passed; returned `[]` |
| `gh repo view Glooring/snap --json ...` | Passed; repo is public with 4 forks, 0 stars, 0 watchers, expected description, and 11 topics |
| PR #7 triage | Passed; maintainer comment posted asking for rebase/update and non-release-looking demo labels |
| PR #8 triage | Passed; closed as duplicate of #7 with contributor guidance |
| Issue #2 triage | Passed; maintainer guidance posted and #1/#3 suggested for additional contributors |

Metrics after Sprint 12:

| Area | Result |
| --- | ---: |
| `src/**/*.rs` files | 33 |
| `tests/**/*.rs` files | 1 |
| `doc/*.md` files | 21 |
| `docs/**/*.md` tracked audit files | 18 |
| `src` Rust LOC | 6,790 |
| `tests` Rust LOC | 3,113 |
| `doc` Markdown LOC | 6,103 |
| `docs` audit Markdown LOC | 4,389 |

Known remaining gaps after Sprint 12:

- No current GitHub release exists.
- PR #7 needs a contributor rebase/update before review or merge.
- PR checks are not yet meaningful for the stale PR branches until contributors update their branches.
- Release checksum automation is still documented but not implemented.
- Namespaced snapshot refs, alternate Linux binary naming, Windows/WSL2 benchmark evidence, Node 20 GitHub Actions deprecation, and `cargo audit` remain issue/backlog work.

Checkpoint:

```bash
snap new oss-s12-public-ci "sprint 12: public CI and contributor triage"
```

### Sprint 13 - Current GitHub Release

Status: completed
Snapshot: `oss-s13-release`
Description: Current GitHub release

Completed:

- Created `docs/architecture-audit/refactor-plans/sprint-13-current-release.md`.
- Linked the Sprint 13 plan from `docs/architecture-audit/refactor-plans/README.md`.
- Added GitHub Actions release workflow `.github/workflows/release.yml`.
- Added manual public-asset smoke workflow `.github/workflows/release-smoke.yml`.
- Added release notes at `doc/releases/v7.2.0.md`.
- Updated `CHANGELOG.md`, `doc/INSTALLATION.md`, and `doc/BUILD_INSTALLERS_WINDOWS_WSL.md` for the current release workflow and assets.
- Published GitHub Release `v7.2.0`: <https://github.com/Glooring/snap/releases/tag/v7.2.0>.
- Published assets:
  - `SHA256SUMS.txt`
  - `snap-v7.2.0-linux-x86_64`
  - `snap-v7.2.0-linux-x86_64.tar.gz`
  - `snap-v7.2.0-windows-x86_64.exe`
  - `snap-v7.2.0-windows-x86_64-setup.exe`
  - `snap-v7.2.0-windows-x86_64.msi`
- Confirmed release tag `v7.2.0` points to `108c20397f13a8e37a684e9a6e3d7f0ce6d083a0`.
- Confirmed local Snap checkpoint tags were not pushed to `origin`.
- Documented the Linux asset compatibility note: `v7.2.0` Linux binary was built on Ubuntu 24.04 and requires glibc 2.39 or newer; older Linux distributions should build from source for now.
- Confirmed no disposable external GitHub sandbox repositories were created.

Validation:

| Command / check | Result |
| --- | --- |
| `git diff --check` | Passed during release workflow fixes and smoke workflow fixes |
| YAML parse for release workflows | Passed locally with Python/PyYAML |
| Local release publish-layout simulation | Passed with nested `linux-assets` and `windows-assets` directories |
| `cargo fmt --check` | Passed before release workflow publication |
| `cargo clippy --all-targets --all-features` | Passed with no warnings before release workflow publication |
| `cargo clippy --all-targets --all-features -- -D warnings` | Passed before release workflow publication |
| `cargo test` | Passed, 107 integration tests, before release workflow publication and inside Linux release asset build |
| `cargo build --release` | Passed before release workflow publication and inside release asset builds |
| `./target/release/snap --help` | Passed before release workflow publication |
| `./target/release/snap doctor` | Passed before release workflow publication |
| Final post-docs local gates | Passed: `git diff --check`, YAML parse, `cargo fmt --check`, strict Clippy, `cargo test`, `cargo build --release`, source-built help, and source-built doctor |
| Final source-built doctor | Passed; repo healthy, 20 snapshot tags checked, latest valid snapshot `oss-s12-public-ci` before Sprint 13 checkpoint |
| Public CI run `27085772488` | Passed on Ubuntu and Windows for release workflow commit `108c20397f13a8e37a684e9a6e3d7f0ce6d083a0` |
| Public CI run `27086128787` | Passed on Ubuntu and Windows for final smoke-workflow commit `17fc054d318a7a78e7388aff1343e761d486d0fe` |
| Release workflow run `27085855559` | Passed; Linux assets 1m13s, Windows assets 4m01s, publish 7s |
| `gh release view v7.2.0 --repo Glooring/snap ...` | Passed; release is published, not draft, not prerelease |
| `git ls-remote --tags origin 'v7.2.0*'` | Passed; tag exists at `108c20397f13a8e37a684e9a6e3d7f0ce6d083a0` |
| Downloaded release checksums | Passed locally from `/tmp/snap-release-v7.2.0-esCfZg`; all five assets verified against `SHA256SUMS.txt` |
| Downloaded Linux archive listing | Passed; tar contains `snap` |
| Local Linux binary execution | Failed on local Ubuntu glibc 2.35 because artifact requires `GLIBC_2.39`; documented as compatibility limitation, not a checksum/build failure |
| Release Smoke run `27086213709` | Passed; public Linux and Windows portable binaries downloaded and smoke-tested in disposable Git repositories |
| Windows public binary smoke | Passed on `windows-latest`; `--version`, `--help`, `init`, `new`, `list`, `diff`, `restore --dry-run`, and `doctor` |
| Linux public binary smoke | Passed on `ubuntu-latest`; checksums, `--version`, `--help`, archive listing, `init`, `new`, `list`, `diff`, `restore --dry-run`, and `doctor` |

Metrics after Sprint 13:

| Area | Result |
| --- | ---: |
| `src/**/*.rs` files | 33 |
| `tests/**/*.rs` files | 1 |
| `doc/*.md` files | 22 |
| `docs/**/*.md` tracked audit files | 19 |
| `.github` tracked files | 7 |
| `src` Rust LOC | 6,790 |
| `tests` Rust LOC | 3,113 |
| `doc` Markdown LOC | 6,189 |
| `docs` audit Markdown LOC | 4,637 |

Known remaining gaps after Sprint 13:

- PR #7 still needs a contributor rebase/update before review or merge.
- The Linux release asset currently targets Ubuntu 24.04/glibc 2.39 or newer; older-glibc/static Linux release work remains a future issue.
- Namespaced snapshot refs, alternate Linux binary naming, Windows/WSL2 benchmark evidence, Node 20 GitHub Actions deprecation, and `cargo audit` remain issue/backlog work.
- External activity is still early interest, not broad adoption.

Checkpoint:

```bash
snap new oss-s13-release "sprint 13: current GitHub release"
```

## Next Up

Application submission and normal maintainer follow-up:

- Use the updated application package with the current release evidence.
- Review PR #7 after the contributor rebases/adapts it to the current README/docs.
- Open issue-driven follow-ups for older-glibc/static Linux artifacts and GitHub Actions Node 20 warnings if desired.
