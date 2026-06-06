# Snap OSS Readiness Audit - Agent Efficiency and CLI Health

## 1. Metadata

| Field | Value |
| --- | --- |
| Repository | `Glooring/snap` |
| Local path | `/home/glooring/projects/snap` |
| Audit date | `2026-06-06` |
| Last refreshed | `2026-06-07T01:27:18+03:00` during Sprint 7 |
| Branch inspected | `main` |
| Commit inspected | `d0460ca72a057e4c29c8f9bb03e24f47684d4cc1` before Sprint 7 Codex docs edits |
| Current audit path | `docs/architecture-audit/agent-efficiency-oss-readiness-audit-2026-06-06.md` |
| Master plan | `docs/architecture-audit/CODEX_OSS_REFACTOR_PLAN.md` |
| Progress log | `docs/architecture-audit/refactor-progress.md` |
| Historical sprint plans | `docs/architecture-audit/refactor-plans/` |
| Local ignored references | `docs/architecture-audit/reference-inputs/` |
| Scope | Current OSS-readiness status after Sprint 7 Codex and AI-agent workflow docs. Runtime behavior was validated with source-built Snap only. |

## 2. Executive Verdict

Snap is already a substantial Rust CLI with a real product story. It is not merely a one-off backup script: it has Git-backed checkpoints, snapshot metadata, restore/diff/delete/edit/update flows, Git health diagnostics, branch/remote/GitHub helpers, release helpers, installer assets, and a meaningful integration test suite.

The strongest public positioning is:

> Snap is a native Rust CLI for Git-powered local checkpoints before risky refactors, AI-agent edits, experiments, and release work. It also helps beginners stop copying entire project folders manually.

The main risk is public readiness. A new user, contributor, or Codex/OpenAI reviewer should understand the value and safety model from the repository itself, without reading old prompt dumps or relying on private context.

The refactor should therefore be an **OSS-readiness and maintainer-quality program**, not an open-ended code cleanup. The highest-value sequence is:

1. lock the baseline with audit/progress/gates;
2. fix positioning and README/docs;
3. add OSS maintainer files and CI;
4. clean obvious unprofessional traces;
5. harden and document safety-sensitive command behavior;
6. address the snapshot-tag model and Linux command-name conflict;
7. prepare a truthful application package.

## 3. Current Baseline Evidence

| Command / inspection | Result | Meaning |
| --- | --- | --- |
| `git rev-parse --abbrev-ref HEAD` | `main` | Current working branch. |
| `git rev-parse HEAD` | `d0460ca72a057e4c29c8f9bb03e24f47684d4cc1` | Starting commit before Sprint 7 Codex docs edits. |
| `git log --oneline -8` | `oss-s6-snapshot-model`, `oss-s5-safety`, `oss-s4-cleanup`, `oss-s3-ci`, `oss-s2-hygiene`, `oss-s1-readme`, `oss-s0-baseline`, `oss-sandbox-test-policy` | OSS-readiness planning has named Snap checkpoints. |
| `git status --short` | Clean before Sprint 7 Codex docs edits | Repo was clean after the Sprint 6 checkpoint. |
| `rustc --version` | `rustc 1.95.0 (59807616e 2026-04-14)` | Rust toolchain used for Sprint 0 validation. |
| `cargo --version` | `cargo 1.95.0 (f2d3ce0bd 2026-03-21)` | Cargo toolchain used for Sprint 0 validation. |
| `git diff --check` | Passed | Whitespace gate is clean. |
| `cargo fmt --check` | Passed | Rust formatting is clean. |
| `cargo clippy --all-targets --all-features` | Passed with no warnings in Sprint 6 | Sprint 7 was docs-only; no Rust changed. |
| `cargo clippy --all-targets --all-features -- -D warnings` | Passed in Sprint 6 | Sprint 7 was docs-only; strict Clippy baseline remains expected. |
| `cargo test` | Passed, 105 tests | Test suite is fast and substantial. |
| `cargo build --release` | Passed | Release build works locally. |
| `cargo run -- --help` | Passed | Source-built debug binary exposes the modern broad command surface. |
| `cargo run -- doctor` | Passed | Source-built debug binary sees the repo as healthy. |
| `./target/release/snap --help` | Passed | Source-built release binary exposes the modern broad command surface. |
| `./target/release/snap doctor` | Passed | Current source-built release binary sees the repo as healthy with 13 snapshot tags checked; this is the correct behavior baseline for the project. |
| Source-built sandbox smoke tests | Passed in Sprint 0 and Sprint 5 | Disposable local repos exercised baseline snapshot behavior plus restore dry-run/rescue, doctor JSON/CI, and purge backup behavior. |
| README positioning | Sprint 1 completed | README now leads with Git-powered local checkpoints, AI-agent edits, risky refactors, beginner workflows, safety, limitations, and source command surface. |
| Public docs | Sprint 1 completed | Added focused docs for AI-agent workflow, beginner workflow, why not Git, safety model, `snap doctor`, and known limitations. |
| OSS root files | Sprint 2 completed | Added `AGENTS.md`, `CONTRIBUTING.md`, `SECURITY.md`, `CHANGELOG.md`, `SUPPORT.md`, `CODE_OF_CONDUCT.md`, and `LICENSE`. |
| GitHub templates | Sprint 2 completed | Added PR template plus bug, feature, and doctor issue templates. |
| CI workflow | Sprint 3 completed | Added `.github/workflows/ci.yml` for Ubuntu and Windows fmt, Clippy, tests, release build, source-built help, and source-built doctor. |
| Strict Clippy cleanup | Sprint 4 completed | Fixed the 9 baseline warnings and made `-D warnings` pass. |
| Temporary patch comments | Sprint 4 completed | Removed patch-marker comments from `Cargo.toml` and `src`. |
| Command construction audit | Sprint 4 completed | Added `docs/architecture-audit/command-construction-audit-2026-06-07.md` and converted obvious formatted Git commands to argv calls. |
| Restore safety | Sprint 5 completed | Added `restore --dry-run`, default rescue snapshots, and `restore --no-rescue`. |
| Doctor automation | Sprint 5 completed | Added `doctor --json`, `doctor --ci`, and documented exit behavior. |
| Purge safety docs | Sprint 5 completed | README/safety docs now describe purge backups, reachability refusal, metadata pinning, and final health checks. |
| Snapshot marker decision | Sprint 6 completed | New source-built snapshots include `Snap-Snapshot: true`; legacy Snap tags remain compatible. |
| Release tag filtering | Sprint 6 completed | Snapshot discovery and doctor ignore ordinary release tags that are not Snap-compatible. |
| Codex workflow docs | Sprint 7 completed | Added `doc/CODEX_WORKFLOW.md`, `doc/CODEX_TASKS.md`, refreshed AI-agent docs, README links, and AGENTS guidance. |
| `cargo audit` | Not installed | Security audit is not available locally yet; add later if desired. |
| Root OSS file inspection | 7 standard files found | Sprint 2 added the expected root hygiene files. |
| `.github` inspection | 5 files found | Issue/PR templates and CI workflow exist. |
| Temporary/placeholder scan | Open findings | `Packager.toml` and old prompt dumps still need a later public-doc cleanup decision. |
| Command construction scan | Sprint 4 partially addressed | Obvious formatted Git commands are hardened; remaining dynamic boundaries are documented. |
| Snapshot metadata scan | Marker-first model | Snapshot discovery still scans `refs/tags`, but filters to marked, metadata-bearing, or legacy Snap-style tags; metadata uses `Snap-Metadata-Ref` and `refs/snap-metadata`. |

