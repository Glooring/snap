# Snap OSS Readiness Audit - Agent Efficiency and CLI Health

## 1. Metadata

| Field | Value |
| --- | --- |
| Repository | `Glooring/snap` |
| Local path | `/home/glooring/projects/snap` |
| Audit date | `2026-06-06` |
| Branch inspected | `main` |
| Commit inspected | `6cf6c2c1563009588138203736354bae94c6140a` |
| Current audit path | `docs/architecture-audit/agent-efficiency-oss-readiness-audit-2026-06-06.md` |
| Master plan | `docs/architecture-audit/CODEX_OSS_REFACTOR_PLAN.md` |
| Progress log | `docs/architecture-audit/refactor-progress.md` |
| Historical sprint plans | `docs/architecture-audit/refactor-plans/` |
| Local ignored references | `docs/architecture-audit/reference-inputs/` |
| Scope | Complete starting baseline for the Snap OSS-readiness refactor. Runtime code was inspected and validated, but this audit update is documentation-only. |

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
| `git rev-parse HEAD` | `6cf6c2c1563009588138203736354bae94c6140a` | Baseline commit before this complete audit update. |
| `git log --oneline -8` | `oss-audit-detailed`, `oss-plan-foundation`, `oss-plan-codex`, initial import | OSS-readiness planning has named Snap checkpoints. |
| `git status --short` | Clean before audit rewrite | Repo was clean after previous checkpoint. |
| `git diff --check` | Passed | Whitespace gate is clean. |
| `cargo fmt --check` | Passed | Rust formatting is clean. |
| `cargo clippy --all-targets --all-features` | Passed with warnings | Clippy is runnable but not clean enough for `-D warnings`. |
| `cargo clippy --all-targets --all-features -- -D warnings` | Failed with 9 warnings-as-errors | Strict Clippy should be a near-term cleanup target before CI enforces it. |
| `cargo test` | Passed, 96 tests | Test suite is fast and substantial. |
| `cargo build --release` | Passed | Release build works locally. |
| global `snap doctor` | Passed | The older globally installed Snap binary can still checkpoint/inspect this repo, but it is not the product-under-test baseline. |
| `./target/release/snap doctor` | Passed | Current source-built binary sees the repo as healthy; this is the correct behavior baseline for the project. |
| `cargo audit` | Not installed | Security audit is not available locally yet; add later if desired. |
| Root OSS file inspection | 0 files found | Missing `AGENTS.md`, `CONTRIBUTING.md`, `SECURITY.md`, `CHANGELOG.md`, `SUPPORT.md`, `CODE_OF_CONDUCT.md`, and visible license file. |
| `.github` inspection | 0 files found | No GitHub Actions, issue templates, or PR template. |
| Temporary/placeholder scan | Open findings | `Cargo.toml`, `Packager.toml`, README/docs, and some command files need cleanup. |
| Command construction scan | Open findings | Several `run_command(&format!(...))` and dynamic `Command::new(...)` sites need audit. |
| Snapshot metadata scan | Open findings | Snapshot discovery still scans `refs/tags`; metadata uses `Snap-Metadata-Ref` and `refs/snap-metadata`. |

## 4. Gate Details

### 4.1 Passing Gates

The following gates are currently green:

```bash
git diff --check
cargo fmt --check
cargo clippy --all-targets --all-features
cargo test
cargo build --release
./target/release/snap doctor
```

`cargo test` result:

```text
96 passed; 0 failed; 0 ignored
```

`./target/release/snap doctor` result summary:

```text
Git repository looks healthy.
Snapshot tags: 3 checked, 0 invalid
Snapshot metadata: 3 checked, 0 active invalid, 0 historical invalid, 0 unpinned
```

### 4.2 Strict Clippy Baseline

Strict Clippy currently fails:

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

Failure categories:

| File | Finding |
| --- | --- |
| `src/commands/branch.rs` | `clippy::print_literal` |
| `src/commands/list.rs` | `clippy::useless_format` |
| `src/commands/options.rs` | `clippy::useless_format`, `clippy::useless_vec` |
| `src/commands/restore.rs` | `clippy::unnecessary_sort_by` |
| `src/config.rs` | `clippy::derivable_impls` |
| `src/git_health.rs` | `clippy::nonminimal_bool`, `clippy::len_zero` |

These are not architectural blockers. They are good early cleanup candidates because they make future CI stricter and more credible.

### 4.3 Missing Gate

`cargo audit` is not installed:

```text
error: no such command: `audit`
```

This should not block Sprint 0, but a later security/OSS hygiene sprint can decide whether to add `cargo-audit` to local instructions or CI.

## 5. Repository Metrics Snapshot

### 5.1 File Counts

| Area | Count |
| --- | ---: |
| `src/**/*.rs` | 33 |
| `tests/**/*.rs` | 1 |
| `doc/*.md` | 8 |
| `docs/**/*.md` | 7 before this expanded audit |
| `.github` tracked files | 0 |
| Standard OSS root files found | 0 |

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
| `src` Rust | 6,445 |
| `tests` Rust | 2,698 |
| `doc` Markdown/text | 8,318 |
| `docs` audit Markdown | 3,752 before this expanded audit |

