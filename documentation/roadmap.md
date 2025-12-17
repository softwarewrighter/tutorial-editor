# Roadmap

## Milestone 0 – Skeleton (this repo)

- Multi-component workspace layout in pure Rust.
- CLI with config file + HTTP health endpoint.
- SQLite connection + `projects` table migration.
- Yew/WASM UI skeleton that can talk to `/api/health`.

## Milestone 1 – Project & Scene Management

- CRUD for projects via CLI + HTTP API.
- Basic `scenes` table + CRUD.
- UI lists projects, allows creation and simple editing.

## Milestone 2 – Avatar & Script Orchestration

- Integrate with one TTS backend via HTTP.
- Integrate with one avatar pipeline (image-to-video + lip sync) via HTTP.
- Define “workflow” model in core:
  - “From idea → script → scene plan → avatar assets.”

## Milestone 3 – MCP/Playwright Automation

- Add adapter crate to call MCP/Playwright automation services.
- Model “scene capture tasks” with references to remote hosts and tools.
- Track captured clips and their asset paths in SQLite.

## Milestone 4 – Timeline & Editor UX

- UI-based scene timeline editor.
- Drag/drop avatar placement, graphic overlays.
- Per-scene preview; per-project preview.

## Milestone 5 – Render Pipeline Integration

- Define render recipes (FFmpeg or similar) in Rust.
- Generate shell scripts or direct process invocations to produce final videos.
- Support multiple output profiles (YouTube, short, etc.).
