# Design Document

## System Design

### Service Integration Model

The orchestrator acts as a hub connecting multiple external services:

```
+-------------------+
|   Orchestrator    |
|    (Rust CLI)     |
+--------+----------+
         |
    +----+----+----+----+----+
    |    |    |    |    |    |
    v    v    v    v    v    v
  LLM  TTS Avatar Lip  BG   MCP/
 Cloud      Sync  Sync Rem  Playwright
```

Each service is accessed via HTTP APIs defined in `config.toml`.

### Trait-Based Adapters

Core logic depends on traits, not concrete implementations:

```rust
// In orchestrator-core::ports
#[async_trait]
pub trait LlmClient: Send + Sync {
    async fn generate_script(&self, prompt: &str) -> Result<String>;
}

#[async_trait]
pub trait AvatarPipelineClient: Send + Sync {
    async fn generate_video(&self, image_path: &Path) -> Result<PathBuf>;
    async fn lip_sync(&self, video: &Path, audio: &Path) -> Result<PathBuf>;
}

#[async_trait]
pub trait TtsClient: Send + Sync {
    async fn synthesize(&self, text: &str) -> Result<PathBuf>;
}

#[async_trait]
pub trait McpClient: Send + Sync {
    async fn capture_scene(&self, target: &SceneTarget) -> Result<PathBuf>;
}
```

Adapters in separate crates implement these traits by calling real services.

### Data Model

**Projects**: Top-level container for a video production.

```
projects
+----+------+-------+----------+-------------+------------+------------+
| id | slug | title | subtitle | description | created_at | updated_at |
+----+------+-------+----------+-------------+------------+------------+
```

**Scenes**: Ordered segments within a project.

```
scenes
+----+------------+-------+-------+----------+------------+------------+
| id | project_id | order | title | duration | created_at | updated_at |
+----+------------+-------+-------+----------+------------+------------+
```

**Assets**: Binary files associated with projects/scenes.

```
assets
+----+------------+----------+------+------+-------------+---------------+
| id | project_id | scene_id | kind | path | duration_ms | metadata_json |
+----+------------+----------+------+------+-------------+---------------+
```

Asset kinds: `avatar_image`, `avatar_video`, `tts_audio`, `screen_capture`, `composited_scene`, `overlay_graphic`, `final_video`

### Workflow Orchestration

A typical video generation workflow:

1. **Script Generation**
   - Input: Project status, user prompt
   - LLM generates: dialog, scene descriptions, overlay text
   - Output: Script with scene list stored in DB

2. **Asset Generation** (per scene)
   - Generate TTS audio from dialog
   - Generate/select avatar image
   - Create avatar video (head movement, blinking)
   - Stretch video to audio length
   - Lip sync avatar to audio
   - Remove background

3. **Scene Capture**
   - MCP/Playwright connects to target via Chrome Remote Desktop
   - Execute scripted interactions
   - Record screen capture
   - Store capture path in assets

4. **Composition**
   - Composite avatar over screen capture
   - Position based on facing direction and text avoidance
   - Add overlay graphics
   - Render final scene video

5. **Assembly**
   - Concatenate scenes with transitions
   - Add intro/outro
   - Render final video

### UI Component Design

**Project List View**
- Display all projects with status indicators
- Create new project button
- Click to open project detail

**Project Detail View**
- Project metadata (title, description)
- Scene list with thumbnails
- Add/reorder/delete scenes

**Scene Editor View**
- Preview pane showing composite
- Avatar selection panel
- Avatar position controls (drag or preset positions)
- Overlay text editor
- Overlay position controls
- Timeline scrubber

**Controls**
- Avatar facing direction selector
- Text avoidance zones (mark areas avatar should avoid)
- Overlay templates (title, subtitle, step indicator)

### File Organization

```
./data/
  video_orchestrator.sqlite3

./assets/
  projects/
    {project-slug}/
      avatars/
        avatar-001.png
        avatar-001-video.mp4
        avatar-001-lipsync.mp4
      audio/
        scene-001-tts.wav
      captures/
        scene-001-capture.mp4
      composites/
        scene-001-final.mp4
      output/
        final-video.mp4
```

### Configuration Schema

```toml
[server]
bind_address = "127.0.0.1"
port = 8080

[storage]
sqlite_path = "./data/video_orchestrator.sqlite3"
assets_root = "./assets"

[services.llm]
cloud_primary = "https://api.example-llm.com"
local_ollama = "http://localhost:11434"
local_llamacpp = "http://localhost:8081"

[services.avatar]
image_to_video = "http://avatar-pipeline:9000"
lip_sync = "http://lip-sync:9100"
background_removal = "http://bg-removal:9200"

[services.tts]
primary = "http://tts:9300"

[services.mcp]
playwright_hub = "http://playwright-mcp:9400"
```

### Error Handling Strategy

- Service failures return structured errors with retry hints
- Long-running operations use polling with status endpoints
- Asset generation failures don't block other scenes
- UI shows per-scene status with error details
- Failed assets can be regenerated individually
