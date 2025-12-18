#!/usr/bin/env bash
# Start the orchestrator backend server on port 1145
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CONFIG_PATH="${1:-$ROOT_DIR/config.toml}"

echo "Starting orchestrator backend..."
echo "  Config: $CONFIG_PATH"
echo "  Port: 1145"
echo ""

cd "$ROOT_DIR"
cargo run --manifest-path components/orch-cli/Cargo.toml -p orchestrator-cli -- -c "$CONFIG_PATH"
