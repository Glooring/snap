# Codex Task Recipes

These recipes show how to wrap common AI-assisted tasks with Snap checkpoints. Adjust labels to match the task, and keep labels non-release-looking.

## Large Refactor

```bash
snap doctor
snap new before-codex-parser-refactor "before parser refactor"
# Run Codex or another agent.
snap new after-codex-parser-refactor "after parser refactor"
snap diff before-codex-parser-refactor after-codex-parser-refactor
snap doctor --json --ci
```

Run the project's normal tests before keeping the result.

## Dependency Upgrade

```bash
snap new before-deps-upgrade "before dependency upgrade"
# Ask Codex to update dependency versions and fix compile errors.
snap status
snap new after-deps-upgrade "after dependency upgrade"
snap diff before-deps-upgrade after-deps-upgrade
```

Use `snap restore before-deps-upgrade --dry-run` before rolling back.

## Formatter Or Mechanical Sweep

```bash
snap new before-format-sweep "before formatter sweep"
# Run formatter, linter autofix, or generated edits.
snap new after-format-sweep "after formatter sweep"
snap diff before-format-sweep after-format-sweep
```

Mechanical changes can be large. The before/after diff helps separate expected churn from accidental edits.

## Generated Code Or Migration Tool

```bash
snap doctor
snap new before-generated-migration "before generated migration"
# Run the generator or migration.
snap status
snap new after-generated-migration "after generated migration"
snap doctor
```

If generated files are wrong, preview restore first:

```bash
snap restore before-generated-migration --dry-run
```

## Release Prep

```bash
snap new before-release-prep "before release prep"
# Update changelog, version references, installer notes, or release assets.
snap new after-release-prep "after release prep"
snap diff before-release-prep after-release-prep
snap doctor --json --ci
```

Avoid labels that look like actual release tags.

## Docs-Only Agent Edit

```bash
snap new before-docs-agent "before docs agent edit"
# Ask Codex to update docs.
snap new after-docs-agent "after docs agent edit"
snap diff before-docs-agent after-docs-agent
```

Run `git diff --check` and any targeted docs scans the project expects.

## Doctor Or CI Triage

```bash
snap doctor --json --ci
snap new before-ci-triage "before CI triage"
# Investigate and fix.
snap new after-ci-triage "after CI triage"
snap doctor --json --ci
```

If `doctor --json --ci` fails, inspect the JSON report before attempting repair. Use `snap doctor --repair` only after reading the repair plan.

## Keep Or Roll Back

Keep the result when tests, review, and doctor checks are clean. Roll back with:

```bash
snap restore <before-label> --dry-run
snap restore <before-label>
```

The restore command creates a rescue snapshot first when needed, so the rejected result remains recoverable unless you explicitly use `--no-rescue`.