## 4. Gate Details

### 4.1 Passing Gates

The following gates are currently green:

```bash
git diff --check
cargo fmt --check
cargo clippy --all-targets --all-features
cargo test
cargo build --release
cargo run -- --help
cargo run -- doctor
./target/release/snap --help
./target/release/snap doctor
```

`cargo test` result:

```text
96 passed; 0 failed; 0 ignored
```

`./target/release/snap doctor` result summary:

```text
Git repository looks healthy.
Snapshot tags: 10 checked, 0 invalid
Snapshot metadata: 10 checked, 0 active invalid, 0 historical invalid, 0 unpinned
```

Sprint 0 also ran one disposable local smoke test using only the source-built release binary at `/home/glooring/projects/snap/target/release/snap`. The sandbox was `/tmp/snap-agent-smoke-s0-hwmvPg` and was removed after validation. It exercised `git init`, an initial commit, `snap init`, two `snap new` calls, `snap list`, `snap diff`, `snap doctor`, and `snap restore s0-first`. Restore verified that `app.txt` returned to `first` and `extra.txt` was removed.

### 4.2 Strict Clippy Baseline

Strict Clippy now passes:

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

Sprint 4 fixed the previous failure categories:

| File | Finding |
| --- | --- |
| `src/commands/branch.rs` | `clippy::print_literal` |
| `src/commands/list.rs` | `clippy::useless_format` |
| `src/commands/options.rs` | `clippy::useless_format`, `clippy::useless_vec` |
| `src/commands/restore.rs` | `clippy::unnecessary_sort_by` |
| `src/config.rs` | `clippy::derivable_impls` |
| `src/git_health.rs` | `clippy::nonminimal_bool`, `clippy::len_zero` |

Strict Clippy is now a credible future CI target.

### 4.3 Missing Gate

`cargo audit` is not installed:

```text
error: no such command: `audit`
```

This should not block Sprint 0, but a later security/OSS hygiene sprint can decide whether to add `cargo-audit` to local instructions or CI.

### 4.4 Sprint 1 Docs Validation

Sprint 1 was documentation-only and ran:

```bash
git diff --check
cargo fmt --check
cargo clippy --all-targets --all-features
cargo test
cargo build --release
./target/release/snap --help
./target/release/snap doctor
```

Results:

- `git diff --check` passed.
- `cargo fmt --check` passed.
- normal Clippy passed with the known 9 warnings.
- `cargo test` passed, 96 integration tests.
- `cargo build --release` passed.
- source-built release help showed the modern broad command surface.
- source-built release doctor reported a healthy repo with 7 snapshot tags and 7 metadata refs checked.
- README/new-doc stale phrase scan passed for `Snap (The Rust Edition)`, `blazing-fast`, `your-username`, and stale private path markers.

Strict Clippy with `-D warnings` was not a Sprint 1 gate because that remains known baseline debt.

### 4.5 Sprint 2 OSS Hygiene Validation

