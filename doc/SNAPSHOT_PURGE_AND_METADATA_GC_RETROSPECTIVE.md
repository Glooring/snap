# Snapshot purge, metadata blobs, and `snap doctor` hardening

## Context

This document records a real incident from `little-fighter-web` and turns it into a concrete design plan for `snap`.

The incident happened after a very large accidental snapshot:

- A snapshot named `v91.84` was created by mistake.
- That snapshot added `original/LF2 All In Ultimate`, roughly several gigabytes of LF2/mod assets.
- The project was still active on `v91.83`, while `v91.84` existed as a later snapshot tag.
- The user wanted to remove `v91.84` and also remove the large blobs from `.git`, not just hide the label from `snap list`.

The important lesson is that there are two different delete operations:

- A normal snapshot delete removes the snapshot label/tag.
- A purge delete removes the snapshot label/tag and then makes Git eligible to physically discard objects that are only reachable from that snapshot.

Today `snap delete` only does the first one.

## What happened

### Initial state

In `little-fighter-web`, `snap list` showed:

```text
v91.84  2026-05-08 16:43
v91.83  2026-05-08 14:12  (active)
v91.82  2026-05-08 13:47
...
```

Git inspection showed:

```text
99ddbbf9b (tag: v91.84) Snapshot: v91.84
9f1dd3a48 (HEAD -> master, tag: v91.83) Snapshot: v91.83
```

So `v91.84` was not the active branch tip. It was a tag pointing to a later commit, and `master` was still at `v91.83`.

That matters because deleting `v91.84` did not require moving `master` or resetting the working tree.

### Manual cleanup that removed the big snapshot

The manual cleanup used this sequence:

```bash
git tag -d v91.84
git reflog expire --expire-unreachable=now --all
git gc --prune=now
```

This did remove the accidental snapshot from visible Git history and allowed Git to discard objects no longer reachable from any ref.

Afterward:

- `v91.84` no longer existed.
- `v91.83` was still active.
- `original/LF2 All In Ultimate` was not present in the active worktree.
- `.git` shrank from about `1.6G` to about `1.1G`.

The reduction was smaller than the raw folder size because Git packfiles compress and deduplicate content.

### New problem after cleanup

After the cleanup, this command failed:

```bash
snap new v91.84
```

with:

```text
[snap] Error: Failed to read metadata blob object 'd84e3507d6a3ff4b84fc1c296b970cba7d9e2ffe'
  Caused by: Command failed: 'git cat-file blob d84e3507d6a3ff4b84fc1c296b970cba7d9e2ffe'
```

At the same time:

```bash
snap list
snap doctor
```

still looked healthy. `snap list` worked, and `snap doctor` reported the Git repository as healthy.

That was misleading.

## Root cause

`snap` stores metadata separately from normal Git file content.

Relevant code:

- `src/utils.rs`
  - `SnapMetadata`
  - `gather_metadata`
  - `hash_metadata_blob`
  - `create_tag_message`
  - `load_metadata_for_snapshot`
- `src/commands/new.rs`
- `src/commands/update.rs`
- `src/commands/delete.rs`
- `src/git_health.rs`

The metadata type is:

```rust
pub struct SnapMetadata {
    pub hidden_paths: Vec<String>,
    pub readonly_paths: Vec<String>,
    pub empty_dirs: Vec<String>,
}
```

When metadata is non-empty, `snap` serializes it as JSON and writes it as a loose Git blob:

```rust
let blob_hash = run_command("git hash-object -w --stdin", Some(&json_content))?;
```

Then `snap` puts that blob hash inside the annotated tag message:

```text
Snap-Metadata-Ref: d84e3507d6a3ff4b84fc1c296b970cba7d9e2ffe
```

This is human-readable, but it is not a Git object edge.

Git does not treat a SHA mentioned in a tag message as an object reference. From Git's object graph perspective, the metadata blob is unreachable unless some real ref, tree, commit, tag object, or other object graph edge points to it.

So this situation can happen:

```text
annotated tag message contains:
  Snap-Metadata-Ref: d84e...

but no Git ref points to d84e...
and no commit tree contains d84e...
and no tag object points to d84e... as its tagged object
```

That blob is vulnerable to:

```bash
git gc --prune=now
```

In the incident, `git gc --prune=now` removed the metadata blob `d84e...` because it was unreachable.

The remaining tags still contained:

```text
Snap-Metadata-Ref: d84e...
```

but the blob itself was gone.

## Why `snap new` failed

`snap new` does more than make a new commit. Before creating a snapshot, it compares:

- current Git file changes, from `git status --porcelain`;
- current metadata, from `gather_metadata`;
- old metadata for the active snapshot, from `load_metadata_for_snapshot`.

Relevant path:

```rust
let old_metadata = match get_active_commit_full()? {
    Some(id) => {
        if let Some(active_snapshot) = find_snapshot(&all_snapshots, &id) {
            load_metadata_for_snapshot(active_snapshot)?
        } else {
            Default::default()
        }
    }
    None => Default::default(),
};
```

`load_metadata_for_snapshot` parses the tag message, extracts `Snap-Metadata-Ref`, and runs:

```rust
git cat-file blob <metadata_hash>
```

Because `d84e...` had been pruned, `git cat-file blob d84e...` failed. That made `snap new` fail before it could create the new snapshot.

## Why `snap list` did not fail

`snap list` calls `get_snapshots()`.

`get_snapshots()` parses tag messages and extracts:

- tag name;
- target commit;
- timestamp;
- user description.

It does not validate or load the metadata blob.

So `snap list` can show a snapshot as valid even if its `Snap-Metadata-Ref` points to a missing blob.

This is acceptable for fast listing if intentional, but it means `snap list` is not a health check.

## Why `snap doctor` did not detect it

`snap doctor` currently checks general Git health:

- `.git` exists;
- empty `.git/objects` or `.git/refs` files;
- `git status`;
- HEAD commit;
- branch ref;
- snapshot tag scan;
- each snapshot tag resolves to a commit.

Relevant code:

```rust
fn collect_snapshot_checks() -> Result<(Vec<SnapshotCheck>, Option<String>)> {
    ...
    let commit = run_git(
        &["rev-parse", "--verify", &format!("{}^{{commit}}", tag)],
        None,
    )?;
    ...
}
```

This only verifies that each tag can resolve to a commit.

It does not:

- parse `Snap-Metadata-Ref`;
- verify that the metadata hash exists;
- verify that the metadata object is a blob;
- verify that the blob contains valid `SnapMetadata` JSON;
- verify that metadata blobs are protected from future `git gc`.

That is why `snap doctor` reported healthy while `snap new` was still broken.

## What fixed the repository manually

The repair sequence was:

1. Detect that active tag `v91.83` had:

   ```text
   Snap-Metadata-Ref: d84e3507d6a3ff4b84fc1c296b970cba7d9e2ffe
   ```

2. Confirm that the blob was missing:

   ```bash
   git cat-file -t d84e3507d6a3ff4b84fc1c296b970cba7d9e2ffe
   ```

   returned:

   ```text
   fatal: git cat-file: could not get object info
   ```

3. Temporarily retag `v91.83` to a valid metadata blob so `snap new` could run.

4. Run:

   ```bash
   snap new v91.84
   ```

   This recreated the original metadata blob `d84e...`, because the current worktree metadata serialized to the same JSON and therefore the same Git blob hash.

5. Pin the metadata blob with a real Git ref:

   ```bash
   git update-ref \
     refs/snap-metadata/d84e3507d6a3ff4b84fc1c296b970cba7d9e2ffe \
     d84e3507d6a3ff4b84fc1c296b970cba7d9e2ffe
   ```

6. Restore `v91.83` to use the original metadata ref.

Final state:

```text
v91.84 active
v91.83 preserved
Snap-Metadata-Ref: d84e3507...
refs/snap-metadata/d84e3507... -> d84e3507...
```

