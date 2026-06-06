# Snap OSS Readiness Audit — Agent Efficiency and CLI Health

## 1. Metadata

| Field | Value |
| --- | --- |
| Repository | `Glooring/snap` |
| Local path | `/home/glooring/projects/snap` |
| Audit date | `2026-06-06` |
| Branch inspected | `main` |
| Commit inspected | `bfcab26eca16d480b009f108154ac3418603d9d4` |
| Current audit path | `docs/architecture-audit/agent-efficiency-oss-readiness-audit-2026-06-06.md` |
| Master plan | `docs/architecture-audit/CODEX_OSS_REFACTOR_PLAN.md` |
| Progress log | `docs/architecture-audit/refactor-progress.md` |
| Historical sprint plans | `docs/architecture-audit/refactor-plans/` |
| Local ignored references | `docs/architecture-audit/reference-inputs/` |
| Scope | Initial OSS-readiness and agent-efficiency audit before the first real Snap refactor sprint. This audit is documentation/analysis only; it records baseline repo state, risks, validation, ownership, and next steps. |

## 2. Executive Verdict

Snap is already a real Rust CLI with a meaningful product story, not just a small personal script. It has a broad command surface, Git-backed checkpoint behavior, Git health diagnostics, metadata handling, tests, release helpers, and installer assets.

The core opportunity is strong:

> Snap can be positioned as a native Rust CLI for Git-powered local checkpoints before risky refactors, AI-agent edits, experiments, and release work, while helping beginners stop copying whole project folders manually.

The main weakness is not the idea. The main weakness is public readiness. A new maintainer, reviewer, or Codex/OpenAI program evaluator should not have to infer the story from old prompt docs, internal implementation files, or personal context.

The next work should be an **OSS-readiness refactor program**, not random cleanup:

- make positioning obvious in the first 10 seconds;
- add standard OSS maintainer files;
- add visible CI and cross-platform proof;
- clean temporary patch comments;
- document destructive/safety-sensitive behavior;
- decide how Snap snapshots relate to normal Git release tags;
- keep a strict sprint workflow with gates and named Snap checkpoints.

## 3. Current Baseline Evidence

Commands and inspections already run for this baseline:

| Command / inspection | Result | Meaning |
| --- | --- | --- |
| `git rev-parse --abbrev-ref HEAD` | `main` | Current baseline branch. |
| `git rev-parse HEAD` | `bfcab26eca16d480b009f108154ac3418603d9d4` | Current baseline commit after foundation docs checkpoint. |
| `git log --oneline -5` | `oss-plan-foundation`, `oss-plan-codex`, initial import | The OSS-readiness docs are already checkpointed with Snap. |
| `cargo fmt --check` | Passed | Formatting gate was clean during master-plan setup. |
| `cargo test` | Passed, `96` tests | Current test suite is runnable locally and substantial. |
| `git diff --check` | Passed | Docs changes had no whitespace errors. |
| Stale path/reference scan | Passed | Committed docs no longer depend on absolute Synthedu paths. |
| Git ignored-reference check | Passed | Copied roadmap/workflow inputs are ignored and not public repo state. |
| Root OSS file inspection | `0` standard OSS root files found | `AGENTS.md`, `CONTRIBUTING.md`, `SECURITY.md`, `CHANGELOG.md`, `SUPPORT.md`, and templates are still missing. |
| `.github` inspection | `0` files found | CI, issue templates, and PR template are still missing. |
| Temporary comment scan | Open findings | `Cargo.toml`, `new.rs`, `diff.rs`, and `update.rs` still contain patch-style comments. |
| Snapshot storage scan | Open findings | Snapshot discovery still touches `refs/tags`; metadata uses `Snap-Metadata-Ref` and `refs/snap-metadata`. |

The next Sprint 0 should rerun the full baseline, especially `cargo clippy --all-targets --all-features` and `cargo build --release`, then record exact outputs here.

## 4. Repository Metrics Snapshot

### 4.1 File Counts

| Area | Count |
| --- | ---: |
| `src/**/*.rs` | `33` |
| `tests/**/*.rs` | `1` |
| `doc/*.md` | `8` |
| `docs/**/*.md` | `7` |
| `.github` tracked files | `0` |
| Standard OSS root files found | `0` |

Standard OSS root files checked:

- `AGENTS.md`
- `CONTRIBUTING.md`
- `SECURITY.md`
- `CHANGELOG.md`
- `SUPPORT.md`
- `CODE_OF_CONDUCT.md`
- `LICENSE` / `LICENSE.md`

### 4.2 LOC Snapshot

| Area | LOC |
| --- | ---: |
| `src` Rust | `6,445` |
| `tests` Rust | `2,698` |
| `doc` Markdown/text | `8,318` |
| `docs` audit Markdown | `3,752` before this expanded audit |

