# Codex Workflow

Snap is useful around Codex and other code agents because agent edits can be broad, fast, and hard to mentally reverse. Snap gives the work a local checkpoint boundary before the task starts and a review boundary after it finishes.

## Before Starting

Run a health check and create a task-specific checkpoint:

```bash
snap doctor
snap new before-codex-auth-refactor "before Codex auth refactor"
```

Prefer labels that describe the task. Avoid release-looking labels such as `v1.2.3`; Snap currently stores snapshots as Git tags, and new source-built Snap snapshots mark their tag messages with `Snap-Snapshot: true`.

## During The Task

Let Codex or another agent make the requested changes. Keep the prompt focused, and ask for tests or validation commands when the change touches behavior.

Useful read-only checks while work is in progress:

```bash
snap status
snap list
snap doctor --json --ci
```

`snap doctor --json --ci` is the automation-friendly health check. It exits non-zero when warnings or errors are found.

## After The Task

Create an after checkpoint, inspect the difference, and run the project gates:

```bash
snap new after-codex-auth-refactor "after Codex auth refactor"
snap diff before-codex-auth-refactor after-codex-auth-refactor
snap doctor
```

For CI-like local checks, use the project-specific commands from `AGENTS.md` or `CONTRIBUTING.md`.

## If The Result Is Wrong

Preview the restore first:

```bash
snap restore before-codex-auth-refactor --dry-run
```

Then restore when you are ready:

```bash
snap restore before-codex-auth-refactor
```

Normal restore creates a `snap-rescue-YYYYMMDD-HHMMSS` snapshot first when the current state is dirty or otherwise unprotected, so the failed attempt remains recoverable. Use `--no-rescue` only when you intentionally want the older discard-confirmation path.

## Non-Interactive Boundary

Good automation targets today:

- `snap status`;
- `snap list`;
- `snap diff <before> <after>`;
- `snap doctor --json --ci`;
- `snap restore <label> --dry-run`.

Destructive or mutating commands still require care and may prompt. Do not add or rely on casual prompt bypasses such as a broad `--yes` mode. A future non-interactive design should specify exact safeguards, confirmation tokens, test coverage, and recovery behavior before changing destructive commands.
