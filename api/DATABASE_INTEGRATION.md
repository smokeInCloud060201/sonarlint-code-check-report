# Database Integration with SeaORM and PostgreSQL

This document describes the database integration added to the SonarQube service, which now stores project information in a PostgreSQL database using SeaORM.

## Overview

The service now automatically saves SonarQube projects to a PostgreSQL database when they are created, providing persistent storage and additional database operations.

## Database Schema

### Projects Table

The `projects` table stores SonarQube project information:

```sql
CREATE TABLE "projects" (
    "id" UUID NOT NULL,
    "sonarqube_key" VARCHAR NOT NULL,
    "name" VARCHAR NOT NULL,
    "visibility" VARCHAR NOT NULL,
    "qualifier" VARCHAR NOT NULL,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "sonarqube_created_at" TIMESTAMPTZ,
    "description" TEXT,
    "language" VARCHAR,
    "tags" TEXT,
    "is_active" BOOLEAN NOT NULL DEFAULT true,
    CONSTRAINT "projects_pkey" PRIMARY KEY ("id")
);
```

**Indexes:**
- Unique index on `sonarqube_key`
- Index on `sonarqube_key` for lookups
- Index on `name` for searches
- Index on `is_active` for filtering
- Index on `created_at` for ordering

## Configuration

### Environment Variables

Add these environment variables to your `.env` file:

```env
# Database Configuration
DB_HOST=localhost
DB_PORT=5432
DB_NAME=sonarqube_projects
DB_USER=postgres
DB_PASSWORD=your_password
DB_MAX_CONNECTIONS=10
DB_MIN_CONNECTIONS=1
DB_CONNECT_TIMEOUT=30
DB_IDLE_TIMEOUT=600
```

### Default Values

If environment variables are not set, the following defaults are used:
- `DB_HOST`: localhost
- `DB_PORT`: 5432
- `DB_NAME`: sonarqube_projects
- `DB_USER`: postgres
- `DB_PASSWORD`: password
- `DB_MAX_CONNECTIONS`: 10
- `DB_MIN_CONNECTIONS`: 1
- `DB_CONNECT_TIMEOUT`: 30
- `DB_IDLE_TIMEOUT`: 600

## Database Operations

### Automatic Project Storage

When a project is created via the SonarQube API (`POST /api/sonarqube/projects`), it is automatically saved to the database. The service:

1. Creates the project in SonarQube
2. Saves the project information to the PostgreSQL database
3. Returns the SonarQube response to the client

### Database API Endpoints

The service provides additional endpoints for database operations:

#### Get All Projects
```http
GET /api/database/projects
```

Returns all active projects from the database.

#### Get Project by ID
```http
GET /api/database/projects/{project_id}
```

Returns a specific project by its database ID (UUID).

#### Get Project by SonarQube Key
```http
GET /api/database/projects/sonarqube/{sonarqube_key}
```

Returns a project by its SonarQube key.

#### Deactivate Project (Soft Delete)
```http
POST /api/database/projects/{project_id}/deactivate
```

Marks a project as inactive without deleting it from the database.

#### Delete Project (Hard Delete)
```http
DELETE /api/database/projects/{project_id}
```

Permanently removes a project from the database.

## Database Models

### Project Model

The `Project` entity includes:

- `id`: UUID primary key
- `sonarqube_key`: Unique SonarQube project key
- `name`: Project name
- `visibility`: Project visibility (public/private)
- `qualifier`: SonarQube qualifier
- `created_at`: Database creation timestamp
- `updated_at`: Last update timestamp
- `sonarqube_created_at`: SonarQube creation timestamp (optional)
- `description`: Project description (optional)
- `language`: Primary language (optional)
- `tags`: JSON string of tags (optional)
- `is_active`: Active status flag

## Error Handling

The database integration includes comprehensive error handling:

- **Connection Errors**: Proper error messages for database connection failures
- **Duplicate Projects**: Handles cases where projects already exist
- **Validation Errors**: Input validation with descriptive error messages
- **Graceful Degradation**: SonarQube operations continue even if database save fails

## Migration Setup

### Running Migrations

To set up the database schema:

1. **Create the database**:
   ```sql
   CREATE DATABASE sonarqube_projects;
   ```

2. **Run the migration**:
   ```bash
   # The migration files are located in:
   # api/migrations/20241201000001_create_projects/
   ```

3. **Manual setup** (if needed):
   ```sql
   -- Run the up.sql file content manually
   ```

### Migration Files

- `migrations/20241201000001_create_projects/up.sql` - Creates the projects table
- `migrations/20241201000001_create_projects/down.sql` - Drops the projects table

## Usage Examples

### Creating a Project (with automatic database storage)

```bash
curl -X POST http://localhost:8080/api/sonarqube/projects \
  -H "Content-Type: application/json" \
  -d '{
    "name": "My Project",
    "project_key": "my-project-key",
    "visibility": "public"
  }'
```

This will:
1. Create the project in SonarQube
2. Automatically save it to the PostgreSQL database
3. Return the SonarQube response

### Retrieving Projects from Database

```bash
# Get all projects
curl http://localhost:8080/api/database/projects

# Get project by SonarQube key
curl http://localhost:8080/api/database/projects/sonarqube/my-project-key

# Get project by database ID
curl http://localhost:8080/api/database/projects/{uuid}
```

### Managing Projects

```bash
# Deactivate a project (soft delete)
curl -X POST http://localhost:8080/api/database/projects/{uuid}/deactivate

# Delete a project (hard delete)
curl -X DELETE http://localhost:8080/api/database/projects/{uuid}
```

## Response Format

All database API responses follow the same format as SonarQube APIs:

```json
{
    "success": true,
    "data": { /* project data */ },
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

## Architecture

The database integration follows a clean architecture pattern:

- **Entities** (`src/database/entities.rs`): SeaORM entity definitions
- **Service** (`src/database/service.rs`): Business logic for database operations
- **Handlers** (`src/database/handlers.rs`): HTTP request handlers
- **Connection** (`src/database/connection.rs`): Database connection management
- **Config** (`src/database/config.rs`): Database configuration

## Dependencies

The following dependencies were added:

- `sea-orm`: ORM for Rust with PostgreSQL support
- `sqlx`: Async SQL toolkit
- `chrono`: Date and time handling
- `uuid`: UUID generation and handling

## Performance Considerations

- **Connection Pooling**: Uses SeaORM's built-in connection pooling
- **Indexes**: Proper database indexes for efficient queries
- **Async Operations**: All database operations are asynchronous
- **Error Handling**: Comprehensive error handling without performance impact

## Security

- **SQL Injection Protection**: SeaORM provides built-in protection
- **Connection Security**: Uses environment variables for sensitive data
- **Input Validation**: Proper validation of all inputs
- **Error Information**: Sensitive information is not exposed in error messages

## Monitoring and Logging

The service includes comprehensive logging for database operations:

- Connection establishment
- Query execution
- Error conditions
- Performance metrics

All database operations are logged with appropriate levels (info, warn, error).

## Future Enhancements

Potential future improvements:

1. **Caching**: Add Redis caching for frequently accessed projects
2. **Audit Trail**: Track all project changes
3. **Bulk Operations**: Support for bulk project operations
4. **Advanced Queries**: More sophisticated query capabilities
5. **Data Export**: Export project data in various formats