The crucial fix is the `refs/snap-metadata/...` ref. It makes the metadata blob reachable, so a future `git gc --prune=now` will keep it.

## Would plain `snap delete` have solved this?

No.

Current `snap delete` only does:

```rust
run_command(&format!("git tag -d {}", snapshot_to_delete.tag), None)?;
```

So plain `snap delete v91.84` would have removed the tag label, but it would not necessarily remove the large data from `.git`.

Whether the data disappears from `.git` depends on Git reachability:

- If the deleted snapshot commit is still reachable from a branch, another tag, or a reflog, the objects stay.
- If the deleted snapshot commit is unreachable but Git has not run GC, the objects usually stay for now.
- Only after reflog expiry and garbage collection can Git physically drop unreachable objects.

So the behavior today is:

```text
snap delete
  = remove tag only
  = does not promise disk-space recovery
  = does not run git gc
  = does not protect snap metadata before gc
```

For the accidental large snapshot case, plain `snap delete` is not enough.

## Should `snap delete` run GC by default?

Probably not.

Running:

```bash
git reflog expire --expire-unreachable=now --all
git gc --prune=now
```

is intentionally destructive for unreachable objects. It can remove:

- deleted snapshots;
- dangling commits;
- temporary work saved only in reflog;
- other unreachable blobs the user might have expected to recover.

So normal `snap delete` should remain conservative.

The better design is a separate explicit purge mode.

## Proposed feature: `snap delete --purge`

Add an explicit purge option:

```bash
snap delete v91.84 --purge
```

or a separate command:

```bash
snap purge v91.84
```

The naming should make the risk visible.

Suggested CLI:

```rust
pub struct DeleteArgs {
    pub id_or_label: Option<String>,

    /// Also prune Git objects reachable only from this snapshot.
    #[arg(long)]
    pub purge: bool,

    /// Show what would be deleted or kept without changing anything.
    #[arg(long)]
    pub dry_run: bool,

    /// Allow purging the active snapshot/branch tip after explicit checks.
    #[arg(long)]
    pub force_active: bool,
}
```

### `snap delete` default behavior

Default behavior should stay:

```text
delete tag only
do not expire reflogs
do not run gc
do not move branches
```

Output should be more explicit:

```text
[snap] Snapshot tag deleted.
[snap] Disk space was not reclaimed.
[snap] To remove objects that were only reachable from this snapshot, run:
       snap delete <label> --purge
```

### `snap delete --purge` behavior

Purge should do a full safety flow.

Recommended flow:

1. Resolve the snapshot tag.

   ```bash
   git rev-parse --verify v91.84^{commit}
   git rev-parse --verify refs/tags/v91.84
   ```

2. Check whether the snapshot commit is active.

   If:

   ```bash
   git rev-parse HEAD
   ```

   equals the target commit, abort unless `--force-active`.

3. Check whether the snapshot commit is reachable from branches or other tags.

   Example:

   ```bash
   git branch --contains <commit>
   git tag --contains <commit>
   ```

   If any branch contains it, purge cannot reclaim those objects without moving or rewriting branch history. `snap` should report that and stop.

4. Pin all metadata blobs used by remaining snapshots before any GC.

   This must happen before reflog expiry or GC.

5. Estimate unique objects.

   Conceptually:

   ```bash
   git rev-list --objects <target_commit> --not <all_other_refs>
   ```

   The implementation should exclude the target tag itself from the set of other refs.

6. Print a clear warning:

   ```text
   This will:
   - delete tag v91.84
   - expire unreachable reflog entries
   - run git gc --prune=now
   - permanently remove objects reachable only from v91.84
   ```

7. Optionally create a safety bundle.

   Example:

   ```bash
   git bundle create .snap-backups/v91.84-<timestamp>.bundle v91.84
   ```

   This is much smaller and more targeted than copying all of `.git`.

8. Delete the tag.

   ```bash
   git tag -d v91.84
   ```

