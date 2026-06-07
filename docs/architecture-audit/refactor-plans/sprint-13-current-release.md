# Sprint 13 - Current GitHub Release

Status: planned at sprint start
Sprint checkpoint: `oss-s13-release`
Date: 2026-06-07
Scope: publish a current Snap release with Windows and Linux artifacts, checksums, release notes, and validation evidence

## Purpose

Sprint 13 closes the remaining public-release gap in the OSS-readiness program.

Sprint 12 made the repository public-ready and CI-backed. The remaining closure item is a current GitHub Release. This sprint should publish `v7.2.0`, matching the current Cargo package version, using GitHub-hosted Windows and Linux runners so release artifacts are built and tested on their target platforms.

## Scope

In scope:

- add a GitHub Actions release workflow;
- build Linux release assets with `scripts/release-linux.sh`;
- build Windows release assets with `scripts/release-windows.ps1`;
- generate `SHA256SUMS.txt` from the exact uploaded assets;
- publish GitHub Release `v7.2.0`;
- add release notes for `v7.2.0`;
- update install/release docs so they match the release workflow;
- verify release assets after publication;
- update progress/audit/application package docs.

Out of scope:

- changing Rust CLI behavior;
- changing the Cargo package version;
- replacing, reinstalling, overwriting, or upgrading global `/usr/local/bin/snap`;
- pushing local Snap checkpoint tags;
- merging PR #7 before the contributor rebases or updates it;
- namespaced snapshot-ref migration;
- alternate Linux binary rename.

## Release Version Decision

Use `v7.2.0`.

Reasoning:

- `Cargo.toml` currently declares package version `7.2.0`.
- No local or remote `v7.2.0` tag exists at sprint start.
- No GitHub release exists at sprint start.
- This is the first public OSS-readiness release for the current source tree.

## Validation Plan

Local validation before publishing:

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

GitHub validation:

```bash
git push origin main
gh run watch <ci-run-id> --repo Glooring/snap --exit-status
gh workflow run release.yml --repo Glooring/snap -f tag=v7.2.0 -f publish=true
gh run watch <release-run-id> --repo Glooring/snap --exit-status
gh release view v7.2.0 --repo Glooring/snap --json tagName,name,isDraft,isPrerelease,publishedAt,isLatest
```

Post-release artifact validation:

```bash
tmpdir="$(mktemp -d /tmp/snap-release-v7.2.0-XXXXXX)"
gh release download v7.2.0 --repo Glooring/snap --dir "$tmpdir"
cd "$tmpdir"
sha256sum -c SHA256SUMS.txt
chmod +x ./snap-v7.2.0-linux-x86_64
./snap-v7.2.0-linux-x86_64 --version
./snap-v7.2.0-linux-x86_64 --help
```

Run a Linux downloaded-asset smoke test in a disposable Git repository:

```bash
smoke="$(mktemp -d /tmp/snap-release-smoke-v7.2.0-XXXXXX)"
cd "$smoke"
git init
git config user.email "snap-release-smoke@example.invalid"
git config user.name "Snap Release Smoke"
echo one > app.txt
git add app.txt
git commit -m "initial"
"$tmpdir/snap-v7.2.0-linux-x86_64" init
"$tmpdir/snap-v7.2.0-linux-x86_64" new release-start "release smoke start"
"$tmpdir/snap-v7.2.0-linux-x86_64" doctor
```

Windows executable behavior is validated on the Windows GitHub runner by the release script. Local Linux can verify downloaded Windows files by checksum and presence, but not execute them.

## Acceptance Criteria

- Sprint 13 plan exists and is linked from the refactor-plan index.
- `v7.2.0` GitHub Release exists and is published.
- Release assets include Windows, Linux, and `SHA256SUMS.txt`.
- Checksums verify after downloading the release.
- Downloaded Linux binary passes version/help and a disposable-repo smoke test.
- Release workflow and public CI are green.
- Audit/progress/application package docs record the release evidence.
- Final checkpoint is created with:

```bash
snap new oss-s13-release "sprint 13: current GitHub release"
snap list
git status --short
```
