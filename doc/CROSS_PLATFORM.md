# Cross-Platform Notes

Snap is intended for Windows, Linux, and WSL2. The implementation keeps Git as the storage engine and stores Snap-specific metadata as Git blobs pinned under `refs/snap-metadata`.

## Current Proof

| Area | Current evidence |
| --- | --- |
| Linux | Local source-built validation runs in the OSS-readiness sprints. |
| Windows | GitHub Actions CI runs on `windows-latest`; Windows-specific metadata code exists in `src/os/windows.rs`. |
| WSL2 | Install and release docs describe native Linux builds for WSL2; run Linux validation inside WSL2 when preparing releases. |
| Source validation | Use `cargo run -- ...`, `./target/debug/snap ...`, `./target/release/snap ...`, `cargo test`, and `cargo build --release`. |

The global `/usr/local/bin/snap` used during this refactor is checkpoint-only and is not evidence for current source behavior.

## Path And Metadata Behavior

Snap records metadata that Git does not normally store:

- nested empty directories;
- hidden paths;
- read-only paths.

Platform differences matter:

- On Unix-like systems, hidden paths are names that start with `.`.
- On Windows, hidden paths use the Windows hidden file attribute.
- Read-only behavior is available on both platforms, but cleanup and restore behavior should be tested carefully because Windows can enforce read-only files more strictly during deletion.
- WSL2 should use a native Linux Snap binary. A Windows `snap.exe` found through WSL PATH import is not the recommended WSL2 runtime.

## Edge Cases To Keep Covered

The test suite should keep coverage for:

- paths with spaces;
- Unicode paths;
- nested empty directories;
- hidden metadata;
- read-only metadata;
- dirty worktree restore safety;
- source-built `snap doctor` on this repository.

Sprint 9 adds focused tests for these path and metadata cases. Windows CI should run the same Rust tests where the platform supports the behavior.

## Release Checklist

Before claiming cross-platform release readiness, record:

```bash
git diff --check
cargo fmt --check
cargo clippy --all-targets --all-features
cargo test
cargo build --release
./target/release/snap --help
./target/release/snap doctor
```

For Windows, use the CI result or a Windows local run:

```powershell
cargo test
cargo build --release
.\target\release\snap.exe --help
.\target\release\snap.exe doctor
```

For WSL2, run the Linux commands inside WSL2 with a native Linux binary.
