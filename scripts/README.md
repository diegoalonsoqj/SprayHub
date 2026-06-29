# Scripts

Helper scripts for local development.

| Script      | Platform        | Purpose                                  |
| ----------- | --------------- | ---------------------------------------- |
| `check.ps1` | Windows         | Run all lint/format/typecheck/test gates |
| `check.sh`  | Linux / macOS   | Same, for Unix shells                    |

Both scripts skip the Rust checks gracefully if `cargo` is not installed.
