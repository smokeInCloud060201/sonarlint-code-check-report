# SonarQube Integration Service

This Rust backend service provides a comprehensive API for interacting with SonarQube, allowing you to create projects, fetch issues, and manage SonarQube resources programmatically.

## Features

- **Project Management**: Create, delete, and check project existence
- **Issue Retrieval**: Fetch project issues with filtering and pagination
- **Health Monitoring**: Check SonarQube server connectivity and version
- **Comprehensive Error Handling**: Proper error responses and logging
- **RESTful API**: Clean HTTP endpoints for all operations

## API Endpoints

### Project Management

#### Create Project
```http
POST /api/sonarqube/projects
Content-Type: application/json

{
    "name": "My Project",
    "project_key": "my-project-key",
    "visibility": "public" // optional: "public" or "private"
}
```

#### Get Project Information
```http
GET /api/sonarqube/projects/{project_key}
```

#### Check Project Existence
```http
GET /api/sonarqube/projects/{project_key}/exists
```

#### Delete Project
```http
DELETE /api/sonarqube/projects/{project_key}
```

### Issue Management

#### Get Project Issues (Paginated)
```http
POST /api/sonarqube/issues
Content-Type: application/json

{
    "project_key": "my-project-key",
    "severities": "BLOCKER,CRITICAL,MAJOR", // optional
    "types": "BUG,VULNERABILITY", // optional
    "statuses": "OPEN,CONFIRMED", // optional
    "created_after": "2024-01-01", // optional ISO 8601 date
    "created_before": "2024-12-31", // optional ISO 8601 date
    "page": 1, // optional, default: 1
    "page_size": 100 // optional, default: 500
}
```

#### Get All Project Issues (Auto-pagination)
```http
POST /api/sonarqube/issues/all
Content-Type: application/json

{
    "project_key": "my-project-key",
    "severities": "BLOCKER,CRITICAL,MAJOR", // optional
    "types": "BUG,VULNERABILITY", // optional
    "statuses": "OPEN,CONFIRMED", // optional
    "created_after": "2024-01-01", // optional ISO 8601 date
    "created_before": "2024-12-31" // optional ISO 8601 date
}
```

### System Information

#### Health Check
```http
GET /api/sonarqube/health
```

#### Get SonarQube Version
```http
GET /api/sonarqube/version
```

## Configuration

The service uses environment variables for configuration. Create a `.env` file in the `api` directory:

```env
# Server Configuration
SERVER_HOST=127.0.0.1
SERVER_PORT=8080

# SonarQube Configuration
SONARQUBE_URL=http://localhost:9000
SONARQUBE_USERNAME=admin
SONARQUBE_PASSWORD=admin
SONARQUBE_TIMEOUT=30
```

### Environment Variables

