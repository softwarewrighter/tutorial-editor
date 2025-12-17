#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

echo "==> Building orchestrator workspace"
(cd "$ROOT_DIR/components/orchestrator" && cargo build)

echo "==> Building UI workbench workspace (core + app)"
(cd "$ROOT_DIR/components/ui_workbench" && cargo build)

echo "Done."
