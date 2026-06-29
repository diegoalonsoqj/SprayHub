<div align="center">

# SprayHub

**Professional, cross-platform suite for managing Source Engine sprays.**

[![CI](https://github.com/diegoalonsoqj/SprayHub/actions/workflows/ci.yml/badge.svg)](https://github.com/diegoalonsoqj/SprayHub/actions/workflows/ci.yml)
[![License: Community](https://img.shields.io/badge/license-Community%20v1.0-blue.svg)](./LICENSE.md)
[![Tauri](https://img.shields.io/badge/Tauri-v2-24C8DB?logo=tauri)](https://tauri.app)
[![React](https://img.shields.io/badge/React-19-61DAFB?logo=react)](https://react.dev)

</div>

---

SprayHub lets you manage a library of Source Engine sprays (`.vtf` + `.vmt`),
preview thumbnails, and apply a selected spray to your favorite game with a
single click. It auto-detects your Steam installation and supported games.

## ✨ Features

- 🎮 **Automatic Steam detection** — reads `libraryfolders.vdf` across all Steam libraries.
- 🗂️ **Multi-game support** — Left 4 Dead 2, Counter-Strike: Source, Team Fortress 2, Half-Life 2: Deathmatch, Garry's Mod, and an extensible architecture for more.
- 🖼️ **Thumbnail grid** — browse your spray library visually with instant search.
- 💾 **Safe apply** — atomic file operations, optional backups, and overwrite confirmation.
- 🌙 **Dark theme by default** — responsive, keyboard-friendly UI.
- 🔒 **Security first** — path validation, no path traversal, no admin privileges, no system commands.
- ⚡ **Async & non-blocking** — async scanning and lazy thumbnail loading.

> **Roadmap:** PNG → VTF conversion (VTFLib), favorites, history, per-game profiles, drag & drop, auto-update, and ES/EN internationalization. See [docs/roadmap](./docs/roadmap/README.md).

## 🧱 Tech Stack

| Layer       | Technology                                  |
| ----------- | ------------------------------------------- |
| Backend     | Rust + Tauri v2                             |
| Frontend    | React 19 + TypeScript + Vite                |
| Styling     | TailwindCSS + shadcn/ui                     |
| Architecture| Clean Architecture (frontend & backend)     |

## 🏛️ Architecture

Both the Rust backend and the React frontend follow **Clean Architecture**, with
strict dependency direction pointing inward (Presentation → Application → Domain).

```
Presentation  →  Application  →  Domain  ←  Infrastructure
 (UI / Tauri      (use cases)     (entities,    (Steam, FS,
  commands)                        contracts)    config, logs)
```

See [docs/architecture](./docs/architecture/README.md) for the full breakdown.

## 🚀 Getting Started

### Prerequisites

- [Node.js](https://nodejs.org/) ≥ 20
- [Rust](https://rustup.rs/) (stable toolchain)
- Tauri v2 [system dependencies](https://tauri.app/start/prerequisites/) for your OS

### Install & run

```bash
# Install JS dependencies
npm install

# Run the desktop app in development mode
npm run tauri:dev
```

### Build a release binary

```bash
npm run tauri:build
```

### Useful scripts

| Script                 | Description                          |
| ---------------------- | ------------------------------------ |
| `npm run dev`          | Vite dev server (frontend only)      |
| `npm run tauri:dev`    | Full Tauri app in dev mode           |
| `npm run tauri:build`  | Production build                     |
| `npm run lint`         | ESLint                               |
| `npm run typecheck`    | TypeScript type checking             |
| `npm run test`         | Frontend unit tests (Vitest)         |
| `npm run format`       | Prettier                             |

Backend: `cargo fmt`, `cargo clippy`, and `cargo test` inside `src-tauri/`.

## 🗺️ Supported Games

| Game                       | Steam App ID | Status      |
| -------------------------- | ------------ | ----------- |
| Left 4 Dead 2              | 550          | ✅ Supported |
| Counter-Strike: Source     | 240          | ✅ Supported |
| Team Fortress 2            | 440          | ✅ Supported |
| Half-Life 2: Deathmatch    | 320          | ✅ Supported |
| Garry's Mod                | 4000         | ✅ Supported |

The catalog is data-driven and extensible — see `src-tauri/src/infrastructure/steam/game_catalog.rs`.

## 🤝 Contributing

Contributions are welcome! Please read [CONTRIBUTING.md](./CONTRIBUTING.md) and
our [CODE_OF_CONDUCT.md](./CODE_OF_CONDUCT.md) first.

## 🔐 Security

Found a vulnerability? Please follow our [security policy](./SECURITY.md).

## 📄 License

SprayHub is released under the **SprayHub Community License v1.0** — free for
personal, educational, research, and non-commercial use. See [LICENSE.md](./LICENSE.md).

© 2026 Diego Quispe.
