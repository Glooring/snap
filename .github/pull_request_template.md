# Summary

Describe the change and why it is needed.

## Scope

- [ ] Docs only
- [ ] Runtime behavior
- [ ] Safety-sensitive behavior
- [ ] GitHub/remote/release behavior

## Validation

List what you ran:

- [ ] `git diff --check`
- [ ] `cargo fmt --check`
- [ ] `cargo clippy --all-targets --all-features`
- [ ] `cargo test`
- [ ] `cargo build --release`
- [ ] `./target/release/snap doctor`

If you skipped a relevant check, explain why.

## Safety Notes

Call out changes to restore, delete, purge, doctor, Git health, metadata, command execution, path handling, remotes, or release helpers.

For remote/GitHub tests, use only disposable sandbox repositories. Never test destructive behavior against an existing user repository.
