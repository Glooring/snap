# AI-Agent Workflow

Snap is useful when an AI agent, migration tool, formatter, or large refactor might touch many files quickly. The goal is simple: create a local checkpoint before the risky work, inspect the result, and keep a direct restore path.

For Codex-specific guidance, see:

- [`CODEX_WORKFLOW.md`](CODEX_WORKFLOW.md)
- [`CODEX_TASKS.md`](CODEX_TASKS.md)

## Recommended Flow

```bash
snap doctor
snap new before-agent-edit "before AI-assisted changes"

# Run Codex, your editor agent, a migration, or a manual refactor.

snap status
snap new after-agent-edit "after AI-assisted changes"
snap diff before-agent-edit after-agent-edit
snap doctor --json --ci
```

If the result is wrong:

```bash
snap restore before-agent-edit --dry-run
snap restore before-agent-edit
```

## Why This Helps

- AI agents can change many files faster than you can review them.
- A named checkpoint gives you a known restore point before the edit.
- `snap diff` summarizes what changed between checkpoints.
- `snap doctor --json --ci` gives automation-friendly Git and Snap metadata health checks.
- Normal restore creates a rescue snapshot first when needed.

## Good Labels

Prefer labels that describe the task:

```bash
snap new before-auth-refactor "before auth refactor"
snap new after-auth-refactor "after auth refactor"
snap new before-deps-upgrade "before dependency upgrade"
```

Avoid release-looking labels such as `v1.2.3` for routine checkpoints. Snap uses Git tags for snapshots, and source-built Snap marks new snapshot tags with `Snap-Snapshot: true`.

## Boundaries

Snap does not review code for you. You should still inspect diffs, run tests, and review security-sensitive changes. Snap is the safety checkpoint around the work, not a substitute for engineering judgment.

Read-only automation is encouraged for `snap status`, `snap list`, `snap diff`, `snap doctor --json --ci`, and `snap restore --dry-run`. Do not bypass destructive prompts casually; prompt-free destructive modes need a separate design with safeguards and tests.
