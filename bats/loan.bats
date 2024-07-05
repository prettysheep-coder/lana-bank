#!/usr/bin/env bats

load "helpers"

setup_file() {
  start_server
}

teardown_file() {
  stop_server
}

@test "loan: can create loan terms" {

  exec_admin_graphql 'current-terms-update' 
  terms_id=$(graphql_output '.data.currentTermsUpdate.terms.termsId')
  [[ "$terms_id" != "null" ]] || exit 1

  username=$(random_uuid)
  token=$(create_user)
  cache_value "alice" "$token"

  exec_graphql 'alice' 'me'
  user_id=$(graphql_output '.data.me.userId')
  btc_address=$(graphql_output '.data.me.btcDepositAddress')
  ust_address=$(graphql_output '.data.me.ustDepositAddress')

  variables=$(
    jq -n \
      --arg address "$btc_address" \
    '{
       address: $address,
       amount: "10",
       currency: "BTC"
    }'
  )
  exec_cala_graphql 'simulate-deposit' "$variables"

  variables=$(
    jq -n \
    --arg userId "$user_id" \
    '{
      input: {
        userId: $userId,
        desiredPrincipal: 10000
      }
    }'
  )

  exec_admin_graphql 'loan-create' "$variables"
  echo $(graphql_output)
  loan_id=$(graphql_output '.data.loanCreate.loan.loanId')
  [[ "$loan_id" != "null" ]] || exit 1

  exec_cala_graphql 'simulate-deposit' "$variables"

}
