# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2026-06-29

First prototype release.

### Added

- Tauri v2 + React 19 + TypeScript + Vite + TailwindCSS + shadcn/ui application.
- Clean Architecture layers for both backend (Rust) and frontend (React).
- Automatic Steam detection via `libraryfolders.vdf` across multiple libraries
  (Windows registry + well-known paths).
- Extensible Source-game catalog with correct per-game `gamedir` sprays path
  (L4D2, CS:S, TF2, HL2:DM, Garry's Mod).
- Spray library scanning (`.vtf` + `.vmt`).
- Full-resolution VTF thumbnails rendered as transparent PNGs (DXT1/DXT3/DXT5
  and uncompressed RGBA/BGRA/ARGB/ABGR/RGB/BGR), decoded without image crates.
- Apply spray flow with atomic operations, optional backup and overwrite confirmation.
- Folder picker, instant search, favorites, dark-themed responsive UI, ES/EN i18n.
- JSON configuration persistence in platform-appropriate directories.
- Rotating file logger.
- Path-safety rules (no traversal) with unit tests; 21 backend tests total.
- CI/CD workflows, issue/PR templates, and full repository documentation.
- Windows installers (NSIS + MSI) via `tauri build`.

[Unreleased]: https://github.com/diegoalonsoqj/SprayHub/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/diegoalonsoqj/SprayHub/releases/tag/v0.1.0
