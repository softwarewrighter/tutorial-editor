# Development Plan

## Current Phase: Milestone 0 - Skeleton

The project skeleton is complete with:
- Multi-component workspace layout
- CLI with config file support
- HTTP health endpoint
- SQLite connection with projects table migration
- Yew/WASM UI skeleton

## Implementation Phases

### Phase 1: Project and Scene Management

**Backend Tasks**:
1. Add `Scene` struct to `orchestrator-core::domain`
2. Define `SceneRepository` trait in `orchestrator-core::ports`
3. Implement scenes table migration in `orchestrator-db`
4. Implement `SqliteSceneRepository` in `orchestrator-db`
5. Add scene CRUD methods to `OrchestratorApp` service
6. Expose scene endpoints in `orchestrator-http`:
   - `GET /api/projects/{id}/scenes`
   - `POST /api/projects/{id}/scenes`
   - `PUT /api/scenes/{id}`
   - `DELETE /api/scenes/{id}`

**Frontend Tasks**:
1. Add `SceneDto` to `ui-core`
2. Implement HTTP client for projects API in `ui-app`
3. Create project list component
4. Create project detail view with scene list
5. Add create/edit project form
6. Add create/edit scene form

### Phase 2: Asset Management

**Backend Tasks**:
1. Add `Asset` struct with kind enum to domain
2. Define `AssetRepository` trait
3. Implement assets table in SQLite
4. Add asset file management utilities
5. Expose asset endpoints:
   - `GET /api/scenes/{id}/assets`
   - `POST /api/scenes/{id}/assets` (with file upload)
   - `DELETE /api/assets/{id}`

**Frontend Tasks**:
1. Add `AssetDto` to `ui-core`
2. Create asset list component per scene
3. Implement file upload component

### Phase 3: Service Client Traits

**Define Traits** (in `orchestrator-core::ports`):
1. `LlmClient` - script generation, content refinement
2. `TtsClient` - text-to-speech synthesis
3. `AvatarPipelineClient` - image-to-video, lip sync, background removal
4. `McpClient` - scene capture via Playwright

**Create Adapter Crate**:
1. Add `orchestrator-adapters` crate
2. Implement HTTP-based adapters for each trait
3. Wire adapters into CLI startup based on config

### Phase 4: Script Generation Workflow

**Implementation**:
1. Add script generation endpoint
2. Implement LLM prompt templates for:
   - Topic suggestion from project status
   - Script generation from idea
   - Scene breakdown from script
   - Overlay text generation
3. Store generated content in scenes/assets
4. Add regeneration support

### Phase 5: Avatar Pipeline Integration

**Implementation**:
1. Implement `AvatarPipelineClient` adapter
2. Add workflow for avatar generation:
   - Select/generate avatar image
   - Create video with head movement
   - Generate TTS audio
   - Stretch video to audio length
   - Lip sync
   - Remove background
3. Track all intermediate assets
4. Add retry/regenerate capabilities

### Phase 6: MCP/Playwright Scene Capture

**Implementation**:
1. Add `orchestrator-mcp` crate
2. Implement `McpClient` for Playwright automation
3. Define scene capture targets (terminal, editor, browser, etc.)
4. Model capture tasks with Chrome Remote Desktop references
5. Execute scripted interactions
6. Store captured video assets

### Phase 7: Scene Editor UI

**Implementation**:
1. Create scene preview component
2. Add avatar selection panel
3. Implement avatar drag-and-drop positioning
4. Add facing direction toggle with auto-positioning
5. Create overlay text editor
6. Implement overlay positioning controls
7. Add text avoidance zone markers

### Phase 8: Video Composition

**Implementation**:
1. Add `orchestrator-render` crate
2. Define composition recipes (FFmpeg command sequences)
3. Implement compositor service:
   - Avatar overlay on captures
   - Position calculation based on facing/avoidance
   - Graphic overlay rendering
4. Track output assets

### Phase 9: Final Assembly and Export

**Implementation**:
1. Scene concatenation with transitions
2. Intro/outro generation
3. Multiple output profiles (YouTube, shorts)
4. Export progress tracking
5. Final video asset management

## Task Tracking

Use the following status markers in commits and status.md:
- `[PLANNED]` - Task defined, not started
- `[IN_PROGRESS]` - Currently being implemented
- `[BLOCKED]` - Waiting on dependency or decision
- `[COMPLETE]` - Implementation done and tested
- `[DEFERRED]` - Postponed to later phase

## Definition of Done

A task is complete when:
1. Code compiles without warnings (`cargo clippy -- -D warnings`)
2. All tests pass (`cargo test`)
3. Code is formatted (`cargo fmt`)
4. Public APIs have doc comments
5. Feature is accessible via CLI or HTTP API
6. UI changes are functional in the browser
