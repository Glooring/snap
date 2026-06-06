# Safety Model

Snap is a local CLI that can change Git history, project files, tags, and metadata. Its safety model is based on explicit commands, confirmations, backups for repair/purge flows, and read-only diagnostics by default.

## Read-Only By Default

`snap doctor` is read-only unless you pass `--repair`.

Use it before and after risky work:

```bash
snap doctor
```

## Destructive Or Mutating Commands

These commands deserve extra care:

- `snap restore <label>` resets project files to a snapshot and can discard uncommitted local changes after confirmation.
- `snap delete <label>` deletes a snapshot tag after confirmation.
- `snap delete <label> --purge` can reclaim disk space by pruning Git objects; it creates a targeted bundle backup by default.
- `snap edit` and `snap update` rewrite snapshot tags or active snapshot state.
- `snap doctor --repair` creates a full `.git.backup.YYYYMMDD-HHMMSS` backup and asks before applying safe repairs.

## What Snap Tries To Protect

- It checks for dirty worktrees before restore.
- It refuses normal write operations when Git health preflight fails.
- It pins snapshot metadata blobs under real Git refs so Git garbage collection keeps them reachable.
- It reports missing or invalid metadata through `snap doctor`.

## Current Safety Gaps

The OSS-readiness roadmap tracks additional safety improvements:

- `snap restore --dry-run`;
- rescue snapshot before restore;
- `snap doctor --json`;
- `snap doctor --ci`;
- documented exit codes.

Until those land, run `snap doctor`, keep important work pushed or backed up, and avoid using Snap as your only backup strategy.