### 5.3 Largest Operational Files

| Rank | File | LOC | Primary concern |
| ---: | --- | ---: | --- |
| 1 | `tests/git_health.rs` | 2,698 | Valuable integration coverage, but already large enough to need section ownership if expanded. |
| 2 | `src/git_health.rs` | 1,017 | High-risk Git diagnosis/repair logic; behavior changes need tests and docs. |
| 3 | `src/git.rs` | 487 | Git wrapper and remote metadata sync boundary. |
| 4 | `src/commands/release.rs` | 471 | Release scripting and GitHub CLI boundary; cross-platform assumptions matter. |
| 5 | `src/commands/doctor.rs` | 434 | User-facing diagnosis/repair UI; must stay read-only unless repair is explicit. |
| 6 | `src/cli.rs` | 426 | Public command contract and help text. |
| 7 | `src/utils.rs` | 420 | Snapshot discovery, metadata, and command helpers; central to tag/ref migration. |
| 8 | `src/commands/delete.rs` | 380 | Destructive snapshot/purge workflow. |
| 9 | `src/commands/list.rs` | 318 | Snapshot discovery/presentation, affected by future tag filtering. |
| 10 | `src/commands/remote.rs` | 304 | GitHub CLI and destructive remote operations. |
| 11 | `src/github.rs` | 282 | GitHub CLI integration and release parsing. |
| 12 | `src/commands/branch.rs` | 207 | Branch workflow wrapper. |
| 13 | `src/commands/options.rs` | 170 | Global Snap options persistence. |
| 14 | `src/commands/edit.rs` | 156 | Tag/message rewrite behavior. |
| 15 | `src/commands/restore.rs` | 150 | Destructive restore workflow; target for dry-run/rescue. |
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

The global binary reports version `7.2.0`, but its `snap --help` output only lists the older snapshot-oriented command set:

```text
init, new, list, restore, delete, edit, update, diff, doctor, options
```

The source-built `./target/release/snap --help` lists the newer command surface.

This matters because the refactor workflow uses the global Snap binary for checkpoints, while validation must use the source-built binary from this repo:

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

Missing or not found:

- `AGENTS.md`
- `CONTRIBUTING.md`
- `SECURITY.md`
- `CHANGELOG.md`
- `SUPPORT.md`
- `CODE_OF_CONDUCT.md`
- `LICENSE` / `LICENSE.md`
- `.github/workflows/ci.yml`
- `.github/ISSUE_TEMPLATE/*`
- `.github/pull_request_template.md`

Priority:

1. `AGENTS.md`
2. `LICENSE`
3. `CONTRIBUTING.md`
4. `SECURITY.md`
5. `CHANGELOG.md`
6. CI workflow
7. issue/PR templates

### 9.2 README Positioning Gaps

Current README still leads with:

- `Snap (The Rust Edition)`
- `blazing-fast`
- Rust rewrite history
- snapshot/backup framing

The public story should instead lead with:

- Git-powered local checkpoints;
- safety before AI-agent edits and risky refactors;
- beginner alternative to copying folders;
- Git is the engine, Snap is the workflow;
- `snap doctor` as a serious differentiator.

The Rust rewrite story can remain, but it should move lower.

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
| Public positioning is unclear | High | Open | README leads with Rust edition/blazing-fast backup framing. | Sprint 1 |
| OSS hygiene files missing | High | Open | 0 standard root OSS files and 0 `.github` files found. | Sprint 2 |
| CI missing | High | Open | No `.github/workflows`. | Sprint 3 |
| Strict Clippy fails | Medium | Open | 9 warnings-as-errors. | Sprint 0 or 4 |
| Temporary patch comments remain | Medium | Open | `Cargo.toml`, `new.rs`, `diff.rs`, `update.rs`. | Sprint 4 |
| Placeholder packaging identifier | Medium | Open | `Packager.toml` has `com.yourname.snap`. | Sprint 8 |
| Public historical prompt noise | Medium | Open | `doc/prompt-2.txt` contains `your-username` and old snippets. | Sprint 1 or 2 |
| Snapshot tags can mix with release tags | High | Open | `refs/tags` scanning and Snap checkpoints are tag-based. | Sprint 6 |
| Linux command-name conflict | High | Open | Project/binary name `snap` conflicts with Canonical Snapcraft on many Linux systems. | Sprint 8 |
| Command construction needs hardening | High | Open | Multiple `run_command(&format!(...))` sites with Git commands. | Sprint 4 |
| Safety docs are fragmented | High | Open | Restore/purge/doctor guarantees spread across docs and code. | Sprint 5 |
| Global checkpoint binary differs from source help | Medium | Open | `/usr/local/bin/snap --help` lacks newer commands while `target/release/snap --help` has them. This is intentional for now but must not affect product validation. | Sprint 8/release |
| Security audit tooling absent | Low | Open | `cargo audit` not installed. | Later OSS hygiene |

## 11. Command Safety Findings

Command construction scan found these notable sites:

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

