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
You'll need the [Tauri prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites) for your OS.

This project uses `pnpm`, the `rust` toolchain, and the `tauri` cli.
Then just clone and
```sh
# Install node packages
pnpm install

# Compile and run peach
cargo tauri dev
```
