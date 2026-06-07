# Beginner Demo Fixture

This folder contains a tiny fixture you can run locally to exercise the core `snap` workflow.

## 1. Create and populate a disposable repo

```bash
SNAP_DEMO_ROOT="$(mktemp -d)"
mkdir -p "$SNAP_DEMO_ROOT/snap-beginner-demo"
cd "$SNAP_DEMO_ROOT/snap-beginner-demo"
git init
mkdir -p src
cat > src/hello.txt <<'TXT'
hello snap
TXT
snap init
snap new demo-start "initial commit"
```

If `snap` is not in `PATH`, use your installed executable path, for example:

```bash
/path/to/snap.exe init
/path/to/snap.exe new demo-start "initial commit"
```

## 2. Make a change and capture a second snapshot

```bash
echo "hello again" >> src/hello.txt
snap new demo-change "update note"
```

## 3. Verify health with doctor

```bash
snap doctor
snap list
```

If `snap doctor` and `snap list` show the expected snapshots, you can try the dry-run restore safely:

```bash
snap restore --dry-run demo-start
```

## 4. Restore for real (optional)

```bash
snap restore demo-start
snap restore --dry-run demo-change
```

## 5. Cleanup

From the temporary directory, remove the folder when done:

```bash
cd ..
rm -rf "$SNAP_DEMO_ROOT/snap-beginner-demo"
```

## Notes

- This demo is intentionally small and resets cleanly with `rm -rf` (or equivalent on Windows).
- The goal is to validate `init`, `new`, `doctor`, and dry-run restore behavior before using `snap` on real repos.