These are not automatically exploitable. Some values are sanitized or resolved from Git, and some dynamic runtimes are intentional. But for OSS credibility, each site should be reviewed and either:

- converted to explicit args;
- documented as safe because input is sanitized/resolved;
- isolated behind a safer command helper;
- covered with tests for labels, paths, spaces, and shell metacharacters.

## 12. Snapshot / Tag Model Findings

Current evidence:

- Snapshot labels are represented as Git tags.
- `snap list` and health checks scan `refs/tags`.
- Snapshot metadata is referenced through `Snap-Metadata-Ref`.
- Metadata blobs are pinned through `refs/snap-metadata`.
- The current Snap repo has 3 Snap snapshot tags:
  - `oss-plan-codex`
  - `oss-plan-foundation`
  - `oss-audit-detailed`

Risk:

Normal Git release tags can be confused with Snap snapshots unless Snap distinguishes its own tags from normal tags.

Preferred future direction:

1. add an explicit `Snap-Snapshot: true` marker to new snapshot tag messages;
2. filter `snap list` / `doctor` to Snap-marked tags, with compatibility handling for older Snap tags;
3. consider `refs/tags/snap/<label>` or `refs/snapshots/<label>` as a later migration;
4. add tests proving normal release tags are not treated as Snap snapshots.

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
| No `AGENTS.md` | Agents lack safety-specific instructions at startup. |
| README lags source command surface | Agent may document or test older behavior. |
| Old prompt docs are tracked | Search results are noisy and include stale placeholders. |
| Clippy strict fails | CI cannot immediately use `-D warnings`. |
| Command execution patterns vary | Agent must manually inspect safety assumptions. |
| Large `git_health.rs` and test file | Harder to make surgical changes without context. |
| Snapshot tag ambiguity | Agents need special checkpoint-label discipline. |

### 13.3 Target Agent Experience

After Sprint 0-3, a new agent should quickly know:

- Snap's product identity;
- which commands are destructive;
- which files are safety-sensitive;
- how to run gates;
- how to create a checkpoint safely;
- how to update progress/audit docs;
- how to avoid confusing release tags with checkpoints.

## 14. CLI Health Assessment

### 14.1 Strong Areas

- Source-built `./target/release/snap doctor` health check is real and passes on the repo.
- Snapshot metadata is pinned and validated.
- Purge/doctor tests show significant attention to Git edge cases.
- Branch/remote/release helpers make Snap broader than a simple snapshot-only tool.
- Current source help is much better than README positioning.

### 14.2 Weak Areas

- Global checkpoint binary help does not match current source help; this is acceptable only if tests use source-built Snap.
- README needs repositioning and command-surface refresh.
- Packaging metadata has placeholders.
- No CI proves cross-platform behavior yet.
- No root license file despite `Cargo.toml` declaring MIT.
- No public-facing security policy for filesystem/Git operations.

## 15. Recommended Validation Contract

### 15.1 Runtime / Code Changes

Use this as the full local gate set:

```bash
git diff --check
cargo fmt --check
cargo clippy --all-targets --all-features
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

Use this as the target strict gate once cleanup lands:

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

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
| 0 | Baseline Audit and Safety Rails | Write the first sprint plan, record full gates, and turn this audit/progress setup into a repeatable workflow. |
| 1 | README Positioning and Public Docs | Highest-leverage OSS readiness fix; makes value obvious. |
| 2 | OSS Hygiene Files | Adds maintainer credibility and gives Codex/agents safe project instructions. |
| 3 | CI on Windows/Linux | Makes cross-platform and test claims visible. |
| 4 | Source Cleanup and Command Hardening Audit | Removes patch traces and reduces command-boundary risk. |
| 5 | Restore/Doctor/Purge Safety Plan | Strengthens trust around data-loss and repair operations. |
| 6 | Snapshot Tags/Refs Decision | Resolves central Git integration ambiguity. |
| 7 | Codex/AI-Agent Workflow Docs | Makes the project directly relevant to AI-assisted coding. |
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
- Strict Clippy is either green or intentionally deferred with an issue.
- Temporary patch comments and placeholder metadata are cleaned.
- Snapshot/tag ambiguity is fixed or has a concrete migration issue.
- Linux name conflict is documented.
- Packaging/release docs explain artifacts and checksums.
- A current release exists.
- GitHub About/topics are set.
- There is some real feedback or issue activity.
- OpenAI/Codex application answers are honest and backed by visible repository evidence.

Do not continue refactoring only for aesthetics. Once these criteria are met, move remaining work into normal issue-driven maintenance.

## 19. Next Recommendation

Proceed to Sprint 0: Baseline Audit and Safety Rails.

Sprint 0 should:

- create `docs/architecture-audit/refactor-plans/sprint-0-baseline-safety-rails.md`;
- update `docs/architecture-audit/refactor-progress.md`;
- rerun full gates and record exact outputs;
- decide whether to fix strict Clippy warnings immediately or track them for Sprint 4;
- update this audit with any new findings;
- create checkpoint `oss-s0-baseline`.

After Sprint 0, move to Sprint 1: README Positioning and Public Docs.
