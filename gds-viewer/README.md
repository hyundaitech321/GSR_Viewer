# GDS Recorded File Viewer (beta) - exe

INTERNAL USE ONLY - DO NOT SHARE. Built by Jerry Lawson, Hyundai Techline, Team High Voltage.

A small native Windows program that shows the HTML viewer (`viewer_source.html`) in its own window using the
WebView2 runtime that comes with Windows 10/11 (the same engine as Edge). Everything is compiled into one exe;
nothing is uploaded. The status labels (`status_lookup.json`) are baked in, so the exe contains Hyundai text.

## Build
1. Install Rust from https://rustup.rs (default MSVC toolchain) and Python 3.
2. In PowerShell, in this folder:
   `.\build.ps1 -Version 0.48 -Labels "C:\Users\Jerry\Downloads\xef_out\status_lookup.json"`
3. Result: `GDS-Recorded-File-Viewer_0.48.exe` (about 4 MB) in this folder.

## Use
- Double-click the exe, then drop a recording on the window or use Open recording.
- Or drag a recording onto the exe, or run `GDS-Recorded-File-Viewer_0.48.exe file.msr [second.gsr]`.
- To make it the default for .MSR/.GSR/.GSRZ: right-click a recording, Open with, Choose another app, pick the exe.
- Save picture / Export CSV ask where to save the file.
- Remembered settings (collapsed PID list, etc.) live in `%LOCALAPPDATA%\GDSRecordedFileViewer`.

## Updating
Edit `viewer_source.html` (or replace it with a newer GSR-Viewer.html), rebuild the labels if needed, run `build.ps1` again.
If a PC has no WebView2 runtime (rare): https://developer.microsoft.com/microsoft-edge/webview2/
