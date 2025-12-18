#!/usr/bin/env bash
# Start the UI frontend server on port 1146
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
UI_DIR="$ROOT_DIR/components/ui-shell/crates/ui-shell"

echo "Starting UI frontend..."
echo "  URL: http://127.0.0.1:1146"
echo "  API proxy: http://127.0.0.1:1145/api/"
echo ""

cd "$UI_DIR"
trunk serve
