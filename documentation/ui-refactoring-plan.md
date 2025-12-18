# UI Component Refactoring Plan

## Current State: 32 Warnings, 0 Failures

### Progress Log
| Date | Action | Before | After | Delta |
|------|--------|--------|-------|-------|
| Start | Initial state | 36 | 36 | 0 |
| Phase 0 | Created ui-macros | 36 | 36 | 0 |
| Phase 3 | Refactored ui-asset with macros | 36 | 35 | -1 |
| Phase 3 | Refactored ui-scene with macros | 35 | 34 | -1 |
| Phase 4 | Removed stale mod declarations | 34 | 33 | -1 |
| Phase 4 | Refactored ui-shell callbacks.rs (5→2) | 33 | 33 | 0 |
| Phase 4 | Refactored ui-shell callbacks2.rs (7→4) | 33 | 32 | -1 |
| Phase 4 | Split ui-shell → ui-app + ui-shell | 32 | 32 | 0 |

### Component Status After Split
| Component | Modules | Function Warns | LOC Warns | Status |
|-----------|---------|----------------|-----------|--------|
| ui-shell | 1 | 0 | 0 | ALL PASS |
| ui-app | 7 | 1 (render 5) | 2 | 4 warnings |
| ui-scene-detail | 3 | 1 (asset_callbacks 6) | 3 | 4 warnings |
| ui-asset | 3 | 0 | 1 | 1 warning |
| ui-scene | 4 | 0 | 1 | 1 warning |
| ui-api | 4 | 0 | 2 | 2 warnings |
| ui-project | 3 | 0 | 0 | ALL PASS |
| ui-core | 2 | 0 | 0 | ALL PASS |
| ui-macros | 3 | 0 | 0 | ALL PASS |

### Key Lessons Learned

**1. Never copy code - always move.** Attempted to create ui-app by copying from ui-shell,
which increased warnings from 34 to 40. Reverted. Correct approach: apply macros in-place
first, only restructure after in-place optimization is exhausted.

**2. Inlining can cause LOC regressions.** Attempted to reduce render.rs from 5 to 3 functions
by inlining private helpers. This eliminated the function count warning but added 2 LOC warnings.
Net effect: +1 warning. Reverted. Sometimes separate helper functions are better.

**3. Validate after EVERY change.** Always run `sw-checklist 2>&1 | grep Summary` after each
refactoring step. If warning count increases, revert immediately.

## Design Principles

### 1. Rust Macros for Repetitive Patterns

**Callback Builder Macro** (eliminates ~15 similar functions):
```rust
// Before: 5 lines per simple callback
fn build_show_project_form(show: &UseStateHandle<bool>) -> Callback<MouseEvent> {
    let show = show.clone();
    Callback::from(move |_| show.set(true))
}

// After: 1 line
state_callback!(show, true, MouseEvent)
```

**Form Field Macro** (eliminates ~60 lines in asset_form.rs):
```rust
// Before: 15 lines per text field
fn render_name_field(name: &UseStateHandle<String>) -> Html { ... }

// After: 1 line
text_field!(name, "Name", "asset-name", required = true)
```

**Async Callback Macro** (eliminates ~20 lines per async handler):
```rust
// Before: verbose async spawn
fn build_project_submit(...) -> Callback<(String, String)> { ... }

// After: declarative
async_submit!(
    inputs: (slug: String, title: String),
    api: create_project(&slug, &title),
    on_success: [show.set(false), refresh.increment()]
)
```

### 2. Pure Functions (Extract Side Effects)

**Pattern: Separate Pure Transform from Impure Action**
```rust
// Pure: transform data
fn build_asset_dto(props: &AssetFormProps, state: &FormState) -> AssetDto {
    AssetDto { /* pure construction */ }
}

// Impure: handle event
fn handle_submit(e: SubmitEvent, props: &AssetFormProps, state: &FormState) {
    e.prevent_default();
    let dto = build_asset_dto(props, state); // call pure function
    props.on_save.emit(dto);
}
```

### 3. Loose Coupling via Traits

**Define Callback Traits for Components**:
```rust
// In ui-core
pub trait FormCallbacks<T> {
    fn on_save(&self) -> &Callback<T>;
    fn on_cancel(&self) -> &Callback<()>;
}

pub trait ListCallbacks<T> {
    fn on_add(&self) -> &Callback<()>;
    fn on_edit(&self) -> &Callback<T>;
    fn on_delete(&self) -> &Callback<i64>;
}
```