9. Expire unreachable reflogs and run GC.

   ```bash
   git reflog expire --expire-unreachable=now --all
   git gc --prune=now
   ```

10. Verify.

    - target tag is gone;
    - remaining snapshot metadata blobs exist;
    - `snap doctor` is clean;
    - `.git` size changed or a message explains why it did not.

### Purge should not move branches by default

If the target commit is the tip of `master`, or is contained by `master`, `snap delete --purge` should not silently move `master`.

That is a different operation:

```text
restore/rollback branch to an older snapshot
```

Purge should be for snapshots that are not branch-critical.

## Proposed feature: metadata pinning

Every metadata blob created by `snap` should be protected by a real Git ref.

When `hash_metadata_blob` returns a blob hash, `snap` should run:

```bash
git update-ref refs/snap-metadata/<hash> <hash>
```

Suggested helper:

```rust
pub fn pin_metadata_blob(hash: &str) -> Result<()> {
    run_command(
        &format!("git update-ref refs/snap-metadata/{} {}", hash, hash),
        None,
    )?;
    Ok(())
}
```

Then `new` and `update` should do:

```rust
let metadata_blob_hash = hash_metadata_blob(&current_metadata)?;
if let Some(hash) = metadata_blob_hash.as_deref() {
    pin_metadata_blob(hash)?;
}
```

This solves the core issue: Git will consider the metadata blob reachable.

### Cleanup unused metadata refs

If metadata refs are pinned forever, they can accumulate.

Add a cleanup step:

```bash
snap doctor --repair
```

or:

```bash
snap metadata prune
```

It should:

1. collect all `Snap-Metadata-Ref` hashes from current snapshot tags;
2. list refs under `refs/snap-metadata/`;
3. delete metadata refs that are no longer used by any snapshot tag;
4. never delete the blob directly; let normal Git GC handle it later.

## Proposed `snap doctor` improvements

`snap doctor` should be extended from general Git health to snap-specific health.

### New checks

For each snapshot tag:

1. Resolve tag to commit.

   Current behavior already does this.

2. Parse `Snap-Metadata-Ref`.

   If absent, this is valid and means default empty metadata.

3. If present, verify object existence.

   ```bash
   git cat-file -e <hash>^{blob}
   ```

4. Verify JSON parse.

   ```rust
   serde_json::from_str::<SnapMetadata>(&json)
   ```

5. Verify metadata blob is pinned.

   ```bash
   git show-ref --verify refs/snap-metadata/<hash>
   ```

6. Verify metadata ref points to the same blob hash.

   ```bash
   git rev-parse refs/snap-metadata/<hash>
   ```

7. Detect orphan metadata refs.

   A ref under `refs/snap-metadata/` that is not used by any snapshot tag should be reported as unused, not necessarily an error.

### Report model changes

Extend `GitHealthReport` with snap metadata checks:

```rust
pub struct MetadataBlobCheck {
    pub snapshot_tag: String,
    pub blob_hash: String,
    pub exists: bool,
    pub valid_json: bool,
    pub pinned: bool,
    pub error: Option<String>,
}

pub struct GitHealthReport {
    ...
    pub metadata_blobs: Vec<MetadataBlobCheck>,
    pub unused_metadata_refs: Vec<String>,
}
```

Then `has_errors()` should include:

```rust
self.metadata_blobs.iter().any(|m| !m.exists || !m.valid_json)
```

Missing pin refs could be either:

- warning in `doctor`;
- automatic repair in `doctor --repair`.

Given the incident, unpinned metadata should be treated as at least a warning:

```text
WARN Snapshot metadata blob exists but is not protected from git gc.
```

### `doctor --repair` behavior

Safe automatic repairs:

1. Existing metadata blob is valid but unpinned:

   ```bash
   git update-ref refs/snap-metadata/<hash> <hash>
   ```

2. Metadata ref exists but points to the wrong object:

   ```bash
   git update-ref refs/snap-metadata/<hash> <hash>
   ```

