#!/usr/bin/env bash
set -euo pipefail

# pick the first engine found in $PATH
ENGINE=$(command -v podman >/dev/null 2>&1 && echo podman || echo docker)

# run the chosen engine
"$ENGINE" compose -f docker-compose.yml down -t 1
