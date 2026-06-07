# Changelog

This project is moving toward a clearer public release process. Until release automation and changelog policy are finalized, this file records high-level OSS-readiness milestones.

## Unreleased

- No unreleased changes yet.

## 7.2.1 - 2026-06-07

- Added branch-aware discovery to top-level help, including direct hints for `snap list --help`, `snap history --help`, and `snap branch --help`.
- Added `snap history --branch <name>` and `snap history --all-branches` for visual Git graph inspection across local branches and snapshot tags.
- Clarified `snap list --all-branches` branch ownership output with a `-` placeholder and legend for snapshot tags not reachable from any local branch.
- Expanded integration tests for branch-aware list/history help, graph behavior, branch filtering, and placeholder output.

## 7.2.0 - 2026-06-07

- Repositioned README around Git-powered local checkpoints, AI-agent edits, risky refactors, beginner workflows, safety, limitations, and `snap doctor`.
- Added public docs for AI-agent workflow, beginner workflow, why not Git, safety model, `snap doctor`, and known limitations.
- Added OSS hygiene files and GitHub issue/PR templates.
- Added public CI for Ubuntu and Windows.
- Published GitHub Release `v7.2.0` with Linux and Windows assets plus `SHA256SUMS.txt`.
- Added a manual release-smoke workflow that downloads public assets and tests the portable Linux and Windows binaries.
- Documented that the `v7.2.0` Linux asset targets Ubuntu 24.04/glibc 2.39 or newer; older Linux distributions should build from source for now.
- Existing source command surface includes snapshots, daily Git workflow helpers, branch helpers, GitHub/remote helpers, release helpers, examples, `doctor`, and `options`.
