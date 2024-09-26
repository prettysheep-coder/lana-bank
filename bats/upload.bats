#!/usr/bin/env bats

load "helpers"

@test "customer: can upload a file" {
  # Generate a temporary file
  temp_file=$(mktemp)
  echo "Test content" > "$temp_file"
  
  # Prepare the GraphQL query
  query='mutation($file: Upload!) { upload(file: $file) }'
  
  # Execute the GraphQL mutation using curl
  response=$(curl -s -X POST \
    -H "Content-Type: multipart/form-data" \
    -F "operations={\"query\":\"$query\", \"variables\": {\"file\": null}}" \
    -F "map={\"0\":[\"variables.file\"]}" \
    -F "0=@$temp_file;filename=test_file.txt" \
    "${GQL_ADMIN_ENDPOINT}")
  
  # Output response for debugging
  echo "$response"
  
  # Check the response
  success=$(echo "$response" | jq -r '.data.upload')
  [[ "$success" == "true" ]] || exit 1
  
  # Verify that the file was created (this part depends on how you can access the server's file system)
  # You might need to adjust this based on your setup
#   uploaded_file_path=$(echo "$response" | grep -o '/tmp/.*test_file.txt')
#   [[ -f "$uploaded_file_path" ]] || exit 1
  
  # Clean up the temporary file
#   rm "$temp_file"
}