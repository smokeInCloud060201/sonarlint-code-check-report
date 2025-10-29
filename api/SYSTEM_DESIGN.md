# SonarQube Code Check Report System - Architecture Overview

## 🎯 **System Purpose**
This system automates SonarQube project management and code quality scanning. It provides a REST API that allows users to:
- Create new projects in SonarQube automatically
- Execute code scans and retrieve quality reports
- Manage project configurations and tokens centrally

## 🏗️ **System Architecture**

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Client App    │    │   Rust API      │    │   SonarQube     │
│   (Frontend)    │◄──►│   (Backend)     │◄──►│   Server        │
└─────────────────┘    └─────────────────┘    └─────────────────┘
                              │
                              ▼
                       ┌─────────────────┐
                       │   PostgreSQL    │
                       │   Database      │
                       └─────────────────┘
```

## 📁 **Project Structure**

```
api/
├── src/
│   ├── main.rs                 # Application entry point
│   ├── config/                 # Configuration management
│   ├── database/              # Database layer
│   │   ├── mod.rs             # Database connection setup
│   │   ├── entities.rs        # SeaORM models
│   │   └── service.rs         # Business logic & DB operations
│   ├── sonarqube/             # SonarQube integration
│   │   ├── mod.rs             # Module exports
│   │   ├── client.rs          # SonarQube API client
│   │   └── handlers.rs        # REST API endpoints
│   └── web/                   # Web server
│       ├── mod.rs
│       └── server.rs          # Actix-web server setup
├── migrations/                # Database schema
└── examples/                  # API usage examples
```

## 🔄 **Data Flow**

### **1. Create Project Flow**
```
Client Request → API Handler → SonarQube API → Database Storage
     ↓              ↓              ↓              ↓
POST /projects → create_project → Create Project → Store Config
                → Generate Token → Return Token → Update Record
```

### **2. Scan Project Flow**
```
Client Request → API Handler → Database Lookup → Execute Scan → Get Results
     ↓              ↓              ↓              ↓              ↓
