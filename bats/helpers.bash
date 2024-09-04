REPO_ROOT=$(git rev-parse --show-toplevel)
COMPOSE_PROJECT_NAME="${COMPOSE_PROJECT_NAME:-${REPO_ROOT##*/}}"

CACHE_DIR=${BATS_TMPDIR:-tmp/bats}/galoy-bats-cache
mkdir -p "$CACHE_DIR"

KRATOS_PUBLIC_ENDPOINT="http://localhost:4455"
GQL_PUBLIC_ENDPOINT="http://localhost:4455/graphql"
GQL_ADMIN_ENDPOINT="http://localhost:4455/admin/graphql"
GQL_CALA_ENDPOINT="http://localhost:2252/graphql"
NEXTAUTH_URL="http://localhost:4455/admin-panel/api/auth"
CALLBACK_URL="/admin-panel/profile"

LAVA_HOME="${LAVA_HOME:-.lava}"
export LAVA_CONFIG="${REPO_ROOT}/bats/lava.yml"
SERVER_PID_FILE="${LAVA_HOME}/server-pid"

reset_pg() {
  docker exec "${COMPOSE_PROJECT_NAME}-core-pg-1" psql $PG_CON -c "DROP SCHEMA public CASCADE"
  docker exec "${COMPOSE_PROJECT_NAME}-core-pg-1" psql $PG_CON -c "CREATE SCHEMA public"
  docker exec "${COMPOSE_PROJECT_NAME}-cala-pg-1" psql $PG_CON -c "DROP SCHEMA public CASCADE"
  docker exec "${COMPOSE_PROJECT_NAME}-cala-pg-1" psql $PG_CON -c "CREATE SCHEMA public"
}

server_cmd() {
  server_location="${REPO_ROOT}/target/debug/lava-core"
  if [[ ! -z ${CARGO_TARGET_DIR} ]]; then
    server_location="${CARGO_TARGET_DIR}/debug/lava-core"
  fi

  bash -c ${server_location} $@
}

start_server() {
  # Check for running server
  if [ -n "$BASH_VERSION" ]; then
    server_process_and_status=$(
      ps a | grep 'target/debug/lava-core' | grep -v grep
      echo ${PIPESTATUS[2]}
    )
  elif [ -n "$ZSH_VERSION" ]; then
    server_process_and_status=$(
      ps a | grep 'target/debug/lava-core' | grep -v grep
      echo ${pipestatus[3]}
    )
  else
    echo "Unsupported shell."
    exit 1
  fi
  exit_status=$(echo "$server_process_and_status" | tail -n 1)
  if [ "$exit_status" -eq 0 ]; then
    rm -f "$SERVER_PID_FILE"
    return 0
  fi

  # Start server if not already running
  background server_cmd >.e2e-logs 2>&1
  for i in {1..20}; do
    if head .e2e-logs | grep -q 'Starting graphql server on port'; then
      break
    elif head .e2e-logs | grep -q 'Connection reset by peer'; then
      stop_server
      sleep 1
      background server_cmd >.e2e-logs 2>&1
    else
      sleep 1
    fi
  done
}

stop_server() {
  if [[ -f "$SERVER_PID_FILE" ]]; then
    kill -9 $(cat "$SERVER_PID_FILE") || true
  fi
}

gql_query() {
  cat "$(gql_file $1)" | tr '\n' ' ' | sed 's/"/\\"/g'
}

gql_file() {
  echo "${REPO_ROOT}/bats/gql/$1.gql"
}

gql_admin_query() {
  cat "$(gql_admin_file $1)" | tr '\n' ' ' | sed 's/"/\\"/g'
}

gql_admin_file() {
  echo "${REPO_ROOT}/bats/admin-gql/$1.gql"
}

gql_cala_query() {
  cat "$(gql_cala_file $1)" | tr '\n' ' ' | sed 's/"/\\"/g'
}

gql_cala_file() {
  echo "${REPO_ROOT}/bats/cala-gql/$1.gql"
}

