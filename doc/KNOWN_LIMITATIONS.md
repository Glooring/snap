# Known Limitations

This document records current public limitations so Snap's README can stay honest.

## Local-First Tool

Snap is not a cloud backup service. It creates local Git-backed checkpoints. Use Git remotes, normal backups, or other storage if you need off-machine recovery.

## Git Is Required

Snap uses Git for storage and history. If Git is missing or the repository is corrupted, Snap commands may fail until `snap doctor` or manual Git repair resolves the problem.

## Snapshot Tags

Snap snapshots are currently represented as Git tags. This keeps snapshots visible and portable, but it can confuse normal release tags with Snap snapshot tags.

New Snap-created tags include `Snap-Snapshot: true`, and Snap snapshot views filter to marked tags, metadata-bearing tags, and legacy Snap-style tags. Ordinary release tags are no longer treated as Snap snapshots.

Until a namespaced ref model is designed, avoid routine checkpoint labels that look like releases, such as `v1.2.3`.

## Linux Command Name Conflict

On many Linux systems, `snap` already means Canonical Snapcraft. Check your PATH before installing this project as `snap`.

The current strategy is documented in [Installation and release assets](INSTALLATION.md): keep the project and built binary named `Snap`/`snap` for now, but use a conflict-safe local filename such as `gitsnap` on Linux or WSL2 when Canonical Snapcraft already owns `snap`.

A future packaging decision may add an official alternate Linux binary name. That would need a separate decision record and compatibility plan.

## Safety Work Still Planned

The roadmap tracks:

- deciding whether to move snapshots away from ordinary Git tag names;
- broader cross-platform destructive-operation tests;
- automating release checksums and deciding whether to publish an official alternate Linux binary name.

Restore dry-run, restore rescue snapshots, doctor JSON/CI modes, documented doctor exit behavior, stricter command-construction cleanup, Windows/Linux CI, and core OSS hygiene files are present after the Sprint 1-5 OSS-readiness work.

## Strict Clippy

Strict `cargo clippy --all-targets --all-features -- -D warnings` is expected to pass after the Sprint 4 cleanup. Future changes should keep that baseline green.
