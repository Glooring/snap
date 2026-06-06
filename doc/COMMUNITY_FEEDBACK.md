# Community Feedback Readiness

This document prepares Snap for real public feedback. It is not a launch-hype checklist and should not ask for artificial stars, follows, or upvotes.

## GitHub About

Recommended repository description:

```text
Git-powered local checkpoint CLI for risky refactors, AI-agent edits, experiments, and beginner-friendly project history.
```

Recommended topics:

```text
rust
cli
git
snapshot
developer-tools
version-control
ai-assisted-coding
codex
wsl2
windows
linux
```

Suggested command:

```bash
gh repo edit Glooring/snap \
  --description "Git-powered local checkpoint CLI for risky refactors, AI-agent edits, experiments, and beginner-friendly project history." \
  --add-topic rust,cli,git,snapshot,developer-tools,version-control,ai-assisted-coding,codex,wsl2,windows,linux
```

Sprint 10 status:

- GitHub About description updated for `Glooring/snap`.
- GitHub topics set to the list above.
- Repository visibility remained public.

## Good-First Issue Candidates

### Document Windows And WSL2 Benchmark Results

Labels: `good first issue`, `documentation`

Created: <https://github.com/Glooring/snap/issues/1>

Why it helps:

Snap now has benchmark scripts and methodology, but this Linux workspace cannot prove Windows or WSL2 results. A contributor can run the scripts on those platforms and add measured local notes.

Acceptance:

- Run `pwsh -File scripts/benchmark.ps1` on Windows.
- Run `bash scripts/benchmark.sh` inside WSL2 on a native Linux filesystem.
- Record OS, filesystem, Git version, Snap commit, and command output summary.
- Update `doc/PERFORMANCE.md` or `doc/CROSS_PLATFORM.md` without making universal speed claims.

### Add A Tiny Demo Fixture For Beginner Docs

Labels: `good first issue`, `documentation`

Created: <https://github.com/Glooring/snap/issues/2>

Why it helps:

Beginner docs explain the workflow, but a tiny copy-paste demo project would make the first `snap init`, `snap new`, `snap list`, and `snap restore` loop easier to try.

Acceptance:

- Add a small documented demo under `doc/` or `examples/`.
- Use a disposable local repo in the instructions.
- Include `snap doctor` and `snap restore --dry-run`.
- Avoid private paths and unsupported performance claims.

### Improve Release Checksum Instructions

Labels: `good first issue`, `documentation`

Created: <https://github.com/Glooring/snap/issues/3>

Why it helps:

Sprint 8 documents `SHA256SUMS.txt`, but release checksum automation is still manual. A docs-first issue can clarify the manual steps before code automation.

Acceptance:

- Update `doc/INSTALLATION.md` and `doc/BUILD_INSTALLERS_WINDOWS_WSL.md` if needed.
- Include Linux and PowerShell examples.
- Make clear which steps are manual today.

## Safety And Hardening Issue Candidates

### Design Namespaced Snapshot Refs Migration

Labels: `enhancement`, `help wanted`

Created: <https://github.com/Glooring/snap/issues/4>

Why it helps:

Sprint 6 added marker-first snapshot tags, but snapshots still share the ordinary Git tag namespace. A future design should decide whether Snap moves toward `refs/tags/snap/<label>` or `refs/snapshots/<label>`.

Acceptance:

- Compare marker-only tags, `refs/tags/snap/<label>`, and `refs/snapshots/<label>`.
- Document migration and backward-compatibility behavior.
- Include push/pull/sync implications.
- Include tests that prove ordinary release tags stay out of `snap list`.

### Automate Release Checksums

Labels: `enhancement`, `help wanted`

Created: <https://github.com/Glooring/snap/issues/5>

Why it helps:

Release docs require `SHA256SUMS.txt`, but scripts and `snap release upload` do not automate generation or upload yet.

Acceptance:

- Generate SHA-256 checksums for all release assets.
- Include checksums in the release folder.
- Ensure upload behavior is explicit and testable.
- Keep Windows and Linux script behavior understandable.

### Audit Snap-Aware Push/Pull Tag Refspecs

Labels: `enhancement`, `help wanted`

Created: <https://github.com/Glooring/snap/issues/6>

Why it helps:

Snap currently syncs snapshot tags and metadata refs. A future snapshot namespace migration may need narrower refspecs so normal release tags and Snap snapshots remain clearly separated.

Acceptance:

- Document current push/pull/sync tag behavior.
- Propose a migration-compatible refspec strategy.
- Include tests with ordinary release tags and Snap snapshot tags.
- Avoid changing remote behavior without explicit migration docs.

## Feedback Post Drafts

### Maintainer/Developer Communities

```text
I am preparing Snap, a small MIT-licensed Rust CLI for Git-powered local checkpoints, for broader OSS feedback.

It is meant for developers who want a quick local checkpoint before risky refactors, AI-agent edits, dependency upgrades, or release work. It keeps Git as the engine and adds a smaller workflow around `snap new`, `snap list`, `snap diff`, `snap restore --dry-run`, and `snap doctor`.

I would appreciate practical feedback on the README, safety model, install docs, and whether the workflow makes sense for real projects. Bug reports and sharp critique are more useful than stars.

Repo: https://github.com/Glooring/snap
```

### Beginner-Friendly Programming Communities

```text
I am looking for feedback on Snap, a Rust CLI that helps beginners stop copying whole project folders as backups.

Snap uses Git underneath, but gives a smaller workflow: `snap init`, `snap new first-working-version`, `snap list`, and `snap restore --dry-run`. The goal is to be a bridge into Git-powered history, not a replacement for learning Git.

I would especially like feedback on whether the beginner docs are clear and what feels confusing in the first 10 minutes.

Repo: https://github.com/Glooring/snap
```

### AI-Assisted Coding Communities

```text
I am testing Snap as a local safety tool around Codex/editor-agent workflows.

The idea is simple: create a Git-backed checkpoint before an AI agent changes many files, inspect with `snap diff` and `snap doctor --json --ci`, then use `snap restore --dry-run` before rolling back.

I would value feedback from people who use AI coding agents on real repos: what safety checks are missing, what would make the workflow easier to trust, and which docs are unclear?

Repo: https://github.com/Glooring/snap
```

## Real Feedback Signals

Useful signals:

- reproducible bug reports;
- doctor reports from real repositories with private details removed;
- docs confusion from first-time users;
- Windows, Linux, and WSL2 install feedback;
- benchmark runs with environment details;
- pull requests that improve tests, docs, or safety.

Not useful:

- artificial stars;
- vague hype posts;
- broad claims without evidence;
- destructive testing on existing user repositories.
