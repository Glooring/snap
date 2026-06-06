# Sprint 9 - Cross-Platform And Performance Proof

Status: planned
Sprint checkpoint: `oss-s9-platform`
Date: 2026-06-07
Scope: performance methodology, cross-platform evidence, benchmark scripts, and focused edge-case tests

## Purpose

Sprint 9 replaces broad performance and cross-platform claims with evidence a maintainer can run and inspect.

Snap can still say it is useful for fast local checkpoints, but public docs should avoid unsupported claims such as being faster than Git or "blazing fast" unless a current benchmark proves the specific behavior. This sprint should make the claim surface calmer and more testable.

## Scope

In scope:

- add `doc/PERFORMANCE.md` with benchmark methodology, representative scenarios, and interpretation rules;
- add `doc/CROSS_PLATFORM.md` with what is verified on Linux, Windows, and WSL2;
- add benchmark helper scripts for Linux/macOS/WSL and Windows PowerShell;
- update README public docs and limitation wording;
- remove remaining unmeasured "blazing fast" CLI wording from source help;
- retire stale tracked prompt dumps if they still carry unsupported performance claims or placeholder public links;
- add focused tests for paths with spaces, Unicode paths, nested empty directories, and hidden/read-only metadata where supported;
- update progress and audit docs with exact results.

Out of scope:

- claiming published benchmark numbers from this local machine as universal performance data;
- adding a benchmark dependency or requiring external tools such as `hyperfine`;
- changing snapshot storage, restore behavior, or metadata format beyond tests;
- changing release packaging, GitHub remote behavior, or global Snap installation;
- creating GitHub sandbox repositories.

## Decisions

1. Performance docs will define methodology and local commands, not universal speed claims.
2. Benchmark scripts will be optional maintainer tools and will write outputs to ignored/local paths.
3. Cross-platform docs will distinguish current proof from intent:
   - Linux: validated locally in this sprint.
   - Windows: covered by CI and platform-specific code/tests, but not locally executed from this Linux workspace.
   - WSL2: documented and structurally similar to Linux, but separate validation should be recorded when run in WSL2.
4. Source-built Snap remains the only product validation binary during the sprint.
5. The global `snap` binary remains checkpoint-only.

## Implementation Checklist

- Create `doc/PERFORMANCE.md`.
- Create `doc/CROSS_PLATFORM.md`.
- Create `scripts/benchmark.sh`.
- Create `scripts/benchmark.ps1`.
- Update README public-doc links.
- Replace the remaining source help "blazing fast" wording with conservative language.
- Remove obsolete tracked prompt dumps if they are only stale implementation prompts.
- Add tests for:
  - paths with spaces;
  - Unicode paths;
  - nested empty directories through snapshot/restore;
  - hidden metadata on Unix through dotfiles and Windows through hidden attributes when CI runs on Windows;
  - read-only metadata restore where the platform supports it.
- Record unsupported/local-only areas honestly.

## Validation Plan

Run:

```bash
git diff --check
cargo fmt --check
cargo clippy --all-targets --all-features
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo build --release
./target/release/snap --help
./target/release/snap doctor
```

Run benchmark-script smoke checks that do not overwrite global Snap:

```bash
bash scripts/benchmark.sh --help
pwsh -File scripts/benchmark.ps1 -Help
```

If PowerShell is unavailable locally, record that instead of installing it.

Run targeted scans:

```bash
rg -n "blazing fast|blazing-fast|faster than Git|instantaneous|well under a second|benchmark|cross-platform|spaces|Unicode|readonly|hidden" README.md doc Cargo.toml src tests scripts docs/architecture-audit -g '!docs/architecture-audit/reference-inputs/**'
```

## Acceptance Criteria

- Sprint 9 plan exists and is linked from the refactor-plan index.
- Public docs include performance methodology and cross-platform support evidence.
- Benchmark scripts exist for shell and PowerShell and are documented as local tools.
- Source help no longer uses unmeasured "blazing fast" language.
- Obsolete prompt dumps no longer expose placeholder links or unsupported performance claims.
- Tests cover path spaces, Unicode paths, nested empty directories, and metadata attribute behavior where supported.
- Validation gates pass with source-built Snap.
- No global Snap install/update occurs.
- No GitHub sandbox repositories are created.
- Final checkpoint is created with:

```bash
snap new oss-s9-platform "sprint 9: cross-platform and performance proof"
snap list
git status --short
```
