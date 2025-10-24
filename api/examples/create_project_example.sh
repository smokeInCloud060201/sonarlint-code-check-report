#!/bin/bash

# Create Project Example - Following the Complete Flow
# This example demonstrates the complete project creation flow as shown in the sequence diagram

API_BASE_URL="http://localhost:8080"

echo "=== Create Project Example - Complete Flow ==="
echo

# Create a new project with folder path
echo "1. Creating a new SonarQube project with folder path..."
curl -X POST "${API_BASE_URL}/api/sonarqube/projects" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "My Test Project",
    "project_key": "my-test-project",
    "visibility": "private",
    "project_folder_path": "/path/to/my/project/source"
  }' | jq .
echo
echo

echo "=== Expected Flow ==="
echo "1. ✅ Create project in SonarQube"
echo "2. ✅ Save project information + folder path to database"
echo "3. ✅ Create project token in SonarQube"
echo "4. ✅ Save token to database"
echo "5. ✅ Return complete response to user"
echo
echo "=== Example completed ==="
