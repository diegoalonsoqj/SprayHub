# SprayHub Backend API (Tauri Commands)

The frontend communicates with the Rust backend exclusively through Tauri
commands invoked via `@tauri-apps/api/core`'s `invoke`. All commands are
`async` and return a typed result; errors are returned as a serializable
`AppError` string payload.

## Commands

### `get_config() -> AppConfig`

Returns the persisted application configuration (creating defaults on first run).

### `save_config(config: AppConfig) -> AppConfig`

Validates and persists the configuration. Returns the saved config.

### `detect_steam() -> SteamDetection`

Detects the Steam installation(s) and enumerates all library folders found in
`libraryfolders.vdf`.

```ts
interface SteamDetection {
  steamRoot: string | null;
  libraries: string[];
}
```

### `list_games() -> GameInfo[]`

Returns the catalog of supported games annotated with installation state and the
resolved destination directory.

```ts
interface GameInfo {
  id: string;          // e.g. "left4dead2"
  name: string;        // e.g. "Left 4 Dead 2"
  appId: number;       // Steam App ID
  installed: boolean;
  installDir: string | null;
  spraysDir: string | null; // materials/vgui/logos
}
```

### `scan_sprays(libraryDir: string) -> Spray[]`

Asynchronously scans a directory for spray pairs (`.vtf` + optional `.vmt`).

```ts
interface Spray {
  id: string;
  name: string;
  vtfPath: string;
  vmtPath: string | null;
  sizeBytes: number;
  modifiedAt: number; // unix seconds
}
```

### `get_thumbnail(vtfPath: string) -> string`

Returns a data URL (PNG) thumbnail decoded from the VTF, for lazy loading.

### `apply_spray(request: ApplySprayRequest) -> ApplyResult`

Copies the selected spray into the game's `logos` directory atomically, with an
optional backup of any existing file.

```ts
interface ApplySprayRequest {
  sprayId: string;
  vtfPath: string;
  vmtPath: string | null;
  destinationDir: string;
  createBackup: boolean;
  overwrite: boolean;
}

interface ApplyResult {
  appliedFiles: string[];
  backupDir: string | null;
}
```

## Error Model

Commands reject with a string-serialized `AppError`. The frontend maps these to
user-facing messages. Categories include: `Config`, `Steam`, `Filesystem`,
`Validation`, and `NotFound`.
