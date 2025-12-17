# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Avatar-orchestrated video pipeline for creating narrated educational videos. Rust-first, modular architecture with a web editor (Yew/WASM) for scene editing.

## Build and Run Commands

```bash
# Build all components
./scripts/build_all.sh

# Run orchestrator server (uses config.toml by default)
./scripts/run_orchestrator.sh
./scripts/run_orchestrator.sh /path/to/custom/config.toml

# Run UI dev server with hot-reload
./scripts/dev_ui.sh

# Build/test individual workspaces
cd components/orchestrator && cargo build
cd components/orchestrator && cargo test
cd components/ui_workbench && cargo build
```

## Pre-Commit Quality Process (Mandatory)

Run in order before every commit:

```bash
# 1. Tests (in each workspace)
cd components/orchestrator && cargo test
cd components/ui_workbench && cargo test

# 2. Linting (zero warnings allowed)
cd components/orchestrator && cargo clippy --all-targets --all-features -- -D warnings
cd components/ui_workbench && cargo clippy --all-targets --all-features -- -D warnings

# 3. Formatting
cd components/orchestrator && cargo fmt --all
cd components/ui_workbench && cargo fmt --all

# 4. Markdown validation
markdown-checker -f "**/*.md"

# 5. Project checklist
sw-checklist
```

## Architecture

**No root Cargo.toml** - Each component is an independent Rust workspace.

### Component Workspaces

**Orchestrator** (`components/orchestrator/`):
- `orchestrator-cli` - Binary entrypoint, CLI args, config loading
- `orchestrator-core` - Domain logic, project/scene/asset CRUD, workflow services
- `orchestrator-http` - Warp HTTP API (health, projects endpoints)
- `orchestrator-db` - SQLite via rusqlite, repository implementations

**UI Workbench** (`components/ui_workbench/`):
- `ui-core` - Shared types, DTOs, validation (no web dependencies)
- `ui-app` - Yew/WASM frontend, built with Trunk

### Data Flow

- SQLite stores metadata (projects, scenes, assets)
- Large assets (video/audio/images) stored on filesystem under `assets_root`
- Config in `config.toml` defines server bind, DB path, and service endpoints

## Code Standards

- Rust 2024 Edition
- No TypeScript, no Python in project tree
- Keep files under 500 lines, functions under 50 lines
- Use inline format arguments: `format!("{name}")` not `format!("{}", name)`
- Module docs use `//!`, item docs use `///`
- Max 3 TODOs per file, never commit FIXMEs

## Testing

- Write tests in Rust, not Python
- Use `wasm-bindgen-test` for WASM-specific tests
- Unit tests: `#[cfg(test)]` modules in same file
- Integration tests: test database operations with `tempfile::tempdir()`
