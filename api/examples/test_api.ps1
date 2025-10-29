# Example PowerShell script to test the SonarQube API

$API_BASE_URL = "http://localhost:8080/api"

Write-Host "=== Testing SonarQube Code Check Report API ===" -ForegroundColor Green
Write-Host ""

# Test 1: Create a new project
Write-Host "1. Creating a new project..." -ForegroundColor Yellow
$createProjectBody = @{
    project_key = "test-project"
    project_name = "Test Project"
    project_path = "/path/to/test/project"
    language = "java"
    sources_path = "src/main/java"
    tests_path = "src/test/java"
    coverage_report_path = "build/reports/jacoco/test/jacocoTestReport.xml"
} | ConvertTo-Json

try {
    $createResponse = Invoke-RestMethod -Uri "$API_BASE_URL/projects" -Method POST -Body $createProjectBody -ContentType "application/json"
    $createResponse | ConvertTo-Json -Depth 10
} catch {
    Write-Host "Error creating project: $($_.Exception.Message)" -ForegroundColor Red
}

Write-Host ""
Write-Host "2. Scanning the project..." -ForegroundColor Yellow
$scanProjectBody = @{
    project_path = "/path/to/test/project"
} | ConvertTo-Json

try {
    $scanResponse = Invoke-RestMethod -Uri "$API_BASE_URL/scan" -Method POST -Body $scanProjectBody -ContentType "application/json"
    $scanResponse | ConvertTo-Json -Depth 10
} catch {
    Write-Host "Error scanning project: $($_.Exception.Message)" -ForegroundColor Red
}

Write-Host ""
Write-Host "=== Test completed ===" -ForegroundColor Green
