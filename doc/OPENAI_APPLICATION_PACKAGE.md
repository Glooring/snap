# OpenAI Application Package

Status: public evidence prepared with current release, not submitted
Last refreshed: 2026-06-07T11:33:33Z after `v7.2.2` release publication and public release smoke tests
Repository: <https://github.com/Glooring/snap>

This package intentionally does not rely on a hard-coded local commit SHA or checkpoint label. Before submission, run `git rev-parse HEAD` and verify that the public GitHub repository shows the same content.

## Submission Readiness

The OSS-readiness commits are public on `origin/main`, the public CI workflow has passed on Ubuntu, Windows, and macOS, and the current GitHub Release exists with Linux, Windows, and macOS assets.

Current state:

- release tag `v7.2.2` targets `7a271a093259bceff7b2854f9e3ab48a705864eb` for the Sprint 18 macOS release commit;
- GitHub About description and topics are live;
- GitHub issues #1-#6 are live;
- public README/docs/CI/community package changes are visible on GitHub;
- `gh workflow list --repo Glooring/snap` shows active CI/release workflows;
- public CI run `27091175225` passed on Ubuntu, Windows, and macOS for commit `7a271a093259bceff7b2854f9e3ab48a705864eb`;
- GitHub Release `v7.2.2` is published at <https://github.com/Glooring/snap/releases/tag/v7.2.2>;
- release workflow run `27091240489` passed and published Linux, Windows, macOS Apple Silicon, macOS Intel, and `SHA256SUMS.txt` assets;
- release smoke run `27091340545` passed on Ubuntu, Windows, macOS Apple Silicon, and macOS Intel against the public release assets;
- early public interest exists: 4 forks, issue #2 comments, PR #7 open, and PR #8 closed as duplicate;

Recommended submission decision:

- Ready to submit with current public CI, current release, and conservative community-signal caveats.

## Ready-To-Paste Answers

### Role

```text
Primary maintainer
```

### Short Qualification

```text
Snap is an MIT-licensed Rust CLI for Git-powered local checkpoints. It helps beginners stop copying whole project folders manually and helps developers save a local checkpoint before risky refactors, AI-agent edits, dependency upgrades, or release work. The current OSS-readiness branch documents the safety model, known limitations, Codex/AI-agent workflow, packaging, cross-platform notes, benchmark methodology, and community feedback paths. I am the primary maintainer.
```

### API Credits Usage

```text
I will use API credits for open-source maintenance: issue triage, documentation, tests, release automation, and review of safety-sensitive changes. For Snap, I will use Codex/OpenAI tools to improve restore and doctor safety, release checksum automation, namespaced snapshot-ref design, Windows/Linux/macOS/WSL2 validation, contributor docs, and bug fixes. All AI-assisted changes will be manually reviewed, tested locally with source-built Snap, and kept behind ordinary GitHub review and issue tracking.
```

### Project Summary

```text
Snap is a Rust command-line tool that uses Git as its storage engine and adds a smaller workflow for local checkpoints: snap new, snap list, snap diff, snap restore --dry-run, and snap doctor. It is meant for developers who want a quick safety point before broad edits, and for beginners who need a bridge from folder-copy backups toward Git-backed history. The project is MIT-licensed and maintained publicly at https://github.com/Glooring/snap.
```

## Evidence Checklist

Local evidence prepared in this branch:

| Evidence | Location |
| --- | --- |
| Product positioning | `README.md` |
| AI-agent workflow | `doc/AI_AGENT_WORKFLOW.md` |
| Codex workflow | `doc/CODEX_WORKFLOW.md` |
| Codex task recipes | `doc/CODEX_TASKS.md` |
| Beginner workflow | `doc/BEGINNER_WORKFLOW.md` |
| Why Snap is not a Git replacement | `doc/WHY_NOT_GIT.md` |
| Safety model | `doc/SAFETY_MODEL.md` |
| Doctor docs | `doc/SNAP_DOCTOR.md` |
| Known limitations | `doc/KNOWN_LIMITATIONS.md` |
| Installation and packaging | `doc/INSTALLATION.md` and `doc/BUILD_INSTALLERS_WINDOWS_WSL.md` |
| Performance methodology | `doc/PERFORMANCE.md` |
| Cross-platform notes | `doc/CROSS_PLATFORM.md` |
| Community feedback readiness | `doc/COMMUNITY_FEEDBACK.md` |
| Contributor and maintainer files | `AGENTS.md`, `CONTRIBUTING.md`, `SECURITY.md`, `SUPPORT.md`, `CODE_OF_CONDUCT.md`, `CHANGELOG.md`, `LICENSE` |
| GitHub templates and CI | `.github/ISSUE_TEMPLATE/*`, `.github/pull_request_template.md`, `.github/workflows/ci.yml`, `.github/workflows/release.yml`, `.github/workflows/release-smoke.yml` |
| Release notes | `doc/releases/v7.2.2.md` |
| Current validation history | `docs/architecture-audit/refactor-progress.md` |
| Current audit | `docs/architecture-audit/agent-efficiency-oss-readiness-audit-2026-06-06.md` |