### 4.3 Largest Operational Files

| Rank | File | LOC | Primary concern |
| ---: | --- | ---: | --- |
| 1 | `tests/git_health.rs` | `2,698` | Very valuable integration coverage, but large enough to need section ownership if expanded further. |
| 2 | `src/git_health.rs` | `1,017` | High-risk Git diagnosis/repair logic; needs careful tests and documentation for every behavior change. |
| 3 | `src/git.rs` | `487` | Git wrapper and remote metadata sync boundary; safety-sensitive because it executes Git commands. |
| 4 | `src/commands/release.rs` | `471` | Release scripting/GitHub CLI boundary; cross-platform/runtime assumptions matter. |
| 5 | `src/commands/doctor.rs` | `434` | User-facing diagnosis/repair UI; should remain read-only unless repair is explicit. |
| 6 | `src/cli.rs` | `426` | Public command contract; changes affect help text, docs, and user expectations. |
| 7 | `src/utils.rs` | `420` | Shared snapshot discovery, metadata, command helpers; contains tag/ref behavior central to future migration. |
| 8 | `src/commands/delete.rs` | `380` | Destructive snapshot/purge workflow; needs strong tests and explicit safety docs. |
| 9 | `src/commands/list.rs` | `318` | Snapshot discovery/presentation; affected by tag namespacing or marker filtering. |
| 10 | `src/commands/remote.rs` | `304` | GitHub CLI command boundary and destructive remote operations. |
| 11 | `src/github.rs` | `282` | GitHub CLI integration and release parsing. |
| 12 | `src/commands/branch.rs` | `207` | Branch workflow wrapper; must preserve Git expectations. |
| 13 | `src/commands/options.rs` | `170` | Global Snap options persistence. |
| 14 | `src/commands/edit.rs` | `156` | Tag/message rewrite behavior; safety-sensitive because it mutates refs. |
| 15 | `src/commands/restore.rs` | `150` | Destructive restore workflow; primary target for dry-run/rescue improvements. |
| 16 | `src/commands/new.rs` | `136` | Snapshot creation; central to label/tag behavior and metadata pinning. |
| 17 | `src/commands/update.rs` | `133` | Snapshot amend/update; central to tag and metadata rewrite behavior. |
| 18 | `src/commands/diff.rs` | `123` | Snapshot comparison; useful in AI-agent workflow docs. |

## 5. Architecture Map

### 5.1 Runtime Shape

```text
snap
├── CLI parser and command definitions
├── command modules
│   ├── daily workflow: status, save, pull, push, sync, history
│   ├── snapshots: init, new, list, diff, restore, delete, edit, update
│   ├── Git health: doctor and git_health internals
│   ├── GitHub/remote: remote, setup-repo, visibility, delete-repo
│   └── release: local script runner and GitHub release upload/list
├── Git command wrappers and utilities
├── metadata capture/storage
├── OS-specific hidden/readonly behavior
└── integration tests
```

### 5.2 Main Domains

| Domain | Primary files | Current state |
| --- | --- | --- |
| CLI contract | `src/cli.rs`, `src/main.rs` | Broad, useful command surface; README command list is not yet aligned with all newer workflow commands. |
| Snapshot model | `src/commands/new.rs`, `update.rs`, `edit.rs`, `list.rs`, `utils.rs` | Core value of the tool; currently coupled to annotated Git tags and `Snap-Metadata-Ref`. |
| Restore/delete safety | `src/commands/restore.rs`, `delete.rs`, `git_health.rs` | High-risk user trust area; tests exist but docs and dry-run/rescue behavior need improvement. |
| Doctor/Git health | `src/commands/doctor.rs`, `src/git_health.rs`, `doc/GIT_HEALTH_STABILIZATION.md`, `doc/REPAIR_GIT_ERRORS.md` | Strong differentiator; should be promoted and made machine-readable in future. |
| Git integration | `src/git.rs`, `src/utils.rs` | Critical command boundary; some operations already use explicit args, while some still use formatted command strings. |
| GitHub/release | `src/github.rs`, `src/commands/remote.rs`, `src/commands/release.rs`, scripts | Useful but needs CI/release documentation polish before public promotion. |
| Cross-platform metadata | `src/os/*`, metadata helpers, docs | Strong product claim; should be backed with Windows/Linux CI and docs. |
| Docs/OSS presentation | `README.md`, `doc/*`, `docs/architecture-audit/*` | Lots of raw material exists; public-facing docs need consolidation and sharper positioning. |

## 6. Agent-Coding Efficiency Assessment

### 6.1 What Is Already Good

