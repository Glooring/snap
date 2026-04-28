# Build and install Snap on Windows and WSL

This document explains the release/build flow for the `snap` CLI after the Git-health stabilization changes.

There are two targets:

- **Windows installer artifacts**
  - `target\wix\snap-7.2.0-x86_64.msi`
  - `snap-setup.exe`
- **WSL/Linux binary**
  - native Linux executable installed as `/usr/local/bin/snap`
  - optional archive: `target/linux-dist/snap-linux-x86_64.tar.gz`

## 1. Build Windows installers

Run from Windows PowerShell or Command Prompt:

```powershell
cd D:\Projects\snap
cargo test
cargo make installers
```

Expected output includes a release build and NSIS installer creation:

```text
Finished `release` profile
Output: "D:\Projects\snap\snap-setup.exe"
Build Done
```

After the command finishes, verify the artifacts:

```powershell
dir target\release\snap.exe
dir target\wix\snap-7.2.0-x86_64.msi
dir snap-setup.exe
```

Expected files:

```text
D:\Projects\snap\target\release\snap.exe
D:\Projects\snap\target\wix\snap-7.2.0-x86_64.msi
D:\Projects\snap\snap-setup.exe
```

You can also verify the freshly built executable directly, without installing:

```powershell
.\target\release\snap.exe --version
.\target\release\snap.exe doctor
```

Expected:

```text
snap 7.2.0
[snap] Git repository looks healthy.
```

## 2. Which Windows installer to use

Keep only one Windows installation of `snap` in PATH. If `where snap` prints more than one path, Windows uses the first one and ignores the others unless the first path is removed.

Example:

```powershell
where snap
```

Possible output:

```text
D:\Apps\snap\bin\snap.exe
C:\Program Files\snap\bin\snap.exe
```

In this case Windows runs `D:\Apps\snap\bin\snap.exe`. Keeping both paths can make upgrades confusing, because one copy may be updated while the other remains old.

Recommended local setup for this machine:

```text
D:\Apps\snap\bin\snap.exe
```

Manual update:

```powershell
cd D:\Projects\snap
copy /Y target\release\snap.exe D:\Apps\snap\bin\snap.exe
```

Then open a new terminal and verify:

```powershell
where snap
snap --version
snap doctor
```

Ideal `where snap` output for this setup:

```text
D:\Apps\snap\bin\snap.exe
```

If `C:\Program Files\snap\bin\snap.exe` also appears and you do not want to use it, uninstall that Snap installation from Windows Apps/Programs or remove `C:\Program Files\snap\bin` from PATH. Delete the folder only after uninstalling/removing the PATH entry.

### MSI

Use:

```text
target\wix\snap-7.2.0-x86_64.msi
```

The WiX installer installs `snap.exe` under Program Files and includes PATH integration. This is the better installer for normal Windows installation and upgrades.

Install by double-clicking the MSI, or from an elevated terminal:

```powershell
msiexec /i target\wix\snap-7.2.0-x86_64.msi
```

Then open a new terminal and verify:

```powershell
where snap
snap --version
snap doctor
```

Use the MSI when you want a normal Windows installer and Program Files installation. If you prefer the custom `D:\Apps\snap\bin` setup, do not install the MSI on your own machine unless you also remove the `D:\Apps\snap\bin` PATH entry or intentionally switch to Program Files.

### NSIS EXE

Use:

```text
snap-setup.exe
```

This installer is produced by `snap.nsi`. It installs the executable and creates uninstall/start-menu entries. If PATH is not updated in a new terminal after using the EXE installer, prefer the MSI or add the install directory to PATH manually.

You can use `snap-setup.exe`, but it is mainly useful for distribution. On this machine it may recreate or update the `C:\Program Files\snap` installation, which can conflict with the preferred `D:\Apps\snap\bin` copy if both are in PATH.

## 3. Build and install for WSL / Ubuntu

The Windows `.exe` and `.msi` are not the right artifacts for WSL. WSL should use a native Linux build.

Open WSL:

```powershell
wsl -d Ubuntu-22.04
```

Go to the project:

```bash
cd /mnt/d/Projects/snap
```

If Rust is not installed in WSL, install it:

```bash
curl https://sh.rustup.rs -sSf | sh
source "$HOME/.cargo/env"
```

Verify Rust:

```bash
cargo --version
rustc --version
```

Build and test:

```bash
cargo test
cargo build --release
```

Install the Linux binary:

```bash
sudo cp target/release/snap /usr/local/bin/snap
sudo chmod +x /usr/local/bin/snap
```

Verify:

```bash
which snap
snap --version
snap doctor
```

Expected `which snap`:

```text
/usr/local/bin/snap
```

If it points to `/mnt/c/.../snap.exe`, WSL is still using the Windows binary. Put `/usr/local/bin` earlier in PATH or remove the Windows path entry from the WSL PATH.

## 4. Optional Linux archive

If you want a portable Linux archive:

```bash
cd /mnt/d/Projects/snap
mkdir -p target/linux-dist
cp target/release/snap target/linux-dist/
tar -czf target/linux-dist/snap-linux-x86_64.tar.gz -C target/linux-dist snap
```

Windows path:

```text
D:\Projects\snap\target\linux-dist\snap-linux-x86_64.tar.gz
```

WSL path:

```text
/mnt/d/Projects/snap/target/linux-dist/snap-linux-x86_64.tar.gz
```

## 5. Recommended release checklist

Use this checklist every time you prepare a new release:

```powershell
cd D:\Projects\snap
cargo test
cargo make installers
.\target\release\snap.exe doctor
```

Then install/test Windows:

For the preferred local `D:\Apps\snap\bin` setup:

```powershell
copy /Y target\release\snap.exe D:\Apps\snap\bin\snap.exe
```

Open a new terminal:

```powershell
where snap
snap --version
snap doctor
```

For a Program Files installer test instead:

```powershell
msiexec /i target\wix\snap-7.2.0-x86_64.msi
```

Open a new terminal:

```powershell
where snap
snap --version
snap doctor
```

Then install/test WSL:

```bash
wsl -d Ubuntu-22.04
cd /mnt/d/Projects/snap
cargo test
cargo build --release
sudo cp target/release/snap /usr/local/bin/snap
which snap
snap --version
snap doctor
```

## 6. Notes from the latest build

The latest Windows build produced:

```text
D:\Projects\snap\snap-setup.exe
D:\Projects\snap\target\release\snap.exe
D:\Projects\snap\target\wix\snap-7.2.0-x86_64.msi
```

The freshly built Windows executable reported:

```text
snap 7.2.0
```

And `snap doctor` reported the repository as healthy.
