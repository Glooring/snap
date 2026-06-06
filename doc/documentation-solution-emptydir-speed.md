# Metadata Scan Performance Note

This note records the implementation lesson behind Snap's empty-directory metadata scan. It is historical context, not a benchmark claim.

Git does not store empty directories or platform-specific hidden/read-only attributes. Snap records that extra metadata in JSON, stores it as a Git blob, and pins the blob under `refs/snap-metadata/<hash>`.

## Current Approach

The metadata scan walks the project once to collect directories and file attributes, then checks candidate empty directories in parallel with Rayon.

The important design points are:

- keep Git as the source of truth for file contents;
- record only metadata Git does not store;
- avoid making `snap list` a full health check;
- keep full validation in `snap doctor`;
- measure performance with repeatable scripts instead of broad claims.

See [Performance methodology](PERFORMANCE.md) for current benchmark guidance.

## What Not To Claim

Do not use this historical note to claim Snap is faster than Git, instant, or universally fast. Performance depends on repository size, filesystem, WSL2 mount location, antivirus, Git version, terminal output, and metadata shape.

Use local measurements instead:

```bash
bash scripts/benchmark.sh
```

or on Windows:

```powershell
pwsh -File scripts/benchmark.ps1
```