- The Rust module structure is easy to navigate.
- Command modules are separated by user-facing command.
- Tests are substantial and fast locally.
- `snap doctor` and Git health code already encode many safety invariants.
- The new `docs/architecture-audit/` area now gives agents a place for audit/progress/plans.
- Local copied reference inputs are ignored, so committed docs can stay public-safe.

### 6.2 What Slows Agents Down

| Problem | Impact | Example |
| --- | --- | --- |
| Public story is scattered | Agents/reviewers need old docs and context to understand why Snap matters. | README still leads with "Rust edition" and speed more than AI-agent/beginner safety. |
| No root `AGENTS.md` | Coding agents lack project-specific safety rules by default. | Restore/delete/doctor changes need stronger instructions. |
| No visible CI | Agents cannot rely on repository-hosted gates. | `.github/workflows` is absent. |
| Large safety files | Agents must be careful editing Git health or tests. | `src/git_health.rs`, `tests/git_health.rs`. |
| Command execution patterns are mixed | Harder to reason about injection/path safety. | formatted `git tag`, `git reset`, `git cat-file`, `git update-ref`. |
| Historical prompt docs are noisy | Old prompt files contain placeholders and old code snippets. | `doc/prompt-2.txt` contains `your-username` examples. |
| Snapshot tags look like release tags | Refactor workflow can accidentally create ambiguous labels. | Avoid `v7.3`-style snapshots in this repo. |

### 6.3 Target Agent Experience

A future coding agent should be able to answer quickly:

- What user problem does Snap solve?
- Which commands are destructive or safety-sensitive?
- Which gates must run before a PR?
- Which files should not be touched without tests?
- How are snapshots stored?
- What is the current migration plan for snapshot tags?
- How should a sprint be checkpointed without confusing release tags?

The current repo partially answers these through code and docs. Sprint 0-3 should make the answers explicit.

## 7. CLI and OSS Health Assessment

### 7.1 Current Strengths

- Real command surface and real workflows.
- Strong test suite with 96 passing tests in baseline.
- Clear safety-oriented feature: `snap doctor`.
- Uses Git as a mature storage engine.
- Metadata work covers empty directories and file attributes beyond plain Git.
- Release scripts and installer files already exist.
- The idea is easy to connect to modern AI-assisted coding workflows.

### 7.2 Current Health Risks

| Risk | User / maintainer consequence |
| --- | --- |
| Weak first impression | A reviewer may see "backup tool" instead of "developer safety workflow for AI/refactor era". |
| Missing OSS files | Contributors and Codex agents lack clear rules. |
| No CI | Cross-platform and safety claims rely on local trust. |
| Temporary comments | Looks like unfinished generated patches. |
| Mixed command execution | Increases audit burden for user-controlled labels/paths. |
| Tag/snapshot ambiguity | Normal release tags can be confused with Snap snapshots. |
| Linux command conflict | `snap` name can collide with Canonical Snapcraft. |
| Safety docs spread across files | Restore/purge/doctor guarantees are harder to verify. |

## 8. Critical Hotspots and Refactor Direction

### 8.1 `src/git_health.rs`

Recommended mode: `stabilize_before_extract`.

Why:

- It owns diagnosis and repair semantics.
- It is large but heavily tested.
- Behavior matters more than LOC reduction.

Near-term action:

- Do not split in Sprint 0-3.
- Improve docs and tests first.
- Add machine-readable `doctor --json` / `--ci` plan before large movement.

### 8.2 `src/utils.rs`

Recommended mode: `audit_then_extract`.

Why:

- It owns snapshot discovery, metadata messages, and shared command helpers.
- It currently scans `refs/tags`, which is central to the release-tag risk.

Near-term action:

- Audit snapshot tag filtering.
- Decide marker/namespace path.
- Introduce tests before changing discovery behavior.

### 8.3 `src/commands/restore.rs` and `src/commands/delete.rs`

Recommended mode: `safety_feature_first`.

Why:

- These commands can discard or purge data.
- User trust depends on clear prompts, dry-run behavior, backups, and tests.

Near-term action:

- Plan `restore --dry-run`.
- Plan rescue snapshot before restore.
- Keep `delete --purge` backup behavior prominent in docs.

### 8.4 `src/commands/new.rs`, `update.rs`, and `edit.rs`

Recommended mode: `command_hardening`.

Why:

- These commands create or rewrite Git tags.
- They include user-facing labels and descriptions.

Near-term action:

- Clean patch comments.
- Replace formatted command strings with explicit args where user input is involved.
- Preserve existing behavior with tests.

### 8.5 Public Docs

Recommended mode: `positioning_first`.

Why:

- Most technical strengths already exist.
- The repo needs to communicate them cleanly before deeper refactor work.

Near-term action:

- Rewrite README top sections.
- Promote `snap doctor`.
- Add AI-agent and beginner workflows.
- Add known limitations and Linux name conflict note.

## 9. Validation Contract

