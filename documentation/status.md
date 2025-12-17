# Project Status

## Current Milestone: 1 - Scene Management

**Status**: COMPLETE

## Completed Work

### Milestone 0 - Skeleton

#### Infrastructure
- [x] Multi-component workspace layout (no root Cargo.toml)
- [x] Rust 2024 Edition configuration
- [x] Build scripts (`scripts/build_all.sh`)
- [x] Run scripts (`scripts/run_orchestrator.sh`, `scripts/dev_ui.sh`)
- [x] Configuration file (`config.toml`)

#### Orchestrator Component
- [x] `orchestrator-cli` - Binary with `-c/--config-file` argument
- [x] `orchestrator-core` - Config loading, domain types, service traits
- [x] `orchestrator-db` - SQLite setup, projects table migration
- [x] `orchestrator-http` - Warp server with health and projects endpoints

#### UI Workbench Component
- [x] `ui-core` - Shared DTOs
- [x] `ui-app` - Yew/WASM skeleton with Trunk build

#### Documentation
- [x] `documentation/architecture.md`
- [x] `documentation/roadmap.md`
- [x] `documentation/process.md`
- [x] `documentation/tools.md`
- [x] `documentation/ai_agent_instructions.md`
- [x] `documentation/prd.md`
- [x] `documentation/design.md`
- [x] `documentation/plan.md`
- [x] `documentation/status.md`
- [x] `CLAUDE.md`

### Milestone 1 - Scene Management

#### Backend
- [x] Scene struct in `orchestrator-core/src/domain.rs`
- [x] SceneRepository trait in `orchestrator-core/src/ports.rs`
- [x] Scenes table with foreign key in `orchestrator-db/src/schema.rs`
- [x] SqliteSceneRepository in `orchestrator-db/src/scene_repo.rs`
- [x] Timestamp helpers in `orchestrator-db/src/timestamps.rs`
- [x] Scene mapping helpers in `orchestrator-db/src/scene_mapping.rs`
- [x] Scene HTTP handlers in `orchestrator-http/src/handlers.rs`
- [x] Route modules split for sw-checklist compliance
- [x] OrchestratorApp scene methods via `project_ops.rs` and `scene_ops.rs`

#### Frontend
- [x] SceneDto in `ui-core/src/lib.rs`
- [x] API client module `ui-app/src/api.rs`
- [x] SceneList component `ui-app/src/scene_list.rs`
- [x] ProjectList with on_select callback
- [x] App state management for project selection and scene loading

## Working Features

**HTTP API**:
- `GET /api/health` - Returns `{"status":"ok"}`
- `GET /api/projects` - List all projects
- `POST /api/projects` - Create new project
- `GET /api/projects/{id}/scenes` - List scenes in project
- `POST /api/projects/{id}/scenes` - Create scene in project
- `PUT /api/scenes/{id}` - Update scene
- `DELETE /api/scenes/{id}` - Delete scene

**CLI**:
- Loads config from file
- Initializes SQLite database with projects and scenes tables
- Starts HTTP server with full scene CRUD

**UI**:
- Yew application with project list
- Project selection triggers scene list fetch
- API client for backend communication

## Next Milestone: 2 - Script Editor

**Status**: NOT STARTED

### Planned Work
- [ ] Script text editing in scene detail view
- [ ] Scene reordering (drag-and-drop or manual sort)
- [ ] Project creation form in UI
- [ ] Scene creation form in UI
- [ ] Auto-save or explicit save for script changes

## Known Issues

None currently.

## Blockers

None currently.

## Recent Changes

| Date | Change |
|------|--------|
| 2025-12-17 | Initial skeleton complete |
| 2025-12-17 | Documentation suite created |
| 2025-12-17 | Phase 1 Scene Management complete |

## Metrics

| Metric | Value |
|--------|-------|
| Components | 2 |
| Crates (orchestrator) | 4 |
| Crates (ui_workbench) | 2 |
| HTTP Endpoints | 7 |
| SQLite Tables | 2 (projects, scenes) |
