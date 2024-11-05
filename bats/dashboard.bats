#!/usr/bin/env bats

load "helpers"

setup_file() {
  start_server
}

teardown_file() {
  stop_server
}

@test "dashboard: counts facilities" {
  customer_id=$(create_customer)

  exec_admin_graphql 'dashboard'
  pending_facilities=$(graphql_output '.data.dashboard.pendingFacilities')
  [[ "$pending_facilities" != "null" ]] || exit 1
}
