# Sprint 10 - Community Feedback

Status: planned
Sprint checkpoint: `oss-s10-community`
Date: 2026-06-07
Scope: community launch readiness, issue candidates, repo metadata recommendations, and feedback materials

## Purpose

Sprint 10 prepares Snap for real community feedback without artificial hype.

The goal is to make it easy for a maintainer to set public repo metadata, create useful beginner/safety issues, and ask for feedback in a way that invites real use and critique rather than vanity metrics.

## Scope

In scope:

- create a community feedback/readiness doc;
- record recommended GitHub About description and topics;
- create good-first issue candidates with acceptance criteria;
- create safety/hardening issue candidates with acceptance criteria;
- prepare short feedback post drafts for appropriate communities;
- document which real feedback signals are still missing;
- attempt actual GitHub metadata/topic and issue creation only if authenticated tooling is available and clearly targets `Glooring/snap`;
- update audit and progress docs.

Out of scope:

- asking for artificial stars, follows, upvotes, or engagement;
- creating disposable GitHub sandbox repositories;
- changing runtime behavior, snapshot storage, release packaging, or CLI output;
- creating a full OpenAI application package;
- making unsupported adoption, performance, or ecosystem-importance claims.

## External-Action Rule

This sprint may touch the real `Glooring/snap` repository metadata or issues only when all are true:

- the command targets `Glooring/snap` exactly;
- authentication is already available;
- the action is scoped to metadata/topics or issue creation described in this plan;
- the progress log records what was changed.

If authenticated GitHub tooling is unavailable, do not install tooling. Document exact maintainer commands and mark the external step as not run.

## Implementation Checklist

- Create `doc/COMMUNITY_FEEDBACK.md`.
- Include a recommended GitHub About description.
- Include recommended topics:
  - `rust`
  - `cli`
  - `git`
  - `snapshot`
  - `developer-tools`
  - `version-control`
  - `ai-assisted-coding`
  - `codex`
  - `wsl2`
  - `windows`
  - `linux`
- Add good-first issue candidates.
- Add safety/hardening issue candidates.
- Add feedback post drafts that ask for usage reports, bug reports, and critique.
- Update README public-doc links if useful.
- Check whether `gh` can view/edit `Glooring/snap`.
- If safe and authenticated, apply metadata/topics and create the chosen issues.
- Record any actual external GitHub changes or skipped external actions.

## Validation Plan

Run:

```bash
git diff --check
cargo fmt --check
cargo test
cargo build --release
./target/release/snap --help
./target/release/snap doctor
```

Run targeted scans:

```bash
rg -n "star|upvote|viral|growth hack|artificial|good first|safety|hardening|GitHub About|topics|feedback" README.md doc docs/architecture-audit .github
```

If actual GitHub actions are performed, verify with read-only `gh repo view` and/or issue listing.

## Acceptance Criteria

- Sprint 10 plan exists and is linked from the refactor-plan index.
- Community feedback/readiness doc exists.
- Repo About/topic recommendations are explicit.
- Good-first and safety/hardening issue candidates are ready or created.
- Feedback post drafts avoid artificial engagement asks.
- Actual GitHub changes are either completed and recorded or explicitly documented as not run.
- Validation gates pass with source-built Snap.
- Final checkpoint is created with:

```bash
snap new oss-s10-community "sprint 10: community feedback readiness"
snap list
git status --short
```