### 4. Design Patterns

| Pattern | Usage | Benefit |
|---------|-------|---------|
| Builder | Callback construction | Type-safe callback creation |
| Factory | Form field generation | Consistent field rendering |
| Strategy | Validation rules | Pluggable validation |
| Composite | Component hierarchy | Consistent rendering |

### 5. New Component: ui-macros

Create `components/ui-macros/` with:
- `callback_macros.rs` - state_callback!, async_submit!
- `form_macros.rs` - text_field!, select_field!, textarea_field!
- `html_macros.rs` - common HTML patterns

This reduces code volume by ~40% and automatically reduces function counts

### Warning Breakdown by Type

| Type | Count | Threshold |
|------|-------|-----------|
| Function LOC | 14 | >25 lines |
| Module Function Count | 20 | >4 functions |
| Crate Module Count | 1 | >4 modules |
| Binary Freshness | 1 | not installed |

## UI Component Warnings (16)

### Function LOC Warnings (6)
| Component | File | Function | Lines | Fix |
|-----------|------|----------|-------|-----|
| ui-api | asset.rs | create_asset | 29 | Extract request builder |
| ui-api | asset.rs | update_asset | 26 | Extract request builder |
| ui-asset | asset_list.rs | render_asset_item | 29 | Extract HTML fragments |
| ui-scene | scene_edit_form.rs | scene_edit_form | 32 | Extract form sections |
| ui-scene-detail | asset_callbacks.rs | build_on_save | 34 | Extract async handler |
| ui-scene-detail | scene_detail.rs | scene_detail | 38 | Extract setup code |
| ui-scene-detail | scene_detail.rs | render_assets_section | 29 | Extract form/list renderers |
| ui-shell | mod.rs | app | 32 | Extract state setup |
| ui-shell | callbacks2.rs | build_save_scene | 29 | Extract async handler |

### Module Function Count Warnings (9)
| Component | File | Functions | Fix |
|-----------|------|-----------|-----|
| ui-asset | asset_form.rs | 7 | Split into form_state.rs + form_render.rs |
| ui-scene | scene_edit_form.rs | 5 | Split into form_state.rs + form_render.rs |
| ui-scene-detail | asset_callbacks.rs | 6 | Split into callbacks_crud.rs + callbacks_nav.rs |
| ui-shell | callbacks.rs | 5 | Split into callbacks_nav.rs + callbacks_show.rs |
| ui-shell | callbacks2.rs | 7 | Split into callbacks_project.rs + callbacks_scene.rs |
| ui-shell | render.rs | 5 | Split into render_sidebar.rs + render_content.rs |

### Crate Module Count Warnings (1)
| Component | Modules | Fix |
|-----------|---------|-----|
| ui-shell | 7 | Split into ui-shell + ui-app components |

## Target Architecture

```
components/
  ui-core/           # Shared DTOs (2 modules) - NO WARNINGS
  ui-api/            # HTTP client (4 modules after split) - NO WARNINGS
    project.rs       # 2 functions
    scene.rs         # 3 functions
    asset_create.rs  # 2 functions (split from asset.rs)
    asset_update.rs  # 2 functions (split from asset.rs)

  ui-project/        # Project components (3 modules) - NO WARNINGS

  ui-scene/          # Scene components (4 modules after split) - NO WARNINGS
    scene_form.rs    # ≤4 functions
    scene_list.rs    # ≤4 functions
    scene_edit_state.rs   # 2-3 functions (split from scene_edit_form)
    scene_edit_render.rs  # 2-3 functions (split from scene_edit_form)

  ui-asset/          # Asset components (4 modules after split) - NO WARNINGS
    asset_list.rs    # ≤4 functions
    asset_form_state.rs   # 3-4 functions (split from asset_form)
    asset_form_render.rs  # 3-4 functions (split from asset_form)

  ui-scene-detail/   # Scene detail view (4 modules after split) - NO WARNINGS
    scene_view.rs         # 2-3 functions (split from scene_detail)
    scene_render.rs       # 2-3 functions (split from scene_detail)
    callbacks_crud.rs     # 3 functions (split from asset_callbacks)
    callbacks_nav.rs      # 3 functions (split from asset_callbacks)

  ui-app/            # Main app logic (NEW - 4 modules) - NO WARNINGS
    app.rs           # Main App component, ≤4 functions
    hooks.rs         # Data fetching hooks
    callbacks_project.rs  # 3-4 functions
    callbacks_scene.rs    # 3-4 functions

  ui-shell/          # Entry point (3 modules after split) - NO WARNINGS
    main.rs          # Entry point
    footer.rs        # Footer component
    render.rs        # Render helpers (split if >4 functions)
```

