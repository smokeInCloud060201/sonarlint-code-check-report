# Admin Token Management Feature

## Overview
The system now manages SonarQube admin tokens dynamically instead of relying on environment variables. Users can create admin tokens using their SonarQube username and password, and the system stores these tokens in the database for future use.

## New API Endpoint

### Create Admin Token
**POST** `/api/admin-token`

Creates a new admin token in SonarQube and stores it in the database.

**Request Body:**
```json
{
  "username": "admin",
  "password": "admin_password",
  "token_name": "api_admin_token",
  "sonar_host_url": "http://localhost:9000"
}
```

**Response:**
```json
{
  "id": 1,
  "username": "admin",
  "token_name": "api_admin_token",
  "token_value": "sqp_...",
  "sonar_host_url": "http://localhost:9000",
  "created_at": "2024-12-01T10:00:00",
  "updated_at": "2024-12-01T10:00:00"
}
```

## Database Changes

### New Table: `admin_tokens`
```sql
CREATE TABLE admin_tokens (
    id SERIAL PRIMARY KEY,
    username VARCHAR(255) NOT NULL,
    token_name VARCHAR(255) NOT NULL,
    token_value TEXT NOT NULL,
    sonar_host_url VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
```

### Migration
Run the migration to create the admin_tokens table:
```bash
psql -U sonar -d sonar_db -f migrations/20241201000002_create_admin_tokens/up.sql
```

## Updated Workflow

### 1. Create Admin Token (First Time Setup)
```bash
curl -X POST http://localhost:8080/api/admin-token \
  -H "Content-Type: application/json" \
  -d '{
    "username": "admin",
    "password": "admin_password",
    "token_name": "api_admin_token",
    "sonar_host_url": "http://localhost:9000"
  }'
```

### 2. Create Project
```bash
curl -X POST http://localhost:8080/api/projects \
  -H "Content-Type: application/json" \
  -d '{
    "project_key": "my-project",
    "project_name": "My Project",
    "project_path": "/path/to/project",
    "language": "java",
    "sources_path": "src/main/java",
    "tests_path": "src/test/java",
    "coverage_report_path": "build/reports/jacoco/test/jacocoTestReport.xml"
  }'
```

### 3. Generate Command
```bash
curl -X POST http://localhost:8080/api/generate-command \
  -H "Content-Type: application/json" \
  -d '{"project_path": "/path/to/project"}'
```

### 4. Get Results
```bash
curl -X POST http://localhost:8080/api/results \
  -H "Content-Type: application/json" \
  -d '{"project_path": "/path/to/project"}'
```

## Changes Made

### Files Modified

1. **`api/src/database/admin_token_entity.rs`** (NEW)
   - SeaORM entity for admin tokens

2. **`api/src/database/service.rs`**
   - Added `CreateAdminTokenRequest` and `AdminTokenResponse` structs
   - Added `create_admin_token()`, `get_admin_token()`, `update_admin_token_value()` methods

3. **`api/src/sonarqube/client.rs`**
   - Added `generate_admin_token()` method

4. **`api/src/sonarqube/handlers.rs`**
   - Added `create_admin_token()` handler
   - Updated `create_project()` and `get_project_results()` to use database tokens

5. **`api/src/web/server.rs`**
   - Added `/api/admin-token` route

6. **`api/migrations/20241201000002_create_admin_tokens/`** (NEW)
   - Database migration for admin tokens table

## Environment Variables

### No Longer Required
- `SONAR_ADMIN_TOKEN` - Now managed in database

### Still Required
- `SERVER_HOST` - Server host
- `SERVER_PORT` - Server port
- `DATABASE_URL` - Database connection string
- `SONAR_HOST_URL` - SonarQube server URL (used to lookup admin tokens)

## Benefits

- ✅ **Dynamic Token Management**: No need to manually set admin tokens in environment
- ✅ **Multiple SonarQube Instances**: Support for different SonarQube servers
- ✅ **Secure Storage**: Tokens stored securely in database
- ✅ **User-Friendly**: Simple API to create admin tokens with username/password
- ✅ **Automatic Lookup**: System automatically finds the right token for each SonarQube instance

## Error Handling

If no admin token is found for a SonarQube instance, the API returns:
```json
{
  "error": "No admin token found for this SonarQube instance. Please create an admin token first."
}
```

This guides users to create an admin token before using other endpoints.
