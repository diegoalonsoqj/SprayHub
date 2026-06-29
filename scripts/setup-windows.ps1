<#
.SYNOPSIS
  One-shot setup for SprayHub on Windows: installs the toolchain (Rust, MSVC
  C++ Build Tools, WebView2), installs JS dependencies, and launches the app
  in development mode.

.DESCRIPTION
  Idempotent: each tool is only installed if missing. Uses winget for system
  packages. Some installs trigger a UAC prompt - accept them. After installing
  Rust/MSVC the script refreshes PATH in the current session so 'cargo' is
  found without reopening the terminal.

.EXAMPLE
  powershell -ExecutionPolicy Bypass -File scripts/setup-windows.ps1

  # Skip launching the app (install only):
  powershell -ExecutionPolicy Bypass -File scripts/setup-windows.ps1 -NoRun
#>
[CmdletBinding()]
param(
  [switch]$NoRun
)

$ErrorActionPreference = "Stop"

function Write-Step($msg) { Write-Host "`n==> $msg" -ForegroundColor Cyan }
function Write-Ok($msg)   { Write-Host "    $msg"   -ForegroundColor Green }
function Write-Skip($msg) { Write-Host "    $msg"   -ForegroundColor DarkGray }

function Test-Command($name) {
  return [bool](Get-Command $name -ErrorAction SilentlyContinue)
}

function Update-SessionPath {
  # Reload PATH from machine + user scopes, then ensure cargo's bin is present.
  $machine = [System.Environment]::GetEnvironmentVariable("Path", "Machine")
  $user    = [System.Environment]::GetEnvironmentVariable("Path", "User")
  $env:Path = ($machine, $user -join ";")
  $cargoBin = Join-Path $env:USERPROFILE ".cargo\bin"
  if ((Test-Path $cargoBin) -and ($env:Path -notlike "*$cargoBin*")) {
    $env:Path = "$cargoBin;$env:Path"
  }
}

# Move to the repository root (parent of this script's folder).
$repoRoot = Split-Path -Parent $PSScriptRoot
Set-Location $repoRoot
Write-Host "SprayHub setup - repo: $repoRoot" -ForegroundColor White

# ---------------------------------------------------------------------------
# 0. Prerequisites: winget
# ---------------------------------------------------------------------------
Write-Step "Checking winget"
if (-not (Test-Command "winget")) {
  throw "winget is not available. Install 'App Installer' from the Microsoft Store, then re-run this script."
}
Write-Ok "winget found"

# ---------------------------------------------------------------------------
# 1. Node.js
# ---------------------------------------------------------------------------
Write-Step "Checking Node.js"
if (Test-Command "node") {
  Write-Ok "Node $(node --version) already installed"
} else {
  Write-Host "    Installing Node.js LTS..." -ForegroundColor Yellow
  winget install --id OpenJS.NodeJS.LTS -e --accept-package-agreements --accept-source-agreements
  Update-SessionPath
}

# ---------------------------------------------------------------------------
# 2. MSVC C++ Build Tools (provides the linker Rust needs on Windows)
# ---------------------------------------------------------------------------
Write-Step "Checking MSVC C++ Build Tools"
$hasMsvc = (Test-Command "cl") -or (Test-Path "${env:ProgramFiles(x86)}\Microsoft Visual Studio\2022\BuildTools") -or (Test-Path "${env:ProgramFiles}\Microsoft Visual Studio\2022")
if ($hasMsvc) {
  Write-Skip "Visual Studio C++ build tools appear to be present"
} else {
  Write-Host "    Installing Visual Studio 2022 Build Tools with the C++ workload (large download)..." -ForegroundColor Yellow
  $vsArgs = "--quiet --wait --norestart --add Microsoft.VisualStudio.Workload.VCTools --includeRecommended"
  winget install --id Microsoft.VisualStudio.2022.BuildTools -e --accept-package-agreements --accept-source-agreements --override $vsArgs
}

# ---------------------------------------------------------------------------
# 3. WebView2 Runtime (Tauri's web engine)
# ---------------------------------------------------------------------------
Write-Step "Checking WebView2 Runtime"
$webview2Key = "HKLM:\SOFTWARE\WOW6432Node\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}"
if (Test-Path $webview2Key) {
  Write-Skip "WebView2 Runtime already installed"
} else {
  Write-Host "    Installing WebView2 Runtime..." -ForegroundColor Yellow
  try {
    winget install --id Microsoft.EdgeWebView2Runtime -e --accept-package-agreements --accept-source-agreements
  } catch {
    Write-Skip "WebView2 install reported an issue (often bundled with Windows) - continuing"
  }
}

# ---------------------------------------------------------------------------
# 4. Rust (rustup + stable toolchain)
# ---------------------------------------------------------------------------
Write-Step "Checking Rust"
Update-SessionPath
if (Test-Command "cargo") {
  Write-Ok "cargo $(cargo --version) already installed"
} else {
  Write-Host "    Installing Rust (rustup)..." -ForegroundColor Yellow
  winget install --id Rustlang.Rustup -e --accept-package-agreements --accept-source-agreements
  Update-SessionPath
  if (-not (Test-Command "rustup")) {
    throw "rustup was installed but is not on PATH yet. Close and reopen the terminal, then re-run this script."
  }
  rustup default stable
}
# Ensure the stable toolchain is actually present (rustup may install lazily).
if (Test-Command "rustup") {
  rustup toolchain install stable | Out-Null
  rustup default stable | Out-Null
}

# ---------------------------------------------------------------------------
# 5. JS dependencies
# ---------------------------------------------------------------------------
Write-Step "Installing JS dependencies"
if (Test-Path "node_modules") {
  Write-Skip "node_modules present - running 'npm install' to sync"
}
npm install

# ---------------------------------------------------------------------------
# 6. Sanity check
# ---------------------------------------------------------------------------
Write-Step "Toolchain summary"
Update-SessionPath
Write-Host ("    node  : " + (node --version))
Write-Host ("    npm   : " + (npm --version))
if (Test-Command "cargo") { Write-Host ("    cargo : " + (cargo --version)) }
else { Write-Warning "cargo still not on PATH - reopen the terminal before running 'npm run tauri:dev'." }

# ---------------------------------------------------------------------------
# 7. Launch
# ---------------------------------------------------------------------------
if ($NoRun) {
  Write-Host "`nSetup complete. Run 'npm run tauri:dev' to start the app." -ForegroundColor Green
  return
}

if (-not (Test-Command "cargo")) {
  Write-Warning "Skipping launch because cargo is not on PATH in this session. Reopen the terminal and run: npm run tauri:dev"
  return
}

Write-Step "Launching SprayHub (npm run tauri:dev)"
Write-Host "    First Rust build takes a few minutes; subsequent runs are cached." -ForegroundColor DarkGray
npm run tauri:dev
