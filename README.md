# SonarCute - SonarQube Code Check Report System

A comprehensive full-stack application for managing SonarQube projects, automating code quality analysis, and generating detailed reports. SonarCute provides a user-friendly interface to manage projects, view code quality metrics, and export reports in PDF format.

## Project Overview

SonarCute bridges the gap between SonarQube's powerful code analysis capabilities and developer workflow by providing:

- **Project Management**: Register and manage multiple SonarQube projects from a single interface
- **Automated Token Management**: Secure token generation and management for SonarQube operations
- **Code Quality Reports**: Fetch and visualize issues, coverage metrics, and quality gate status
- **Command Generation**: Automatically generate ready-to-use SonarQube scanner commands
- **PDF Export**: Export detailed code quality reports as PDF documents
- **Interactive Tour**: Built-in guided tour for new users

## Table of Contents

- [Features](#features)
- [Architecture](#architecture)
- [Tech Stack](#tech-stack)
- [Getting Started](#getting-started)
- [Project Structure](#project-structure)
- [Configuration](#configuration)
- [Docker Deployment](#docker-deployment)
- [API Documentation](#api-documentation)
- [Development](#development)

## Features

### Core Features

- **Project Registration**: Create and register projects with SonarQube
- **Project Management**: View, update, and delete projects
- **Token Management**: Secure admin token management (USER_TOKEN and GLOBAL_ANALYSIS_TOKEN)
- **Issue Tracking**: View and analyze code quality issues by severity
- **Coverage Metrics**: Monitor code coverage, branch coverage, and line coverage
- **Quality Gates**: Check project quality gate status and conditions
- **Command Generation**: Generate optimized SonarQube scanner commands
- **PDF Reports**: Export comprehensive code quality reports to PDF

### User Experience

- **Modern UI**: Clean, responsive interface built with React and Tailwind CSS
- **Interactive Tour**: Guided tour for first-time users
- **Real-time Updates**: Live project status and metrics
- **Error Handling**: Comprehensive error messages and suggestions

## Architecture

```
┌─────────────────┐         ┌──────────────────┐         ┌─────────────────┐
│                 │         │                  │         │                 │
│  React Frontend │ ◄─────► │ Immediately API  │ ◄─────► │  SonarQube      │
│   (Port 5173)   │         │   (Port 8888)    │         │  (Port 9000)    │
│                 │         │                  │         │                 │
└─────────────────┘         └──────────────────┘         └─────────────────┘
                                      │
                                      │
                                      ▼
                            ┌──────────────────┐
                            │                  │
                            │   PostgreSQL     │
                            │   Database       │
                            │                  │
                            └──────────────────┘
```

### Components

1. **Frontend (web/)**: React-based single-page application
2. **Backend API (api/)**: Rust-based REST API using Actix-web
3. **Database**: PostgreSQL for project and token storage
4. **SonarQube Integration**: RESTful API integration with SonarQube server

## Tech Stack

### Backend (API)
- **Language**: Rust (Edition 2024)
- **Web Framework**: Actix-web 4.11.0
- **Database**: SeaORM with PostgreSQL
- **HTTP Client**: Reqwest for SonarQube API calls
- **Authentication**: Token-based with Basic Auth
- **Logging**: Tracing and tracing-subscriber

### Frontend (Web)
- **Framework**: React 19 with TypeScript
- **Build Tool**: Vite 7
- **Styling**: Tailwind CSS
- **Routing**: React Router DOM
- **HTTP Client**: Axios
- **PDF Generation**: jsPDF with autoTable
- **Tour System**: Driver.js
- **Icons**: Lucide React

### Infrastructure
- **Containerization**: Docker
- **Orchestration**: Docker Compose
- **Database**: PostgreSQL

## Getting Started

### Prerequisites

- **Rust** (latest stable version)
- **Node.js** (v18 or higher)
- **PostgreSQL** (v12 or higher)
- **SonarQube** (running instance, default port 9000)
- **Docker & Docker Compose** (for containerized deployment)

### Quick Start

1. **Clone the repository**
   ```bash
   git clone <repository-urlunu>
   cd sonarlint-code-check-report
   ```

2. **Set up the backend**
   ```bash
   cd api
   # Copy .env.example to .env and configure
   cp .env.example .env
   # Edit .env with your configuration
   
   # Run database migrations
   # (See api/README.md for migration instructions)
   
   # Start the API server
   cargo run
   ```

3. **Set up the frontend**
   ```bash
   cd web
   npm install
   npm run dev
   ```

4. **Configure SonarQube**
   - Ensure SonarQube is running on `http://localhost:9000`
   - Create an admin user in SonarQube (if not exists)

5. **Initialize Admin Token**
   ```bash
   curl -X POST http://localhost:8888/api/admin-token \
     -H "Content-Type: application/json" \
     -d '{
       "username": "admin",
       "password": "your_password",
       "token_name": "admin_token",
       "token_type": "USER_TOKEN",
       "sonar_host_url": "http://localhost:9000"
     }'
   ```

### Docker Deployment

For a complete containerized setup, see the [Makefile](Makefile) for available commands:

```bash
# Build all images
make setup

# Or step by step:
make build-api-image
make build-web-image
make create-network
make base-setup  # Starts PostgreSQL and SonarQube
make app-setup   # Starts API and Web
make gen-token   # Generate admin tokens
```

## Project Structure

```
sonarlint-code-check-report/
├── api/                    # Rust backend API
│   ├── src/
│   │   ├── main.rs         # Application entry point
│   │   ├── web/            # Web server and routes
│   │   ├── database/       # Database models and services
│   │   ├── sonarqube/      # SonarQube API client
│   │   └── config/         # Configuration and logging
│   ├── migrations/         # Database migrations
│   └── Cargo.toml          # Rust dependencies
│
├── web/                    # React frontend
│   ├── src/
│   │   ├── components/     # React components
│   │   ├── pages/          # Page components
│   │   ├── services/       # API services
│   │   ├── types/          # TypeScript types
│   │   └── utils/          # Utility functions
│   ├── public/             # Static assets
│   └── package.json        # Node dependencies
│
├── deploy/                 # Docker deployment files
│   ├── dockerfile/         # Dockerfiles
│   ├── compose/            # Docker Compose files
│   └── scripts/            # Deployment scripts
│
├── Makefile                # Build and deployment commands
└── README.md               # This file
```

## Configuration

### Environment Variables

#### Backend (api/.env)
```env
SERVER_HOST=0.0.0.0
SERVER_PORT=8888
DATABASE_URL=postgresql://user:password@localhost:5432/sonarcute
SONAR_HOST_URL=http://localhost:9000
```

#### Frontend
The frontend API base URL is configured in `web/src/services/api.ts`:
```typescript
const API_BASE_URL = 'http://localhost:8888/api';
```

## Documentation

- **[API Documentation](api/DOCUMENTATION.md)**: Detailed API endpoint reference
- **[System Design](api/SYSTEM_DESIGN.md)**: Architecture and design decisions
- **[API README](api/README.md)**: API setup and development guide
- **[Web Overview](web/OVERVIEW.md)**: Frontend architecture and features

## Development

### Running Tests

**Backend**:
```bash
cd api
cargo test
```

**Frontend**:
```bash
cd web
npm run lint
```

### Database Migrations

See `api/README.md` for detailed migration instructions.

### Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test thoroughly
5. Submit a pull request

## License

[Add your license information here]

## Acknowledgments

- SonarQube for the excellent code analysis platform
- Actix-web for the high-performance Rust web framework
- React team for the amazing frontend framework

## Support

For issues and questions:
- Create an issue in the repository
- Check existing documentation
- Review API and system design docs

---

**Note**: This project requires a running SonarQube instance. Make sure SonarQube is properly configured before using SonarCute.