## Orchestrator Warnings (19)

### Function LOC Warnings (5)
| Crate | File | Function | Lines | Fix |
|-------|------|----------|-------|-----|
| orchestrator-avatar-pipeline | ops_impl.rs | avatar_pipeline | 29 | Extract step functions |
| orchestrator-cli | main.rs | run_server | 32 | Extract setup functions |
| orchestrator-db-asset | write.rs | create_asset | 29 | Extract SQL builder |
| orchestrator-db-scene | write.rs | create_scene | 27 | Extract SQL builder |
| orchestrator-script | generation.rs | generate_project_script | 42 | Extract prompt builder, JSON parser |

### Module Function Count Warnings (14)
| Crate | File | Functions | Fix |
|-------|------|-----------|-----|
| orchestrator-adapter-avatar | client.rs | 5 | Split HTTP methods |
| orchestrator-avatar-pipeline | ops_impl.rs | 7 | Split by operation type |
| orchestrator-avatar-pipeline | ops_trait.rs | 6 | OK (traits can have more) |
| orchestrator-avatar-service | ops_impl.rs | 5 | Split by operation type |
| orchestrator-avatar-service | ops_trait.rs | 5 | OK (traits can have more) |
| orchestrator-http-asset | handler.rs | 5 | Split CRUD handlers |
| orchestrator-http-asset | routes.rs | 6 | Split CRUD routes |
| orchestrator-http-avatar | handler.rs | 6 | Split by avatar operation |
| orchestrator-http-avatar | routes.rs | 7 | Split by avatar operation |
| orchestrator-http-scene | routes.rs | 5 | Split CRUD routes |
| orchestrator-ops-asset | ops_impl.rs | 6 | Split CRUD operations |
| orchestrator-ops-asset | ops_trait.rs | 6 | OK (traits can have more) |
| orchestrator-ops-scene | ops_impl.rs | 5 | Split CRUD operations |
| orchestrator-ops-scene | ops_trait.rs | 5 | OK (traits can have more) |

## Revised Execution Order (Macro-First, Bottom-Up)

### Phase 0: Foundation - Create ui-macros Component
Create `components/ui-macros/` with procedural and declarative macros:

```
ui-macros/
  Cargo.toml (proc-macro = true for procedural macros)
  crates/ui-macros/src/
    lib.rs          # Export all macros
    callback.rs     # state_callback!, async_callback!
    form.rs         # text_field!, select_field!, textarea_field!
```

**Expected Impact**: Foundation for all other components

### Phase 1: Core Types (no dependencies)
1. **ui-core** - Add callback traits (FormCallbacks, ListCallbacks)
   - Already 2 modules, add traits.rs (3 modules total)
   - Expected warnings: 0

### Phase 2: API Layer (depends on ui-core)
2. **ui-api** - Refactor using pure functions
   - Extract `build_*_request` helpers for LOC reduction
   - Split asset.rs → asset.rs (2 functions) using macro
   - Expected warnings: 0

### Phase 3: UI Components (depends on ui-core, ui-macros)
3. **ui-project** - Already clean (3 modules)
4. **ui-asset** - Refactor with form_field! macro
   - asset_form.rs: 7 functions → 3 functions via macros
   - Expected warnings: 0
5. **ui-scene** - Refactor with form_field! macro
   - scene_edit_form.rs: 5 functions → 2-3 via macros
   - Expected warnings: 0

### Phase 4: Composite Components (depends on Phase 3)
6. **ui-scene-detail** - Refactor with callback macros
   - Use async_callback! for build_on_save
   - Split large functions, not modules
   - Expected warnings: 0
7. **ui-shell** - Restructure
   - Create ui-app component for app logic
   - ui-shell: main.rs, footer.rs (2 modules)
   - ui-app: mod.rs, hooks.rs + macro-based callbacks (3 modules)
   - Expected warnings: 0

### Phase 5: Orchestrator (parallel track)
8. Fix orchestrator function LOC issues:
   - Extract helper functions (pure transforms)