graphql_output() {
  echo $output | jq -r "$@"
}

exec_graphql() {
  local token_name=$1
  local query_name=$2
  local variables=${3:-"{}"}

  AUTH_HEADER="Authorization: Bearer $(read_value "$token_name")"

  if [[ "${BATS_TEST_DIRNAME}" != "" ]]; then
    run_cmd="run"
  else
    run_cmd=""
  fi

  ${run_cmd} curl -s \
    -X POST \
    ${AUTH_HEADER:+ -H "$AUTH_HEADER"} \
    -H "Content-Type: application/json" \
    -d "{\"query\": \"$(gql_query $query_name)\", \"variables\": $variables}" \
    "${GQL_PUBLIC_ENDPOINT}"
}

exec_admin_graphql() {
  local query_name=$1
  local variables=${2:-"{}"}

  if [[ "${BATS_TEST_DIRNAME}" != "" ]]; then
    run_cmd="run"
  else
    run_cmd=""
  fi

  ${run_cmd} curl -s \
    -X POST \
    -H "Content-Type: application/json" \
    -d "{\"query\": \"$(gql_admin_query $query_name)\", \"variables\": $variables}" \
    "${GQL_ADMIN_ENDPOINT}"
}
exec_cala_graphql() {
  local query_name=$1
  local variables=${2:-"{}"}

  if [[ "${BATS_TEST_DIRNAME}" != "" ]]; then
    run_cmd="run"
  else
    run_cmd=""
  fi

  ${run_cmd} curl -s \
    -X POST \
    ${AUTH_HEADER:+ -H "$AUTH_HEADER"} \
    -H "Content-Type: application/json" \
    -d "{\"query\": \"$(gql_cala_query $query_name)\", \"variables\": $variables}" \
    "${GQL_CALA_ENDPOINT}"
}

# Run the given command in the background. Useful for starting a
# node and then moving on with commands that exercise it for the
# test.
#
# Ensures that BATS' handling of file handles is taken into account;
# see
# https://github.com/bats-core/bats-core#printing-to-the-terminal
# https://github.com/sstephenson/bats/issues/80#issuecomment-174101686
# for details.
background() {
  "$@" 3>- &
  echo $!
}

# Taken from https://github.com/docker/swarm/blob/master/test/integration/helpers.bash
# Retry a command $1 times until it succeeds. Wait $2 seconds between retries.
retry() {
  local attempts=$1
  shift
  local delay=$1
  shift
  local i

  for ((i = 0; i < attempts; i++)); do
    run "$@"
    if [[ "$status" -eq 0 ]]; then
      return 0
    fi
    sleep "$delay"
  done

  echo "Command \"$*\" failed $attempts times. Output: $output"
  false
}

random_uuid() {
  if [[ -e /proc/sys/kernel/random/uuid ]]; then
    cat /proc/sys/kernel/random/uuid
  else
    uuidgen
  fi
}

cache_value() {
  echo $2 >${CACHE_DIR}/$1
}

read_value() {
  cat ${CACHE_DIR}/$1
}

KRATOS_PG_CON="postgres://dbuser:secret@localhost:5434/default?sslmode=disable"

getEmailCode() {
  local email="$1"
  local query="SELECT body FROM courier_messages WHERE recipient='${email}' ORDER BY created_at DESC LIMIT 1;"

  local result=$(psql $KRATOS_PG_CON -t -c "${query}")

  if [[ -z "$result" ]]; then
    echo "No message for email ${email}" >&2
    exit 1
  fi

  local code=$(echo "$result" | grep -Eo '[0-9]{6}' | head -n1)

  echo "$code"
}

generate_email() {
  echo "user$(date +%s%N)@example.com" | tr '[:upper:]' '[:lower:]'
}