Live GitHub evidence already visible:

| Evidence | Status |
| --- | --- |
| Repository visibility | Public |
| About description | `Git-powered local checkpoint CLI for risky refactors, AI-agent edits, experiments, and beginner-friendly project history.` |
| Topics | `ai-assisted-coding`, `cli`, `codex`, `developer-tools`, `git`, `linux`, `rust`, `snapshot`, `version-control`, `windows`, `wsl2` |
| Good-first issue #1 | <https://github.com/Glooring/snap/issues/1> |
| Good-first issue #2 | <https://github.com/Glooring/snap/issues/2> |
| Good-first issue #3 | <https://github.com/Glooring/snap/issues/3> |
| Safety/hardening issue #4 | <https://github.com/Glooring/snap/issues/4> |
| Safety/hardening issue #5 | <https://github.com/Glooring/snap/issues/5> |
| Safety/hardening issue #6 | <https://github.com/Glooring/snap/issues/6> |
| Active CI workflow | `CI`, active |
| Latest verified CI run | <https://github.com/Glooring/snap/actions/runs/27091175225>, passed on Ubuntu, Windows, and macOS |
| Current release | <https://github.com/Glooring/snap/releases/tag/v7.2.2> |
| Release workflow | <https://github.com/Glooring/snap/actions/runs/27091240489>, passed and published Linux/Windows/macOS assets plus `SHA256SUMS.txt` |
| Public release smoke | <https://github.com/Glooring/snap/actions/runs/27091340545>, passed on Ubuntu, Windows, macOS Apple Silicon, and macOS Intel |
| Fork count | 4 |
| External contributor activity | Issue #2 comments, PR #7 open, PR #8 closed as duplicate |

## Current Validation Evidence

Sprint 18 validation completed with:

```text
git diff --check: passed
cargo fmt --check: passed
cargo clippy --all-targets --all-features: passed with no warnings
cargo clippy --all-targets --all-features -- -D warnings: passed
cargo test: passed, 113 integration tests
cargo build --release: passed
./target/release/snap --help: passed
./target/release/snap doctor: passed
public CI run 27091175225: passed on Ubuntu, Windows, and macOS
release workflow run 27091240489: passed and published v7.2.2
release smoke run 27091340545: passed on Ubuntu, Windows, macOS Apple Silicon, and macOS Intel
downloaded release checksums: passed for all Linux, Windows, and macOS assets
```

Before the Sprint 18 release, source-built doctor reported:

```text
Snapshot tags: 25 checked, 0 invalid
Snapshot metadata: 25 checked, 0 active invalid, 0 historical invalid, 0 unpinned
Latest valid snapshot: oss-s17-v721-release-prep
```

## Honest Gaps To Disclose

- External activity is early interest, not adoption: 4 forks, one active PR direction, one duplicate PR, and issue comments.
- PR #7 needs a rebase/update before review or merge.
- The `v7.2.2` Linux asset is built on Ubuntu 24.04 and requires glibc 2.39 or newer; older Linux distributions should build from source for now.
- The `v7.2.2` macOS assets are unsigned and not notarized; install docs include checksum verification and Gatekeeper/quarantine guidance.
- Snapshot tags use a marker-first filter, but a namespaced snapshot-ref migration is deferred to issue #4.
- Snap still uses the `snap` binary name; Linux/WSL2 docs explain the Canonical Snapcraft conflict and suggest a local `gitsnap` filename when needed.
- GitHub Actions reports a non-blocking Node.js 20 deprecation warning for `actions/cache@v4` and `actions/checkout@v4`.
- `cargo audit` is not installed locally.

## Do Not Claim

