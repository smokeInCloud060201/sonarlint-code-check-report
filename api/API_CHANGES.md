# API Changes Summary

## Overview
The API has been refactored to separate concerns:
1. **Generate Command**: Returns the exact command users need to run manually
2. **Get Results**: Fetches scan results (issues and coverage) from SonarQube

## API Endpoints

### 1. Generate Sonar Command
**POST** `/api/generate-command`

Generates the gradlew sonar command that users can copy and run in their project directory.

**Request Body:**
```json
{
  "project_path": "/path/to/project"
}
```

**Response:**
```json
{
  "command": "./gradlew sonar \\\n  -Dsonar.token=sqp_... \\\n  -Dsonar.host.url=http://localhost:9000 \\\n  -Dsonar.projectKey=my-project \\\n  -Dsonar.projectName=My Project \\\n  -Dsonar.coverage.jacoco.xmlReportPaths=build/reports/jacoco/test/jacocoTestReport.xml \\\n  -Dsonar.language=java \\\n  -Dsonar.sources=src/main/java \\\n  -Dsonar.tests=src/test/java",
  "project_path": "/path/to/project"
}
```

### 2. Get Project Results
**POST** `/api/results`

Fetches issues and coverage metrics from SonarQube for a scanned project (doesn't execute the scan).

**Request Body:**
```json
{
  "project_path": "/path/to/project"
}
```

**Response:**
```json
{
  "project": {
    "id": 1,
    "project_key": "my-project",
    "project_name": "My Project",
    "project_path": "/path/to/project",
    "sonar_token": "sqp_...",
    "sonar_host_url": "http://localhost:9000",
    "language": "java",
    "sources_path": "src/main/java",
    "tests_path": "src/test/java",
    "coverage_report_path": "build/reports/jacoco/test/jacocoTestReport.xml",
    "created_at": "2024-12-01T10:00:00",
    "updated_at": "2024-12-01T10:00:00"
  },
  "issues": {
    "issues": [...],
    "paging": {...}
  },
  "coverage": {
    "component": {
      "id": "...",
      "key": "my-project",
      "name": "My Project",
      "measures": [
        {
          "metric": "coverage",
          "value": "85.5"
        },
        {
          "metric": "branch_coverage",
          "value": "78.2"
        },
        {
          "metric": "line_coverage",
          "value": "90.1"
        }
      ]
    }
  }
}
```

## Changes Made

### Updated Files

1. **`api/src/sonarqube/client.rs`**
   - Added `CoverageResponse`, `Component`, `Measure`, `Period` structs
   - Added `get_project_coverage()` method to fetch coverage metrics

2. **`api/src/sonarqube/handlers.rs`**
   - Removed local scan execution logic
   - Added `generate_sonar_command()` handler
   - Refactored `scan_project()` to `get_project_results()` 
   - Now fetches issues and coverage from SonarQube API only

3. **`api/src/web/server.rs`**
   - Removed old `/api/scan` route
   - Added `/api/generate-command` route
   - Added `/api/results` route

## Usage Flow

1. **User creates project**: `POST /api/projects`
   - Registers project in SonarQube
   - Stores configuration in database
   - Returns project with token

2. **User requests command**: `POST /api/generate-command`
   - Looks up project by path
   - Returns exact command to run
   - User copies and executes command in their project directory

3. **User fetches results**: `POST /api/results`
   - Looks up project by path
   - Fetches issues from SonarQube
   - Fetches coverage metrics from SonarQube
   - Returns combined results

## Benefits

- ✅ **Separation of Concerns**: Scan execution is manual, API only fetches results
- ✅ **Flexibility**: Users can run the command any way they want
- ✅ **Reliability**: No process execution management issues
- ✅ **Better UX**: Clear command format that users can see and copy
- ✅ **Parallel Fetching**: Issues and coverage fetched concurrently for better performance
