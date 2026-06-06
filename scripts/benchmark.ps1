param(
    [string]$SnapBin = "",
    [string]$WorkDir = "",
    [switch]$Keep,
    [switch]$Help
)

if ($Help) {
    Write-Host @"
Usage: pwsh -File scripts/benchmark.ps1 [-SnapBin PATH] [-WorkDir DIR] [-Keep] [-Help]

Runs a local Snap benchmark smoke test in a disposable Git repository.

Options:
  -SnapBin PATH  Use an existing source-built Snap binary.
  -WorkDir DIR   Benchmark workspace root. Defaults to target/snap-benchmarks.
  -Keep          Keep the disposable benchmark repository after the run.
  -Help          Print this help text.
"@
    exit 0
}

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$ScriptDir = Split-Path -Parent $PSCommandPath
$RepoRoot = (Resolve-Path (Join-Path $ScriptDir "..")).Path
Set-Location $RepoRoot

if ([string]::IsNullOrWhiteSpace($SnapBin)) {
    & cargo build --release
    if ($LASTEXITCODE -ne 0) {
        throw "cargo build --release failed."
    }
    $SnapBin = Join-Path $RepoRoot "target\release\snap.exe"
}

if (-not (Test-Path $SnapBin)) {
    throw "Snap binary was not found: $SnapBin"
}

if ([string]::IsNullOrWhiteSpace($WorkDir)) {
    $WorkDir = Join-Path $RepoRoot "target\snap-benchmarks"
}

New-Item -ItemType Directory -Force -Path $WorkDir | Out-Null
$BenchDir = Join-Path $WorkDir ("bench-" + (Get-Date -Format "yyyyMMdd-HHmmss"))
New-Item -ItemType Directory -Force -Path $BenchDir | Out-Null

function Invoke-Timed {
    param(
        [Parameter(Mandatory = $true)][string]$Label,
        [Parameter(Mandatory = $true)][string]$FilePath,
        [string[]]$Arguments = @()
    )

    $watch = [System.Diagnostics.Stopwatch]::StartNew()
    & $FilePath @Arguments | Out-Null
    if ($LASTEXITCODE -ne 0) {
        throw "$Label failed with exit code $LASTEXITCODE."
    }
    $watch.Stop()
    "{0,-28} {1,8} ms" -f $Label, [int]$watch.ElapsedMilliseconds
}

try {
    Write-Host "[snap-bench] Snap binary: $SnapBin"
    Write-Host "[snap-bench] Repository: $BenchDir"

    Set-Location $BenchDir
    & git init | Out-Null
    & git config user.email "snap-benchmark@example.invalid"
    & git config user.name "Snap Benchmark"

    New-Item -ItemType Directory -Force -Path "src\module with spaces" | Out-Null
    New-Item -ItemType Directory -Force -Path "data\unicode-λ" | Out-Null
    New-Item -ItemType Directory -Force -Path "metadata\nested\empty\leaf" | Out-Null

    foreach ($i in 1..250) {
        "line {0:D3}" -f $i | Set-Content -NoNewline -Path ("src\file-{0:D3}.txt" -f $i)
    }
    "space path" | Set-Content -NoNewline -Path "src\module with spaces\file one.txt"
    "unicode path" | Set-Content -NoNewline -Path "data\unicode-λ\naïve-Δ.txt"
    "hidden" | Set-Content -NoNewline -Path ".hidden-benchmark"
    Set-ItemProperty -Path ".hidden-benchmark" -Name Attributes -Value ([System.IO.FileAttributes]::Hidden)
    Set-ItemProperty -Path "src\file-250.txt" -Name IsReadOnly -Value $true

    & git add .
    & git commit -m "initial benchmark content" | Out-Null

    ""
    "{0,-28} {1}" -f "command", "duration"
    "{0,-28} {1}" -f "-------", "--------"
    Invoke-Timed "snap init" $SnapBin @("init")
    Invoke-Timed "snap new baseline" $SnapBin @("new", "bench-baseline", "--include-metadata-only", "baseline benchmark snapshot")

    foreach ($i in 1..50) {
        Add-Content -Path ("src\file-{0:D3}.txt" -f $i) -Value ("changed {0:D3}" -f $i)
    }
    New-Item -ItemType Directory -Force -Path "metadata\new empty\child" | Out-Null
    "new file" | Set-Content -NoNewline -Path "src\module with spaces\new file.txt"

    Invoke-Timed "snap new changed" $SnapBin @("new", "bench-changed", "--include-metadata-only", "changed benchmark snapshot")
    Invoke-Timed "snap list" $SnapBin @("list")
    Invoke-Timed "snap diff" $SnapBin @("diff", "bench-baseline", "bench-changed")
    Invoke-Timed "snap doctor" $SnapBin @("doctor")
    Invoke-Timed "restore dry-run" $SnapBin @("restore", "bench-baseline", "--dry-run")

    ""
    "[snap-bench] Complete. Results are local observations only."
}
finally {
    Set-Location $RepoRoot
    if (-not $Keep) {
        Remove-Item -Recurse -Force -Path $BenchDir -ErrorAction SilentlyContinue
    } else {
        Write-Host "[snap-bench] Kept benchmark repository: $BenchDir"
    }
}
