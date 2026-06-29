# SprayHub Architecture

SprayHub is a Tauri v2 desktop application. It is split into a **Rust backend**
(`src-tauri/`) and a **React frontend** (`src/`). Both halves follow
**Clean Architecture**, keeping the dependency rule: source code dependencies
only point *inward*, toward higher-level policy.

```
            ┌─────────────────────────────────────────┐
            │              Presentation                │
            │   React UI  ·  Tauri commands (Rust)     │
            └───────────────────┬─────────────────────┘
                                │ depends on
            ┌───────────────────▼─────────────────────┐
            │              Application                 │
            │            Use cases / services          │
            └───────────────────┬─────────────────────┘
                                │ depends on
            ┌───────────────────▼─────────────────────┐
            │                 Domain                   │
            │   Entities · Value objects · Contracts   │
            └───────────────────▲─────────────────────┘
                                │ implements contracts
            ┌───────────────────┴─────────────────────┐
            │             Infrastructure               │
            │  Steam · Filesystem · Config · Logging   │
            └─────────────────────────────────────────┘
```

## Layers

### Domain

Pure business model with **no external dependencies**.

- **Backend** (`src-tauri/src/domain/`): `Spray`, `Game`, `SteamLibrary`,
  `AppConfig` entities; repository/service **traits**; domain errors.
- **Frontend** (`src/domain/`): the same concepts as TypeScript types plus
  repository **interfaces** the UI depends on.

### Application

Use cases that orchestrate the domain to fulfill a user intent. They depend only
on domain contracts, never on concrete infrastructure.

Examples: `ScanSprayLibrary`, `DetectSteamGames`, `ApplySprayToGame`,
`LoadConfig`, `SaveConfig`.

### Infrastructure

Concrete implementations of domain contracts.

- **Steam**: parse `libraryfolders.vdf`, resolve install dirs, game catalog.
- **Filesystem**: scan sprays, read thumbnails, atomic copy + backup.
- **Config**: JSON persistence in AppData / `~/.config`.
- **Logging**: rotating file logger.

### Presentation

- **Backend**: thin Tauri `#[command]` handlers that call use cases and map
  results to serializable DTOs.
- **Frontend**: React components, hooks, and state. The frontend's
  infrastructure layer talks to the backend through the Tauri `invoke` bridge.

## Dependency Rule

A higher layer may depend on a lower one, **never the reverse**. Infrastructure
depends on Domain by *implementing* its traits/interfaces, enabling dependency
inversion and testability (use cases are tested against in-memory fakes).

## Data Flow: "Apply a spray"

1. **UI** (presentation) dispatches `applySpray(sprayId, gameId)`.
2. Frontend **infrastructure** calls Tauri `invoke("apply_spray", …)`.
3. Backend **command** (presentation) deserializes input and calls the
   `ApplySprayToGame` **use case**.
4. The use case validates paths via **domain** rules and delegates the atomic
   copy + backup to the filesystem **infrastructure**.
5. The result bubbles back up as a typed DTO and the UI updates.

See [decisions](../decisions/) for the Architecture Decision Records (ADRs).
