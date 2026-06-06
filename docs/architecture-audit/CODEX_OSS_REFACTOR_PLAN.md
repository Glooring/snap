# Snap Codex OSS Refactor Master Plan

Status: draft master plan  
Target repo: `Glooring/snap`  
Local path: `/home/glooring/projects/snap`  
Prepared from: copied local reference inputs, Snap OSS roadmap, and current repo baseline  
Date: 2026-06-06

## Purpose

This plan prepares Snap to look and operate like a credible open-source Rust CLI project, suitable for Codex/OpenAI open-source promotion and for real users evaluating the repository.

The goal is not to inflate claims. The goal is to make the real project easier to understand, safer to maintain, and more believable as active OSS:

> Snap is a native Rust CLI for Git-powered local project checkpoints. It helps developers create fast save points before risky refactors, AI-agent edits, experiments, or release work, while also helping beginners stop copying entire project folders manually.

This document is the master plan. It should guide the next implementation sprints, not replace sprint-specific plans, progress logs, or release notes.

## Reference Inputs

This plan merges four inputs.

1. Reusable refactor workflow:
   - Local ignored copy: `docs/architecture-audit/reference-inputs/refactor-workflow.md`
   - Original source: Synthedu architecture-audit workflow documentation.
   - Key loop: `plan -> scoped change -> full gates -> progress/audit update -> snap -> next up`
   - Lesson: small sprints with named Snap checkpoints made a long-running refactor reviewable and reversible.

2. Snap OSS roadmap:
   - Local ignored copy: `docs/architecture-audit/reference-inputs/ROADMAP_SNAP_CODEX_OSS.md`
   - Original source: private planning roadmap imported from the Synthedu workspace.
   - Key direction: improve positioning, OSS hygiene, CI, safety docs, code cleanup, tag model, Codex workflow, packaging, and community feedback.

3. Current Snap repo baseline:
   - `Cargo.toml` version is currently `7.2.0`.
   - The CLI already has a broad command surface: `init`, `new`, `list`, `status`, `save`, `push`, `pull`, `sync`, `history`, `branch`, `remote`, `release`, `update-repo`, `setup-repo`, `make-public`, `make-private`, `delete-repo`, `examples`, `restore`, `delete`, `edit`, `update`, `diff`, `doctor`, and `options`.
   - Existing docs include installer, Git health, repair, workflow, and release materials under `doc/`.
   - Existing release scripts and packaging files are present.
   - The largest technical areas are `src/git_health.rs`, `tests/git_health.rs`, `src/git.rs`, and several command modules.
   - Standard OSS root files such as `AGENTS.md`, `CONTRIBUTING.md`, `SECURITY.md`, `CHANGELOG.md`, issue templates, PR template, and GitHub Actions CI were not present in the baseline inspection.
   - Temporary patch comments still exist in places such as `Cargo.toml`, `src/commands/new.rs`, `src/commands/diff.rs`, and `src/commands/update.rs`.
   - Snapshots are still tied to Git tags, so release-looking checkpoint labels can be confused with release tags until namespacing or markers are improved.

4. Official Codex for Open Source criteria:
   - Source: https://openai.com/form/codex-for-oss/
   - The application asks for maintainer role, public GitHub profile/repository, why the repository qualifies, and how API credits would support open-source work.
   - OpenAI describes relevant signals as active maintenance, usage or ecosystem importance, issue triage, PR review, release management, and maintainer workflows.

Before application submission, verify the official page again because program wording and criteria can change.

The copied reference inputs are intentionally ignored by Git. This committed plan and the current audit must stay self-contained so the public Snap repo does not depend on private absolute paths from another project.

## Product Positioning

Snap should be presented as a workflow tool, not as a Git replacement.

Core sentence:

> Git is the engine. Snap is the workflow.

Primary audiences:

1. Developers using AI-assisted coding:
   - AI agents can modify many files quickly.
   - Refactors and migrations need fast rollback points.
   - Recommended flow: `snap new before-codex-task`, run Codex, then inspect with `snap diff` and `snap doctor`.

2. Students, beginners, and indie developers:
   - Many beginners copy whole project folders to save versions.
   - Snap gives them a smaller, safer workflow: `snap init`, `snap new`, `snap list`, `snap restore`.
   - It is a bridge into Git-powered history, not a replacement for learning Git.

3. Maintainers and power users:
   - `snap doctor`, metadata repair, purge backups, release helpers, and GitHub wrappers make Snap more than a toy wrapper.
   - These features need strong docs, tests, and safety language.

Claims to avoid:

- Do not claim Snap replaces Git.
- Do not claim it is faster than Git without measured benchmarks.
- Do not claim broad adoption, critical infrastructure status, or monthly usage without evidence.
- Do not claim the name conflict on Linux is solved until the packaging strategy is decided.

Claims to make clearly:

- Native Rust CLI.
- Git-powered local checkpoints.
- Useful before risky refactors, AI-agent edits, release work, and experiments.
- Beginner-friendly alternative to copying entire project folders.
- Cross-platform intent: Windows, Linux, WSL2.
- Safety-first behavior: confirmations, backups, `doctor` read-only by default.

## Workflow For This Refactor

Use the Synthedu sprint workflow, adapted for a Rust CLI.

The standard loop is:

```text
choose next sprint from evidence
write a plan before mutation
make only scoped changes
run the gates
update progress docs
update the main audit/status doc
create a Snap checkpoint
record next up
```

Every sprint should be small enough to review and rollback. A sprint should have one clear purpose, one acceptance target, and one checkpoint.

Do not continue sprints only to reduce LOC. Continue only when a sprint improves positioning, safety, validation, ownership, user trust, or application readiness.

## Recommended Snap Refactor Files

The operational documents for this refactor live under `docs/architecture-audit/`:

- `docs/architecture-audit/refactor-progress.md`: official sprint-by-sprint journal.
- `docs/architecture-audit/refactor-plans/`: historical plans for individual sprints.
- `docs/architecture-audit/agent-efficiency-oss-readiness-audit-2026-06-06.md`: current audit/status, risk register, metrics, and next recommendation.
- `docs/architecture-audit/reference-inputs/`: local ignored copies of imported planning references.

This master plan should remain at:

- `docs/architecture-audit/CODEX_OSS_REFACTOR_PLAN.md`

The plan is strategic. The progress log is operational. The audit is the current source of truth for status and risk.

## Snap-Specific Gates

For code or behavior changes, use the full Rust gate set:

```bash
git diff --check
cargo fmt --check
cargo clippy --all-targets --all-features
cargo test
cargo build --release
```

Recommended scans before every OSS-readiness checkpoint:

```bash
rg -n "your-username|TODO|FIXME|FINAL|START: THE FIX|END: THE FIX|CORRECTED LINE" . -g '!target'
rg -n "format!\\(\"git |format!\\(\"gh |run_command\\(&format!" src
rg -n "refs/tags|refs/snap|Snap-Snapshot|Snap-Metadata" src doc README.md
```

For docs-only sprints:

```bash
git diff --check
rg -n "your-username|TODO|FIXME|FINAL|START: THE FIX|END: THE FIX|CORRECTED LINE" README.md doc Cargo.toml src -g '!target'
```

For release or packaging sprints, add platform-specific checks only when the environment supports them:

```bash
cargo build --release
cargo make linux-package
pwsh -File build-installers.ps1
```

Do not claim a gate passed unless it was run in the current workspace.

## Snapshot Rule

Use the stable installed Snap binary for checkpointing the Snap repo, not a freshly modified `target/debug/snap`, unless the sprint is intentionally testing the new binary.

Verify before using:

```bash
command -v snap
snap --version
```