Sprint 2 was documentation/template-only and ran:

```bash
git diff --check
cargo fmt --check
cargo clippy --all-targets --all-features
cargo test
cargo build --release
./target/release/snap --help
./target/release/snap doctor
```

Results:

- `git diff --check` passed.
- `cargo fmt --check` passed.
- normal Clippy passed with the known 9 warnings.
- `cargo test` passed, 96 integration tests.
- `cargo build --release` passed.
- source-built release help showed the modern broad command surface.
- source-built release doctor reported a healthy repo with 8 snapshot tags and 8 metadata refs checked.
- root OSS file existence checks passed.
- `.github` template listing found 4 files.
- `AGENTS.md` safety/binary-rule scan passed.
- contributor validation-command scan passed.

Strict Clippy with `-D warnings` was not a Sprint 2 gate because that remains known baseline debt.

### 4.6 Sprint 3 CI Validation

Sprint 3 was CI/documentation-only and ran:

```bash
git diff --check
cargo fmt --check
cargo clippy --all-targets --all-features
cargo test
cargo build --release
./target/release/snap --help
./target/release/snap doctor
```

Results:

- `git diff --check` passed.
- `cargo fmt --check` passed.
- normal Clippy passed with the known 9 warnings.
- `cargo test` passed, 96 integration tests.
- `cargo build --release` passed.
- source-built release help showed the modern broad command surface.
- source-built release doctor reported a healthy repo with 9 snapshot tags and 9 metadata refs checked.
- workflow existence/content scan passed.
- README CI badge scan passed.
- `actionlint` was not installed locally.
- Ruby YAML syntax parser was not available locally.

Strict Clippy with `-D warnings` was not a Sprint 3 gate because that remains known baseline debt.

### 4.7 Sprint 4 Source Cleanup Validation

Sprint 4 changed Rust source in narrow cleanup/hardening paths and ran:

```bash
git diff --check
cargo fmt --check
cargo clippy --all-targets --all-features
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo build --release
./target/release/snap --help
./target/release/snap doctor
```

Results:

- `git diff --check` passed.
- `cargo fmt --check` passed.
- normal Clippy passed with no warnings.
- strict Clippy passed with `-D warnings`.
- `cargo test` passed, 96 integration tests.
- `cargo build --release` passed.
- source-built release help showed the modern broad command surface.
- source-built release doctor reported a healthy repo with 10 snapshot tags and 10 metadata refs checked.
- patch-marker scan for `Cargo.toml` and `src` passed with no matches.
- formatted Git command scan passed with no `run_command(&format!(...))` matches.
- remaining dynamic command boundaries are documented in `docs/architecture-audit/command-construction-audit-2026-06-07.md`.

### 4.8 Sprint 5 Restore/Doctor/Purge Safety Validation

Sprint 5 changed Rust source in restore, doctor, health-report serialization, CLI help, docs, and tests. It ran:

```bash
git diff --check
cargo fmt --check
cargo clippy --all-targets --all-features
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo build --release
./target/release/snap --help
./target/release/snap restore --help
./target/release/snap doctor --help
./target/release/snap doctor
```

Results:

- `git diff --check` passed.
- `cargo fmt --check` passed.
- normal Clippy passed with no warnings.
- strict Clippy passed with `-D warnings`.
- `cargo test` passed, 101 integration tests.
- `cargo build --release` passed.
- source-built top-level help showed restore dry-run and doctor JSON/CI examples.
- source-built `restore --help` showed `--dry-run` and `--no-rescue`.
- source-built `doctor --help` showed `--json` and `--ci`.
- source-built release doctor reported a healthy repo with 11 snapshot tags and 11 metadata refs checked.

Sprint 5 also ran a disposable local smoke test at `/tmp/snap-agent-smoke-s5-jLGMAo` using only `/home/glooring/projects/snap/target/release/snap`. It exercised `init`, `new`, `restore --dry-run`, rescue-backed `restore`, `doctor --json`, `doctor --ci`, `delete --purge`, and final `doctor --ci`. It verified that dry-run was read-only, a `snap-rescue-20260607-011015` tag preserved dirty/untracked work, restore returned `app.txt` to `one` and removed `extra.txt`, doctor JSON reported `"status": "ok"`, purge created a bundle backup, and final doctor CI passed. The sandbox was removed after validation. No GitHub sandbox repositories were created.

### 4.9 Sprint 6 Snapshot Tag Model Validation

Sprint 6 changed snapshot tag-message creation, snapshot discovery filtering, doctor snapshot scanning, docs, and tests. It ran:

```bash
git diff --check
cargo fmt --check
cargo clippy --all-targets --all-features
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo build --release
./target/release/snap --help
./target/release/snap list
./target/release/snap doctor
```

Results:

- `git diff --check` passed.
- `cargo fmt --check` passed.
- normal Clippy passed with no warnings.
- strict Clippy passed with `-D warnings`.
- `cargo test` passed, 105 integration tests.
- `cargo build --release` passed.
- source-built top-level help passed.
- source-built `snap list` showed all 12 legacy OSS-readiness checkpoints, proving compatibility with unmarked global-checkpoint tags.
- source-built release doctor reported a healthy repo with 12 snapshot tags and 12 metadata refs checked.