- Do not claim broad adoption or third-party production usage.
- Do not overstate early forks/comments/PRs as production use or broad community traction.
- Do not claim universal or platform-wide speed improvements.
- Do not use `blazing-fast`, `guaranteed`, or similar unsupported language.
- Do not claim snapshot refs have been fully migrated away from ordinary Git tags.
- Do not claim broad Linux binary compatibility beyond the documented Ubuntu 24.04/glibc 2.39 release asset baseline.

## Final Pre-Submission Checklist

- Keep application answers aligned with the actual public repo state on the day of submission.
- Reconfirm `gh release view v7.2.2 --repo Glooring/snap` and the latest CI/release-smoke run links if submitting later.

## Publication Decision Runbook

This section is a maintainer runbook. Option A was executed for the docs/CI publication path. Option B was later executed in Sprint 13 after the maintainer requested a current release. Option C was executed in Sprint 18 after the maintainer requested macOS release support.

### Option A: Publish Docs And CI Without A Release

Executed on 2026-06-07 after maintainer approval:

```bash
git status --branch --short
git rev-parse HEAD
git push origin main
gh workflow list --repo Glooring/snap
gh repo view Glooring/snap --json nameWithOwner,description,repositoryTopics,isPrivate,url
gh issue list --repo Glooring/snap --state open --limit 20
```

Do not push all local Snap checkpoint tags by default. They are useful local refactor checkpoints but can clutter public GitHub tags and are not required for README/docs/CI evidence.

After pushing, GitHub was verified to show:

- updated README positioning;
- `doc/OPENAI_APPLICATION_PACKAGE.md`;
- `.github/workflows/ci.yml`;
- issue templates and PR template;
- existing About description and topics;
- issues #1-#6.

Local Snap checkpoint tags were not pushed.

### Option B: Publish A Current Release

Executed on 2026-06-07 after maintainer approval:

```bash
gh workflow run release.yml --repo Glooring/snap -f tag=v7.2.0 -f publish=true
gh run watch 27085855559 --repo Glooring/snap --exit-status
gh release view v7.2.0 --repo Glooring/snap --json tagName,name,isDraft,isPrerelease,publishedAt,createdAt,url,assets,targetCommitish
gh release download v7.2.0 --repo Glooring/snap --dir /tmp/snap-release-v7.2.0-esCfZg
sha256sum -c SHA256SUMS.txt
gh workflow run release-smoke.yml --repo Glooring/snap -f tag=v7.2.0
gh run watch 27086213709 --repo Glooring/snap --exit-status
```

Results:

- GitHub Release `v7.2.0` exists and is published.
- Release assets include Linux, Windows portable, Windows setup, Windows MSI, Linux tar archive, and `SHA256SUMS.txt`.
- Release workflow built Linux assets on Ubuntu and Windows assets on Windows.
- Downloaded release checksums verified.
- Public release smoke passed on Ubuntu and Windows.
- The global `/usr/local/bin/snap` was not overwritten or upgraded.

### Option C: Publish A macOS-Capable Current Release

Executed on 2026-06-07 after maintainer approval:

```bash
git tag -a v7.2.2 -m "snap v7.2.2"
git push origin main v7.2.2
gh workflow run release.yml --repo Glooring/snap -f tag=v7.2.2 -f publish=true
gh run watch 27091240489 --repo Glooring/snap --exit-status
gh release view v7.2.2 --repo Glooring/snap --json tagName,name,isDraft,isPrerelease,publishedAt,createdAt,url,assets,targetCommitish
gh release download v7.2.2 --repo Glooring/snap --dir /tmp/snap-release-v7.2.2-X2Ri3Q
sha256sum -c SHA256SUMS.txt
gh workflow run release-smoke.yml --repo Glooring/snap -f tag=v7.2.2
gh run watch 27091340545 --repo Glooring/snap --exit-status
```

Results:

- GitHub Release `v7.2.2` exists and is published.
- Release assets include Linux, Windows portable, Windows setup, Windows MSI, macOS Apple Silicon, macOS Intel, matching tar archives, and `SHA256SUMS.txt`.
- Release workflow built Linux assets on Ubuntu, Windows assets on Windows, macOS Apple Silicon assets on `macos-15`, and macOS Intel assets on `macos-15-intel`.
- Downloaded release checksums verified locally for all Linux, Windows, and macOS assets.
- Public release smoke passed on Ubuntu, Windows, macOS Apple Silicon, and macOS Intel.
- The global `/usr/local/bin/snap` was not overwritten or upgraded.