### 9.1 Full Gate Set

For runtime or safety-sensitive changes:

```bash
git diff --check
cargo fmt --check
cargo clippy --all-targets --all-features
cargo test
cargo build --release
```

### 9.2 OSS Readiness Scans

```bash
rg -n "your-username|TODO|FIXME|FINAL|START: THE FIX|END: THE FIX|CORRECTED LINE" . -g '!target'
rg -n "format!\\(\"git |format!\\(\"gh |run_command\\(&format!" src
rg -n "refs/tags|refs/snap|Snap-Snapshot|Snap-Metadata" src doc README.md docs/architecture-audit
```

### 9.3 Docs-Only Gate Set

For docs-only organization work:

```bash
git diff --check
rg -n "stale-path-or-private-absolute-path" docs doc README.md
```

Optional but encouraged if the docs are part of a sprint checkpoint:

```bash
cargo fmt --check
cargo test
```

## 10. Snapshot Discipline

Because Snap itself currently uses Git tags for checkpoints, the refactor should use non-release-looking labels.

Good:

```bash
snap new oss-plan-foundation "Audit and local reference foundations"
snap new oss-s0-baseline "OSS readiness baseline"
snap new oss-s1-readme "README positioning and docs"
```

Avoid:

```bash
snap new v7.3 "sprint 1"
snap new v8.0 "release prep"
```

The `vX` style looks like release tags and conflicts with the exact snapshot-tag model risk this audit tracks.

## 11. Refactor Progress Updates

### Foundation Update — 2026-06-06

Completed:

- Created the Codex OSS master plan.
- Moved audit/planning docs under `docs/architecture-audit/`.
- Created `refactor-progress.md`.
- Created `refactor-plans/README.md`.
- Created ignored `reference-inputs/` area for local roadmap/workflow copies.
- Removed committed-doc dependence on absolute Synthedu paths.
- Created Snap checkpoint `oss-plan-foundation`.

Validation:

| Command | Result |
| --- | --- |
| `git diff --check` | Passed |
| Stale path/reference scan | Passed |
| Git ignored-reference check | Passed |
| `cargo fmt --check` | Passed before this rename/detail pass |
| `cargo test` | Passed before this rename/detail pass, 96 tests |

## 12. Recommended Roadmap

| Sprint | Recommendation | Why |
| --- | --- | --- |
| 0 | Baseline Audit and Safety Rails | Capture exact metrics/gates before mutating README or code. |
| 1 | README Positioning and Public Docs | First impression is the highest-leverage OSS readiness change. |
| 2 | OSS Hygiene Files | Adds maintainer credibility and agent instructions. |
| 3 | CI on Windows/Linux | Makes claims verifiable and future refactors safer. |
| 4 | Source Cleanup and Command Hardening Audit | Removes unprofessional traces and reduces command-boundary risk. |
| 5 | Restore/Doctor/Purge Safety Plan | Strengthens trust where data loss/repair is possible. |
| 6 | Snapshot Tags/Refs Decision | Addresses the central Git integration ambiguity. |
| 7 | Codex/AI-Agent Workflow Docs | Makes the project directly relevant to AI-assisted coding. |
| 8 | Packaging/Name Conflict | Makes installation honest and practical. |
| 9 | Cross-Platform and Performance Proof | Replaces broad claims with measured evidence. |
| 10 | Community Feedback | Produces real OSS signals without artificial hype. |
| 11 | OpenAI Application Package | Submit only after claims are visible and verifiable. |

## 13. Closure Criteria

Close the OSS-readiness refactor when all are true:

- README explains Snap's purpose clearly in under 10 seconds.
- Standard OSS files and templates exist.
- CI runs on Windows and Linux.
- `cargo fmt`, clippy, tests, and release build are green or documented with explicit exceptions.
- Temporary patch comments are cleaned.
- Safety-sensitive commands have clear docs and tests.
- Snapshot/tag model risk is fixed or has a tracked migration plan.
- Linux name conflict is documented.
- A current release exists.
- GitHub About/topics are set.
- There is some real feedback or issue activity.
- The OpenAI/Codex application answers are honest and backed by repository evidence.

Do not continue refactoring only for aesthetics. Once these criteria are met, move remaining work into normal issue-driven maintenance.

## 14. Next Recommendation

Proceed with Sprint 0: Baseline Audit and Safety Rails.

Sprint 0 should:

- write `docs/architecture-audit/refactor-plans/sprint-0-baseline-safety-rails.md`;
- update `docs/architecture-audit/refactor-progress.md`;
- rerun and record full gates, including clippy and release build where feasible;
- update this audit with exact command outputs and any new findings;
- create checkpoint `oss-s0-baseline`.

After Sprint 0, continue to Sprint 1: README Positioning and Public Docs.
