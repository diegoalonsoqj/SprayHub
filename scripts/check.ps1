# Run all quality gates locally (Windows / PowerShell).
# Usage:  powershell -ExecutionPolicy Bypass -File scripts/check.ps1
$ErrorActionPreference = "Stop"

Write-Host "==> Frontend: lint" -ForegroundColor Cyan
npm run lint

Write-Host "==> Frontend: format check" -ForegroundColor Cyan
npm run format:check

Write-Host "==> Frontend: typecheck" -ForegroundColor Cyan
npm run typecheck

Write-Host "==> Frontend: tests" -ForegroundColor Cyan
npm run test

if (Get-Command cargo -ErrorAction SilentlyContinue) {
  Write-Host "==> Backend: fmt" -ForegroundColor Cyan
  cargo fmt --manifest-path src-tauri/Cargo.toml --all -- --check
  Write-Host "==> Backend: clippy" -ForegroundColor Cyan
  cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings
  Write-Host "==> Backend: tests" -ForegroundColor Cyan
  cargo test --manifest-path src-tauri/Cargo.toml
} else {
  Write-Warning "cargo not found; skipping Rust checks. Install Rust from https://rustup.rs/"
}

Write-Host "All checks passed." -ForegroundColor Green
