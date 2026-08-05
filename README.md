# Anvil Password Manager

A cross-platform password manager written in Rust.

Anvil aims to be a modern password manager compatible with the KeePass KDBX format. The project consists of a reusable Rust library for reading and writing KDBX databases, along with cross-platform applications built on top of it.

[!NOTE]
Anvil currently writes KeePass KDBX4 databases. Support for additional database versions may be added in the future.

## Status
The core vault library is functional and includes:

- KDBX4 read/write support via keepass-rs
- Entry & group management
- Save/load support
- Tests for core functionality

## Planned

- Basic CLI interface (viewing and inspection only; no CRUD)
- Cross-platform GUI (Tauri)
- Search improvements
