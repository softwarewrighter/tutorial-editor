# Project Status

## Current Milestone: 3 - Asset Management

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

### Milestone 2 - Scene Detail View & Forms

#### Frontend Components
- [x] `scene_detail.rs` - Read-only scene display with script preview
- [x] `scene_edit_form.rs` - Edit scene metadata (title, description, sort_order)
- [x] `project_form.rs` - Create new project form
- [x] `scene_form.rs` - Create new scene form
- [x] `scene_list.rs` - Added on_select callback for scene selection

#### App Architecture
- [x] Modular app structure: `app/mod.rs`, `callbacks.rs`, `callbacks2.rs`, `hooks.rs`, `render.rs`
- [x] Custom Yew hooks with `#[hook]` macro: `use_fetch_projects`, `use_fetch_scenes`
- [x] AppCallbacks struct for centralized callback management
- [x] Refresh trigger pattern for data reload after mutations

#### API Client
- [x] `update_scene_metadata()` function for PUT /api/scenes/{id}

### Milestone 3 - Asset Management

#### Backend (orchestrator-core)
- [x] Asset struct in `domain.rs` with project_id, scene_id (optional), name, asset_type, file_path, url, metadata
- [x] AssetRepository trait in `ports.rs` with CRUD methods
- [x] `asset_ops.rs` - Service methods for asset operations
- [x] Third generic param `A: AssetRepository` added to OrchestratorApp

#### Backend (orchestrator-db)
- [x] Assets table in `schema.rs` with foreign keys to projects and scenes
- [x] `asset_mapping.rs` - Row mapping helper
- [x] `asset_repo.rs` - SqliteAssetRepository implementation

#### Backend (orchestrator-http)
- [x] Asset request types (CreateAssetRequest, UpdateAssetRequest) in `handlers.rs`
- [x] Asset handlers: list_project_assets, list_scene_assets, create, update, delete
- [x] `asset_routes.rs` - Asset HTTP routes
- [x] Third generic param added to all route modules

#### Frontend (ui-core)
- [x] AssetDto struct

#### Frontend (ui-app)
- [x] `asset_api.rs` - Asset API functions (fetch, create, update, delete)
- [x] `asset_list.rs` - Asset list component with add/edit/delete actions
- [x] `asset_form.rs` - Asset create/edit form
- [x] `asset_callbacks.rs` - Asset callback builders
- [x] `scene_detail.rs` - Integrated asset list with local state management

## Working Features

**HTTP API**:
- `GET /api/health` - Returns `{"status":"ok"}`
- `GET /api/projects` - List all projects
- `POST /api/projects` - Create new project
- `GET /api/projects/{id}/scenes` - List scenes in project
- `POST /api/projects/{id}/scenes` - Create scene in project
- `PUT /api/scenes/{id}` - Update scene
- `DELETE /api/scenes/{id}` - Delete scene
- `GET /api/projects/{id}/assets` - List project assets
- `GET /api/scenes/{id}/assets` - List scene assets
- `POST /api/assets` - Create asset
- `PUT /api/assets/{id}` - Update asset
- `DELETE /api/assets/{id}` - Delete asset

**CLI**:
- Loads config from file
- Initializes SQLite database with projects, scenes, and assets tables
- Starts HTTP server with full scene and asset CRUD

**UI**:
- Yew application with project list and scene list
- Project selection triggers scene list fetch
- Scene selection shows scene detail with read-only script preview
- Project creation form with slug and title fields
- Scene creation form with title and sort_order fields
- Scene metadata editing form
- Asset list in scene detail with add/edit/delete functionality
- Asset creation and editing forms
- API client for all backend operations

## Next Milestone: 4 - Service Integration

**Status**: NOT STARTED

### Planned Work
- [ ] LLM service integration for script generation
- [ ] TTS service integration for audio generation
- [ ] Avatar service integration for video generation
- [ ] Job queue for async processing
- [ ] Status tracking for service operations

## Known Issues

- **sw-checklist crate module count**: ui-app has 18 modules (max 7). This is a structural issue that would require significant refactoring to fix - either consolidating modules into fewer files or splitting into sub-crates.

## Blockers

None currently.

## Recent Changes

| Date | Change |
|------|--------|
| 2025-12-17 | Initial skeleton complete |
| 2025-12-17 | Documentation suite created |
| 2025-12-17 | Phase 1 Scene Management complete |
| 2025-12-17 | Phase 2 Scene Detail View & Forms complete |
| 2025-12-17 | Phase 3 Asset Management complete |

## Metrics

| Metric | Value |
|--------|-------|
| Components | 2 |
| Crates (orchestrator) | 4 |
| Crates (ui_workbench) | 2 |
| UI Modules | 18 (+ 5 app submodules) |
| HTTP Endpoints | 12 |
| SQLite Tables | 3 (projects, scenes, assets) |
