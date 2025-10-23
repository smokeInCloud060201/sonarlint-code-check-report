#!/bin/bash

# SonarQube Service API Examples
# This script demonstrates how to use the SonarQube integration service

API_BASE="http://localhost:8080/api/sonarqube"
PROJECT_KEY="example-project"

echo "=== SonarQube Service API Examples ==="
echo

# Health Check
echo "1. Health Check:"
curl -s "$API_BASE/health" | jq '.'
echo
echo

# Get SonarQube Version
echo "2. SonarQube Version:"
curl -s "$API_BASE/version" | jq '.'
echo
echo

# Create Project
echo "3. Creating Project:"
curl -s -X POST "$API_BASE/projects" \
  -H "Content-Type: application/json" \
  -d "{
    \"name\": \"Example Project\",
    \"project_key\": \"$PROJECT_KEY\",
    \"visibility\": \"public\"
  }" | jq '.'
echo
echo

# Check Project Existence
echo "4. Checking Project Existence:"
curl -s "$API_BASE/projects/$PROJECT_KEY/exists" | jq '.'
echo
echo

# Get Project Information
echo "5. Getting Project Information:"
curl -s "$API_BASE/projects/$PROJECT_KEY" | jq '.'
echo
echo

# Get Project Issues (first page)
echo "6. Getting Project Issues (first page):"
curl -s -X POST "$API_BASE/issues" \
  -H "Content-Type: application/json" \
  -d "{
    \"project_key\": \"$PROJECT_KEY\",
    \"severities\": \"BLOCKER,CRITICAL,MAJOR\",
    \"page\": 1,
    \"page_size\": 10
  }" | jq '.'
echo
echo

# Get All Project Issues
echo "7. Getting All Project Issues:"
curl -s -X POST "$API_BASE/issues/all" \
  -H "Content-Type: application/json" \
  -d "{
    \"project_key\": \"$PROJECT_KEY\",
    \"severities\": \"BLOCKER,CRITICAL\",
    \"types\": \"BUG,VULNERABILITY\"
  }" | jq '.data | length'
echo "Total issues found"
echo
echo

# Delete Project (uncomment to actually delete)
# echo "8. Deleting Project:"
# curl -s -X DELETE "$API_BASE/projects/$PROJECT_KEY" | jq '.'
# echo
# echo

echo "=== Examples Complete ==="
echo
echo "Note: Make sure your SonarQube server is running at http://localhost:9000"
echo "and the API service is running at http://localhost:8080"
echo
echo "To run the API service:"
echo "  cd api && cargo run"
echo
echo "To install jq for JSON formatting:"
echo "  sudo apt-get install jq  # Ubuntu/Debian"
echo "  brew install jq         # macOS"
