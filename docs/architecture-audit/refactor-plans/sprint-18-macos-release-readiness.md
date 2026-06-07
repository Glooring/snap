# Sprint 18 - macOS Release Readiness

## Summary

Add macOS as a first-class supported release platform for Snap without weakening the existing Windows/Linux release flow.

This sprint must keep source behavior validation tied to source-built Snap, preserve the globally installed `/usr/local/bin/snap` as a checkpoint-only binary, and publish a new GitHub release only after macOS CI, macOS release asset creation, and public asset smoke tests pass.

## Scope

- Add macOS to CI.
- Add macOS release assets for Apple Silicon and Intel where GitHub-hosted runners support them.
- Add a macOS release script for portable `.tar.gz` assets.
- Extend release helper commands and help text with `snap release macos`.
- Extend the GitHub release workflow and release-smoke workflow to include macOS assets.
- Update install, cross-platform, release, changelog, and README docs.
- Keep the generated README demo GIF/MP4 in `docs/assets/` and linked from README.

## Release Assets

Expected new assets:

```text
snap-vX.Y.Z-macos-aarch64
snap-vX.Y.Z-macos-aarch64.tar.gz
snap-vX.Y.Z-macos-x86_64
snap-vX.Y.Z-macos-x86_64.tar.gz
```

Existing Windows and Linux assets must remain unchanged:

```text
snap-vX.Y.Z-windows-x86_64.exe
snap-vX.Y.Z-windows-x86_64-setup.exe
snap-vX.Y.Z-windows-x86_64.msi
snap-vX.Y.Z-linux-x86_64
snap-vX.Y.Z-linux-x86_64.tar.gz
SHA256SUMS.txt
```

## Validation Contract

Local validation:

```bash
git diff --check
cargo fmt --check
cargo clippy --all-targets --all-features
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo build --release
./target/release/snap --help
./target/release/snap release --help
./target/release/snap doctor
```

GitHub validation:

- CI passes on Ubuntu, Windows, and macOS.
- Release workflow publishes Windows, Linux, and macOS assets.
- Release Smoke downloads public assets, verifies `SHA256SUMS.txt`, and smoke-tests Linux, Windows, and macOS portable binaries in disposable Git repositories.

## Acceptance Criteria

- macOS compatibility is tested in CI.
- The public GitHub release includes macOS assets and checksums.
- macOS install docs are present and honest about unsigned/not notarized binaries.
- README shows the realistic generated demo and links to MP4/transcript.
- Existing Windows/Linux release paths still work.
- A non-release-looking local checkpoint is created after the sprint.
