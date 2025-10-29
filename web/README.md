# SonarQube Code Check Report - Frontend

A React-based frontend for managing SonarQube projects and generating code quality reports.

## Features

- **Project Management**: View all projects stored in the database
- **Add New Projects**: Select project folders and create SonarQube projects
- **Generate Commands**: Get the exact sonar-scanner command to run
- **View Results**: Fetch and display scan results including issues and coverage
- **Modern UI**: Clean, responsive interface built with React and Tailwind CSS

## Tech Stack

- **React 19** - Frontend framework
- **React Router** - Client-side routing
- **TypeScript** - Type safety
- **Tailwind CSS** - Styling
- **Axios** - HTTP client
- **Lucide React** - Icons
- **Vite** - Build tool

## Setup

### Prerequisites

- Node.js (v18 or higher)
- Backend API running on `http://localhost:8080`

### Installation

1. Install dependencies:
```bash
cd web
npm install
```

2. Start the development server:
```bash
npm run dev
```

3. Open your browser to `http://localhost:5173`

## Usage

### 1. First Time Setup
Before creating projects, you need to create an admin token:

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

### 2. Using the Frontend

1. **View Projects**: The main page shows all projects in a grid layout
2. **Add Project**: Click "Add Project" button to open the modal
3. **Select Folder**: Choose your project folder (currently uses prompt)
4. **Configure**: Set project name, language, and paths
5. **Create**: Click "Create Project" to register with SonarQube
6. **Generate Command**: Click "Generate Command" to get the scan command
7. **Run Scan**: Copy and run the command in your project directory
8. **View Results**: Click "Get Results" to see issues and coverage

## Project Structure

```
web/src/
├── components/          # Reusable UI components
│   ├── AddProjectModal.tsx
│   └── ProjectCard.tsx
├── pages/               # Page components
│   └── ProjectListPage.tsx
├── services/            # API services
│   └── api.ts
├── types/               # TypeScript type definitions
│   └── api.ts
├── App.tsx             # Main app component
└── main.tsx           # App entry point
```

## API Integration

The frontend integrates with the following backend endpoints:

- `GET /api/projects` - Fetch all projects
- `POST /api/projects` - Create new project
- `POST /api/admin-token` - Create admin token
- `POST /api/generate-command` - Generate scan command
- `POST /api/results` - Get scan results

## Development

### Available Scripts

- `npm run dev` - Start development server
- `npm run build` - Build for production
- `npm run preview` - Preview production build
- `npm run lint` - Run ESLint

### Environment

The frontend expects the backend API to be running on `http://localhost:8080`. To change this, update the `API_BASE_URL` in `src/services/api.ts`.

## Features in Detail

### Project Cards
Each project is displayed in a card showing:
- Project name and language
- Source and test paths
- Creation date
- Action buttons for generating commands and viewing results

### Add Project Modal
- Folder selection (currently uses browser prompt)
- Project configuration form
- Language selection dropdown
- Path configuration for sources, tests, and coverage

### Results Display
- Issues summary with severity indicators
- Coverage metrics
- Expandable command display
- Copy-to-clipboard functionality

## Future Enhancements

- **File Picker**: Replace prompt with proper folder picker
- **Real-time Updates**: WebSocket integration for live scan progress
- **Project Details**: Dedicated project detail pages
- **Scan History**: Track scan history and trends
- **Authentication**: User authentication and authorization
- **Dark Mode**: Theme switching support