Because Snap currently stores snapshots through Git tags, do not use release-looking checkpoint labels such as `v7.3` or `v8.0` while working on Snap itself. Those labels can be confused with release tags and with the exact tag-model risk this plan wants to address.

Use operational labels:

```bash
snap new oss-s0-baseline "OSS readiness baseline"
snap new oss-s1-readme "README positioning sprint"
snap new oss-s2-hygiene "OSS hygiene files"
snap new oss-plan-codex "Codex OSS refactor master plan"
```

After every checkpoint:

```bash
snap list
git status --short
```

If `snap` resolves to the Canonical Snapcraft command on a Linux machine, use an explicit binary path for this project and document the conflict.

## Baseline Findings

The project is stronger than a simple personal script:

- It is a native Rust CLI with a broad command surface.
- It has a real product story: replacing manual folder copies with Git-powered checkpoints.
- It already has snapshot, diff, restore, delete, edit, update, doctor, status, save, sync, branch, remote, release, and examples flows.
- It includes Git health and metadata repair work.
- It has tests, including a large `tests/git_health.rs`.
- It has packaging/release assets for Windows and Linux paths.

The repo still needs OSS-readiness work:

- README positioning should be sharpened for AI-agent workflows, beginners, safety, and "why not just Git".
- The repo needs standard OSS files and templates.
- CI should run on Windows and Linux.
- Temporary patch comments should be cleaned.
- Shell/string command construction should be audited and hardened around user input.
- Snapshot storage should stop treating arbitrary release tags as Snap snapshots, or at minimum use an explicit marker and migration plan.
- `snap doctor`, restore, purge, and metadata behavior need clearer docs and machine-readable modes.
- Linux name conflict with Canonical `snap` needs a documented strategy.
- The project needs a public release/feedback/application checklist.

## Sprint Roadmap

| Sprint | Title | Goal | Acceptance |
| --- | --- | --- | --- |
| 0 | Baseline audit and safety rails | Create Snap-native progress/audit docs, capture metrics, define checkpoint labels, and run baseline gates. | `docs/architecture-audit/refactor-progress.md`, the current architecture audit, and `docs/architecture-audit/refactor-plans/` exist; gates and warnings are recorded; first checkpoint is created. |
| 1 | README positioning and docs rewrite | Make the first 10 seconds of the repo explain Snap clearly. | README has strong headline, AI-agent workflow, beginner workflow, CLI-first rationale, "Why not Git", safety model, known limitations, and `snap doctor` prominence. |
| 2 | OSS hygiene files | Make the repo look maintained and easy to contribute to. | Add `AGENTS.md`, `CONTRIBUTING.md`, `SECURITY.md`, `CHANGELOG.md`, `SUPPORT.md`, PR template, and issue templates. |
| 3 | CI and release quality baseline | Prove the project can be maintained across platforms. | GitHub Actions runs `cargo fmt --check`, `cargo clippy`, and `cargo test` on Ubuntu and Windows; README shows CI status. |
| 4 | Source cleanup and command hardening audit | Remove unprofessional patch traces and identify unsafe command construction. | Temporary comments are cleaned; command string risks are listed or fixed; safety-sensitive areas have issues/tests. |
| 5 | Restore, doctor, and purge safety plan | Strengthen trust around destructive or repair operations. | `restore --dry-run`, rescue snapshot, `doctor --json`, `doctor --ci`, exit codes, and safety docs are implemented or tracked as issues with clear specs. |
| 6 | Snapshot refs and tag model | Resolve or formally plan the release-tag confusion risk. | Decision recorded for tag marker, `refs/tags/snap/<label>`, or dedicated refs; migration path and tests are planned or implemented. |
| 7 | Codex and AI-agent workflow | Make Snap excellent for AI-assisted development workflows. | Add Codex workflow docs, task recipes, non-interactive roadmap, and `AGENTS.md` sensitive-area rules. |
| 8 | Packaging, install, and name conflict | Make installation credible and honest. | Windows/Linux install docs are clear; checksums/release assets are documented; Canonical `snap` conflict strategy is explicit. |
| 9 | Cross-platform and performance proof | Replace broad performance claims with evidence. | Add performance methodology, benchmark scripts or issues, cross-platform docs, and path edge-case tests. |
| 10 | Community launch and feedback | Get real feedback without artificial hype. | GitHub About/topics set; good-first issues created; feedback posts prepared; real issues/stars/feedback tracked honestly. |
| 11 | OpenAI application package | Submit with a truthful, evidence-backed story. | 500-character answers are prepared, repo is public and polished, release is current, and claims are verifiable. |

