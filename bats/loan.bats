#!/usr/bin/env bats

load "helpers"

setup_file() {
  start_server
}

teardown_file() {
  stop_server
}

@test "loan: can create loan terms" {

  exec_admin_graphql 'term-values-create'
  echo $(graphql_output '.data.termValuesCreate')
  terms_id=$(graphql_output '.data.termValuesCreate.terms.termsId')
  [[ "$terms_id" != "null" ]] || exit 1

}
