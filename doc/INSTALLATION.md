# Installation And Release Assets

This document explains how to install or verify Snap release assets without confusing this project with Canonical Snapcraft's `snap` command on Linux.

Snap's current project name and built binary name are still `snap`. On Linux and WSL2, you may choose a different local filename such as `gitsnap` when installing the same binary to avoid a PATH conflict.

## Release Assets

GitHub Releases use versioned asset names:

```text
snap-vX.Y.Z-windows-x86_64.exe
snap-vX.Y.Z-windows-x86_64-setup.exe
snap-vX.Y.Z-windows-x86_64.msi
snap-vX.Y.Z-linux-x86_64
snap-vX.Y.Z-linux-x86_64.tar.gz
SHA256SUMS.txt
```

The Windows assets are built on Windows. The Linux assets are built on Linux or WSL2. The tar archive contains the same Linux binary as the standalone `snap-vX.Y.Z-linux-x86_64` asset.

## Checksums

Each public release should include a `SHA256SUMS.txt` file with SHA-256 checksums for every uploaded binary or archive asset.

On Linux or WSL2:

```bash
sha256sum -c SHA256SUMS.txt
```

On Windows PowerShell:

```powershell
Get-FileHash .\snap-vX.Y.Z-windows-x86_64.exe -Algorithm SHA256
Get-Content .\SHA256SUMS.txt
```

Compare the hash values before running a downloaded binary. The current release scripts build the platform artifacts; checksum publication remains part of the release checklist until it is automated.

## Verify A Binary

Before installing, run the binary directly when possible.

Windows PowerShell:

```powershell
.\snap-vX.Y.Z-windows-x86_64.exe --help
```

Linux or WSL2:

```bash
chmod +x ./snap-vX.Y.Z-linux-x86_64
./snap-vX.Y.Z-linux-x86_64 --help
```

`snap doctor` checks the current Git repository, so run it from inside a Git repository:

```bash
git status
./snap-vX.Y.Z-linux-x86_64 doctor
```

If you build from source, validate the source-built release binary:

```bash
cargo build --release
./target/release/snap --help
./target/release/snap doctor
```

## Windows Install

Recommended normal install:

- use `snap-vX.Y.Z-windows-x86_64.msi`;
- install under `C:\Program Files\snap`;
- open a new terminal after installation;
- verify with `where snap`.

Portable install:

- put `snap-vX.Y.Z-windows-x86_64.exe` in a directory you control, such as `D:\Apps\snap\bin\snap.exe` or `%USERPROFILE%\bin\snap.exe`;
- add that directory to PATH if desired;
- keep only one Snap install directory early in PATH.

Verify:

```powershell
where snap
snap --help
snap doctor
```

If `where snap` prints more than one path, Windows runs the first one. Remove stale PATH entries or uninstall the older copy before assuming an upgrade worked.

## Linux Install

First check whether the command name is already taken:

```bash
command -v snap || true
snap version 2>/dev/null || true
```

If `snap` is Canonical Snapcraft, do not overwrite it casually. Prefer a conflict-safe local filename:

```bash
mkdir -p "$HOME/.local/bin"
install -m 0755 ./snap-vX.Y.Z-linux-x86_64 "$HOME/.local/bin/gitsnap"
command -v gitsnap
gitsnap --help
```

If `~/.local/bin` is not on PATH, add it using your shell's normal profile file and open a new terminal.

If you intentionally want this project to be the `snap` command and no system package already depends on Canonical Snapcraft in your PATH, install to a user-controlled location:

```bash
mkdir -p "$HOME/.local/bin"
install -m 0755 ./snap-vX.Y.Z-linux-x86_64 "$HOME/.local/bin/snap"
command -v snap
snap --help
```

Avoid replacing `/usr/bin/snap` or another package-manager-owned command. A future packaging decision may add an official alternate Linux binary name, but Sprint 8 does not rename the project or binary.

## WSL2 Install

Use the Linux asset or a Linux source build inside WSL2. Do not use the Windows `.exe` or MSI as the WSL2 install.

Recommended conflict-safe WSL2 install:

```bash
mkdir -p "$HOME/.local/bin"
install -m 0755 ./snap-vX.Y.Z-linux-x86_64 "$HOME/.local/bin/gitsnap"
command -v gitsnap
gitsnap --help
```

If `command -v snap` points to a Windows path such as `/mnt/c/.../snap.exe`, WSL2 is resolving the Windows binary through PATH import. Prefer the native Linux binary through `gitsnap`, or put the desired WSL2 path earlier in PATH.

## Maintainer Release Checklist

For maintainer build commands, see [Windows and WSL installer build notes](BUILD_INSTALLERS_WINDOWS_WSL.md).

Before publishing a release:

```bash
cargo test
cargo build --release
./target/release/snap --help
./target/release/snap doctor
```

Then prepare the platform assets, generate `SHA256SUMS.txt`, upload artifacts to a draft GitHub Release, and verify the published asset list before marking the release as public.
