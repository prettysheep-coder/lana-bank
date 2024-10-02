#!/usr/bin/env bats

load "helpers"

setup_file() {
  start_server
}

teardown_file() {
  stop_server
}

@test "customer: can upload a file and retrieve documents" {
  # Create a customer
  customer_email=$(generate_email)
  telegramId=$(generate_email)

  variables=$(
    jq -n \
    --arg email "$customer_email" \
    --arg telegramId "$telegramId" \
    '{
      input: {
        email: $email,
        telegramId: $telegramId
      }
    }'
  )
  
  exec_admin_graphql 'customer-create' "$variables"
  customer_id=$(graphql_output .data.customerCreate.customer.customerId)
  echo "$output"
  [[ "$customer_id" != "null" ]] || exit 1

  # Generate a temporary file
  temp_file=$(mktemp)
  echo "Test content" > "$temp_file"
  
  # Prepare the variables for file upload
  variables=$(jq -n \
    --arg customerId "$customer_id" \
    '{
      "customerId": $customerId,
      "file": null
    }')

  # Execute the GraphQL mutation for file upload
  response=$(exec_admin_graphql_upload "customer-document-attach" "$variables" "$temp_file")  
  document_id=$(echo "$response" | jq -r '.data.customerDocumentAttach.document.id')
  [[ "$document_id" != "" ]] || exit 1
  
  # Clean up the temporary file
  rm "$temp_file"

  # Fetch the document by ID
  variables=$(jq -n \
    --arg documentId "$document_id" \
    '{
      "id": $documentId
    }')

  exec_admin_graphql 'document' "$variables"
  fetched_document_id=$(graphql_output .data.document.id)
  [[ "$fetched_document_id" == "$document_id" ]] || exit 1

  fetched_customer_id=$(graphql_output .data.document.customerId)
  [[ "$fetched_customer_id" == "$customer_id" ]] || exit 1

  # Fetch documents for the customer
  variables=$(jq -n \
    --arg customerId "$customer_id" \
    '{
      "customerId": $customerId
    }')

  exec_admin_graphql 'documents-for-customer' "$variables"

  documents_count=$(graphql_output '.data.customer.documents | length')
  [[ "$documents_count" -ge 1 ]] || exit 1

  first_document_id=$(graphql_output '.data.customer.documents[0].id')
  [[ "$first_document_id" == "$document_id" ]] || exit 1
}