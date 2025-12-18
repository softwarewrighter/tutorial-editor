# Physical Design: Tutorial Editor

This document defines the target physical architecture organized by functional layers. Each component is a separate Cargo workspace in `components/`.

## Target Component Architecture

### Layer 1: Shared Foundation

| Component | Purpose | Crates |
|-----------|---------|--------|
| `domain` | Shared domain types (Project, Scene, Asset, Config) | domain |
| `common` | Shared utilities, error types | common |
| `macros` | Proc macros for reducing boilerplate | macros |

### Layer 2: Ports (Trait Definitions)

| Component | Purpose | Crates |
|-----------|---------|--------|
| `ports` | Repository and service client traits | ports-project, ports-scene, ports-asset, ports-services |

### Layer 3: Infrastructure Implementations

| Component | Purpose | Crates |
|-----------|---------|--------|
| `db` | SQLite repository implementations | db-core, db-project, db-scene, db-asset |
| `adapters` | HTTP service client implementations | adapter-llm, adapter-tts, adapter-avatar, adapter-mcp |

### Layer 4: Application Logic

| Component | Purpose | Crates |
|-----------|---------|--------|
| `app` | OrchestratorApp container and operations | app, ops-project, ops-scene, ops-asset |
| `avatar` | Avatar pipeline orchestration | avatar-service, avatar-pipeline |
| `script` | Script generation | script |

### Layer 5: HTTP Layer

| Component | Purpose | Crates |
|-----------|---------|--------|
| `http` | Warp HTTP server and routes | http-core, http-health, http-project, http-scene, http-asset, http-avatar, http-script |

### Layer 6: Entry Points

| Component | Purpose | Crates |
|-----------|---------|--------|
| `cli` | CLI binary | cli |

### Layer 7: UI (WebAssembly)

| Component | Purpose | Crates |
|-----------|---------|--------|
| `ui-core` | Shared UI DTOs | ui-core |
| `ui-api` | HTTP API client | ui-api |
| `ui-project` | Project UI components | ui-project |
| `ui-scene` | Scene UI components | ui-scene |
| `ui-asset` | Asset UI components | ui-asset |
| `ui-shell` | Main app shell | ui-shell |

### Layer 8: Quality

| Component | Purpose | Crates |
|-----------|---------|--------|
| `integration-test` | End-to-end tests | integration-test |

---

## Dependency Graph (Target)

```
                          cli
                           │
                         http ─────────────────┐
                           │                   │
        ┌──────────────────┼──────────────────┐│
        │                  │                  ││
    http-project      http-scene         http-asset
        │                  │                  │
    ops-project        ops-scene          ops-asset
        │                  │                  │
        └──────────────────┼──────────────────┘
                           │
                          app
                           │
        ┌──────────────────┼──────────────────┐
        │                  │                  │
    ports-project     ports-scene        ports-asset
        │                  │                  │
        └──────────────────┼──────────────────┘
                           │
                        domain
                           │
                        common
```

---

## Current State vs Target

### Current (2 components, 31 crates total)

| Component | Crates | Failures | Warnings |
|-----------|--------|----------|----------|
| orchestrator | 29 | 0 | 20 |
| ui_workbench | 2 | 1 | 16 |
| **Total** | **31** | **1** | **36** |

### Target (~15 components, ~45 crates)

Each component would have:
- 1-4 crates per component
- 1-4 modules per crate
- 1-4 functions per module

This provides room for growth while staying well under sw-checklist limits.

---

## Migration Strategy

### Phase 1: Fix Immediate FAILURE
1. Split ui_workbench into: ui-core, ui-api, ui-project, ui-scene, ui-asset, ui-shell

### Phase 2: Reorganize orchestrator by Layer
2. Extract `domain` component (shared types)
3. Extract `ports` component (trait definitions)
4. Extract `db` component (SQLite implementations)
5. Extract `adapters` component (HTTP service clients)
6. Extract `app` component (business logic)
7. Extract `http` component (routes and handlers)
8. Extract `cli` component (entry point)

### Phase 3: Add Infrastructure
9. Create `common` component (shared utilities)
10. Create `macros` component (proc macros for boilerplate)
11. Create `integration-test` component

### Phase 4: Reduce Warnings
12. Fix all function LOC warnings (extract helpers)
13. Fix all module function count warnings (split modules)
14. Verify 0 warnings with sw-checklist

---

## Module Limits (sw-checklist)

| Metric | Target | Warning | Failure |
|--------|--------|---------|---------|
| Crates per component | ≤4 | - | - |
| Modules per crate | ≤4 | >4 | >7 |
| Functions per module | ≤4 | >4 | >7 |
| Lines per function | ≤25 | >25 | >50 |

**Key insight:** More components with fewer crates each is better than fewer components with many crates.

---

## Design Principles

1. **Layers are components**: Each architectural layer is a separate component
2. **Data flows down**: Higher layers depend on lower layers, never reverse
3. **Components are small**: Target 1-4 crates per component
4. **Traits separate concerns**: Ports define interfaces, implementations are in separate components
5. **Macros reduce boilerplate**: Use proc macros to eliminate repetitive code
6. **Tests are separate**: Integration tests in their own component