POST /scan → scan_project → Find Project → Run sonar-scanner → Return Issues
```

## 🗄️ **Database Schema**

### **Projects Table**
```sql
CREATE TABLE projects (
    id SERIAL PRIMARY KEY,
    project_key VARCHAR(255) NOT NULL UNIQUE,      -- SonarQube project key
    project_name VARCHAR(255) NOT NULL,            -- Human-readable name
    project_path VARCHAR(500) NOT NULL UNIQUE,     -- File system path
    sonar_token TEXT NOT NULL,                     -- Generated auth token
    sonar_host_url VARCHAR(255) NOT NULL,         -- SonarQube server URL
    language VARCHAR(50) NOT NULL,                 -- Programming language
    sources_path VARCHAR(500) NOT NULL,           -- Source code directory
    tests_path VARCHAR(500) NOT NULL,              -- Test code directory
    coverage_report_path VARCHAR(500),             -- Coverage report path
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
```

### **Admin Tokens Table**
```sql
CREATE TABLE admin_tokens (
    id SERIAL PRIMARY KEY,
    username VARCHAR(255) NOT NULL,                -- SonarQube username
    token_name VARCHAR(255) NOT NULL,              -- Token name
    token_value TEXT NOT NULL,                     -- Generated admin token
    sonar_host_url VARCHAR(255) NOT NULL,         -- SonarQube server URL
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
```

## 🔌 **API Endpoints**

### **POST /api/admin-token**
**Purpose**: Create admin token for SonarQube authentication

**Request Body**:
```json
{
  "username": "admin",
  "password": "admin_password",
  "token_name": "api_admin_token",
  "sonar_host_url": "http://localhost:9000"
}
```

**Process**:
1. Generate admin token in SonarQube using username/password
2. Store token in database
3. Return token details

### **POST /api/projects**
**Purpose**: Create a new project in SonarQube and store configuration

**Request Body**:
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

**Process**:
1. Get admin token from database
2. Validate input data
3. Create project in SonarQube via API
4. Generate project-specific token
5. Store project configuration in database
6. Return complete project details

### **POST /api/generate-command**
**Purpose**: Generate sonar-scanner command for manual execution

**Request Body**:
```json
{
  "project_path": "/path/to/project"
}
```

**Process**:
1. Lookup project by path in database
2. Build sonar-scanner command with project config
3. Return formatted command string

### **POST /api/results**
**Purpose**: Fetch scan results from SonarQube

**Request Body**:
```json
{
  "project_path": "/path/to/project"
}
```

**Process**:
1. Lookup project by path in database
2. Get admin token from database
3. Retrieve issues and coverage from SonarQube API
4. Return scan results and metrics

## 🛠️ **Key Components**

### **Database Service (`ProjectService`)**
- **Purpose**: Manages all database operations
- **Key Methods**:
  - `create_project()` - Store new project configuration
  - `get_project_by_path()` - Find project by file system path
  - `update_sonar_token()` - Update project with generated token

### **SonarQube Client (`SonarQubeClient`)**
- **Purpose**: Handles all SonarQube API interactions
- **Key Methods**:
  - `create_project()` - Create project in SonarQube
  - `create_project_token()` - Generate analysis token
  - `get_project_issues()` - Retrieve scan results

### **API Handlers**
- **Purpose**: Process HTTP requests and coordinate operations
- **Key Functions**:
  - `create_project()` - Orchestrates project creation flow
  - `scan_project()` - Orchestrates scanning flow

## 🔧 **Configuration**

### **Environment Variables**
```env
# Server Configuration
SERVER_HOST=127.0.0.1
SERVER_PORT=8080

# Database Configuration  
DATABASE_URL=postgresql://sonar:sonar@localhost:5432/sonarcute

# SonarQube Configuration
SONAR_HOST_URL=http://localhost:9000
```

### **Dependencies**
- **actix-web**: Web framework for REST API
- **sea-orm**: Database ORM for PostgreSQL
- **reqwest**: HTTP client for SonarQube API calls
- **serde**: JSON serialization/deserialization
- **tokio**: Async runtime

## 🚀 **Deployment & Usage**

### **Prerequisites**
- Rust (latest stable)
- PostgreSQL running on port 5432
- SonarQube running on port 9000
- SonarQube Scanner CLI installed

### **Setup Steps**
1. **Database Setup**:
   ```sql
   CREATE DATABASE sonarcute;
   CREATE USER sonar WITH PASSWORD 'sonar';
   GRANT ALL PRIVILEGES ON DATABASE sonarcute TO sonar;
   ```

2. **Run Migrations**:
   ```bash
   psql -U sonar -d sonarcute -f migrations/20241201000001_create_projects/up.sql
   ```

3. **Configure Environment**:
   Create `.env` file with required variables

4. **Start Application**:
   ```bash
   cd api
   cargo run
   ```

## 🔍 **How It Works**

### **Project Creation Process**
1. Client sends project details to `/api/projects`
2. API validates input and calls SonarQube API to create project
3. System generates a project-specific analysis token
4. Project configuration is stored in PostgreSQL database
5. Complete project details (including token) are returned to client

### **Project Scanning Process**
1. Client sends project path to `/api/scan`
2. System looks up project configuration in database
3. Builds sonar-scanner command with project-specific parameters:
   ```bash
   sonar-scanner \
     -Dsonar.token=sqp_... \
     -Dsonar.host.url=http://localhost:9000 \
     -Dsonar.projectKey=my-project \
     -Dsonar.projectName=My Project \
     -Dsonar.language=java \
     -Dsonar.sources=src/main/java \
     -Dsonar.tests=src/test/java \
     -Dsonar.coverage.jacoco.xmlReportPaths=build/reports/jacoco/test/jacocoTestReport.xml
   ```
4. Executes scan in project directory
5. Retrieves issues from SonarQube API
6. Returns scan results and quality report to client

## 🎯 **Benefits**

- **Automation**: No manual SonarQube project setup
- **Centralized Management**: All project configs in one database
- **Token Management**: Automatic token generation and storage
- **RESTful API**: Easy integration with any frontend
- **Error Handling**: Comprehensive error responses
- **Scalable**: Built with async Rust for high performance

## 🔮 **Future Enhancements**

- Project listing and management endpoints
- Scan history and trend analysis
- Webhook support for scan completion notifications
- Multi-language support improvements
- Docker containerization
- Authentication and authorization
