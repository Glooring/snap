# Beginner Workflow

Snap can help beginners stop copying entire project folders to make backups. It gives a small set of commands for local checkpoints while still using Git underneath.

## First Project

From the project folder:

```bash
snap init
snap new first-working-version "project runs"
snap list
```

After more changes:

```bash
snap new added-login-page "login page works"
snap list
```

To go back:

```bash
snap restore first-working-version
```

## What To Remember

- `snap init` prepares the folder once.
- `snap new <label> "description"` saves a checkpoint.
- `snap list` shows available checkpoints.
- `snap diff old-label new-label` shows what changed.
- `snap restore <label>` returns the project files to that checkpoint.
- `snap doctor` checks whether the Git/Snap metadata looks healthy.

## How This Relates To Git

Snap uses Git. It creates commits and annotated tags for snapshots. You can keep learning Git over time, and Snap will still leave your project in a normal Git repository.

## Safety Notes

`snap restore` can discard uncommitted local changes after asking for confirmation. If you are unsure, create a new checkpoint first:

```bash
snap new before-restore "before trying restore"
```
