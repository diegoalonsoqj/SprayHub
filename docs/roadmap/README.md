# SprayHub Roadmap

Versioning follows [Semantic Versioning](https://semver.org/).

| Version | Theme                     | Highlights                                        |
| ------- | ------------------------- | ------------------------------------------------- |
| 0.1.0   | Initial prototype         | Scaffold, Clean Architecture, dark UI             |
| 0.2.0   | Multi-game support        | Extensible game catalog, per-game destinations    |
| 0.3.0   | Steam detection           | `libraryfolders.vdf` parsing, multi-library       |
| 0.4.0   | PNG → VTF conversion       | VTFLib wrapper integration                         |
| 0.5.0   | Favorites & history       | Persisted favorites, recent applies               |
| 0.6.0   | UX improvements           | Drag & drop, keyboard navigation, polish          |
| 0.7.0   | Linux stable              | Hardened Linux paths and packaging                |
| 0.8.0   | Auto-update               | Tauri updater + signing                           |
| 0.9.0   | Release candidate         | Stabilization, i18n ES/EN complete                |
| 1.0.0   | Stable                    | First stable release                              |

## Long-term ecosystem

- **SprayHub Desktop** — this application.
- **SprayHub CLI** — headless spray management.
- **SprayHub Core** — shared Rust crate with the domain + use cases.
- **SprayHub SDK** — plugin/extension API.
- **VTFLib Wrapper** — safe Rust bindings for VTF/VMT handling.

The Clean Architecture split is designed so Core can be extracted into a shared
crate consumed by Desktop, CLI, and SDK without breaking changes.