## Sprint Details

### Sprint 0 - Baseline audit and safety rails

Create the working documents before making broad changes:

- `docs/architecture-audit/refactor-progress.md`
- `docs/architecture-audit/refactor-plans/README.md`
- `docs/architecture-audit/agent-efficiency-oss-readiness-audit-2026-06-06.md`

Record:

- current command surface;
- current docs list;
- missing OSS files;
- CI status;
- test status;
- largest files by LOC;
- known warning scans;
- snapshot/tag risk;
- next sprint recommendation.

Recommended commands:

```bash
git diff --check
cargo fmt --check
cargo clippy --all-targets --all-features
cargo test
cargo build --release
rg -n "your-username|TODO|FIXME|FINAL|START: THE FIX|END: THE FIX|CORRECTED LINE" . -g '!target'
```

Checkpoint:

```bash
snap new oss-s0-baseline "OSS readiness baseline"
```

### Sprint 1 - README positioning and docs rewrite

Rewrite the top of README so a new visitor understands Snap quickly.

Required sections:

- What Snap is.
- Why Snap exists.
- AI-agent workflow.
- For students and beginners.
- Why not just Git.
- Why CLI-first.
- Safety model.
- Known limitations.
- How Snap stores snapshots.
- `snap doctor` overview.

Dedicated docs to add or plan:

- `doc/AI_AGENT_WORKFLOW.md`
- `doc/BEGINNER_WORKFLOW.md`
- `doc/WHY_NOT_GIT.md`
- `doc/SAFETY_MODEL.md`
- `doc/SNAP_DOCTOR.md`
- `doc/KNOWN_LIMITATIONS.md`

Do not oversell. Explain that Snap uses Git, creates local commits and snapshot tags, and has a Linux command-name conflict risk.

Checkpoint:

```bash
snap new oss-s1-readme "README positioning and docs"
```

### Sprint 2 - OSS hygiene files

Add standard project files:

- `AGENTS.md`
- `CONTRIBUTING.md`
- `SECURITY.md`
- `CHANGELOG.md`
- `SUPPORT.md`
- optional `CODE_OF_CONDUCT.md`
- `.github/ISSUE_TEMPLATE/bug_report.yml`
- `.github/ISSUE_TEMPLATE/feature_request.yml`
- `.github/ISSUE_TEMPLATE/doctor_report.yml`
- `.github/pull_request_template.md`

`AGENTS.md` should tell agents:

- Snap is a safety-focused Rust CLI over Git.
- Run fmt, clippy, and tests before submitting.
- Be extra careful in restore, delete, doctor, Git health, metadata, command execution, and path handling.
- Do not change destructive behavior without tests.

Checkpoint:

```bash
snap new oss-s2-hygiene "OSS hygiene files"
```

### Sprint 3 - CI and release quality baseline

Add GitHub Actions CI:

- Ubuntu latest.
- Windows latest.
- stable Rust.
- rustfmt and clippy components.
- Cargo cache.
- `cargo fmt --check`.
- `cargo clippy --all-targets --all-features`.
- `cargo test`.

If clippy is too noisy, record the baseline and create a strictness plan. The target state is clippy clean.

Add README badge only after workflow path is stable.

