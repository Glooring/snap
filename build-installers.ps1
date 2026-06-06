# build-installers.ps1

Write-Host "1) Building Snap release…" -ForegroundColor Cyan
cargo build --release
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

Write-Host "2) Building MSI (WiX) installer…" -ForegroundColor Cyan
cargo wix
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

Write-Host "3) Building NSIS installer…" -ForegroundColor Cyan
makensis snap.nsi
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

Write-Host "`nDone. Installers created:" -ForegroundColor Green
Write-Host " • target\wix\snap-7.2.0-x86_64.msi"
Write-Host " • snap-setup.exe"