3. Metadata ref exists but blob is unused:

   Do not delete by default. Offer a separate cleanup mode.

Repairs requiring user confirmation:

1. Metadata blob missing for the active snapshot.

   If HEAD is at that snapshot, `snap` can gather current metadata and retag the active snapshot with a new valid metadata blob.

   This changes the annotated tag object hash, so it should be done only after backup and clear confirmation.

2. Metadata blob missing for old non-active snapshots.

   This cannot always be reconstructed exactly.

   Reasons:

   - empty directories are not stored in Git commit trees;
   - hidden file attributes can be OS-specific;
   - readonly attributes can be OS-specific;
   - current worktree metadata may not match old snapshot metadata.

   Safe options:

   - mark the snapshot as metadata-damaged;
   - retag with empty metadata only if user explicitly chooses that;
   - copy metadata from nearest later/earlier snapshot only if user explicitly chooses that;
   - restore that snapshot in a temporary worktree and gather metadata there, if possible.

## Proposed write preflight improvements

`ensure_git_healthy_for_write` is currently fast and does not validate metadata blobs.

That is why `snap new` reached `load_metadata_for_snapshot` and failed with a low-level `git cat-file` error.

Before `new` and `update`, `snap` should validate the active snapshot metadata:

```text
if active snapshot has Snap-Metadata-Ref:
  verify blob exists
  verify blob deserializes
  verify blob is pinned or pin it automatically
```

If missing:

```text
[snap] Active snapshot metadata blob is missing:
  Snapshot: v91.83
  Blob: d84e3507...

This usually happens after a manual git gc/prune.
Run:
  snap doctor --repair
```

That error is much more actionable than:

```text
Failed to read metadata blob object
```

## Proposed data model improvement

The current design puts metadata outside the commit tree and references it only through tag message text.

That is lightweight, but fragile unless metadata blobs are pinned.

There are three possible long-term designs.

### Option A: Keep current model and pin blobs with refs

This is the smallest change.

Pros:

- easy to implement;
- no change to commit contents;
- no hidden `.snap` files in every snapshot;
- compatible with existing tags after repair.

Cons:

- requires managing `refs/snap-metadata/*`;
- doctor must know how to repair/purge metadata refs;
- metadata is still not part of the snapshot commit itself.

This is the recommended short-term fix.

### Option B: Store metadata in a tracked file inside each snapshot

For example:

```text
.snap/metadata.json
```

Pros:

- metadata is naturally reachable from the commit tree;
- no custom metadata refs are needed;
- `git gc` cannot prune it while the snapshot commit is reachable.

Cons:

- changes project tree contents;
- may be annoying for users;
- must be ignored from normal project logic;
- retrofitting old snapshots is invasive.

This is robust but less elegant for `snap` as a transparent tool.

### Option C: Use Git notes

Store metadata as notes on snapshot commits.

Pros:

- avoids changing project tree;
- Git-native metadata concept.

Cons:

- notes have their own refs and sync semantics;
- users can accidentally omit notes when pushing/fetching;
- still requires ref management.

This is viable but more complex than Option A.

## Suggested implementation plan

### Phase 1: Prevent future metadata loss

Add:

```rust
pin_metadata_blob(hash: &str)
```

Call it from:

- `src/commands/new.rs`
- `src/commands/update.rs`

Also add a helper:

```rust
pin_all_snapshot_metadata_blobs()
```

and run it before:

- `snap delete --purge`;
- `snap doctor --repair`;
- maybe `snap new` opportunistically.

### Phase 2: Make `doctor` detect real snap metadata health

Add metadata checks to `src/git_health.rs`.

The report should distinguish:

```text
OK   Snapshot tag resolves to commit
ERR  Snapshot metadata blob missing
ERR  Snapshot metadata blob is not valid JSON
WARN Snapshot metadata blob is not pinned
WARN Unused metadata ref
```

### Phase 3: Add safe metadata repair

`snap doctor --repair` should:

