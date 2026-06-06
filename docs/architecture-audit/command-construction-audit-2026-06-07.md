# Snap Command Construction Audit - 2026-06-07

Status: Sprint 4 baseline audit  
Scope: local command execution in `src`

## Summary

Sprint 4 reduced the highest-value command-construction risks by replacing formatted Git command strings with explicit argv calls for snapshot tags, metadata refs, blob reads, commits, deletes, and restore reset.

After the Sprint 4 cleanup, the remaining `Command::new(...)` matches are either central command-runner boundaries, fixed tool invocations, or intentional runtime overrides.

## Hardened In Sprint 4

Converted from formatted command strings to explicit argv APIs:

- `src/utils.rs`
  - `git update-ref <metadata-ref> <hash>`
  - `git cat-file blob <hash>`
- `src/commands/new.rs`
  - `git commit --allow-empty -m <message>`
  - `git tag -a <tag> -F -`
- `src/commands/update.rs`
  - `git tag -a -f <tag> -F -`
- `src/commands/restore.rs`
  - `git reset --hard <snapshot-commit>`
- `src/commands/edit.rs`
  - `git tag -a -f <new-tag> -F - <commit>`
  - `git tag -d <old-tag>`
- `src/git_health.rs`
  - repair tag rewrites with preserved committer date

This removes the previous `run_command(&format!(...))` findings for user-facing snapshot labels, commit IDs, metadata hashes, and tag rewrite paths.

## Remaining Command Boundaries

| File | Boundary | Current assessment |
| --- | --- | --- |
| `src/utils.rs` | `run_command_args_with_env(command, args, input, env)` uses `Command::new(command)` | Central helper. Callers should prefer explicit args. `run_command` still parses fixed string commands through `shlex`; avoid new interpolated strings. |
| `src/github.rs` | `Command::new(&command)` where command comes from `SNAP_GH` or defaults to `gh` | Intentional test/auth override. Arguments are passed as argv. Keep `SNAP_GH` documented as a trusted local override. |
| `src/git_health.rs` | `Command::new("git")` in `run_git` | Fixed executable with argv args. Low risk and preferred pattern. |
| `src/commands/release.rs` | `Command::new(&script.runtime)` where runtime comes from release config/env | Intentional runtime override for PowerShell/Bash in packaging workflows. Arguments are passed separately. Treat env overrides as trusted local developer inputs. |
| `src/commands/release.rs` | `Command::new("cargo")` for package metadata/version lookup | Fixed executable with argv args. Low risk. |

## Remaining Fixed String Calls

These still use `run_command("git ...")` with constant strings and no user interpolation:

- `git status --porcelain`
- `git rev-parse HEAD`
- `git hash-object -w --stdin`
- `git add -A`
- `git commit --amend --no-edit --allow-empty`
- `git init`
- `git reset --hard HEAD`
- `git clean -fd`

These are acceptable for now because they do not mix user input into command strings. Future cleanup can convert them to `run_command_args` opportunistically for consistency.

## Follow-Up Rules

- Do not add new `run_command(&format!(...))` call sites.
- Prefer `run_command_args` or `run_command_args_with_env` for Git commands.
- Keep user-controlled labels, refs, paths, and commit IDs as argv elements, not interpolated command strings.
- Add tests when changing restore, delete, purge, doctor repair, metadata refs, remotes, release helpers, or path handling.
