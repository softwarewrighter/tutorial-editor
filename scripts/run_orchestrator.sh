#!/usr/bin/env bash
# Deprecated: Use start_backend.sh instead
exec "$(dirname "${BASH_SOURCE[0]}")/start_backend.sh" "$@"