- create `.git.backup.<timestamp>`;
- pin all existing valid metadata blobs;
- repair bad metadata refs;
- report missing metadata blobs;
- optionally repair active missing metadata by regenerating from current worktree.

Do not silently invent historical metadata for older snapshots.

### Phase 4: Add purge delete

Add:

```bash
snap delete <label> --purge
```

It should:

- refuse to purge active/branch-reachable snapshots by default;
- pin remaining metadata first;
- optionally create a targeted bundle backup;
- delete the tag;
- expire unreachable reflogs;
- run `git gc --prune=now`;
- re-run doctor;
- print disk-size before/after.

### Phase 5: Add tests

Suggested tests:

1. `new_pins_metadata_blob`

   - create hidden file or empty dir;
   - run `snap new v1`;
   - parse tag `Snap-Metadata-Ref`;
   - assert `git cat-file -t <hash>` is `blob`;
   - assert `refs/snap-metadata/<hash>` exists.

2. `doctor_detects_missing_metadata_blob`

   - create snapshot with metadata;
   - delete the metadata ref;
   - prune the blob;
   - run doctor;
   - assert report contains metadata missing error.

3. `doctor_repair_pins_unpinned_existing_metadata`

   - create metadata blob;
   - remove only `refs/snap-metadata/<hash>`;
   - keep blob object present;
   - run `doctor --repair`;
   - assert ref exists again.

4. `delete_without_purge_does_not_gc`

   - create snapshot with unique large-ish file;
   - run `snap delete v2`;
   - assert tag gone;
   - do not assert disk reclaim.

5. `delete_purge_removes_unreachable_snapshot`

   - create v1, then detached/tag-only v2 or simulate tag-only snapshot;
   - run `snap delete v2 --purge`;
   - assert tag gone;
   - assert metadata for remaining snapshots still works;
   - assert `snap new v3` succeeds.

6. `delete_purge_refuses_branch_reachable_commit`

   - create snapshot at HEAD;
   - run `snap delete active --purge`;
   - assert refusal without `--force-active`.

## Operational playbook until the tool is fixed

If a huge accidental snapshot is created and it is not active:

1. Check active state:

   ```bash
   snap list
   git log --oneline --decorate --all --max-count=10
   ```

2. Check whether a branch contains the bad snapshot commit:

   ```bash
   git branch --contains <bad_commit>
   git tag --contains <bad_commit>
   ```

3. Before any `git gc`, pin all existing `Snap-Metadata-Ref` blobs:

   ```bash
   for hash in $(git for-each-ref refs/tags --format='%(contents)' \
     | sed -n 's/^Snap-Metadata-Ref: //p' \
     | sort -u); do
     git cat-file -e "$hash^{blob}" &&
       git update-ref "refs/snap-metadata/$hash" "$hash"
   done
   ```

4. Delete the bad tag:

   ```bash
   git tag -d <bad_snapshot>
   ```

5. Only if you really want disk reclaim:

   ```bash
   git reflog expire --expire-unreachable=now --all
   git gc --prune=now
   ```

6. Verify:

   ```bash
   snap list
   snap doctor
   snap new <next_label>
   ```

This manual playbook is intentionally cautious. Once `snap delete --purge` exists, the tool should own these steps.

## Summary

The incident was not caused by a corrupted Git repository. It was caused by a gap in `snap`'s metadata storage model.

The big snapshot was removable because it was tag-only and not branch-reachable. However, the manual Git prune also deleted a `snap` metadata blob because `Snap-Metadata-Ref` inside a tag message is text, not a real Git reachability edge.

The main product fixes are:

- pin metadata blobs under real refs such as `refs/snap-metadata/<hash>`;
- teach `snap doctor` to validate metadata blobs and metadata refs;
- keep plain `snap delete` conservative;
- add explicit `snap delete --purge` for disk-space reclamation;
- make purge pin remaining metadata before running any prune/GC.

With those changes, a user can safely recover from an accidental multi-gigabyte snapshot without breaking future `snap new` commands.
