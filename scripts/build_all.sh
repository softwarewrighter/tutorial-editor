#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

echo "==> Building all components"
for d in "$ROOT_DIR"/components/*/; do
    name=$(basename "$d")
    echo "  -> Building $name"
    (cd "$d" && cargo build) || echo "  [WARN] $name failed"
done

echo "==> Build complete"