Checkpoint:

```bash
snap new oss-s3-ci "Windows and Linux CI"
```

### Sprint 4 - Source cleanup and command hardening audit

Clean temporary patch comments:

```bash
rg -n "FINAL|CORRECT|START|END|THE FIX|CORRECTED LINE" Cargo.toml src doc README.md
```

Replace patch artifacts with either no comment or a technical comment that explains the invariant.

Audit risky command construction:

- `git tag` creation and rewriting.
- tag deletion.
- restore/reset flows.
- purge bundle creation.
- GitHub CLI calls.
- shell/runtime calls in release helpers.

Prefer explicit argument APIs for user-controlled input:

```rust
Command::new("git").args(["tag", "-a", tag_name, "-F", "-"])
```

Checkpoint:

```bash
snap new oss-s4-cleanup "source cleanup and command audit"
```

### Sprint 5 - Restore, doctor, and purge safety plan

Prioritize safety-sensitive UX:

- `snap restore --dry-run`
- rescue snapshot before restore;
- `snap doctor --json`;
- `snap doctor --ci`;
- documented exit codes;
- expanded safety docs for restore, delete, purge, and repair.

If implementation is too large, create detailed issues with acceptance criteria. Do not leave safety risks only in this master plan.

Checkpoint:

```bash
snap new oss-s5-safety "restore doctor purge safety plan"
```

### Sprint 6 - Snapshot refs and tag model

Resolve the central Git integration risk: arbitrary Git tags should not be silently treated as Snap snapshots.

Decision options:

1. Add marker in annotated tag message:
   - Example: `Snap-Snapshot: true`
   - Easier migration.
   - Keeps normal Git tags out of `snap list`.

2. Use `refs/tags/snap/<label>`:
   - Clear Git visibility.
   - Avoids top-level release tag confusion.
   - Needs display cleanup and migration.

3. Use dedicated refs such as `refs/snapshots/<label>`:
   - Clean internal model.
   - Less visible in standard Git/GitHub UI.

Recommended path:

1. Add explicit marker and filter `snap list` by marker.
2. Document current compatibility behavior.
3. Plan namespaced refs or migration command after marker tests are stable.

Checkpoint:

```bash
snap new oss-s6-snapshot-model "snapshot refs and tag model"
```

### Sprint 7 - Codex and AI-agent workflow

Make Snap naturally useful to AI-assisted coding workflows.

Add:

- `doc/CODEX_WORKFLOW.md`
- `doc/CODEX_TASKS.md`
- AI-agent examples in README.
- non-interactive roadmap for `doctor --ci`, `restore --dry-run`, and safe confirmations.

Do not add dangerous `--yes` behavior casually. If planned, specify exact safeguards and tests.

Checkpoint:

```bash
snap new oss-s7-codex "Codex and AI-agent workflow docs"
```

### Sprint 8 - Packaging, install, and name conflict

Document install paths clearly.

Required:

- Windows install path.
- Linux install path.
- WSL2 notes.
- how to verify binary.
- release asset naming.
- checksums plan.
- Canonical `snap` conflict note.

Do not rename the project in this sprint unless a separate decision record exists. It is acceptable to keep project name `Snap` while evaluating an alternate Linux binary name such as `gitsnap`.

Checkpoint:

```bash
snap new oss-s8-packaging "packaging and name conflict docs"
```

### Sprint 9 - Cross-platform and performance proof

Replace vague speed claims with measured methodology.

Add or plan:

- `doc/PERFORMANCE.md`
- `doc/CROSS_PLATFORM.md`
- `scripts/benchmark.sh`
- `scripts/benchmark.ps1`
- tests for paths with spaces;
- tests for Unicode paths;
- tests for nested empty directories;
- Windows readonly/hidden behavior where CI supports it.

Checkpoint:

```bash
snap new oss-s9-platform "cross-platform and performance proof"
```

### Sprint 10 - Community launch and feedback

