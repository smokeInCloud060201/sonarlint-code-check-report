# SonarQube Code Check Report API

A Rust-based API for managing SonarQube projects and generating code quality reports.

## Features

- **Create Project**: Automatically creates projects in SonarQube and generates project tokens
- **Generate Command**: Returns the exact sonar-scan command to run
- **Get Results**: Fetches scan results (issues and coverage) from SonarQube

## Setup

### Prerequisites

- Rust (latest stable version)
- PostgreSQL running on port 5432
- SonarQube running on port 9000

### Database Setup

1. Create PostgreSQL database:
```sql
CREATE DATABASE sonarcute;
CREATE USER sonar WITH PASSWORD 'sonar';
GRANT ALL PRIVILEGES ON DATABASE sonarcute TO sonar;
```

2. Run migrations:
```bash
cd api
psql -U sonar -d sonarcute -f migrations/20241201000001_create_projects/up.sql
```

### Environment Configuration

Create a `.env` file in the `api` directory:

```env
# Server Configuration
SERVER_HOST=127.0.0.1
SERVER_PORT=8080

# Database Configuration
DATABASE_URL=postgresql://sonar:sonar@localhost:5432/sonarcute

# SonarQube Configuration
SONAR_HOST_URL=http://localhost:9000
```

### Running the Application

```bash
cd api
cargo run
```

## API Endpoints

### Create Admin Token

**POST** `/api/admin-token`

Creates a new admin token in SonarQube and stores it in the database. This is required before creating projects.

Request body:
```json
{
  "username": "admin",
  "password": "admin_password",
  "token_name": "api_admin_token",
  "sonar_host_url": "http://localhost:9000"
}
```

Response:
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

### Create Project

**POST** `/api/projects`

Creates a new project in SonarQube and stores project information in the database.

Request body:
```json
{
  "project_key": "my-project",
  "project_name": "My Project",
  "project_path": "/path/to/project",
  "language": "java",
  "sources_path": "src/main/java",
  "tests_path": "src/test/java",
  "coverage_report_path": "build/reports/jacoco/test/jacocoTestReport.xml"
}
```

Response:
```json
{
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
}
```

### Generate Sonar Command

**POST** `/api/generate-command`

Generates the gradlew sonar command that users can copy and run in their project directory.

Request body:
```json
{
  "project_path": "/path/to/project"
}
```

Response:
```json
{
  "command": "./gradlew sonar \\\n  -Dsonar.token=sqp_... \\\n  -Dsonar.host.url=http://localhost:9000 \\\n  -Dsonar.projectKey=my-project \\\n  -Dsonar.projectName=My Project \\\n  -Dsonar.coverage.jacoco.xmlReportPaths=build/reports/jacoco/test/jacocoTestReport.xml \\\n  -Dsonar.language=java \\\n  -Dsonar.sources=src/main/java \\\n  -Dsonar.tests=src/test/java",
  "project_path": "/path/to/project"
}
```

### Get Project Results

**POST** `/api/results`

Fetches issues and coverage metrics from SonarQube for a scanned project.

Request body:
```json
{
  "project_path": "/path/to/project"
}
```

Response:
```json
{
  "project": { ... },
  "issues": {
    "issues": [
      {
        "key": "issue_key",
        "rule": "rule_key",
        "severity": "MAJOR",
        "component": "component_path",
        "project": "project_key",
        "line": 42,
        "message": "Issue description",
        "status": "OPEN",
        "type": "CODE_SMELL"
      }
    ],
    "paging": {
      "pageIndex": 1,
      "pageSize": 500,
      "total": 10
    }
  },
  "coverage": {
    "component": {
      "measures": [
        {
          "metric": "coverage",
          "value": "85.5"
        },
        {
          "metric": "branch_coverage",
          "value": "78.2"
        }
      ]
    }
  }
}
```

## Usage Flow

1. **Create admin token** using `/api/admin-token` (first time only)
2. **Create a project** using `/api/projects`
3. **Get the command** using `/api/generate-command`
4. **Run the command** in your project directory
5. **Fetch results** using `/api/results`

### Example Workflow

```bash
# 1. Create admin token (first time only)
curl -X POST http://localhost:8080/api/admin-token \
  -H "Content-Type: application/json" \
  -d '{
    "username": "admin",
    "password": "admin_password",
    "token_name": "api_admin_token",
    "sonar_host_url": "http://localhost:9000"
  }'

# 2. Create project
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

# 3. Get the scan command
curl -X POST http://localhost:8080/api/generate-command \
  -H "Content-Type: application/json" \
  -d '{"project_path": "/path/to/project"}'

# 4. Copy and run the returned command in your project directory

# 5. Get scan results
curl -X POST http://localhost:8080/api/results \
  -H "Content-Type: application/json" \
  -d '{"project_path": "/path/to/project"}'
```

## Project Structure

```
api/
├── src/
│   ├── database/          # Database models and services
│   ├── sonarqube/         # SonarQube API client and handlers
│   ├── web/              # Web server configuration
│   └── main.rs           # Application entry point
├── migrations/           # Database migrations
└── Cargo.toml           # Dependencies
```

## Dependencies

- **actix-web**: Web framework
- **sea-orm**: Database ORM
- **reqwest**: HTTP client for SonarQube API
- **serde**: Serialization/deserialization
- **tokio**: Async runtime
