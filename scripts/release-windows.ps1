# Creates Windows release assets for GitHub Releases.
# Edit these defaults if the release layout or target name changes.
[CmdletBinding()]
param(
    [string]$ReleaseRoot = "release-github",
    [string]$WindowsTarget = "windows-x86_64",
    [string]$AppName = "snap"
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$ScriptDir = Split-Path -Parent $PSCommandPath
$RepoRoot = (Resolve-Path (Join-Path $ScriptDir "..")).Path
Set-Location $RepoRoot

function Require-Command {
    param(
        [Parameter(Mandatory = $true)][string]$Name,
        [Parameter(Mandatory = $true)][string]$InstallHint
    )

    if (-not (Get-Command $Name -ErrorAction SilentlyContinue)) {
        throw "Required command '$Name' was not found. $InstallHint"
    }
}

function Invoke-Native {
    param(
        [Parameter(Mandatory = $true)][string]$Label,
        [Parameter(Mandatory = $true)][string]$FilePath,
        [string[]]$Arguments = @()
    )

    Write-Host "`n[snap-release] $Label" -ForegroundColor Cyan
    & $FilePath @Arguments
    if ($LASTEXITCODE -ne 0) {
        throw "$Label failed with exit code $LASTEXITCODE."
    }
}

function Get-CargoVersion {
    $pkgId = (& cargo pkgid).Trim()
    if ($LASTEXITCODE -ne 0 -or [string]::IsNullOrWhiteSpace($pkgId)) {
        throw "Failed to read package version with 'cargo pkgid'."
    }

    return ($pkgId -split "#")[-1]
}

Require-Command "cargo" "Install Rust from https://rustup.rs/."
Require-Command "cargo-wix" "Install it with: cargo install cargo-wix"
Require-Command "makensis" "Install NSIS and make sure makensis.exe is in PATH."

$Version = Get-CargoVersion
$ReleaseVersion = "v$Version"
$ReleaseRootPath = if ([System.IO.Path]::IsPathRooted($ReleaseRoot)) {
    $ReleaseRoot
} else {
    Join-Path $RepoRoot $ReleaseRoot
}
$ReleaseDir = Join-Path $ReleaseRootPath $ReleaseVersion
New-Item -ItemType Directory -Force -Path $ReleaseDir | Out-Null

$PortableExeSource = Join-Path $RepoRoot "target\release\$AppName.exe"
$PortableExeOut = Join-Path $ReleaseDir "$AppName-$ReleaseVersion-$WindowsTarget.exe"
$SetupExeOut = Join-Path $ReleaseDir "$AppName-$ReleaseVersion-$WindowsTarget-setup.exe"
$MsiOut = Join-Path $ReleaseDir "$AppName-$ReleaseVersion-$WindowsTarget.msi"

Invoke-Native "Running Windows test suite" "cargo" @("test")
Invoke-Native "Building Windows release binary" "cargo" @("build", "--release")
Invoke-Native "Building Windows MSI installer" "cargo" @("wix")
Invoke-Native "Building Windows NSIS setup EXE" "makensis" @(
    "/DAPP_VER=$Version",
    "/DOUT_FILE=$SetupExeOut",
    "snap.nsi"
)

if (-not (Test-Path $PortableExeSource)) {
    throw "Expected release binary was not found: $PortableExeSource"
}

$MsiCandidates = @(Get-ChildItem -Path (Join-Path $RepoRoot "target\wix") -Filter "$AppName-$Version-*.msi" -ErrorAction SilentlyContinue)
if ($MsiCandidates.Count -eq 0) {
    throw "Expected MSI was not found under target\wix for version $Version."
}
if ($MsiCandidates.Count -gt 1) {
    throw "Multiple MSI files matched version $Version under target\wix. Remove stale files or adjust the script."
}

Copy-Item -Force -Path $PortableExeSource -Destination $PortableExeOut
Copy-Item -Force -Path $MsiCandidates[0].FullName -Destination $MsiOut

foreach ($Artifact in @($PortableExeOut, $SetupExeOut, $MsiOut)) {
    if (-not (Test-Path $Artifact)) {
        throw "Release artifact was not created: $Artifact"
    }
}

Invoke-Native "Verifying portable Windows binary version" $PortableExeOut @("--version")

Write-Host "`n[snap-release] Windows release assets are ready:" -ForegroundColor Green
Write-Host "  $PortableExeOut"
Write-Host "  $SetupExeOut"
Write-Host "  $MsiOut"