Sprint 6 also ran a disposable local smoke test at `/tmp/snap-agent-smoke-s6-CAPvOe` using only `/home/glooring/projects/snap/target/release/snap`. It exercised `init`, `new`, tag-message inspection, plain release tag creation, legacy unmarked Snap-style tag creation, `list`, `doctor --json`, and `doctor --ci`. It verified that `s6-one` contained `Snap-Snapshot: true`, `snap list` showed `s6-one` and `legacy-one`, `snap list` did not show `release-1`, doctor JSON reported `"snapshot_count": 2`, and doctor CI passed. The sandbox was removed after validation. No GitHub sandbox repositories were created.

### 4.10 Sprint 7 Codex Workflow Docs Validation

Sprint 7 was documentation-focused and changed README, AGENTS, AI-agent docs, and new Codex workflow/task docs. It ran:

```bash
git diff --check
cargo fmt --check
cargo test
cargo build --release
./target/release/snap --help
./target/release/snap doctor
rg -n "Codex|AI-agent|doctor --json --ci|restore --dry-run|Snap-Snapshot|--yes" README.md AGENTS.md doc/CODEX_WORKFLOW.md doc/CODEX_TASKS.md doc/AI_AGENT_WORKFLOW.md
```

Results:

- `git diff --check` passed.
- `cargo fmt --check` passed.
- `cargo test` passed, 105 integration tests.
- `cargo build --release` passed.
- source-built top-level help passed.
- source-built release doctor reported a healthy repo with 13 snapshot tags and 13 metadata refs checked.
- targeted Codex/AI-agent docs scan passed and found expected safety terms plus the explicit warning against broad destructive `--yes` behavior.
- No GitHub sandbox repositories were created because Sprint 7 was documentation-only.

## 5. Repository Metrics Snapshot

### 5.1 File Counts

| Area | Count |
| --- | ---: |
| `src/**/*.rs` | 33 |
| `tests/**/*.rs` | 1 |
| `doc/*.md` | 16 after adding Codex workflow docs |
| `docs/**/*.md` | 13 after adding the Sprint 7 plan |
| `.github` tracked files | 5 |
| Standard OSS root files found | 7 |

Standard OSS root files checked:

- `AGENTS.md`
- `CONTRIBUTING.md`
- `SECURITY.md`
- `CHANGELOG.md`
- `SUPPORT.md`
- `CODE_OF_CONDUCT.md`
- `LICENSE` / `LICENSE.md`

### 5.2 LOC Snapshot

| Area | LOC |
| --- | ---: |
| `src` Rust | 6,786 after Sprint 6 marker/filtering changes |
| `tests` Rust | 2,932 after Sprint 6 snapshot/tag tests |
| `doc` Markdown/text | 5,419 after Sprint 7 Codex docs |
| `docs` audit Markdown | 3,126 after Sprint 7 plan/progress/audit updates |

### 5.3 Largest Operational Files

| Rank | File | LOC | Primary concern |
| ---: | --- | ---: | --- |
| 1 | `tests/git_health.rs` | 2,932 | Valuable integration coverage, but already large enough to need section ownership if expanded. |
| 2 | `src/git_health.rs` | 1,042 | High-risk Git diagnosis/repair logic; behavior changes need tests and docs. |
| 3 | `src/git.rs` | 487 | Git wrapper and remote metadata sync boundary. |
| 4 | `src/commands/release.rs` | 471 | Release scripting and GitHub CLI boundary; cross-platform assumptions matter. |
| 5 | `src/commands/doctor.rs` | 565 | User-facing diagnosis/repair/automation UI; must stay read-only unless repair is explicit. |
| 6 | `src/cli.rs` | 438 | Public command contract and help text. |
| 7 | `src/utils.rs` | 459 | Snapshot discovery, marker filtering, metadata, and command helpers; central to tag/ref migration. |
| 8 | `src/commands/delete.rs` | 380 | Destructive snapshot/purge workflow. |
| 9 | `src/commands/list.rs` | 318 | Snapshot discovery/presentation, affected by future tag filtering. |
| 10 | `src/commands/remote.rs` | 304 | GitHub CLI and destructive remote operations. |
| 11 | `src/github.rs` | 282 | GitHub CLI integration and release parsing. |
| 12 | `src/commands/branch.rs` | 207 | Branch workflow wrapper. |
| 13 | `src/commands/options.rs` | 170 | Global Snap options persistence. |
| 14 | `src/commands/edit.rs` | 156 | Tag/message rewrite behavior. |
| 15 | `src/commands/restore.rs` | 324 | Destructive restore workflow with dry-run/rescue behavior. |
| 16 | `src/commands/new.rs` | 136 | Snapshot creation, labels, tags, metadata pinning. |
| 17 | `src/commands/update.rs` | 133 | Snapshot amend/update and tag rewrite behavior. |
| 18 | `src/commands/diff.rs` | 123 | Snapshot comparison, important for AI-agent workflow. |

## 6. Product And CLI Baseline

### 6.1 Source Command Surface

The current source-built binary exposes a broad command surface:

```text
init
new
list
status
save
push
pull
sync
history
branch
remote
release
update-repo
setup-repo
make-public
make-private
delete-repo
examples
restore
delete
edit
update
diff
doctor
options
```

The source help also includes workflow groups:

```text
Daily workflow
Snapshots
Branches
Remote/GitHub
Release
Diagnostics
Learn by example
```

This is a strength. The README should be updated to reflect this modern command surface instead of presenting Snap mostly as the older snapshot-only tool.

### 6.2 Global Checkpoint Binary vs Source-Built Product

The globally installed Snap binary at `/usr/local/bin/snap` is intentionally an older stable tool in the user's environment. It is useful for checkpointing this refactor, but it is not the binary that should be used to validate the current project behavior.

Sprint 0 product validation did not rely on global `snap --help`, global `snap doctor`, or global `snap --version` as product evidence. The source-built `./target/release/snap --help` lists the newer command surface. This matters because the refactor workflow uses the global Snap binary for checkpoints, while validation must use the source-built binary from this repo:

```bash
cargo test
cargo build --release
./target/release/snap --help
./target/release/snap doctor
```

Do not use the global `snap` binary to prove features from the current source tree.

### 6.3 Binary Sizes

Local inspected binaries:

| Binary | Approx size | Note |
| --- | ---: | --- |
| `/usr/local/bin/snap` | 1.7 MB | Older global binary used only for refactor checkpoints. |
| `target/release/snap` | 1.9 MB | Current source release build. |
| `target/debug/snap` | 33 MB | Debug build. |

## 7. Architecture Map

### 7.1 Runtime Shape

```text
snap
├── CLI parser and command definitions
├── command modules
│   ├── daily workflow: status, save, pull, push, sync, history
│   ├── snapshots: init, new, list, diff, restore, delete, edit, update
│   ├── Git health: doctor and git_health internals
│   ├── GitHub/remote: remote, setup-repo, visibility, delete-repo
│   └── release: local scripts and GitHub release upload/list
├── Git command wrappers and utilities
├── metadata capture/storage
├── OS-specific hidden/readonly behavior
└── integration tests
```

### 7.2 Domain Ownership

| Domain | Primary files | Current state |
| --- | --- | --- |
| CLI contract | `src/cli.rs`, `src/main.rs` | Broad and useful, but README/install docs lag behind source help. |
| Snapshot creation/update | `src/commands/new.rs`, `update.rs`, `edit.rs`, `utils.rs` | Core model; tied to Git commits, tags, and metadata refs. |
| Snapshot listing/diff | `src/commands/list.rs`, `diff.rs`, `utils.rs` | Important for AI-agent compare/rollback workflow. |
| Restore/delete safety | `src/commands/restore.rs`, `delete.rs` | Destructive flows; should get dry-run/rescue docs/features. |
| Doctor/Git health | `src/commands/doctor.rs`, `src/git_health.rs` | Strong differentiator; should be promoted and eventually get JSON/CI modes. |
| Git helpers | `src/git.rs`, `src/utils.rs` | Safety-sensitive command boundary. |
| GitHub/remote | `src/github.rs`, `src/commands/remote.rs`, `setup_repo.rs` | Useful, but needs clearer auth/error docs. |
| Release | `src/commands/release.rs`, `scripts/*`, installer files | Good foundation, but public release workflow needs polish/checksums. |
| Config/options | `src/config.rs`, `src/commands/options.rs` | User preference state; clippy cleanup is simple here. |
| OS metadata | `src/os/*`, metadata helpers | Cross-platform claim; should be backed by Windows/Linux CI. |
| Tests | `tests/git_health.rs` | Valuable, broad coverage; could be sectioned later if it grows. |

## 8. Dependency And Packaging Baseline

### 8.1 Cargo Dependencies

Direct runtime dependencies:

```text
anyhow
chrono
clap
colored
hex
inquire
rayon
serde
serde_json
sha1
shlex
walkdir
```

Unique normal dependency tree count from `cargo tree -e normal`: 71 packages.

Dev dependencies:

```text
assert_cmd
assert_fs
predicates
```

### 8.2 Packaging Assets

Existing packaging/release assets:

- `Makefile.toml`
- `Packager.toml`
- `build-installers.ps1`
- `scripts/release-linux.sh`
- `scripts/release-windows.ps1`
- `snap.nsi`
- `wix/main.wxs`
- `wix/License.rtf`
- `README_INSTALLER.md`
- `doc/BUILD_INSTALLERS_WINDOWS_WSL.md`

Open packaging issues:

- `Packager.toml` still uses placeholder identifier `com.yourname.snap`.
- Cargo license is `MIT`, but no root `LICENSE`/`LICENSE.md` file was found.
- Linux command-name conflict with Canonical `snap` is not yet handled as a clear install policy.
- Release docs should explain checksums and artifact provenance.

## 9. OSS Readiness Gaps

### 9.1 Missing Standard Files

Sprint 2 added:

- `AGENTS.md`
- `CONTRIBUTING.md`
- `SECURITY.md`
- `CHANGELOG.md`
- `SUPPORT.md`
- `CODE_OF_CONDUCT.md`
- `LICENSE` / `LICENSE.md`
- `.github/ISSUE_TEMPLATE/bug_report.yml`
- `.github/ISSUE_TEMPLATE/feature_request.yml`
- `.github/ISSUE_TEMPLATE/doctor_report.yml`
- `.github/pull_request_template.md`

Still missing:

- release/checksum automation polish
- issue-driven cleanup records for strict Clippy, tag/ref model, and command hardening

