NEXTAUTH_URL="http://localhost:4455/admin-panel/api/auth"
EMAIL="admin@galoy.io"
CALLBACK_URL="/admin-panel/profile"
COOKIE_FILE="cookie.jar"

get_csrf_token() {
  csrf_response=$(curl -s -X GET "$NEXTAUTH_URL/csrf" --cookie-jar $COOKIE_FILE)
  echo "$csrf_response" | grep -oP '(?<="csrfToken":")[^"]*'
}

initiate_sign_in() {
  csrf_token=$1

  sign_in_response=$(curl -s -o /dev/null -w "%{http_code}" -X POST "$NEXTAUTH_URL/signin/email" \
    -H "Content-Type: application/json" \
    -b $COOKIE_FILE \
    -d '{
      "email": "'"$EMAIL"'",
      "csrfToken": "'"$csrf_token"'",
      "callbackUrl": "'"$CALLBACK_URL"'",
      "json": true
    }')

  if [ "$sign_in_response" -eq 302 ]; then
    echo "Email sign-in initiated successfully. Check your email for the login link."
  else
    echo "Failed to send login email. Response Code: $sign_in_response"
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
  magic_link=$1
  curl -s "$magic_link" -b $COOKIE_FILE -c $COOKIE_FILE -o /dev/null
}

get_session_and_csrf_token() {
  session_token=$(grep 'next-auth.session-token' $COOKIE_FILE | awk '{print $7}')
  csrf_token=$(grep 'next-auth.csrf-token' $COOKIE_FILE | awk '{print $7}' | cut -d'%' -f1)
  if [ -z "$session_token" ] || [ -z "$csrf_token" ]; then
    echo "Failed to retrieve session cookies."
    exit 1
  fi
  echo "$session_token|$csrf_token"
}

query_graphql() {
  session_token=$1
  csrf_token=$2

  curl 'http://localhost:4455/admin/graphql' \
    -H 'Content-Type: application/json' \
    -H "Cookie: next-auth.csrf-token=${csrf_token}%7C$(grep 'next-auth.csrf-token' $COOKIE_FILE | cut -d'%' -f2); next-auth.session-token=${session_token}" \
    --data-raw '{"operationName":"Me","variables":{},"query":"query Me {\n  me {\n    userId\n    email\n    roles\n    visibleNavigationItems {\n      loan\n      term\n      user\n      customer\n      deposit\n      withdraw\n      audit\n      financials\n      __typename\n    }\n    __typename\n  }\n}"}'
}

# Main script logic
csrf_token=$(get_csrf_token)
if [ -z "$csrf_token" ]; then
  echo "Failed to retrieve CSRF token"
  exit 1
fi

initiate_sign_in "$csrf_token"

magic_link=$(get_magic_link)
if [ -z "$magic_link" ]; then
  echo "Failed to retrieve magic link."
  exit 1
fi

echo "Magic Link: $magic_link"
use_magic_link "$magic_link"

tokens=$(get_session_and_csrf_token)
session_token=$(echo $tokens | cut -d'|' -f1)
csrf_token=$(echo $tokens | cut -d'|' -f2)

echo "Session Token: $session_token"
echo "CSRF Token: $csrf_token"

query_graphql "$session_token" "$csrf_token"
