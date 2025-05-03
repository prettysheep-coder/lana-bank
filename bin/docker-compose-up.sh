#!/usr/bin/env bash
set -euo pipefail

BASE=docker-compose.yml
OVERRIDE=docker-compose.docker.yml   # contains the extra_hosts entry

# Prefer podman when both are installed
ENGINE=$(command -v podman >/dev/null 2>&1 && echo podman || echo docker)

FILES=(-f "$BASE")
[ "$ENGINE" = docker ] && FILES+=(-f "$OVERRIDE")   # always safe on Docker ≥ 20.10

exec "$ENGINE" compose "${FILES[@]}" up --wait -d "$@"