create_user() {
  email=$(echo "user$(date +%s%N)@example.com" | tr '[:upper:]' '[:lower:]')

  flowId=$(curl -s -X GET \
    -H "Accept: application/json" \
    "$KRATOS_PUBLIC_ENDPOINT/self-service/registration/api" | jq -r '.id')

  response=$(curl -s -X POST "$KRATOS_PUBLIC_ENDPOINT/self-service/registration?flow=$flowId" \
    -H "Content-Type: application/json" \
    -d '{
    "method": "code",
    "traits": {
      "email": "'"$email"'"
    }
  }')

  code=$(getEmailCode "$email")

  verification_response=$(curl -s -X POST "$KRATOS_PUBLIC_ENDPOINT/self-service/registration?flow=$flowId" \
    -H "Content-Type: application/json" \
    -d '{
    "code": "'"$code"'",
    "method": "code",
    "traits": {
      "email": "'"$email"'"
    }
  }')

  # Extract and print the session token if needed
  token=$(echo $verification_response | jq -r '.session_token')
  echo $token
}

create_customer() {
  customer_email=$(generate_email)

  variables=$(
    jq -n \
    --arg email "$customer_email" \
    '{
      input: {
        email: $email
        }
      }'
  )

  exec_admin_graphql 'customer-create' "$variables"
  customer_id=$(graphql_output .data.customerCreate.customer.customerId)
  [[ "$customer_id" != "null" ]] || exit 1
  echo $customer_id
}

add() {
  sum=0
  for num in "$@"; do
    sum=$(echo "scale=2; $sum + $num" | bc)
  done
  echo $sum
}

sub() {
  diff=$1
  shift
  for num in "$@"; do
    diff=$(echo "scale=2; $diff - $num" | bc)
  done
  echo $diff
}

assert_balance_sheet_balanced() {
  variables=$(
    jq -n \
    --arg from "$(from_utc)" \
    '{ from: $from }'
  )
  exec_admin_graphql 'balance-sheet' "$variables"
  echo $(graphql_output)

  balance_usd=$(graphql_output '.data.balanceSheet.balance.usd.balancesByLayer.settled.netDebit')
  balance=${balance_usd}
  [[ "$balance" == "0" ]] || exit 1

  debit_usd=$(graphql_output '.data.balanceSheet.balance.usd.balancesByLayer.settled.debit')
  debit=${debit_usd}
  [[ "$debit" -gt "0" ]] || exit 1

  credit_usd=$(graphql_output '.data.balanceSheet.balance.usd.balancesByLayer.settled.credit')
  credit=${credit_usd}
  [[ "$credit" == "$debit" ]] || exit 1
}

assert_trial_balance() {
  variables=$(
    jq -n \
    --arg from "$(from_utc)" \
    '{ from: $from }'
  )
  exec_admin_graphql 'trial-balance' "$variables"

  all_btc=$(graphql_output '.data.trialBalance.total.btc.balancesByLayer.all.netDebit')
  [[ "$all_btc" == "0" ]] || exit 1

  all_usd=$(graphql_output '.data.trialBalance.total.usd.balancesByLayer.all.netDebit')
  [[ "$all_usd" == "0" ]] || exit 1
}

assert_accounts_balanced() {
  assert_balance_sheet_balanced
  assert_trial_balance
}

net_usd_revenue() {
  variables=$(
    jq -n \
    --arg from "$(from_utc)" \
    '{ from: $from }'
  )
  exec_admin_graphql 'profit-and-loss' "$variables"

  revenue_usd=$(graphql_output '.data.profitAndLossStatement.net.usd.balancesByLayer.all.netCredit')
  echo $revenue_usd
}

from_utc() {
  date -u -d @0 +"%Y-%m-%dT%H:%M:%S.%3NZ"
}

get_csrf_token() {
  local admin_email=$1
  local cookie_file="${CACHE_DIR}/admin/${admin_email}-cookie.jar"
  mkdir -p "${CACHE_DIR}/admin"
  csrf_response=$(curl -s -X GET "$NEXTAUTH_URL/csrf" --cookie-jar "$cookie_file")
  echo "$csrf_response" | grep -oP '(?<="csrfToken":")[^"]*'
}