### 9.2 README Positioning Gaps

Sprint 1 replaced the old README opening that led with:

- `Snap (The Rust Edition)`
- `blazing-fast`
- Rust rewrite history
- snapshot/backup framing

The README now leads with:

- Git-powered local checkpoints;
- safety before AI-agent edits and risky refactors;
- beginner alternative to copying folders;
- Git is the engine, Snap is the workflow;
- `snap doctor` as a serious differentiator.

The Rust rewrite story is no longer the first impression. The README also links focused public docs for AI-agent workflow, beginner workflow, why not Git, safety model, `snap doctor`, and known limitations.

### 9.3 Old Prompt / Historical Docs

Tracked `doc/prompt-*.txt` files contain old snippets and placeholders such as `your-username`. These may be useful historically, but they are noisy for a public OSS repo.

Options:

1. move them into an ignored local notes folder;
2. rewrite them into clean historical docs;
3. delete them after extracting useful content.

Do not leave placeholder GitHub links in public docs before application.

## 10. Risk Register

| Risk | Severity | Status | Evidence | Recommended sprint |
| --- | --- | --- | --- | --- |
| Public positioning is unclear | High | Addressed in Sprint 1 | README now leads with Git-powered local checkpoints, AI-agent edits, risky refactors, beginner workflows, and safety docs. | Monitor for drift |
| OSS hygiene files missing | High | Addressed in Sprint 2 | Standard root files and issue/PR templates now exist. | Monitor for drift |
| CI missing | High | Addressed in Sprint 3 | `.github/workflows/ci.yml` runs Ubuntu/Windows fmt, normal Clippy, tests, release build, source-built help, and source-built doctor. | Monitor first GitHub run |
| Strict Clippy fails | Medium | Addressed in Sprint 4 | `cargo clippy --all-targets --all-features -- -D warnings` now passes. | Monitor in CI |
| Temporary patch comments remain | Medium | Addressed in Sprint 4 | Patch-marker comments removed from `Cargo.toml` and `src`. | Monitor for recurrence |
| Placeholder packaging identifier | Medium | Open | `Packager.toml` has `com.yourname.snap`. | Sprint 8 |
| Public historical prompt noise | Medium | Open | `doc/prompt-2.txt` contains `your-username` and old snippets. | Later docs cleanup |
| Snapshot tags can mix with release tags | High | Partially addressed in Sprint 6 | New Snap tags carry `Snap-Snapshot: true`, discovery filters ordinary release tags, and legacy Snap tags remain visible. Namespaced refs remain deferred. | Future namespace migration |
| Linux command-name conflict | High | Open | Project/binary name `snap` conflicts with Canonical Snapcraft on many Linux systems. | Sprint 8 |
| Command construction needs hardening | High | Partially addressed in Sprint 4 | Obvious formatted Git commands converted to argv; remaining dynamic boundaries documented. | Follow-up as needed |
| Safety docs are fragmented | High | Addressed in Sprint 5 | README, `doc/SAFETY_MODEL.md`, `doc/SNAP_DOCTOR.md`, and `doc/KNOWN_LIMITATIONS.md` now cover restore dry-run/rescue, doctor JSON/CI, exit behavior, purge backups, reachability refusal, and final health checks. | Monitor for drift |
| Global checkpoint binary differs from source help | Medium | Open | `/usr/local/bin/snap --help` lacks newer commands while `target/release/snap --help` has them. This is intentional for now but must not affect product validation. | Sprint 8/release |
| Security audit tooling absent | Low | Open | `cargo audit` not installed. | Later OSS hygiene |

## 11. Command Safety Findings

Sprint 4 moved the detailed command-construction status to:

- `docs/architecture-audit/command-construction-audit-2026-06-07.md`

Before Sprint 4, command construction scan found these notable sites:

| File | Pattern |
| --- | --- |
| `src/utils.rs` | `Command::new(command)`, `git update-ref` format string, `git cat-file` format string |
| `src/commands/new.rs` | `git commit` format string, `git tag -a` format string |
| `src/commands/update.rs` | `git tag -a -f` format string |
| `src/commands/restore.rs` | `git reset --hard` format string |
| `src/commands/edit.rs` | `git tag -d` format string |
| `src/git_health.rs` | `git tag -a -f` format strings during repair |
| `src/github.rs` | dynamic `Command::new(&command)` |
| `src/commands/release.rs` | dynamic script runtime command |

These were reviewed in Sprint 4. The obvious formatted Git command strings were converted to explicit argv calls. Remaining dynamic command boundaries are central helpers or intentional local runtime overrides and are documented for future hardening.

- converted to explicit args;
- documented as safe because input is sanitized/resolved;
- isolated behind a safer command helper;
- covered with tests for labels, paths, spaces, and shell metacharacters.

## 12. Snapshot / Tag Model Findings

Current evidence:

- Snapshot labels are still represented as Git tags.
- New source-built Snap tag messages include `Snap-Snapshot: true`.
- `snap list` and health checks scan `refs/tags`, then filter to marked, metadata-bearing, or legacy Snap-style tags.
- Snapshot metadata is referenced through `Snap-Metadata-Ref`.
- Metadata blobs are pinned through `refs/snap-metadata`.
- The current Snap repo has 12 Snap snapshot tags before the Sprint 6 checkpoint:
  - `oss-plan-codex`
  - `oss-plan-foundation`
  - `oss-audit-detailed`
  - `oss-audit-baseline`
  - `oss-global-binary-rule`
  - `oss-sandbox-test-policy`
  - `oss-s0-baseline`
  - `oss-s1-readme`
  - `oss-s2-hygiene`
  - `oss-s3-ci`
  - `oss-s4-cleanup`
  - `oss-s5-safety`

Risk:

Normal Git release tags can still share the same Git namespace, but they are no longer treated as Snap snapshots unless they satisfy the marker/metadata/legacy compatibility contract.

Remaining future direction:

1. monitor the marker-first model while preserving legacy tags;
2. decide whether push/pull/sync should move away from all-tags refspecs;
3. consider `refs/tags/snap/<label>` or `refs/snapshots/<label>` as a later migration;
4. document and test migration/compatibility before changing storage.

Until then, this repo should avoid release-looking checkpoint labels like `v7.3`.

## 13. Agent-Coding Efficiency Assessment

### 13.1 What Is Already Good

- Rust modules are organized by command/domain.
- The test suite is fast enough for frequent agent use.
- `./target/release/snap doctor` provides a real health signal after changes.
- Architecture-audit docs now exist inside the Snap repo.
- Ignored local reference inputs avoid public dependency on Synthedu paths.
- Snap checkpoints give clean rollback labels during the refactor.

### 13.2 What Slows Agents Down

| Problem | Agent impact |
| --- | --- |
| `AGENTS.md` must stay current | Agents rely on it for the global/source-built Snap binary rule and validation discipline. |
| README drift can recur | Agents may document or test older behavior if README is not kept aligned with source help. |
| Old prompt docs are tracked | Search results are noisy and include stale placeholders. |
| Strict Clippy must stay clean | CI can later enforce `-D warnings`, but future changes must preserve the new green baseline. |
| Command execution patterns vary | Agent must manually inspect safety assumptions. |
| Large `git_health.rs` and test file | Harder to make surgical changes without context. |
| Snapshot tag ambiguity | Agents need special checkpoint-label discipline. |

### 13.3 Target Agent Experience

After Sprint 0-5, a new agent should quickly know:

- Snap's product identity;
- which commands are destructive;
- which files are safety-sensitive;
- how to run gates;
- how to create a checkpoint safely;
- how to update progress/audit docs;
- how to avoid confusing release tags with checkpoints;
- how restore dry-run/rescue and doctor JSON/CI modes behave.

## 14. CLI Health Assessment

### 14.1 Strong Areas

- Source-built `./target/release/snap doctor` health check is real and passes on the repo.
- Snapshot metadata is pinned and validated.
- Purge/doctor tests show significant attention to Git edge cases.
- Branch/remote/release helpers make Snap broader than a simple snapshot-only tool.
- Source help, README, and safety docs now describe the current restore/doctor safety surface.

### 14.2 Weak Areas

- Global checkpoint binary help does not match current source help; this is acceptable only if tests use source-built Snap.
- Packaging metadata has placeholders.
- Snapshot tags still share ordinary Git tag namespace.
- Linux command-name conflict remains unresolved.
- Historical prompt dumps are still tracked and noisy.

## 15. Recommended Validation Contract

### 15.1 Runtime / Code Changes

Use this as the full local gate set:

```bash
git diff --check
cargo fmt --check
cargo clippy --all-targets --all-features
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo build --release
./target/release/snap doctor
```

Use global `snap` only for checkpointing after gates pass. To test the current source tree, use:

```bash
cargo build --release
./target/release/snap doctor
./target/release/snap --help
```

Strict Clippy is part of runtime/code sprint validation after Sprint 4.

### 15.2 Docs-Only Changes

Use:

```bash
git diff --check
rg -n "stale-path-or-private-absolute-path" docs doc README.md
```

Optional but encouraged:

```bash
cargo fmt --check
cargo test
```

### 15.3 OSS Readiness Scans

Run before application-related checkpoints:

```bash
rg -n "your-username|TODO|FIXME|FINAL|START: THE FIX|END: THE FIX|CORRECTED LINE" . -g '!target'
rg -n "format!\\(\"git |format!\\(\"gh |run_command\\(&format!" src
rg -n "refs/tags|refs/snap|Snap-Snapshot|Snap-Metadata" src doc README.md docs/architecture-audit
```

### 15.4 Sandbox And External Test Policy

This project should be tested aggressively with source-built Snap. Agents are allowed and encouraged to create disposable projects, Git repositories, and filesystem scenarios to prove behavior.

Preferred local sandbox pattern:

```bash
tmpdir="$(mktemp -d /tmp/snap-agent-smoke-XXXXXX)"
cd "$tmpdir"
git init
/home/glooring/projects/snap/target/release/snap init
```

Good local smoke targets:

- normal snapshot lifecycle: `init`, `new`, `list`, `diff`, `restore`;
- dirty worktree restore warnings;
- `delete` and `delete --purge` safety behavior;
- `edit` and `update` tag/metadata behavior;
- `doctor` healthy and intentionally damaged repositories;
- branch workflows;
- paths with spaces;
- Unicode paths;
- nested empty directories;
- hidden/readonly metadata where the platform supports it;
- release command dry/preflight behavior where dependencies are available.

