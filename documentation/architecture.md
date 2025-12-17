# Avatar-Orchestrated Video Pipeline – Architecture Overview

## Goals

- Automate creation of narrated educational videos (Twitch-style live coding but tightly edited).
- Use local and cloud LLMs plus existing tools:
  - Avatar image generation
  - Image-to-video (head movement, blinking)
  - TTS with cloned voice
  - Video stretching & lip sync
  - Background removal
- Provide a Rust-first, testable, modular orchestration layer with a web editor (Yew/WASM) for fine tuning scenes.

## High-Level Architecture

Top-level layout:

- `documentation/` – design docs, HOWTOs, architecture notes.
- `scripts/` – small shell helpers to build and run components.
- `components/orchestrator/` – Rust workspace for CLI + HTTP server + orchestration core + SQLite access.
- `components/ui_workbench/` – Rust workspace for Yew/WASM editor + shared UI core.

Each `components/<name>/` directory is an independent Rust workspace with its own `Cargo.toml` and `crates/*` members.
The repo *root* deliberately has **no** `Cargo.toml`.

## Orchestrator Component

Workspace: `components/orchestrator/`

Crates:

- `orchestrator-cli`
  - Binary entrypoint.
  - Parses CLI args (`-c/--config-file`, default `./config.toml`).
  - Initializes tracing/logging.
  - Loads config, bootstraps orchestrator-core, starts HTTP server.

- `orchestrator-core`
  - Domain + application services for:
    - Projects, scenes, assets CRUD (backed by SQLite via orchestrator-db).
    - High-level workflows (e.g., “generate script”, “plan scenes”, “invoke avatar pipeline”).
  - Pure Rust logic; no IO except via trait-based ports to infra crates.

- `orchestrator-http`
  - HTTP API using Warp.
  - Endpoints (initially stubs):
    - `GET /api/health`
    - `GET /api/projects`
    - `POST /api/projects`
  - Depends on `orchestrator-core` via traits, so it can be swapped or tested independently.

- `orchestrator-db`
  - SQLite integration via `rusqlite`.
  - Provides implementations of repository traits defined in `orchestrator-core`.
  - Responsible for:
    - Opening DB file (path comes from config).
    - Running migrations on startup (simple `CREATE TABLE IF NOT EXISTS ...`).

### Data & Files

- SQLite DB:
  - **Not** checked into git.
  - Path configurable in `config.toml`.
- Large binaries (video/audio/image assets):
  - Stored on filesystem under a configurable root directory.
  - Only metadata (paths, durations, associations) live in SQLite.

### Config

File: `config.toml` (or supplied via `-c/--config-file`):

- HTTP server (bind, port).
- Paths for:
  - SQLite DB file.
  - Assets root directory.
- URLs/hosts for:
  - Cloud LLMs
  - Local LLMs (Ollama, llama.cpp HTTP, etc.).
  - MCP/Playwright integration service endpoints.
  - Avatar/TTS/image/video services.

## UI Workbench Component

Workspace: `components/ui_workbench/`

Crates:

- `ui-core`
  - Shared UI types (DTOs), validation utilities.
  - Typed mirror of the HTTP API payloads (projects, scenes, assets).
  - No web-specific dependencies.

- `ui-app`
  - Yew/WASM frontend.
  - Built with `trunk`.
  - Responsibilities:
    - Load list of projects/scenes from orchestrator HTTP API.
    - Provide basic skeleton editor (placeholder view in initial template).
    - Later: drag/drop overlays, avatar placement, asset selection, scene timeline editing.

Build/development:

- `Trunk.toml` + `index.html` in `ui-app` crate.
- Use `trunk serve` during development for hot-reload.
- Production: `trunk build` then serve static files from the orchestrator HTTP server (future step).

## Non-Functional Requirements

- **Rust 2024 Edition** everywhere (via workspace default + per crate `edition = "2024"`).
- Minimal to no custom JavaScript; rely on Rust/WASM and Yew.
- No TypeScript, no Python in the project tree.
- Small, focused crates:
  - Few functions per module.
  - Few modules per crate.
  - Few crates per component, unlimited components.
- Testability:
  - Core logic isolated in `orchestrator-core` and `ui-core`.
  - HTTP and DB crates thin adapters around core traits.
- Portability:
  - CLI server should run on Linux/macOS with minimal dependencies (SQLite bundled).

## Initial HTTP & DB Design

### Example API (early stub)

- `GET /api/health` → `{ "status": "ok" }`
- `GET /api/projects` → list of projects.
- `POST /api/projects` → create new project.

### SQLite Schema (first pass)

- `projects`:
  - `id INTEGER PRIMARY KEY`
  - `slug TEXT UNIQUE NOT NULL`
  - `title TEXT NOT NULL`
  - `subtitle TEXT`
  - `description TEXT`
  - `created_at TEXT NOT NULL`
  - `updated_at TEXT NOT NULL`

- `assets` (future):
  - `id INTEGER PRIMARY KEY`
  - `project_id INTEGER NOT NULL`
  - `kind TEXT NOT NULL` (e.g., `avatar_image`, `avatar_video`, `tts_audio`, `screen_capture`, `composited_scene`, etc.)
  - `path TEXT NOT NULL`
  - `duration_ms INTEGER`
  - `metadata_json TEXT`
  - Foreign key to `projects(id)` (enforced at app level for now).

## Next Steps

1. Implement basic project CRUD in `orchestrator-core` and `orchestrator-db`.
2. Flesh out HTTP routes in `orchestrator-http`.
3. Have `ui-app` render:
   - Project list
   - “Create project” form
4. Add minimal “scene” concept:
   - Each project has ordered scenes with titles, asset references, and timing.
5. Gradually integrate external services (LLMs, avatar pipeline, Playwright/MCP) behind trait-based adapters.
