# Security Policy

Snap is a local CLI that can modify Git repositories and project files. Please report security issues privately before opening public issues.

## Reporting A Vulnerability

Email the maintainer at `zintax13223@gmail.com` with:

- a short description of the issue;
- affected command(s);
- operating system and Git version;
- whether the issue requires malicious input, a crafted repository, or normal use;
- reproduction steps in a disposable repository if possible.

Please do not include secrets or private repository contents in the report.

## Scope

Security-sensitive areas include:

- command execution and argument handling;
- restore/delete/purge behavior;
- `snap doctor --repair`;
- snapshot metadata parsing and refs;
- GitHub remote/visibility/delete helpers;
- release upload helpers;
- path handling across Windows, Linux, and WSL2.

## Current Status

This repository does not yet have automated public security advisories or CI-backed security scanning. Those are planned as part of OSS-readiness work. `cargo audit` is not currently installed in the local baseline.
