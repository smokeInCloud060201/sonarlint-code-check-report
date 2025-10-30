# SonarCute API - Backend Documentation

Rust-based REST API for managing SonarQube projects, tokens, and code quality reports.

## Table of Contents

- [Overview](#overview)
- [Tech Stack](#tech-stack)
- [Setup](#setup)
- [Configuration](#configuration)
- [Database](#database)
- [Running the Server](#running-the-server)
- [API Endpoints](#api-endpoints)
- [Development](#development)

## Overview

The SonarCute API is a high-performance backend service built with Rust and Actix-web. It provides:

- Project management (create, read, delete)
- Admin token management for SonarQube operations
- SonarQube API integration
- Code quality metrics retrieval (issues, coverage, quality gates)
- SonarQube scanner command generation

## Tech Stack

- **Rust** (Edition 2024)
- **Actix-web** 4.11.0 - Web framework
- **SeaORM** 0.12 - Database ORM
- **PostgreSQL** - Database
- **Reqwest** - HTTP client for SonarQube API
- **Serde** - Serialization/deserialization
- **Tracing** - Structured logging

## Setup

### Prerequisites

- Rust (latest stable version)
- PostgreSQL 12+
- SonarQube instance (running on port 9000 by default)
- Cargo (Rust package manager)

### Installation

1. **Navigate to the API directory**
   ```bash
   cd api
   ```

2. **Create environment file**
   ```bash
   cp .env.example .env
   # Edit .env with your configuration
   ```

3. **Install dependencies**
   ```bash
   cargo build
   ```

4. **Set up the database**
   ```bash
   # Ensure PostgreSQL is running
   # Run migrations (see Database section)
   ```

### Configuration

Create a `.env` file in the `api/` directory:

```env
SERVER_HOST=0.0.0.0
SERVER_PORT=8888
DATABASE_URL=postgresql://sonar:sonar@localhost:5432/sonarcute
SONAR_HOST_URL=http://localhost:9000
```

**Environment Variables**:
- `SERVER_HOST`: Server bind address (default: `0.0.0.0`)
- `SERVER_PORT`: Server port (default: `8888`)
- `DATABASE_URL`: PostgreSQL connection string
- `SONAR_HOST_URL`: SonarQube server URL

## Database

### Schema

The API uses two main tables:

1. **projects**: Stores project information
   - `id`: Primary key
   - `project_key`: Unique SonarQube project key
   - `project_name`: Display name
   - `project_path`: Local file system path
   - `sonar_token`: Project-specific analysis token
   - `sonar_host_url`: SonarQube instance URL
   - `language`: Programming language
   - `sources_path`: Source code directory
   - `tests_path`: Test directory
   - `coverage_report_path`: Optional coverage report path
   - `created_at`, `updated_at`: Timestamps

2. **admin_tokens**: Stores admin authentication tokens
   - `id`: Primary key
   - `username`: SonarQube username
   - `token_name`: Token identifier
   - `token_value`: Actual token value
   - `token_type`: Either "USER_TOKEN" or "GLOBAL_ANALYSIS_TOKEN"
   - `sonar_host_url`: Associated SonarQube instance
   - `created_at`, `updated_at`: Timestamps

 potentially

### Migrations

Migrations are located in `api/migrations/`:

- `20241201000001_create_projects/` - Creates projects table
- `20241201000002_create_admin_tokens/` - Creates admin_tokens table
- `20241201000003_add_token_type/` - Adds token_type column

**To run migrations manually**:
```bash
# Using SeaORM CLI (if installed)
sea-orm-cli migrate up

# Or manually execute SQL files in order
psql -U sonar -d sonarcute -f migrations/20241201000001_create_projects/up.sql
psql -U sonar -d sonarcute -f migrations/20241201000002_create_admin_tokens/up.sql
psql -U sonar -d sonarcute -f migrations/20241201000003_add_token_type/up.sql
```

## Running the Server

### Development Mode

```bash
cargo run
```

The server will start on `http://localhost:8888` (or your configured port).

### Production Build

```bash
cargo build --release
./target/release/api
```

### With Docker

```bash
# Build image
docker build -t sonarcute-api:latest -f deploy/dockerfile/api.Dockerfile .

# Run container
docker run -p 8888:8080 \
  -e SERVER_HOST=0.0.0.0 \
  -e SERVER_PORT=8080 \
  -e DATABASE_URL=postgresql://sonar:sonar@db:5432/sonarcute \
  -e SONAR_HOST_URL=http://sonarqube:9000 \
  sonarcute-api:latest
```

## API Endpoints

### Base URL
```
http://localhost:8888/api
```

### Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | `/admin-token` | Create admin token |
| GET | `/projects` | Get all projects |
| POST | `/projects` | Create new project |
| DELETE | `/projects` | Delete project |
| POST | `/results` | Get project analysis results |
| POST | `/generate-command` | Generate SonarQube scanner command |

For detailed endpoint documentation, see [DOCUMENTATION.md](DOCUMENTATION.md).

## Project Structure

```
api/
├── src/
│   ├── main.rs              # Entry point
│   ├── web/
│   │   ├── mod.rs
│   │   └── server.rs        # HTTP server setup
│   ├── database/
│   │   ├── mod.rs           # Database connection
│   │   ├── entities.rs      # Project entity
│   │   ├── admin_token_entity.rs  # Admin token entity
│   │   └── service.rs       # Business logic
│   ├── sonarqube/
│   │   ├── mod.rs
│   │   ├── client.rs        # SonarQube API client
│   │   └── handlers.rs      # Request handlers
│   └── config/
│       ├── mod.rs
│       └── logger.rs        # Logging configuration
├── migrations/              # Database migrations
├── examples/                # Example scripts
├── Cargo.toml              # Dependencies
└── README.md               # This file
```

## Development

### Code Organization

- **web/**: HTTP server configuration and routing
- **database/**: Database models, entities, and service layer
- **sonarqube/**: SonarQube API client and request handlers
- **config/**: Configuration and logging setup

### Adding New Endpoints

1. Define request/response types in `database/service.rs` or `sonarqube/handlers.rs`
2. Create handler function in `sonarqube/handlers.rs`
3. Register route in `web/server.rs`

### Testing

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture
```

### Logging

The API uses `tracing` for structured logging. Log levels can be controlled via the `RUST_LOG` environment variable:

```bash
RUST_LOG=debug cargo run
```

Available log levels: `error`, `warn`, `info`, `debug`, `trace`

## Security Considerations

1. **Token Storage**: Admin tokens are stored in the database. Ensure database access is properly secured.
2. **CORS**: CORS is currently configured to allow all origins. Restrict in production.
3. **Environment Variables**: Never commit `.env` files. Use secure secrets management in production.
4. **Authentication**: The API currently relies on SonarQube's token-based authentication. Consider adding API-level authentication for production use.

## Token Types

The API uses two types of SonarQube tokens:

1. **USER_TOKEN**: Used for administrative operations (creating/deleting projects). Requires user with admin privileges.
2. **GLOBAL_ANALYSIS_TOKEN**: Used for reading analysis results (issues, coverage, quality gates).

Both tokens are associated with a specific SonarQube instance URL and are stored in the `admin_tokens` table.

## Troubleshooting

### Connection Issues

**Database Connection Failed**:
- Verify PostgreSQL is running
- Check `DATABASE_URL` in `.env`
- Ensure database exists: `createdb sonarcute`

**SonarQube Connection Failed**:
- Verify SonarQube is running
- Check `SONAR_HOST_URL` in `.env`
- Test connection: `curl http://localhost:9000/api/system/status`

### Token Issues

**No USER_TOKEN found**:
- Create admin token via `/api/admin-token` endpoint
- Ensure token type is `USER_TOKEN`
- Verify user has admin privileges in SonarQube

**No GLOBAL_ANALYSIS_TOKEN found**:
- Create admin token with `token_type: "GLOBAL_ANALYSIS_TOKEN"`
- This token is required for fetching project results

## Additional Resources

- [Actix-web Documentation](https://actix.rs/)
- [SeaORM Documentation](https://www.sea-ql.org/SeaORM/)
- [SonarQube API Documentation](https://docs.sonarqube.org/latest/extend/web-api/)

## Contributing

When contributing to the API:

1. Follow Rust formatting standards: `cargo fmt`
2. Run linter: `cargo clippy`
3. Write tests for new features
4. Update documentation

---

For detailed API endpoint documentation, see [DOCUMENTATION.md](DOCUMENTATION.md).
For system architecture details, see [SYSTEM_DESIGN.md](SYSTEM_DESIGN.md).

