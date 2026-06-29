# Contributing to SprayHub

Thank you for your interest in contributing! This document describes the
workflow, standards, and expectations for contributing to SprayHub.

## Code of Conduct

By participating, you agree to uphold our [Code of Conduct](./CODE_OF_CONDUCT.md).

## Branch Strategy

We use a Git-flow-inspired model. **Never develop directly on `main`.**

| Branch       | Purpose                                  |
| ------------ | ---------------------------------------- |
| `main`       | Stable, released code                    |
| `develop`    | Integration branch for the next release  |
| `feature/*`  | New features                             |
| `fix/*`      | Bug fixes                                |
| `hotfix/*`   | Urgent fixes against `main`              |
| `release/*`  | Release preparation                      |

Create your branch off `develop`:

```bash
git checkout develop
git pull
git checkout -b feature/my-awesome-feature
```

## Conventional Commits

All commits must follow [Conventional Commits](https://www.conventionalcommits.org/):

```
feat: add Steam auto detection
fix: validate destination folder
perf: optimize thumbnail loading
docs: update architecture
refactor: extract spray repository
test: add steam parser tests
build: bump tauri to 2.2
ci: add windows build job
chore: update dependencies
```

Allowed types: `feat`, `fix`, `refactor`, `perf`, `docs`, `style`, `test`,
`build`, `ci`, `chore`, `revert`.

## Coding Standards

### Backend (Rust)

- Format with `cargo fmt`.
- Lint with `cargo clippy -- -D warnings`.
- No `unsafe` unless justified with a comment explaining the invariant.
- Keep Clean Architecture boundaries: `domain` depends on nothing; `application`
  depends only on `domain`; `infrastructure` and `presentation` depend inward.

### Frontend (TypeScript / React)

- TypeScript `strict` mode — no `any` without justification.
- Lint with ESLint; format with Prettier.
- **Functional components only** — no class components.
- Keep business logic in `application`/`domain`; React lives in `presentation`.

## Pull Requests

Every PR must include:

- **Summary** — what changed.
- **Motivation** — why.
- **Screenshots** — if there are UI changes.
- **Test evidence** — output of tests / manual verification.
- **Checklist** — see the PR template.

CI must pass (build Windows + Linux, tests, lint, format) before review.

## Running Locally

```bash
npm install
npm run tauri:dev
```

See the [README](./README.md) for the full list of scripts.

## Reporting Bugs / Requesting Features

Use the issue templates under **New Issue**. Provide as much detail as possible.
