# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Initial project scaffold: Tauri v2 + React 19 + TypeScript + Vite + TailwindCSS + shadcn/ui.
- Clean Architecture layers for both backend (Rust) and frontend (React).
- Automatic Steam detection via `libraryfolders.vdf` across multiple libraries.
- Extensible Source-game catalog (L4D2, CS:S, TF2, HL2:DM, Garry's Mod).
- Spray library scanning (`.vtf` + `.vmt`) with thumbnail support.
- Apply spray flow with atomic operations, optional backup and overwrite confirmation.
- JSON configuration persistence in platform-appropriate directories.
- Rotating file logger.
- Dark theme by default and responsive UI.
- CI/CD workflows, issue/PR templates, and full repository documentation.

## [0.1.0] - 2026-06-29

- Initial prototype.

[Unreleased]: https://github.com/diegoalonsoqj/SprayHub/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/diegoalonsoqj/SprayHub/releases/tag/v0.1.0
