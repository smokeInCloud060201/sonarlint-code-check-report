#!/bin/bash

# SonarQube Token Management Examples
# Make sure the API server is running before executing these examples

API_BASE_URL="http://localhost:8080"

echo "=== SonarQube Token Management Examples ==="
echo

# 1. Generate a new token
echo "1. Generating a new SonarQube token..."
curl -X POST "${API_BASE_URL}/api/sonarqube/tokens" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "my-project-token",
    "project_key": "my-project",
    "description": "Token for my project",
    "created_by": "admin"
  }' | jq .
echo
echo

# 2. List all tokens from SonarQube
echo "2. Listing all tokens from SonarQube..."
curl -X GET "${API_BASE_URL}/api/sonarqube/tokens" | jq .
echo
echo

# 3. Get tokens from database
echo "3. Getting tokens from database..."
curl -X GET "${API_BASE_URL}/api/database/tokens" | jq .
echo
echo

# 4. Get tokens by project
echo "4. Getting tokens for specific project..."
curl -X GET "${API_BASE_URL}/api/database/tokens/project/my-project" | jq .
echo
echo

# 5. Revoke a token
echo "5. Revoking a token..."
curl -X DELETE "${API_BASE_URL}/api/sonarqube/tokens/my-project-token" | jq .
echo
echo

echo "=== Examples completed ==="