- `SERVER_HOST`: Host address for the API server (default: 127.0.0.1)
- `SERVER_PORT`: Port for the API server (default: 8080)
- `SONARQUBE_URL`: SonarQube server URL (default: http://localhost:9000)
- `SONARQUBE_USERNAME`: SonarQube username (default: admin)
- `SONARQUBE_PASSWORD`: SonarQube password (default: admin)
- `SONARQUBE_TIMEOUT`: Request timeout in seconds (default: 30)

## Response Format

All API responses follow a consistent format:

```json
{
    "success": true,
    "data": { /* response data */ },
    "error": null
}
```

For error responses:
```json
{
    "success": false,
    "data": null,
    "error": "Error message describing what went wrong"
}
```

### Issues Response Structure

The issues API returns a comprehensive response with the following structure:

```json
{
    "success": true,
    "data": {
        "paging": {
            "pageIndex": 1,
            "pageSize": 100,
            "total": 1
        },
        "issues": [
            {
                "key": "01fc972e-2a3c-433e-bcae-0bd7f88f5123",
                "component": "com.example:project:src/main/java/Example.java",
                "project": "com.example:project",
                "rule": "java:S1144",
                "cleanCodeAttribute": "CLEAR",
                "cleanCodeAttributeCategory": "INTENTIONAL",
                "issueStatus": "ACCEPTED",
                "prioritizedRule": false,
                "impacts": [
                    {
                        "softwareQuality": "SECURITY",
                        "severity": "HIGH"
                    }
                ],
                "message": "Remove this unused private method.",
                "messageFormattings": [
                    {
                        "start": 0,
                        "end": 4,
                        "type": "CODE"
                    }
                ],
                "line": 81,
                "hash": "a227e508d6646b55a086ee11d63b21e9",
                "author": "Developer 1",
                "effort": "2h1min",
                "creationDate": "2013-05-13T17:55:39+0200",
                "updateDate": "2013-05-13T17:55:39+0200",
                "tags": ["bug"],
                "comments": [...],
                "transitions": ["reopen"],
                "actions": ["comment"],
                "textRange": {...},
                "flows": [...],
                "quickFixAvailable": false,
                "ruleDescriptionContextKey": "spring",
                "codeVariants": ["windows", "linux"],
                "internalTags": ["advanced", "sast"]
            }
        ],
        "components": [...],
        "rules": [...],
        "users": [...],
        "facets": [...]
    }
}
```

## Issue Filtering Options

### Severities
- `BLOCKER`: Critical issues that must be fixed
- `CRITICAL`: High priority issues
- `MAJOR`: Medium priority issues
- `MINOR`: Low priority issues
- `INFO`: Informational issues

### Types
- `CODE_SMELL`: Code quality issues
- `BUG`: Potential bugs
- `VULNERABILITY`: Security vulnerabilities
- `SECURITY_HOTSPOT`: Security hotspots

### Statuses
- `OPEN`: New issues
- `CONFIRMED`: Confirmed issues
- `REOPENED`: Reopened issues
- `RESOLVED`: Resolved issues
- `CLOSED`: Closed issues

## Usage Examples

### Create a Project
```bash
curl -X POST http://localhost:8080/api/sonarqube/projects \
  -H "Content-Type: application/json" \
  -d '{
    "name": "My Awesome Project",
    "project_key": "awesome-project",
    "visibility": "public"
  }'
```

### Get All Critical Issues
```bash
curl -X POST http://localhost:8080/api/sonarqube/issues/all \
  -H "Content-Type: application/json" \
  -d '{
    "project_key": "awesome-project",
    "severities": "BLOCKER,CRITICAL",
    "types": "BUG,VULNERABILITY"
  }'
```

### Check Project Existence
```bash
curl http://localhost:8080/api/sonarqube/projects/awesome-project/exists
```

### Health Check
```bash
curl http://localhost:8080/api/sonarqube/health
```

## Running the Service

1. **Install Dependencies**:
   ```bash
   cd api
   cargo build
   ```

2. **Configure Environment**:
   ```bash
   cp .env.example .env
   # Edit .env with your SonarQube configuration
   ```

3. **Start the Service**:
   ```bash
   cargo run
   ```

4. **Verify Service**:
   ```bash
   curl http://localhost:8080/api/sonarqube/health
   ```

## Architecture

The service is built with the following components:

- **Models** (`src/sonarqube/models.rs`): Data structures for SonarQube API requests and responses
- **Client** (`src/sonarqube/client.rs`): HTTP client for SonarQube API communication
- **Service** (`src/sonarqube/service.rs`): High-level business logic for SonarQube operations
- **Handlers** (`src/sonarqube/handlers.rs`): HTTP request handlers for REST API endpoints
- **Server** (`src/web/server.rs`): Actix Web server configuration and routing

## Error Handling

The service includes comprehensive error handling:

- **SonarQube API Errors**: Properly parsed and returned to clients
- **Network Errors**: Timeout and connection issues are handled gracefully
- **Validation Errors**: Input validation with descriptive error messages
- **Logging**: All operations are logged with appropriate levels (info, warn, error)

## Dependencies

- **actix-web**: Web framework for HTTP server
- **reqwest**: HTTP client for SonarQube API calls
- **serde**: Serialization/deserialization
- **anyhow**: Error handling
- **tracing**: Logging and observability
- **urlencoding**: URL encoding for query parameters

## Development

To extend the service:

1. **Add New Endpoints**: Create handlers in `src/sonarqube/handlers.rs`
2. **Add New Models**: Define data structures in `src/sonarqube/models.rs`
3. **Add New Service Methods**: Implement business logic in `src/sonarqube/service.rs`
4. **Update Routes**: Add new routes in `src/web/server.rs`

## License

This project is part of the SonarLint Code Check Report system.
