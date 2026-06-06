# Why Not Just Git?

Git is powerful enough to do everything Snap does. Snap exists because many developers want a smaller workflow for a specific job: local, named project checkpoints before risky work.

## Use Git Directly When

- you need detailed branching and merge control;
- you are collaborating through pull requests;
- you are bisecting or rewriting history;
- you want full control over tags, refs, remotes, and release metadata.

## Use Snap When

- you want a quick checkpoint before a risky refactor;
- an AI agent is about to edit many files;
- a beginner would otherwise copy the whole project folder;
- you want `snap list`, `snap diff`, and `snap restore` around named restore points;
- you want `snap doctor` to inspect Git and Snap metadata health.

## The Design Line

Snap is not a Git replacement.

Snap uses:

- normal Git repositories;
- Git commits for file state;
- annotated Git tags for snapshot labels;
- Git blobs and `refs/snap-metadata/*` for Snap-specific metadata.

The point is not to hide Git forever. The point is to make one repeated workflow easier and safer.
