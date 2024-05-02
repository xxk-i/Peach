# Peach 
A WIP cross-platform file b(r)owser, in pink.

## Features
  - Directory Pinning
  - Tab System
  - Folder syncing over LAN
  - Application browser (MACOS only currently)

## Upcoming Features
  - Visual scripting for automatic installations
  - Themes
  - Native context menu forwarding
  - Undo history

## Building
This project uses `pnpm`, the `rust` toolchain, and the `tauri` cli.
```
npm install -g pnpm
cargo install tauri-cli
```
Then just clone and
```
pnpm install
cargo tauri dev
```
