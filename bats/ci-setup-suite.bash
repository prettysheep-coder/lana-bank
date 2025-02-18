#!/usr/bin/env bash

export REPO_ROOT=$(git rev-parse --show-toplevel)
source "${REPO_ROOT}/bats/helpers.bash"

setup_suite() {
  start_server
}

teardown_suite() {
  stop_server
}
