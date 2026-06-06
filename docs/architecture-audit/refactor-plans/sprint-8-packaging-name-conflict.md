# Sprint 8 - Packaging And Name Conflict

Status: planned
Sprint checkpoint: `oss-s8-packaging`
Date: 2026-06-07
Scope: packaging metadata and public installation/release documentation

## Purpose

Sprint 8 makes Snap's install and release story credible for public OSS users without changing runtime behavior.

The sprint should answer the practical questions a new user or maintainer will have:

- which release asset do I download;
- where should the binary live on Windows, Linux, and WSL2;
- how do I verify the binary I am about to run;
- how do checksums fit into releases;
- what should I do if `snap` already means Canonical Snapcraft on Linux;
- what packaging metadata is still placeholder-grade.

## Scope

In scope:

- add or refresh public install/release docs;
- document Windows install paths, Linux install paths, and WSL2-specific behavior;
- document release asset names and the checksum/provenance plan;
- document source-built verification commands;
- make the Canonical Snapcraft `snap` command conflict explicit;
- clean the placeholder `Packager.toml` identifier if a conservative OSS value is clear;
- update README, known limitations, audit, and progress docs;
- run source-built validation gates.

Out of scope:

- renaming the project or changing the Rust binary name;
- changing CLI behavior, command parsing, storage schema, snapshot refs, or public API;
- changing release scripts beyond documentation unless a packaging metadata fix requires it;
- installing this repo's build over `/usr/local/bin/snap`;
- replacing, reinstalling, overwriting, or upgrading the globally installed checkpoint Snap binary;
- using global `snap --help`, global `snap doctor`, or global `snap --version` as source behavior evidence;
- creating GitHub sandbox repositories, because this sprint does not touch remote or visibility behavior.

## Decisions

1. Keep the project name `Snap` and the built binary name `snap` for Sprint 8.
2. Do not introduce an alternate binary name such as `gitsnap` in code during this sprint.
3. Public docs may mention `gitsnap` as a local conflict-safe filename for the current binary, but not as an official project or binary rename.
4. Linux docs must tell users to check `command -v snap` and avoid overwriting Canonical Snapcraft accidentally.
5. Linux/WSL install examples must show a conflict-safe option before any `/usr/local/bin/snap` strategy.
6. Checksums are a release requirement: release notes should include SHA-256 checksums for every uploaded artifact.
7. Sprint 8 validation uses source-built Snap only; global Snap remains checkpoint-only.

## Baseline Findings To Address

- `Packager.toml` uses placeholder identifier `com.yourname.snap`.
- README says release packaging is still being polished and only briefly warns about the Linux name conflict.
- `doc/BUILD_INSTALLERS_WINDOWS_WSL.md` includes install examples that copy a source-built binary to `/usr/local/bin/snap` without first addressing Canonical Snapcraft.
- Release docs list artifact names but do not describe checksum expectations clearly enough.
- `doc/KNOWN_LIMITATIONS.md` still says the packaging/name-conflict strategy is planned.
- The audit still lists Sprint 8 packaging/name-conflict work as open.

## Implementation Checklist

- Create or update a central install/release doc.
- Update README Installation and Public Docs sections to point to the central install/release doc.
- Update `doc/BUILD_INSTALLERS_WINDOWS_WSL.md` so it is compatible with the public install policy.
- Update `doc/KNOWN_LIMITATIONS.md` to reflect the documented Linux conflict strategy.
- Replace the `Packager.toml` placeholder identifier with a conservative OSS identifier.
- Update `docs/architecture-audit/refactor-progress.md`.
- Update `docs/architecture-audit/agent-efficiency-oss-readiness-audit-2026-06-06.md`.

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

Run targeted scans:

```bash
rg -n "com\\.yourname|your-username|TODO|FIXME|FINAL|START: THE FIX|END: THE FIX|CORRECTED LINE" README.md doc docs/architecture-audit Packager.toml -g '!docs/architecture-audit/reference-inputs/**'
rg -n "Canonical Snapcraft|command -v snap|/usr/local/bin|gitsnap|checksums|SHA-256|release-github|Program Files|WSL" README.md doc docs/architecture-audit Packager.toml -g '!docs/architecture-audit/reference-inputs/**'
```

Do not run install commands that mutate global PATH locations. Do not run `sudo cp target/release/snap /usr/local/bin/snap`.

## Acceptance Criteria

- Sprint 8 plan exists and is linked from the refactor-plan index.
- Public docs explain Windows, Linux, and WSL2 installation paths.
- Public docs explain release artifact names and checksum expectations.
- Public docs explicitly warn about Canonical Snapcraft and provide a conflict-safe Linux/WSL option.
- `Packager.toml` no longer contains `com.yourname.snap`.
- Audit and progress docs record Sprint 8 results and next recommendation.
- Validation gates pass with source-built Snap.
- No GitHub sandbox repositories are created.
- Final checkpoint is created with:

```bash
snap new oss-s8-packaging "sprint 8: packaging and name conflict docs"
snap list
git status --short
```