9. Fix orchestrator function count issues:
   - Split modules by operation type (create, read, update, delete)
10. Final cleanup

## Macro Implementation Details

### callback.rs - Declarative Macros

```rust
/// Creates a callback that sets a state handle to a value
#[macro_export]
macro_rules! state_callback {
    ($state:ident, $value:expr, $event:ty) => {{
        let state = $state.clone();
        Callback::from(move |_: $event| state.set($value))
    }};
}

/// Creates an async callback that calls an API and updates state on success
#[macro_export]
macro_rules! async_callback {
    (
        state: [$($state:ident),+],
        action: $action:expr,
        on_success: { $($success:stmt);* }
    ) => {{
        $(let $state = $state.clone();)+
        Callback::from(move |_| {
            $(let $state = $state.clone();)+
            wasm_bindgen_futures::spawn_local(async move {
                if $action.is_ok() {
                    $($success)*
                }
            });
        })
    }};
}
```

### form.rs - Declarative Macros

```rust
/// Creates a text input field with state binding
#[macro_export]
macro_rules! text_field {
    ($state:ident, $label:expr, $id:expr) => {{
        let oninput = {
            let state = $state.clone();
            Callback::from(move |e: InputEvent| {
                if let Some(input) = e.target()
                    .and_then(|t| t.dyn_into::<HtmlInputElement>().ok()) {
                    state.set(input.value());
                }
            })
        };
        html! {
            <div class="form-field">
                <label for={$id}>{ $label }</label>
                <input type="text" id={$id} value={$state.to_string()} {oninput} />
            </div>
        }
    }};
    ($state:ident, $label:expr, $id:expr, required = true) => {{
        // Same but with required=true on input
    }};
}
```

## Expected Warning Reduction

| Phase | Before | After | Change |
|-------|--------|-------|--------|
| Phase 0 | 36 | 36 | +0 (foundation) |
| Phase 1 | 36 | 36 | +0 (types only) |
| Phase 2 | 36 | 34 | -2 (ui-api LOC) |
| Phase 3 | 34 | 28 | -6 (ui-asset, ui-scene) |
| Phase 4 | 28 | 18 | -10 (ui-shell, ui-scene-detail) |
| Phase 5 | 18 | 0 | -18 (orchestrator) |

## Validation After Each Phase

After each phase, run:
```bash
cd /Users/mike/github/softwarewrighter/tutorial-editor && sw-checklist 2>&1 | grep "Summary"
```

Expected progression:
- Phase 1: 36 warnings → 36 (no change, ui-core already clean)
- Phase 2: 36 → ~28 (fixing 8 warnings)
- Phase 3: ~28 → ~16 (fixing ui-shell and ui-scene-detail)
- Phase 4: ~16 → 0 (fixing orchestrator)

## Key Constraints

1. **Never merge modules** if it increases function count
2. **Never merge functions** if it increases function LOC
3. **Always split** to reduce metrics
4. **Validate after each change** - warning count must decrease or stay same

## Function Split Patterns

### Pattern 1: Extract Helper Function
Before (29 lines):
```rust
fn create_asset(...) {
    // 29 lines of mixed logic
}
```
After:
```rust
fn create_asset(...) {
    let request = build_asset_request(...);  // extracted
    send_request(request).await
}

fn build_asset_request(...) -> Request {
    // 15 lines
}
```

### Pattern 2: Split Module by Operation Type
Before (7 functions in one module):
```rust
// asset_form.rs
fn asset_form() { }
fn handle_name_change() { }
fn handle_type_change() { }
fn handle_url_change() { }
fn handle_path_change() { }
fn handle_submit() { }
fn handle_cancel() { }
```
After:
```rust
// asset_form_handlers.rs (4 functions)
fn handle_name_change() { }
fn handle_type_change() { }
fn handle_url_change() { }
fn handle_path_change() { }

// asset_form.rs (3 functions)
fn asset_form() { }
fn handle_submit() { }
fn handle_cancel() { }
```

### Pattern 3: Split Crate into Multiple Crates
Before:
```
ui-shell/
  src/
    main.rs
    footer.rs
    app/
      mod.rs
      callbacks.rs
      callbacks2.rs
      hooks.rs
      render.rs
```
After:
```
ui-shell/
  src/
    main.rs
    footer.rs

ui-app/
  src/
    app.rs
    hooks.rs
    callbacks_project.rs
    callbacks_scene.rs
```