Prepare the repository for real feedback.

Actions:

- Update GitHub About description.
- Add topics: `rust`, `cli`, `git`, `snapshot`, `developer-tools`, `version-control`, `ai-assisted-coding`, `codex`, `wsl2`, `windows`, `linux`.
- Create good-first issues.
- Create safety/hardening issues.
- Prepare feedback posts without asking for artificial stars.

Checkpoint:

```bash
snap new oss-s10-community "community feedback readiness"
```

### Sprint 11 - Final OpenAI application package

Prepare application answers only after the repo supports the claims.

Recommended role:

```text
Primary maintainer
```

Short qualification answer:

```text
Snap is an MIT-licensed Rust CLI for Git-powered local checkpoints. It helps beginners stop copying project folders manually and helps developers save before risky refactors, AI-agent edits, or releases. It supports Windows/Linux/WSL2, diff/restore, and snap doctor for Git/snapshot metadata health checks. I am the primary maintainer.
```

API credits answer:

```text
I will use credits for OSS maintenance: issue triage, tests, docs, release automation, and review of safety-sensitive changes. For Snap, I will improve CI, restore safety, doctor JSON/CI output, namespaced refs, packaging, and cross-platform tests. All AI-assisted changes will be manually reviewed and tested.
```

Final checkpoint:

```bash
snap new oss-s11-application "OpenAI application package"
```

## Risk Register

| Risk | Why it matters | Mitigation |
| --- | --- | --- |
| Linux command-name conflict | `snap` already exists on many Linux systems as Canonical Snapcraft. | Document clearly, detect conflicts, evaluate alternate binary name. |
| Snapshot tags vs release tags | Current model can confuse normal Git release tags with Snap snapshots. | Add marker or namespace, write migration tests, avoid release-looking checkpoint labels. |
| Destructive operations | Restore, purge, and repair can lose data if wrong. | Confirmations, dry-run, rescue snapshot, backups, tests, safety docs. |
| Command string construction | User input in shell commands can be risky. | Prefer explicit argument APIs, audit command helpers, add tests. |
| Overstated OSS claims | Application can look opportunistic if claims exceed evidence. | Be honest, collect real feedback, document usage and maintenance activity. |
| CI gaps | Cross-platform claims need proof. | Add Windows and Ubuntu CI. |
| Documentation drift | Large docs can become stale. | Keep README concise and point to focused docs. |

## Definition Of Done For The Refactor Program

Close this refactor program when all are true:

- README explains Snap clearly in under 10 seconds.
- README and docs include AI-agent workflow, beginner workflow, safety model, known limitations, and "why not just Git".
- `AGENTS.md`, contributing docs, security policy, changelog, issue templates, and PR template exist.
- CI runs on Windows and Ubuntu.
- `cargo fmt --check`, clippy, tests, and release build are green or documented with intentional exceptions.
- Temporary patch comments are cleaned.
- Snapshot/tag model risk is fixed or tracked with a concrete migration issue.
- Restore, purge, and doctor safety behavior is documented and tested.
- Packaging and Linux name conflict are documented.
- A current release exists.
- GitHub About/topics are set.
- There is at least some real feedback or issue activity.
- OpenAI application answers are truthful and backed by visible repo evidence.

Do not continue the refactor only to polish endlessly. Once these criteria are met, future work should move into normal issue-driven maintenance.

## Immediate Next Step

Start with Sprint 0:

1. Update `docs/architecture-audit/refactor-progress.md`.
2. Create `docs/architecture-audit/refactor-plans/README.md`.
3. Update `docs/architecture-audit/agent-efficiency-oss-readiness-audit-2026-06-06.md`.
4. Run baseline gates.
5. Record exact warnings and risks.
6. Create checkpoint:

```bash
snap new oss-s0-baseline "OSS readiness baseline"
```

Then proceed to Sprint 1: README positioning and docs rewrite.
