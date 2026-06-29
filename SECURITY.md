# Security Policy

## Supported Versions

SprayHub is in active early development. Security fixes are applied to the
latest release and the `develop` branch.

| Version | Supported          |
| ------- | ------------------ |
| 0.x     | :white_check_mark: |

## Reporting a Vulnerability

**Please do not open public issues for security vulnerabilities.**

Instead, report them privately:

- Use GitHub's [private vulnerability reporting](https://docs.github.com/en/code-security/security-advisories/guidance-on-reporting-and-writing-information-about-vulnerabilities/privately-reporting-a-security-vulnerability) on this repository, **or**
- Contact the maintainer (Diego Quispe) directly through the repository profile.

Please include:

- A description of the vulnerability and its impact.
- Steps to reproduce (proof of concept if possible).
- Affected version(s) and platform(s).

We aim to acknowledge reports within **72 hours** and to provide a remediation
timeline after triage.

## Security Design Principles

SprayHub is built defensively:

- **Path validation** — all user-supplied and game paths are validated and
  canonicalized before any filesystem operation.
- **No path traversal** — destination paths are confined to detected game
  directories; `..` segments are rejected.
- **Atomic operations** — applying a spray writes to a temp file and renames,
  preventing partial writes.
- **No system command execution** — the app never shells out.
- **No elevated privileges** — SprayHub never requires administrator/root rights.
- **Least capability** — Tauri capabilities expose only the commands the UI needs.