initiate_sign_in() {
  local admin_email=$1
  local csrf_token=$2
  local cookie_file="${CACHE_DIR}/admin/${admin_email}-cookie.jar"

  sign_in_response=$(curl -s -o /dev/null -w "%{http_code}" -X POST "$NEXTAUTH_URL/signin/email" \
    -H "Content-Type: application/json" \
    -b "$cookie_file" \
    -d '{
      "email": "'"$admin_email"'",
      "csrfToken": "'"$csrf_token"'",
      "callbackUrl": "'"$CALLBACK_URL"'",
      "json": true
    }')

  if [ "$sign_in_response" -eq 302 ]; then
    echo "Email sign-in initiated successfully for $admin_email. Check your email for the login link."
  else
    echo "Failed to send login email for $admin_email. Response Code: $sign_in_response"
    exit 1
  fi
}

get_magic_link() {
  curl -s http://localhost:8025/api/v2/messages |
    jq -r '.items[0].MIME.Parts[0].Body' |
    perl -MMIME::QuotedPrint -pe '$_=MIME::QuotedPrint::decode($_);' |
    grep -o 'http://.*' |
    sed 's/=3D/=/g; s/%3A/:/g; s/%2F/\//g; s/%3F/?/g; s/%3D/=/g; s/%26/\&/g; s/%40/@/g'
}

use_magic_link() {
  local magic_link=$1
  local admin_email=$2
  local cookie_file="${CACHE_DIR}/admin/${admin_email}-cookie.jar"

  curl -s "$magic_link" -b "$cookie_file" -c "$cookie_file" -o /dev/null
}

create_admin_user() {
  local admin_email=$1
  local admin_role=${2:-"ADMIN"}
  variables=$(
    jq -n \
    --arg email "$admin_email" \
    '{
      input: {
        email: $email
        }
      }'
  )

  exec_admin_graphql 'user-create' "$variables"
  user_id=$(graphql_output .data.userCreate.user.userId)
  [[ "$user_id" != "null" ]] || exit 1

  variables=$(
    jq -n \
    --arg userId "$user_id" \
    '{
      input: {
        id: $userId,
        role: "'"$admin_role"'"
        }
      }'
  )

  exec_admin_graphql 'user-assign-role' "$variables" 
  role=$(graphql_output .data.userAssignRole.user.roles[0])
  [[ "$role" = $admin_role ]] || exit 1

  csrf_token=$(get_csrf_token "$admin_email")
  initiate_sign_in "$admin_email" "$csrf_token"
  sleep 3
  magic_link=$(get_magic_link)

  if [ -z "$magic_link" ]; then
    echo "Failed to retrieve magic link."
    exit 1
  fi

  echo "Magic Link: $magic_link"
  use_magic_link "$magic_link" "$admin_email"
  echo "Admin user $admin_email is authenticated and cookies are saved."
}

execute_admin_gql_authed() {
  local query_name=$1
  local variables=${2:-"{}"}
  local admin_email=$3
  local cookie_file="${CACHE_DIR}/admin/${admin_email}-cookie.jar"

  csrf_token=$(grep 'next-auth.csrf-token' "$cookie_file" | awk '{print $7}' | cut -d'%' -f1)
  session_token=$(grep 'next-auth.session-token' "$cookie_file" | awk '{print $7}')

  if [[ -z "$csrf_token" || -z "$session_token" ]]; then
    echo "Failed to retrieve tokens for $admin_email."
    exit 1
  fi

  if [[ "${BATS_TEST_DIRNAME}" != "" ]]; then
    run_cmd="run"
  else
    run_cmd=""
  fi

  ${run_cmd} curl -s \
    -X POST \
    -H "Content-Type: application/json" \
    -H "Cookie: next-auth.csrf-token=${csrf_token}%7C$(grep 'next-auth.csrf-token' "$cookie_file" | cut -d'%' -f2); next-auth.session-token=${session_token}" \
    -d "{\"query\": \"$(gql_admin_query $query_name)\", \"variables\": $variables}" \
    "${GQL_ADMIN_ENDPOINT}"
}
