#!/usr/bin/env bash
set -euo pipefail

echo "--- Checking for Podman (via nix) ---"
command -v podman
echo "--- Podman check done ---"
command -v podman-compose
echo "--- Podman-compose check done ---"

echo "--- Testing Podman basic functionality ---"
podman info || echo "Warning: 'podman info' failed."
echo "--- Podman info done ---"

echo "--- Starting Dependencies with Podman Compose ---"
cd dev
podman-compose -f docker-compose.yml up -d integration-deps
cd .. # Back to repo root
echo "--- Podman-compose up done ---"

echo "--- Waiting for dependencies (sleep 20s) ---"
sleep 20
echo "--- Wait done ---"

# TODO: Implement a more robust wait (e.g., check pg_isready)

echo "--- Setting up Database ---"
cd lana/app
cargo sqlx migrate run
cd ../.. # Back to repo root
echo "--- DB Migration done ---"

echo "--- Running Integration Tests ---"
cargo nextest run --verbose --locked
echo "--- Tests done ---"

echo "--- Cleaning up dependencies ---"
cd dev
podman-compose -f docker-compose.yml down
cd .. # Back to repo root
echo "--- Cleanup done ---"

echo "--- All steps completed ---" 