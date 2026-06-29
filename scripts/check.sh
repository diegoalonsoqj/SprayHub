#!/usr/bin/env bash
# Run all quality gates locally (Linux/macOS).
# Usage:  bash scripts/check.sh
set -euo pipefail

echo "==> Frontend: lint"
npm run lint

echo "==> Frontend: format check"
npm run format:check

echo "==> Frontend: typecheck"
npm run typecheck

echo "==> Frontend: tests"
npm run test

if command -v cargo >/dev/null 2>&1; then
  echo "==> Backend: fmt"
  cargo fmt --manifest-path src-tauri/Cargo.toml --all -- --check
  echo "==> Backend: clippy"
  cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings
  echo "==> Backend: tests"
  cargo test --manifest-path src-tauri/Cargo.toml
else
  echo "WARNING: cargo not found; skipping Rust checks. Install Rust from https://rustup.rs/"
fi

echo "All checks passed."
