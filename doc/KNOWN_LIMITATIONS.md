# Known Limitations

This document records current public limitations so Snap's README can stay honest.

## Local-First Tool

Snap is not a cloud backup service. It creates local Git-backed checkpoints. Use Git remotes, normal backups, or other storage if you need off-machine recovery.

## Git Is Required

Snap uses Git for storage and history. If Git is missing or the repository is corrupted, Snap commands may fail until `snap doctor` or manual Git repair resolves the problem.

## Snapshot Tags

Snap snapshots are currently represented as Git tags. This keeps snapshots visible and portable, but it can confuse normal release tags with Snap snapshot tags.

Until the tag/ref model is improved, avoid routine checkpoint labels that look like releases, such as `v1.2.3`.

## Linux Command Name Conflict

On many Linux systems, `snap` already means Canonical Snapcraft. Check your PATH before installing this project as `snap`. The packaging/name-conflict strategy is still planned.

## Safety Features Still Planned

The roadmap tracks:

- restore dry-run;
- rescue snapshot before restore;
- doctor JSON/CI modes;
- documented exit codes;
- stricter command-construction audit;
- Windows/Linux CI.

## OSS Hygiene Still Planned

The repository still needs standard public project files such as `AGENTS.md`, `CONTRIBUTING.md`, `SECURITY.md`, `CHANGELOG.md`, issue templates, PR template, CI, and a root `LICENSE` file.

## Strict Clippy

Normal Clippy runs, but strict `cargo clippy --all-targets --all-features -- -D warnings` is known baseline debt. The cleanup is planned before strict CI enforcement.
