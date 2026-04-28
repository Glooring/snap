# Git Health Stabilization: `snap doctor`, safer restore, and preflight checks

This document records the Git-stability work added to `snap` after repeated WSL/Remote incidents where Git reported empty loose objects, broken refs, or detached `HEAD` states.

## Problem

Some projects occasionally ended up with a corrupted `.git` state:

```bash
[snap] Error: Command failed: 'git status --porcelain'
error: object file .git/objects/XX/YYYY... is empty
fatal: loose object XXYYYY... is corrupt
```

Related incidents also included:

- empty tag refs in `.git/refs/tags`;
- branch refs such as `refs/heads/master` or `refs/heads/main` pointing to invalid commits;
- `.git/HEAD` containing a raw SHA instead of `ref: refs/heads/<branch>`;
- `snap restore` leaving the repo in detached `HEAD` because it used `git checkout --force <tag>`.

The existing manual repair flow remains documented in `doc/REPAIR_GIT_ERRORS.md`.

## What changed

### New `snap doctor` command

Added a read-only diagnostic command:

```bash
snap doctor
```

It checks:

- whether `.git` exists;
- empty files under `.git/objects` and `.git/refs`;
- whether `git status --porcelain` succeeds;
- whether `HEAD^{commit}` resolves;
- whether `HEAD` is detached;
- whether the active branch ref is valid;
- whether snapshot tags resolve to valid commits;
- the latest valid snapshot tag.

`snap doctor` does not delete files, repair refs, reset the index, or change project state. If it finds a problem, it points the user to `doc/REPAIR_GIT_ERRORS.md`.

### Shared Git health preflight

Write operations now run a health preflight before doing any Git mutation:

- `snap new`
- `snap restore`
- `snap update`
- `snap edit`
- `snap delete`

The preflight blocks writes when it detects:

- empty Git object/ref files;
- `git status` failure;
- detached `HEAD`;
- invalid `HEAD`;
- invalid current branch ref;
- invalid snapshot tags.

`snap new` allows an unborn `HEAD` so the first snapshot in a newly initialized repo still works.

### Safer restore

`snap restore` no longer runs:

```bash
git checkout --force <tag>
```

That command checks out a tag directly and can leave the repository in detached `HEAD`.

The new flow is:

```bash
git rev-parse --verify <tag>^{commit}
git reset --hard <snapshot_commit>
```

This keeps `HEAD` attached to the current branch while moving the branch to the selected snapshot commit. The existing metadata synchronization still runs afterward:

- empty directories;
- hidden paths;
- read-only paths.

### Snapshot listing errors are no longer hidden

Previously, `get_snapshots()` returned an empty list if `git for-each-ref` failed. That could make a broken repository look like it simply had no snapshots.

Now the error is propagated with context:

```text
Failed to inspect snapshot tags. Run `snap doctor` for a read-only diagnosis
```

## Files added or changed

Core implementation:

- `src/git_health.rs` - shared Git health checks and Git command helpers.
- `src/commands/doctor.rs` - `snap doctor` CLI output.
- `src/commands/restore.rs` - restore now resolves tag to commit and uses `git reset --hard`.
- `src/utils.rs` - `get_snapshots()` now reports Git ref errors instead of hiding them.
- command modules for write operations now call the preflight before mutating Git state.

CLI/documentation/tests:

- `src/cli.rs` and `src/main.rs` - registered the `Doctor` command.
- `README.md` - documented `snap doctor` and the new troubleshooting behavior.
- `doc/REPAIR_GIT_ERRORS.md` - updated repair guidance to use the active branch instead of hardcoding `master`.
- `tests/git_health.rs` - integration tests for doctor/preflight/restore behavior.
- `Cargo.toml` and `Cargo.lock` - added integration-test dependencies.

## Test coverage

The integration tests cover:

- `snap doctor` on a healthy repo;
- detection of an empty loose object;
- detection of an empty tag ref;
- detection of detached `HEAD`;
- `snap list` reporting Git ref errors instead of showing an empty snapshot list;
- `snap restore` keeping `HEAD` attached to the branch;
- `snap restore` moving the branch to the target snapshot commit;
- `snap new` stopping before writing when the health preflight fails.

Verification commands used:

```bash
cargo check
cargo test
```

Expected result:

```text
7 passed
```

## WSL notes

The code is cross-platform and applies to Linux/Ubuntu WSL too, but WSL must use a Linux build of `snap`.

In WSL:

```bash
cd /path/to/snap
cargo build --release
cp target/release/snap ~/.local/bin/snap
```

Verify:

```bash
which snap
snap doctor
```

If `which snap` points to a Windows path such as `/mnt/c/.../snap.exe`, WSL is still using the Windows binary. Prefer a native Linux binary such as:

```text
/home/<user>/.local/bin/snap
```

## Current limitations

- `snap doctor --repair` is not implemented.
- `snap doctor` is intentionally read-only.
- If `HEAD` is already detached, write commands stop and ask the user to inspect the repo with `snap doctor` and repair manually.
- The snapshot storage format is unchanged: snapshots remain annotated Git tags with optional metadata blob references.
