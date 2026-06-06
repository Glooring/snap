# Contributing

Thanks for helping improve Snap. This project is a Rust CLI over Git, so correctness and safety matter more than broad refactors.

## Local Setup

```bash
git clone https://github.com/Glooring/snap.git
cd snap
cargo build
```

Use source-built Snap when testing this repository:

```bash
cargo run -- --help
./target/release/snap doctor
```

Do not use a globally installed `snap` binary to validate source behavior.

## Before Opening A Pull Request

Run the relevant checks:

```bash
git diff --check
cargo fmt --check
cargo clippy --all-targets --all-features
cargo test
cargo build --release
./target/release/snap doctor
```

If your change is docs-only, `git diff --check`, `cargo fmt --check`, and targeted docs scans may be enough, but say exactly what you ran.

## Safety-Sensitive Changes

Add or update tests when touching:

- `restore`, `delete`, `edit`, or `update`;
- purge and garbage-collection behavior;
- `doctor` or Git health repair;
- snapshot metadata and refs;
- command execution;
- branch, remote, GitHub, or release helpers;
- platform-specific path, hidden, or read-only behavior.

Use disposable local repositories for behavior tests. Never test destructive remote behavior against an existing user repository.

## Pull Request Expectations

- Keep the scope focused.
- Explain user-visible behavior changes.
- Mention safety implications.
- Include command output summaries for the checks you ran.
- Avoid overstated claims in docs and release notes.
