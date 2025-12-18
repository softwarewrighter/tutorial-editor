# Project Status

## Current Milestone: 6 - Avatar Pipeline Integration

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

### Milestone 4 - Service Integration

#### Backend (orchestrator-core)
- [x] Service client traits in `orchestrator-ports/src/services.rs`: LlmClient, TtsClient, AvatarPipelineClient, McpClient
- [x] `service_ops.rs` - Service wrapper methods: generate_avatar_video, synthesize_speech, lip_sync_video, remove_video_background
- [x] OrchestratorApp builder pattern with `.with_llm()`, `.with_tts()`, `.with_avatar()`, `.with_mcp()`
- [x] Optional service clients using `Option<Arc<dyn Trait>>`

#### Backend (orchestrator-adapters) - NEW CRATE
- [x] `llm_adapter.rs` - HttpLlmClient for LLM API calls
- [x] `tts_adapter.rs` - HttpTtsClient for TTS API calls
- [x] `avatar_adapter.rs` - HttpAvatarClient for avatar pipeline services
- [x] `mcp_adapter.rs` - HttpMcpClient for scene capture
- [x] `stubs.rs` - Stub implementations for testing

#### Configuration
- [x] Service URLs in `config.toml`: llm, tts, avatar (image_to_video, lip_sync, background_removal)

### Milestone 5 - Script Generation Workflow

#### Backend (orchestrator-core)
- [x] `script_ops.rs` - generate_project_script(), generate_scene_script() with prompt building
- [x] Prompt templates with project context for LLM
- [x] JSON response parsing into Scene objects with script_text

#### Backend (orchestrator-http)
- [x] `script_routes.rs` - Script generation endpoints
- [x] `POST /api/projects/{id}/generate-script` - Generate full project script, auto-create scenes
- [x] `POST /api/scenes/{id}/generate-script` - Generate/refine individual scene script

### Milestone 6 - Avatar Pipeline Integration

#### Backend (orchestrator-core)
- [x] `avatar_ops.rs` - 6 pipeline orchestration methods:
  - avatar_generate_audio (TTS)
  - avatar_generate_video (image to video)
  - avatar_stretch_video (match duration to audio)
  - avatar_lip_sync (sync lips to audio)
  - avatar_remove_background
  - avatar_pipeline (full 5-step workflow)
- [x] Each step creates Asset with appropriate asset_type

#### Backend (orchestrator-adapters)
- [x] stretch_video implementation in avatar_adapter.rs

#### Backend (orchestrator-http)
- [x] `avatar_routes.rs` - 6 avatar endpoints
- [x] Request types in orchestrator-http-handlers

#### Configuration
- [x] Added `video_stretch` URL to config.toml

#### Code Quality Refactoring
- [x] Created `orchestrator-domain` crate (config + domain types)
- [x] Created `orchestrator-ports` crate (repository + service traits)
- [x] Created `orchestrator-http-handlers` crate (handler functions)
- [x] Consolidated orchestrator-db helpers into lib.rs (8 → 5 modules)
- [x] Consolidated orchestrator-http routes (9 → 7 modules)
- [x] Reduced orchestrator-core to 7 modules
- [x] sw-checklist: 0 failures (down from 5)

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
- `POST /api/projects/{id}/generate-script` - Generate project script via LLM
- `POST /api/scenes/{id}/generate-script` - Generate/refine scene script
- `POST /api/scenes/{id}/avatar/generate-audio` - TTS audio generation
- `POST /api/scenes/{id}/avatar/generate-video` - Image to video with head movement
- `POST /api/scenes/{id}/avatar/stretch-video` - Match video duration to audio
- `POST /api/scenes/{id}/avatar/lip-sync` - Lip sync video to audio
- `POST /api/scenes/{id}/avatar/remove-background` - Background removal
- `POST /api/scenes/{id}/avatar/pipeline` - Full avatar pipeline (5 steps)

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

## Next Milestone: 7 - Video Composition & Timeline

**Status**: NOT STARTED

### Planned Work
- [ ] Scene timeline composition (audio + video + overlays)
- [ ] FFmpeg integration for video processing
- [ ] Progress tracking for long-running operations
- [ ] Preview generation for scenes
- [ ] Final video export for projects

## Known Issues

- **sw-checklist ui-app module count**: ui-app has 18 modules (max 7). This is a structural issue that would require significant refactoring to fix - either consolidating modules into fewer files or splitting into sub-crates.
- **sw-checklist warnings**: 36 warnings remain for function LOC and module function counts (all within limits but above warning thresholds).

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
| 2025-12-17 | Phase 4 Service Integration complete |
| 2025-12-17 | Phase 5 Script Generation Workflow complete |
| 2025-12-17 | Phase 6 Avatar Pipeline Integration complete |

## Metrics

| Metric | Value |
|--------|-------|
| Components | 2 |
| Crates (orchestrator) | 8 |
| Crates (ui_workbench) | 2 |
| UI Modules | 18 (+ 5 app submodules) |
| HTTP Endpoints | 20 |
| SQLite Tables | 3 (projects, scenes, assets) |
| sw-checklist | 0 failures, 36 warnings |
