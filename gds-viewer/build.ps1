# Builds gds-viewer.exe with the status labels baked in.  INTERNAL USE ONLY - the exe contains Hyundai label text.
#
# One-time setup:  install Rust (https://rustup.rs, choose the default MSVC toolchain) and Python.
# Usage (PowerShell, in this folder):
#   .\build.ps1 -Labels "C:\Users\Jerry\Downloads\xef_out\status_lookup.json"
param([Parameter(Mandatory = $true)][string]$Labels, [string]$Version = "0.48")
$ErrorActionPreference = "Stop"
python bake_internal.py viewer_source.html $Labels assets\viewer.html $Version
cargo build --release
$exe = ".\GDS-Recorded-File-Viewer_$Version.exe"
Copy-Item target\release\gds-viewer.exe $exe -Force
Write-Host "Done: $(Resolve-Path $exe)"
