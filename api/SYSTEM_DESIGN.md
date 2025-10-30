# SonarCute API - System Design

Comprehensive system architecture and design documentation for the SonarCute API backend.

## Table of Contents

- [Architecture Overview](#architecture-overview)
- [System Components](#system-components)
- [Data Flow](#data-flow)
- [Database Design](#database-design)
- [API Design](#api-design)
- [Security Architecture](#security-architecture)
- [Error Handling](#error-handling)
- [Performance Considerations](#performance-considerations)
- [Scalability](#scalability)

## Architecture Overview

### High-Level Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                      Client (Frontend)                       │
└────────────────────────┬────────────────────────────────────┘
                         │ HTTP/REST
                         │
┌────────────────────────▼────────────────────────────────────┐
│                  SonarCute API Server                        │
│  ┌─────────────────────────────────────────────────────┐   │
│  │              HTTP Layer (Actix-web)                  │   │
│  │  ┌─────────────┐  ┌──────────────┐  ┌─────────────┐│   │
│  │  │   Routes    │  │  Middleware  │  │   Handlers  ││   │
│  │  └─────────────┘  └──────────────┘  └─────────────┘│   │
│  └─────────────────────────────────────────────────────┘   │
│                          │                                   │
│  ┌───────────────────────┼────────────────────────────────┐ │
│  │                       │                                 │ │
│  │  ┌────────────────────▼────────┐  ┌──────────────────┐ │ │
│  │  │   Service Layer             │  │ SonarQube Client │ │ │
│  │  │  - Project Service          │  │                  │ │ │
│  │  │  - Token Management         │  │ - HTTP Client    │ │ │
│  │  └─────────────────────────────┘  │ - API Methods    │ │ │
│  │                                   └──────────────────┘ │ │
│  └───────────────────────────────────────────────────────┘ │
└────────────────────┬──────────────────────────┬─────────────┘
                     │                          │
            ┌────────▼────────┐      ┌─────────▼──────────┐
            │   PostgreSQL    │      │    SonarQube       │
            │   Database      │      │    Server          │
            └─────────────────┘      └────────────────────┘
```

## System Components

### 1. HTTP Server Layer

**Technology**: Actix-web 4.11.0

**Responsibilities**:
- HTTP request/response handling
- Route registration and routing
- Middleware (CORS, logging)
- Request validation
- Response serialization

**Key Files**:
- `src/web/server.rs`: Server initialization and route configuration
- `src/sonarqube/handlers.rs`: Request handlers

### 2. Service Layer

**Technology**: Rust (custom service implementation)

**Responsibilities**:
- Business logic orchestration
- Database operations
- Data transformation
- Transaction management

**Key Components**:
- `ProjectService`: Manages project CRUD operations
- Database connection management

**Key Files**:
- `src/database/service.rs`: Service layer implementation

### 3. Database Layer

**Technology**: SeaORM with PostgreSQL

**Responsibilities**:
- Data persistence
- Entity management
- Query building
- Migration management

**Key Files**:
- `src/database/entities.rs`: Project entity definition
- `src/database/admin_token_entity.rs`: Admin token entity
- `src/database/mod.rs`: Database connection

### 4. SonarQube Integration Layer

**Technology**: Reqwest HTTP client

**Responsibilities**:
- SonarQube API communication
- Token-based authentication
- Response parsing
- Error handling for external API

**Key Files**:
- `src/sonarqube/client.rs`: SonarQube API client

### 5. Configuration Layer

**Responsibilities**:
- Environment variable management
- Logger initialization
- Application configuration

**Key Files**:
- `src/config/logger.rs`: Logging setup
- `src/config/mod.rs`: Configuration modules

## Data Flow

### Project Creation Flow

```
1. Client Request
   └─> POST /api/projects
       │
2. HTTP Handler (create_project)
   └─> Validate request
       └─> Extract request body
           │
3. Service Layer (ProjectService)
   └─> Get USER_TOKEN from database
       └─> Check token exists
           │
4. SonarQube Client
   └─> Create project in SonarQube
       └─> Create project token in SonarQube
           │
5. Service Layer
   └─> Save project to database
       └─> Update project with token
           │
6. HTTP Response
   └─> Return ProjectResponse
```

### Project Results Retrieval Flow

```
1. Client Request
   └─> POST /api/results
       │
2. HTTP Handler (get_project_results)
   └─> Validate request
       │
3. Service Layer
   └─> Find project by path
       │
4. Service Layer
   └─> Get GLOBAL_ANALYSIS_TOKEN
       │
5. SonarQube Client (Parallel Requests)
   ├─> Get issues
   ├─> Get coverage
   └─> Get quality gate
       │
6. Aggregate Results
   └─> Combine all responses
       │
7. HTTP Response
   └─> Return ProjectResults
```

## Database Design

### Entity Relationship Diagram

```
┌──────────────────┐
│    projects      │
├──────────────────┤
│ id (PK)          │
│ project_key (UK) │
│ project_name     │
│ project_path (UK)│
│ sonar_token      │
│ sonar_host_url   │
│ language         │
│ sources_path     │
│ tests_path       │
│ coverage_...     │
│ created_at       │
│ updated_at       │
└──────────────────┘

┌──────────────────┐
│  admin_tokens    │
├──────────────────┤
│ id (PK)          │
│ username         │
│ token_name       │
│ token_value      │
│ token_type       │
│ sonar_host_url   │
│ created_at       │
│ updated_at       │
└──────────────────┘

Relationship: No direct FK relationship
Both reference sonar_host_url for SonarQube instance
```

### Table Details

#### projects Table

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| id | SERIAL | PRIMARY KEY | Auto-increment ID |
| project_key | VARCHAR(255) | UNIQUE, NOT NULL | SonarQube project key |
| project_name | VARCHAR(255) | NOT NULL | Display name |
| project_path | VARCHAR(500) | UNIQUE, NOT NULL | Local file system path |
| sonar_token | TEXT | NOT NULL | Project analysis token |
| sonar_host_url | VARCHAR(255) | NOT NULL | SonarQube instance URL |
| language | VARCHAR(50) | NOT NULL | Programming language |
| sources_path | VARCHAR(500) | NOT NULL | Source code path |
| tests_path | VARCHAR(500) | NOT NULL | Test code path |
| coverage_report_path | VARCHAR(500) | NULL | Optional coverage report |
| created_at | TIMESTAMP | NOT NULL | Creation timestamp |
| updated_at | TIMESTAMP | NOT NULL | Last update timestamp |

**Indexes**:
- `idx_projects_project_key` on `project_key`
- `idx_projects_project_path` on `project_path`

#### admin_tokens Table

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| id | SERIAL | PRIMARY KEY | Auto-increment ID |
| username | VARCHAR(255) | NOT NULL | SonarQube username |
| token_name | VARCHAR(255) | NOT NULL | Token identifier |
| token_value | TEXT | NOT NULL | Actual token value |
| token_type | VARCHAR(50) | NOT NULL | USER_TOKEN or GLOBAL_ANALYSIS_TOKEN |
| sonar_host_url | VARCHAR(255) | NOT NULL | SonarQube instance URL |
| created_at | TIMESTAMP | NOT NULL | Creation timestamp |
| updated_at | TIMESTAMP | NOT NULL | Last update timestamp |

**Indexes**:
- `idx_admin_tokens_sonar_host_url` on `sonar_host_url`
- `idx_admin_tokens_username` on `username`

## API Design

### RESTful Principles

The API follows RESTful design principles:

- **Resources**: Projects and admin tokens are resources
- **HTTP Methods**: Use appropriate methods (GET, POST, DELETE)
- **Status Codes**: Return standard HTTP status codes
- **JSON**: Request and response bodies use JSON format

### Endpoint Patterns

#### Resource-Based URLs
```
/api/projects       # Collection resource
/api/admin-token    # Singular resource (token creation)
```

#### Action-Based URLs
```
/api/results            # Action: get results
/api/generate-command   # Action: generate command
```

### Request/Response Format

**Request Headers**:
```
Content-Type: application/json
```

**Response Format**:
```json
{
  "success": true,
  "data": { ... },
  "error": "Optional error message"
}
```

Or error response:
```json
{
  "error": "Error message",
  "suggestion": "Optional suggestion"
}
```

## Security Architecture

### Authentication Flow

1. **Admin Token Creation**:
   - User provides SonarQube username/password
   - API creates token in SonarQube
   - Token stored in database with type (USER_TOKEN or GLOBAL_ANALYSIS_TOKEN)

2. **Token Usage**:
   - USER_TOKEN: Used for admin operations (create/delete projects)
   - GLOBAL_ANALYSIS_TOKEN: Used for reading analysis results

### Token Management

- Tokens are stored encrypted in the database (consider encryption in production)
- Tokens are associated with SonarQube instance URL
- Token types are validated to ensure correct usage

### Security Considerations

1. **CORS**: Currently allows all origins. Restrict in production.
2. **Token Storage**: Tokens stored in database. Ensure database encryption at rest.
3. **API Authentication**: Consider adding API-level authentication for production.
4. **Rate Limiting**: Consider implementing rate limiting for production use.

## Error Handling

### Error Types

1. **Validation Errors** (400 Bad Request)
   - Invalid request format
   - Missing required fields

2. **Not Found Errors** (404 Not Found)
   - Project not found
   - Resource not found

3. **Authentication Errors** (401/403)
   - Missing or invalid tokens
   - Insufficient privileges

4. **Server Errors** (500 Internal Server Error)
   - Database connection failures
   - SonarQube API failures
   - Internal processing errors

### Error Response Format

```json
{
  "error": "Human-readable error message",
  "suggestion": "Optional suggestion for resolution"
}
```

### Error Handling Strategy

- **Early Validation**: Validate requests as early as possible
- **Graceful Degradation**: Handle SonarQube API failures gracefully
- **Detailed Logging**: Log errors with context for debugging
- **User-Friendly Messages**: Provide actionable error messages

## ⚡ Performance Considerations

### Database Optimization

1. **Indexes**: Proper indexes on frequently queried columns
2. **Connection Pooling**: SeaORM handles connection pooling
3. **Query Optimization**: Use efficient queries, avoid N+1 problems

### SonarQube API Optimization

1. **Parallel Requests**: Fetch issues, coverage, and quality gate in parallel
2. **Caching**: Consider caching frequently accessed SonarQube data
3. **Pagination**: Handle pagination for large result sets

### Server Performance

1. **Async I/O**: Actix-web uses async for non-blocking operations
2. **Connection Reuse**: Reuse HTTP client connections
3. **Resource Management**: Proper cleanup of resources

## Scalability

### Horizontal Scaling

- **Stateless Design**: API is stateless, can scale horizontally
- **Database**: Consider connection pooling and read replicas
- **Load Balancing**: API can be behind a load balancer

### Vertical Scaling

- **Resource Limits**: Monitor CPU and memory usage
- **Database Performance**: Optimize database queries and indexes

### Future Enhancements

1. **Caching Layer**: Add Redis for caching
2. **Message Queue**: Use message queue for async operations
3. **Monitoring**: Add metrics and monitoring (Prometheus, Grafana)
4. **API Gateway**: Consider API gateway for rate limiting and authentication

## Transaction Management

### Current Approach

- Database operations use SeaORM's transaction support
- SonarQube operations are external and not transactional

### Consistency Guarantees

- Project creation: Atomic within database, but not with SonarQube
- If SonarQube creation fails, database operation should rollback (consider implementing)

## Logging and Monitoring

### Logging Strategy

- **Structured Logging**: Using `tracing` for structured logs
- **Log Levels**: Configurable via `RUST_LOG` environment variable
- **Request Logging**: Middleware logs all HTTP requests

### Monitoring Considerations

- **Health Checks**: Consider adding `/health` endpoint
- **Metrics**: Consider adding metrics endpoint
- **Error Tracking**: Consider integrating error tracking service

## Testing Strategy

### Unit Tests

- Service layer business logic
- Database operations (with test database)
- SonarQube client (with mocks)

### Integration Tests

- API endpoint tests
- Database integration tests
- SonarQube integration tests (with test instance)

### E2E Tests

- Full workflow tests
- Real SonarQube instance testing

## Deployment Architecture

### Containerization

- Docker image for API server
- Environment-based configuration
- Health check support

### Docker Compose

- API service
- Database service
- Network configuration

### Production Considerations

1. **Secrets Management**: Use secrets management service
2. **Backup Strategy**: Regular database backups
3. **Monitoring**: Production monitoring and alerting
4. **Scaling**: Auto-scaling based on load

---

This document provides a comprehensive overview of the system design. For API endpoint details, see [DOCUMENTATION.md](DOCUMENTATION.md).

