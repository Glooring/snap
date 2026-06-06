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

- `snap restore <label>` resets project files to a snapshot. By default it creates a rescue snapshot first when the current state is dirty or otherwise unprotected by a snapshot tag.
- `snap restore <label> --dry-run` previews the target, worktree state, and rescue behavior without changing files or tags.
- `snap restore <label> --no-rescue` skips rescue snapshot creation and can discard uncommitted local changes after confirmation.
- `snap delete <label>` deletes a snapshot tag after confirmation.
- `snap delete <label> --purge` can reclaim disk space by pruning Git objects. It refuses active or branch-reachable snapshots, creates a targeted bundle backup by default, pins remaining metadata, and runs a final health check.
- `snap edit` and `snap update` rewrite snapshot tags or active snapshot state.
- `snap doctor --repair` creates a full `.git.backup.YYYYMMDD-HHMMSS` backup and asks before applying safe repairs.

## Restore Safety

Use dry-run before restoring when the state is unclear:

```bash
snap restore <label> --dry-run
```

A normal restore creates a rescue snapshot first when needed. Rescue labels use `snap-rescue-YYYYMMDD-HHMMSS`, which avoids release-looking names and gives you a tag to restore back to.

Use `--no-rescue` only when you intentionally want the older discard-confirmation path:

```bash
snap restore <label> --no-rescue
```

## Doctor Automation

Doctor remains read-only unless `--repair` is present.

```bash
snap doctor --json
snap doctor --ci
snap doctor --json --ci
```

`snap doctor --ci` exits successfully only for a clean report. Warnings or errors return a non-zero process exit through the normal Snap error path. Plain `snap doctor` still exits successfully after printing warnings because it is a diagnostic command.

## What Snap Tries To Protect

- It checks for dirty worktrees before restore and creates rescue snapshots before changing files unless `--no-rescue` is passed.
- It refuses normal write operations when Git health preflight fails.
- It pins snapshot metadata blobs under real Git refs so Git garbage collection keeps them reachable.
- It reports missing or invalid metadata through `snap doctor`.
- It lets automation consume doctor results through JSON and CI modes.

## Current Safety Gaps

The OSS-readiness roadmap still tracks safety work that is larger than Sprint 5:

- moving snapshots away from ordinary Git tag names;
- broader cross-platform destructive-operation tests;
- a packaging/name-conflict strategy for systems where `snap` already means Snapcraft.

Run `snap doctor`, keep important work pushed or backed up, and avoid using Snap as your only backup strategy.
