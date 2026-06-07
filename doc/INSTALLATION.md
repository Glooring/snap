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
snap-vX.Y.Z-macos-aarch64
snap-vX.Y.Z-macos-aarch64.tar.gz
snap-vX.Y.Z-macos-x86_64
snap-vX.Y.Z-macos-x86_64.tar.gz
SHA256SUMS.txt
```

The Windows assets are built on Windows. The Linux assets are built on Ubuntu GitHub runners. The macOS assets are built on macOS GitHub runners for Apple Silicon and Intel. The tar archives contain the same binary as the matching standalone platform asset.

For `v7.2.2`, the Linux binary is built on Ubuntu 24.04 and requires glibc 2.39 or newer. On older distributions, such as Ubuntu 22.04, build from source with `cargo build --release` until a future release adds an older-glibc or static Linux target.

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

On macOS:

```bash
shasum -a 256 -c SHA256SUMS.txt
```

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

macOS Apple Silicon:

```bash
chmod +x ./snap-vX.Y.Z-macos-aarch64
./snap-vX.Y.Z-macos-aarch64 --help
```

macOS Intel:

```bash
chmod +x ./snap-vX.Y.Z-macos-x86_64
./snap-vX.Y.Z-macos-x86_64 --help
```

`snap doctor` checks the current Git repository, so run it from inside a Git repository:

```bash
git status
./snap-vX.Y.Z-linux-x86_64 doctor
```

Use the matching macOS binary name when verifying on macOS.

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

## macOS Install

Choose the asset for your Mac:

- Apple Silicon: `snap-vX.Y.Z-macos-aarch64`
- Intel: `snap-vX.Y.Z-macos-x86_64`

Portable install:

```bash
mkdir -p "$HOME/.local/bin"
install -m 0755 ./snap-vX.Y.Z-macos-aarch64 "$HOME/.local/bin/snap"
command -v snap
snap --help
```

Use the Intel filename on Intel Macs.

The macOS binaries are unsigned and not notarized. If Gatekeeper/quarantine blocks a downloaded binary, verify `SHA256SUMS.txt` first, then remove quarantine only from the downloaded Snap binary you intend to run:

```bash
xattr -d com.apple.quarantine "$HOME/.local/bin/snap"
```

If `~/.local/bin` is not on PATH, add it using your shell's normal profile file and open a new terminal.

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

Before publishing a release locally:

```bash
cargo test
cargo build --release
./target/release/snap --help
./target/release/snap doctor
```

The preferred public release path is the GitHub Actions release workflow:

```bash
gh workflow run release.yml --repo Glooring/snap -f tag=vX.Y.Z -f publish=true
gh run watch <run-id> --repo Glooring/snap --exit-status
```

The workflow builds Linux, Windows, and macOS assets on their target runners, generates `SHA256SUMS.txt`, creates the GitHub Release, and uploads all release assets. After publication, download the assets, verify checksums, and smoke-test the Linux binary from a disposable Git repository. Windows and macOS executable behavior is validated on their target runners during the workflow.

For published releases, the manual `Release Smoke` workflow can also be run with a tag such as `v7.2.2`. It downloads the public assets, verifies checksums on Ubuntu, and smoke-tests the public Linux, Windows, and macOS portable binaries in disposable Git repositories.
