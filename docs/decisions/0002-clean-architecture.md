# 2. Adopt Clean Architecture on both backend and frontend

- Status: Accepted
- Date: 2026-06-29

## Context

SprayHub is intended to grow into an ecosystem (Desktop, CLI, Core, SDK,
VTFLib wrapper). We need a structure that keeps business rules independent from
frameworks (Tauri, React) so logic can be reused and tested in isolation, and so
future tools can share the same core concepts.

## Decision

Adopt **Clean Architecture** with four layers — Domain, Application,
Infrastructure, Presentation — in both `src-tauri/` (Rust) and `src/` (React).
The dependency rule is enforced: dependencies point inward toward the Domain.

- Domain has zero framework dependencies.
- Application contains use cases depending only on Domain contracts.
- Infrastructure implements those contracts (Steam, FS, config, logging).
- Presentation (Tauri commands / React UI) is the outermost layer.

## Consequences

- Slightly more boilerplate (traits/interfaces and DTO mapping).
- Use cases are unit-testable with in-memory fakes, with no Tauri runtime.
- Business logic is portable to the future CLI/Core/SDK.
