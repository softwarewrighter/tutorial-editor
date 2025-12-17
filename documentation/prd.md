# Product Requirements Document (PRD)

## Product Overview

**Product Name**: Tutorial Editor (Avatar-Orchestrated Video Pipeline)

**Purpose**: Automate creation of narrated educational YouTube videos (Twitch-like live coding, but scripted and edited for brevity and clarity).

**Target Users**: Content creators producing technical tutorial videos.

## Problem Statement

Creating high-quality educational videos requires:
- Manual orchestration of multiple tools (avatar generation, TTS, lip sync, video editing)
- Tedious scene capture and editing workflows
- Repetitive composition of overlays, avatars, and screen recordings

Current process is time-consuming and error-prone.

## Solution

An AI-orchestrated pipeline that:
1. Generates scripts from project status/ideas
2. Automates scene capture via MCP/Playwright
3. Produces avatar videos with lip-synced narration
4. Composes final videos with overlays and graphics
5. Provides a web-based editor for fine-tuning

## Core Requirements

### R1: Script and Content Generation
- AI reads project status and suggests video topics
- User refines with specific goals/actions
- System generates script with dialog and scene descriptions
- Generates overlay text (titles, subtitles, bullet points)

### R2: Avatar Pipeline Integration
- Generate avatar images (narrator)
- Image-to-video with head movements and blinking
- TTS with cloned voice
- Video stretching to match audio length
- Lip sync avatar to narration
- Background removal for compositing

### R3: Scene Capture Automation
- MCP/Playwright integration for browser automation
- Chrome Remote Desktop views of target hosts
- Capture: terminal, CLI, editor, IDE, browser, desktop apps, mobile emulators
- Track captured clips and associate with scenes

### R4: Video Composition
- Composite avatar over screen captures
- Position avatar based on facing direction (left-facing -> right side, right-facing -> left side)
- Add graphic overlays (step indicators, titles)
- Generate 3-7 minute videos with beginning, middle, end

### R5: Preview and Edit Mode
- Web-based editor (Rust/Yew/WASM)
- Select replacement avatars
- Drag/drop avatar positioning
- Move avatar to avoid obscuring text
- Drag/drop graphic overlay positioning
- Per-scene and per-project preview

### R6: Configuration and Storage
- CLI with `-c/--config-file` (default: `./config.toml`)
- SQLite for metadata (projects, scenes, assets)
- Filesystem storage for large binaries (video, audio, images)
- Neither SQLite DB nor assets tracked by git

## Non-Functional Requirements

### NF1: Technology Stack
- Rust 2024 Edition exclusively
- No TypeScript, no Python in project tree
- Minimal to no custom JavaScript
- Yew/WASM for web UI

### NF2: Architecture
- Multi-component workspace layout
- Small, composable units (few funcs per module, few modules per crate)
- Clean layering: core defines traits, adapters implement them
- Configurable service endpoints

### NF3: Output Specifications
- Target video length: 5 minutes (+/- 2 minutes)
- Multiple output profiles (YouTube standard, shorts)
- Transparent avatar for flexible compositing

## Success Criteria

1. Generate a complete tutorial video from idea to final render with minimal manual intervention
2. Edit and preview videos in the web UI without external tools
3. Produce videos consistently within target length range
4. Support multiple service backends via configuration
