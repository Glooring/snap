# 📦 Snap Installer Setup Guide

This guide walks you through all steps to prepare and build both the **MSI** (via WiX) and **EXE** (via NSIS) installers for the Snap CLI tool on Windows.

---

## 🛠️ Prerequisites

Before you begin, ensure the following tools are installed and available on your `PATH`:

1. **Rust Toolchain** (stable, `x86_64-pc-windows-msvc`).
2. **WiX Toolset v4+** (`candle.exe`, `light.exe`).
3. **NSIS** (`makensis.exe`).
4. **cargo-make** (task runner). Install with:

   ```powershell
   cargo install cargo-make
   ```
5. **cargo-wix** (WiX helper). Install with:

   ```powershell
   cargo install cargo-wix
   ```

---

## 1. Configure `Cargo.toml`

In your project’s `Cargo.toml`, under `[package]`, add the following fields:

```toml
[package]
name = "snap"
version = "7.2.0"
authors = ["Your Name <you@example.com>"]
edition = "2021"
description = "A blazing fast, Git-powered snapshot tool for Windows developers."
license = "MIT"

[package.metadata.wix]
eula = false   # suppress WiX EULA warning if you don’t have an RTF
```

This ensures `cargo wix init` will run without errors and includes metadata for the installer.

---

## 2. Initialize WiX Project

Scaffold the WiX template in `wix/main.wxs`:

```powershell
cd <your-project-root>
cargo wix init
```

This generates `wix\main.wxs`, which you will customize in the next step.

---

## 3. Edit `wix/main.wxs`

Open `wix/main.wxs` and locate the component that installs `snap.exe`. Insert the `<Environment>` element so that the installer updates the **user** `PATH` automatically:

```xml
<Component Id="binary0" Guid="*">
  <File Source="$(var.CargoTargetBinDir)\snap.exe" KeyPath="yes"/>
  <Environment
      Id="AddToPath"
      Name="PATH"
      Action="set"
      Part="last"
      System="no"
      Value="[INSTALLDIR]\bin"/>
</Component>
```

Save your changes.

---

## 4. Create NSIS Script (`snap.nsi`)

At the project root, create `snap.nsi` with the following content:

```nsi
!include "MUI2.nsh"
RequestExecutionLevel admin

!define APP_NAME    "snap"
!define APP_VER     "7.2.0"
!define APP_EXE     "snap.exe"

Name "${APP_NAME} ${APP_VER}"
OutFile "${APP_NAME}-setup.exe"
InstallDir "$PROGRAMFILES\${APP_NAME}"
InstallDirRegKey HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APP_NAME}" ""

!insertmacro MUI_PAGE_WELCOME
!insertmacro MUI_PAGE_LICENSE "wix\License.rtf"
!insertmacro MUI_PAGE_DIRECTORY
!insertmacro MUI_PAGE_INSTFILES
!insertmacro MUI_PAGE_FINISH

!insertmacro MUI_LANGUAGE "English"

Section "Install"
    SetOutPath "$INSTDIR"
    File /oname=${APP_EXE} "target\release\snap.exe"
    WriteUninstaller "$INSTDIR\Uninstall.exe"
    WriteRegStr HKLM "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\${APP_NAME}" \
      "DisplayName" "${APP_NAME} ${APP_VER}"
    WriteRegStr HKLM "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\${APP_NAME}" \
      "UninstallString" '"$INSTDIR\\Uninstall.exe"'
    CreateDirectory "$SMPROGRAMS\\${APP_NAME}"
    CreateShortCut "$SMPROGRAMS\\${APP_NAME}\\${APP_NAME}.lnk" "$INSTDIR\\${APP_EXE}"
SectionEnd

Section "Uninstall"
    Delete "$INSTDIR\\${APP_EXE}"
    Delete "$INSTDIR\\Uninstall.exe"
    RMDir "$INSTDIR"
    Delete "$SMPROGRAMS\\${APP_NAME}\\${APP_NAME}.lnk"
    RMDir "$SMPROGRAMS\\${APP_NAME}"
    DeleteRegKey HKLM "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\${APP_NAME}"
SectionEnd
```

---

## 5. Create `Makefile.toml`

Define your build pipeline with **cargo-make**. Add `Makefile.toml` at the root:

```toml
[tasks.build]
description = "Compile Snap release binary"
command     = "cargo"
args        = ["build", "--release"]

[tasks.wix]
description = "Compile WiX MSI installer"
script = [
  "if exist target\\wix rmdir /S /Q target\\wix",
  "cargo wix"
]

[tasks.nsis]
description = "Compile NSIS EXE installer"
command     = "makensis"
args        = ["snap.nsi"]

[tasks.installers]
description  = "Build both MSI and EXE installers"
dependencies = ["build", "wix", "nsis"]
```

---

## 6. (Optional) PowerShell Helper

You can also use a script `build-installers.ps1`:

```powershell
Write-Host "1) Building Snap release…" -ForegroundColor Cyan
cargo build --release

Write-Host "2) Building MSI (WiX) installer…" -ForegroundColor Cyan
cargo wix

Write-Host "3) Building NSIS installer…" -ForegroundColor Cyan
makensis snap.nsi

Write-Host "`nDone!" -ForegroundColor Green
Write-Host " • target\\wix\\snap-7.2.0-x86_64.msi"
Write-Host " • snap-setup.exe"
```

---

## 7. Build Everything

Finally, run:

```powershell
cargo make installers
```

This will produce:

* `target\wix\snap-7.2.0-x86_64.msi`
* `snap-setup.exe`

Distribute either—or both—to your users. 🎉
