# 3. Detect games by parsing Steam's libraryfolders.vdf

- Status: Accepted
- Date: 2026-06-29

## Context

We must locate installed Source games across one or more Steam libraries on both
Windows and Linux, without requiring admin rights or running external programs.

## Decision

Locate the Steam root from well-known per-OS paths (and the registry on Windows,
read-only). Parse `steamapps/libraryfolders.vdf` to enumerate all library
folders, then for each library read `steamapps/appmanifest_<appid>.acf` and the
`installdir` to resolve each game's directory. A data-driven **game catalog**
maps each supported game to its Steam App ID and the relative
`materials/vgui/logos` destination.

## Consequences

- No dependency on the Steam client running or its Web API.
- Multi-library support comes for free.
- Adding a new Source game is a single catalog entry.
- The minimal VDF/ACF parser must tolerate formatting variations; it is covered
  by unit tests with fixture files.
