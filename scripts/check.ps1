# Run all quality gates locally (Windows / PowerShell).
# Usage:  powershell -ExecutionPolicy Bypass -File scripts/check.ps1
$ErrorActionPreference = "Stop"

function Invoke-Step {
  param([string]$Label, [scriptblock]$Action)
  Write-Host "==> $Label" -ForegroundColor Cyan
  & $Action
  if ($LASTEXITCODE -ne 0) {
    throw "$Label FAILED (exit code $LASTEXITCODE)."
  }
}

# rustup installs cargo to ~/.cargo/bin, which a fresh shell may not have on PATH.
if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
  $cargoBin = Join-Path $env:USERPROFILE ".cargo\bin"
  if (Test-Path (Join-Path $cargoBin "cargo.exe")) {
    $env:Path = "$cargoBin;$env:Path"
  }
}

Invoke-Step "Frontend: lint"         { npm run lint }
Invoke-Step "Frontend: format check" { npm run format:check }
Invoke-Step "Frontend: typecheck"    { npm run typecheck }
Invoke-Step "Frontend: tests"        { npm run test }

if (Get-Command cargo -ErrorAction SilentlyContinue) {
  Invoke-Step "Backend: fmt"    { cargo fmt --manifest-path src-tauri/Cargo.toml --all -- --check }
  Invoke-Step "Backend: clippy" { cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings }
  Invoke-Step "Backend: tests"  { cargo test --manifest-path src-tauri/Cargo.toml }
} else {
  Write-Warning "cargo not found; skipping Rust checks. Install Rust from https://rustup.rs/"
}

Write-Host "All checks passed." -ForegroundColor Green
