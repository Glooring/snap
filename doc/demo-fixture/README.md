# Beginner Demo Fixture

This fixture is a tiny disposable project for trying the core Snap workflow before using Snap on a real repository.

It is adapted from the beginner fixture contribution in [PR #7](https://github.com/Glooring/snap/pull/7) by [@ded-furby](https://github.com/ded-furby).

## What It Covers

- initialize a disposable Git project;
- configure local demo-only Git identity;
- create two non-release-looking Snap checkpoints;
- list and compare checkpoints;
- preview restore safely with `--dry-run`;
- run `snap doctor`;
- delete the disposable project afterward.

## Bash / WSL / macOS / Linux

If `snap` is already installed and available on `PATH`, use:

```bash
SNAP_BIN=snap
```

If you are testing from a source checkout, build Snap first from the Snap repository root:

```bash
cargo build --release
SNAP_BIN="$(pwd)/target/release/snap"
```

Then create the disposable demo project:

```bash
SNAP_DEMO_ROOT="$(mktemp -d)"
mkdir -p "$SNAP_DEMO_ROOT/snap-beginner-demo"
cd "$SNAP_DEMO_ROOT/snap-beginner-demo"

git init
git config user.email "snap-demo@example.invalid"
git config user.name "Snap Demo"

mkdir -p src
cat > src/hello.txt <<'TXT'
hello snap
TXT

"$SNAP_BIN" init
"$SNAP_BIN" new demo-start "initial demo state"
"$SNAP_BIN" list
```

Make a change and capture a second checkpoint:

```bash
echo "hello again" >> src/hello.txt
"$SNAP_BIN" new demo-change "updated demo note"
"$SNAP_BIN" list
```

Compare the two checkpoints:

```bash
"$SNAP_BIN" diff demo-start demo-change
```

Preview a restore before changing files:

```bash
"$SNAP_BIN" restore demo-start --dry-run
```

Check repository health:

```bash
"$SNAP_BIN" doctor
```

Optional real restore inside the disposable project:

```bash
"$SNAP_BIN" restore demo-start
"$SNAP_BIN" list
```

Clean up:

```bash
cd /
rm -rf "$SNAP_DEMO_ROOT"
```

## Windows PowerShell Notes

The same flow works on Windows. Create a disposable directory with PowerShell, then run the same Snap commands:

```powershell
$DemoRoot = Join-Path $env:TEMP "snap-beginner-demo"
Remove-Item -Recurse -Force $DemoRoot -ErrorAction SilentlyContinue
New-Item -ItemType Directory -Path $DemoRoot | Out-Null
Set-Location $DemoRoot

git init
git config user.email "snap-demo@example.invalid"
git config user.name "Snap Demo"

New-Item -ItemType Directory -Path "src" | Out-Null
"hello snap" | Set-Content -Path "src/hello.txt"

snap init
snap new demo-start "initial demo state"
snap list

"hello again" | Add-Content -Path "src/hello.txt"
snap new demo-change "updated demo note"
snap list
snap diff demo-start demo-change
snap restore demo-start --dry-run
snap doctor
```

Delete `$DemoRoot` when done.

## Notes

- The labels `demo-start` and `demo-change` intentionally avoid release-looking names like `v1` or `v2`.
- Git identity is configured only inside the disposable demo repository.
- `snap restore --dry-run` is the safe way to inspect restore impact before changing files.
- `snap doctor` is read-only by default and is useful when learning how Snap checks Git and Snap metadata health.
