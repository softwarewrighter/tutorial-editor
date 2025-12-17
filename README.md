# Tutorial Editor

Avatar-orchestrated video pipeline for creating narrated educational videos.

## Quick Start

```bash
# Build all components
./scripts/build_all.sh

# Run the orchestrator server
./scripts/run_orchestrator.sh

# In another terminal, run the UI dev server
./scripts/dev_ui.sh
```

## API Endpoints

```bash
# Health check
curl http://127.0.0.1:8080/api/health

# List projects
curl http://127.0.0.1:8080/api/projects

# Create project
curl -X POST http://127.0.0.1:8080/api/projects \
     -H 'Content-Type: application/json' \
     -d '{"slug":"demo","title":"Demo Project"}'
```

## Scripts

| Script | Purpose |
|--------|---------|
| [scripts/build_all.sh](scripts/build_all.sh) | Build orchestrator and UI workbench |
| [scripts/run_orchestrator.sh](scripts/run_orchestrator.sh) | Run the backend server |
| [scripts/dev_ui.sh](scripts/dev_ui.sh) | Run Trunk dev server for UI |

## Documentation

| Document | Description |
|----------|-------------|
| [documentation/prd.md](documentation/prd.md) | Product requirements |
| [documentation/architecture.md](documentation/architecture.md) | System architecture |
| [documentation/design.md](documentation/design.md) | Technical design |
| [documentation/plan.md](documentation/plan.md) | Development plan |
| [documentation/status.md](documentation/status.md) | Current progress |
| [documentation/roadmap.md](documentation/roadmap.md) | Milestone roadmap |
| [documentation/process.md](documentation/process.md) | Development workflow |
| [documentation/tools.md](documentation/tools.md) | Recommended tools |

## Project Structure

```
tutorial-editor/
+-- config.toml              # Server and service configuration
+-- scripts/                  # Build and run scripts
+-- documentation/            # Project documentation
+-- components/
    +-- orchestrator/         # Backend workspace
    |   +-- crates/
    |       +-- orchestrator-cli/    # CLI entrypoint
    |       +-- orchestrator-core/   # Domain and services
    |       +-- orchestrator-http/   # HTTP API (Warp)
    |       +-- orchestrator-db/     # SQLite storage
    +-- ui_workbench/         # Frontend workspace
        +-- crates/
            +-- ui-core/      # Shared DTOs
            +-- ui-app/       # Yew/WASM application
```

## Configuration

Edit `config.toml` to configure:
- Server bind address and port
- SQLite database path
- Assets storage directory
- External service endpoints (LLM, TTS, avatar pipeline, MCP)

## Requirements

- Rust (2024 Edition)
- Trunk (`cargo install trunk`) for UI development

## License

See [LICENSE](LICENSE) and [COPYRIGHT](COPYRIGHT).
