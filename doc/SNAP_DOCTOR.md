# Snap Doctor

`snap doctor` is Snap's health check for the current Git repository and Snap snapshot metadata.

It is read-only by default:

```bash
snap doctor
```

## What It Checks

`snap doctor` checks:

- empty Git object/ref files;
- `git status`;
- `HEAD` and current branch health;
- snapshot tag validity;
- Snap metadata blobs referenced by snapshot tags;
- metadata pin refs under `refs/snap-metadata/*`;
- active versus historical metadata problems.

## Repair Mode

Safe repair cases are available through:

```bash
snap doctor --repair
```

Repair mode creates a full `.git.backup.YYYYMMDD-HHMMSS` backup and asks before changing anything.

If old historical snapshots reference metadata blobs that are already gone, doctor can leave that as a warning. To explicitly accept that unrecoverable historical metadata loss:

```bash
snap doctor --repair --accept-metadata-loss
```

This rewrites affected historical tags without broken `Snap-Metadata-Ref` lines after creating a backup.

## JSON And CI Modes

Use JSON output when automation needs to parse the health report:

```bash
snap doctor --json
```

Use CI mode when a workflow should fail on either warnings or errors:

```bash
snap doctor --ci
```

JSON and CI mode can be combined:

```bash
snap doctor --json --ci
```

`--json` and `--ci` are read-only modes and cannot be combined with `--repair`.

## Exit Behavior

- `snap doctor` exits `0` after printing the report, even when it found warnings or repairable problems.
- `snap doctor --json` exits `0` after printing JSON, even when the JSON status is `warning` or `error`.
- `snap doctor --ci` exits `0` only when the report is clean.
- `snap doctor --ci` exits non-zero when warnings or errors are found.
- Command failures, invalid flag combinations, and repair failures also exit non-zero through Snap's normal error path.

## When To Run It

Run `snap doctor`:

- before and after risky refactors;
- after AI-agent edits;
- before restore, purge, or repair work;
- after manual Git cleanup;
- when Snap reports Git health errors.

For deeper manual repair guidance, see `doc/REPAIR_GIT_ERRORS.md` and `doc/GIT_HEALTH_STABILIZATION.md`.