GitHub sandbox testing is allowed when relevant:

- create disposable repositories only;
- use a unique prefix such as `snap-agent-smoke-YYYYMMDD-HHMMSS`;
- default to private repos unless testing public/private visibility;
- never use an existing user repo as a test target;
- delete the repo after testing, or record the leftover repo and reason in `refactor-progress.md`;
- keep commands and results in the sprint notes.

This permission is meant to make the project highly testable and agent-friendly. It does not change the global-binary rule: use global `snap` only for refactor checkpoints, and use source-built Snap for product tests.

## 16. Snapshot Discipline

Use the stable globally installed Snap binary for refactor checkpoints only. The user intentionally keeps this global binary older for now.

Do not replace, reinstall, overwrite, or upgrade the global Snap binary during this refactor. In particular, do not copy `target/release/snap` to `/usr/local/bin/snap` and do not run an install command that changes the global binary unless the user explicitly approves it after the refactor goal is complete.

Use source-built Snap for product validation:

```bash
cargo run -- doctor
./target/debug/snap doctor
./target/release/snap doctor
```

Do not treat global `snap --help` or global `snap doctor` as proof of current source behavior.

Because Snap itself currently uses Git tags for checkpoints, use non-release-looking labels:

```bash
snap new oss-s0-baseline "OSS readiness baseline"
snap new oss-s1-readme "README positioning and docs"
snap new oss-s2-hygiene "OSS hygiene files"
```

After each sprint, use this checkpoint pattern:

```bash
snap new oss-sN-short-name "sprint N: short description"
```

Avoid:

```bash
snap new v7.3 "sprint 1"
snap new v8.0 "release prep"
```

Reason: those look like release tags and reinforce the current snapshot/tag ambiguity.

## 17. Recommended Roadmap

| Sprint | Recommendation | Why |
| --- | --- | --- |
| 0 | Baseline Audit and Safety Rails | Completed in Sprint 0: plan, gates, metrics, source-built smoke test, strict Clippy deferral, and checkpoint discipline. |
| 1 | README Positioning and Public Docs | Completed in Sprint 1: README and focused public docs now explain Snap's identity, workflows, safety model, limitations, storage model, and source command surface. |
| 2 | OSS Hygiene Files | Completed in Sprint 2: root OSS files, PR template, and issue templates now exist. |
| 3 | CI on Windows/Linux | Completed in Sprint 3: GitHub Actions workflow exists for Ubuntu and Windows. |
| 4 | Source Cleanup and Command Hardening Audit | Completed in Sprint 4: patch traces removed, strict Clippy green, command audit documented, and obvious formatted Git commands hardened. |
| 5 | Restore/Doctor/Purge Safety Plan | Completed in Sprint 5: restore dry-run/rescue, doctor JSON/CI, documented exit behavior, purge safety docs, and local smoke coverage. |
| 6 | Snapshot Tags/Refs Decision | Completed in Sprint 6: marker-first tag messages, compatibility filtering, release-tag exclusion, and deferred namespace migration decision. |
| 7 | Codex/AI-Agent Workflow Docs | Completed in Sprint 7: Codex workflow docs, task recipes, README links/examples, AI-agent doc refresh, and AGENTS safety guidance. |
| 8 | Packaging/Name Conflict | Makes installation honest and practical. |
| 9 | Cross-Platform and Performance Proof | Replaces broad claims with evidence. |
| 10 | Community Feedback | Produces real OSS signals without artificial hype. |
| 11 | OpenAI Application Package | Submit only after claims are visible and verifiable. |

## 18. Closure Criteria

Close the OSS-readiness refactor when all are true:

- README explains Snap's purpose clearly in under 10 seconds.
- README/docs cover AI-agent workflow, beginner workflow, safety model, "why not Git", known limitations, and `snap doctor`.
- `AGENTS.md`, `CONTRIBUTING.md`, `SECURITY.md`, `CHANGELOG.md`, `SUPPORT.md`, license file, issue templates, and PR template exist.
- CI runs on Windows and Linux.
- `cargo fmt`, clippy, tests, release build, and doctor are green.
- Strict Clippy is green.
- Temporary patch comments are cleaned; placeholder packaging metadata remains for Sprint 8.
- Snapshot/tag ambiguity is mitigated by marker-first filtering or has a concrete namespace migration issue.
- Linux name conflict is documented.
- Packaging/release docs explain artifacts and checksums.
- A current release exists.
- GitHub About/topics are set.
- There is some real feedback or issue activity.
- OpenAI/Codex application answers are honest and backed by visible repository evidence.

Do not continue refactoring only for aesthetics. Once these criteria are met, move remaining work into normal issue-driven maintenance.

## 19. Next Recommendation

Proceed to Sprint 8: Packaging/Name Conflict.

Sprint 8 should:

- document install paths for Windows, Linux, and WSL2;
- decide how public docs handle the Canonical Snapcraft `snap` command conflict;
- document release asset names, verification, and checksum expectations;
- clean or track placeholder packaging metadata such as `Packager.toml`;
- avoid replacing the maintainer's global `/usr/local/bin/snap` during validation.

After Sprint 8, move to Sprint 9: Cross-Platform and Performance Proof.
