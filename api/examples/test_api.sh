#!/bin/bash

# Example script to test the SonarQube API

API_BASE_URL="http://localhost:8080/api"

echo "=== Testing SonarQube Code Check Report API ==="
echo

# Test 1: Create a new project
echo "1. Creating a new project..."
curl -X POST "$API_BASE_URL/projects" \
  -H "Content-Type: application/json" \
  -d '{
    "project_key": "test-project",
    "project_name": "Test Project",
    "project_path": "/path/to/test/project",
    "language": "java",
    "sources_path": "src/main/java",
    "tests_path": "src/test/java",
    "coverage_report_path": "build/reports/jacoco/test/jacocoTestReport.xml"
  }' | jq '.'

echo
echo "2. Scanning the project..."
curl -X POST "$API_BASE_URL/scan" \
  -H "Content-Type: application/json" \
  -d '{
    "project_path": "/path/to/test/project"
  }' | jq '.'

echo
echo "=== Test completed ==="
