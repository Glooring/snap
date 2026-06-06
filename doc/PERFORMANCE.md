# Performance Methodology

Snap should not claim to be faster than Git or "blazing fast" without current benchmark evidence. Git is the storage engine, and Snap adds workflow, metadata, safety checks, and friendlier commands around it.

Use this document to gather repeatable local measurements and to interpret them conservatively.

## What To Measure

Measure workflows that match real Snap use:

- `snap init` in a fresh Git repository;
- `snap new` with normal file changes;
- `snap new` when empty-directory, hidden-file, or read-only metadata exists;
- `snap list` on repositories with many snapshot tags;
- `snap diff` between two snapshots;
- `snap doctor` on a healthy repository;
- `snap restore --dry-run` before a destructive restore.

Avoid measuring only the Git subcommands underneath Snap. That hides the user-facing cost of metadata scan, tag filtering, safety preflight, terminal output, and health checks.

## Local Benchmark Scripts

Linux, macOS, or WSL2:

```bash
bash scripts/benchmark.sh
```

Windows PowerShell:

```powershell
pwsh -File scripts/benchmark.ps1
```

Both scripts:

- build `target/release/snap` unless an explicit binary path is provided;
- create a disposable Git repository under `target/snap-benchmarks/`;
- exercise representative Snap commands;
- print command durations;
- remove the disposable repository by default.

Keep benchmark output local unless you also record the machine, OS, filesystem, Git version, Rust version, Snap commit, file counts, and exact command line.

## Interpreting Results

Report performance as local observations, not universal promises.

Prefer:

```text
On this machine, with 500 files and 25 directories, `snap doctor` completed in N ms.
```

Avoid:

```text
Snap is faster than Git.
Snap is instant.
Snap is blazing fast.
```

Small differences can come from filesystem cache warmth, antivirus, WSL2 mount location, terminal output, Git version, CPU power settings, and whether the repository is on a native Linux filesystem or a mounted Windows drive.

## Suggested Manual Matrix

When preparing release evidence, run the benchmark script in at least these environments if available:

| Environment | Filesystem | Notes |
| --- | --- | --- |
| Linux | native ext4/btrfs/xfs | Best local Linux signal. |
| WSL2 | Linux home directory | Preferred WSL2 signal. |
| WSL2 | `/mnt/c` or `/mnt/d` | Useful because Windows-mounted paths can behave differently. |
| Windows | NTFS | Needed for Windows hidden/read-only metadata behavior. |

Record unavailable environments explicitly instead of implying they were tested.

## Current Baseline

Sprint 9 adds benchmark scripts and edge-case tests. It does not publish universal benchmark numbers. Future release notes can include measured results after running the scripts on release hardware.
