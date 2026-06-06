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

## When To Run It

Run `snap doctor`:

- before and after risky refactors;
- after AI-agent edits;
- before restore, purge, or repair work;
- after manual Git cleanup;
- when Snap reports Git health errors.

For deeper manual repair guidance, see `doc/REPAIR_GIT_ERRORS.md` and `doc/GIT_HEALTH_STABILIZATION.md`.
