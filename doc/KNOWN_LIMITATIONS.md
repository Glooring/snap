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

## Safety Work Still Planned

The roadmap tracks:

- moving snapshots away from ordinary Git tag names;
- broader cross-platform destructive-operation tests;
- packaging and command-name conflict decisions for Linux systems where `snap` already means Snapcraft.

Restore dry-run, restore rescue snapshots, doctor JSON/CI modes, documented doctor exit behavior, stricter command-construction cleanup, Windows/Linux CI, and core OSS hygiene files are present after the Sprint 1-5 OSS-readiness work.

## Strict Clippy

Strict `cargo clippy --all-targets --all-features -- -D warnings` is expected to pass after the Sprint 4 cleanup. Future changes should keep that baseline green.
