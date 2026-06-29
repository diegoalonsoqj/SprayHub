# Scripts

Helper scripts for local development.

| Script             | Platform        | Purpose                                              |
| ------------------ | --------------- | ---------------------------------------------------- |
| `setup-windows.ps1`| Windows         | Install the full toolchain (Rust, MSVC, WebView2, deps) and launch `tauri:dev` |
| `check.ps1`        | Windows         | Run all lint/format/typecheck/test gates             |
| `check.sh`         | Linux / macOS   | Same, for Unix shells                                |

The `check` scripts skip the Rust checks gracefully if `cargo` is not installed.

## First-time setup (Windows)

```powershell
powershell -ExecutionPolicy Bypass -File scripts/setup-windows.ps1
```

Installs Node (if missing), Visual Studio Build Tools (C++ workload), the
WebView2 Runtime and Rust, then runs `npm install` and starts the app. Accept
any UAC prompts. Add `-NoRun` to install without launching.
