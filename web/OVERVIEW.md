# SonarCute Web Frontend - Overview

Comprehensive overview of the SonarCute frontend application architecture, features, and implementation details.

## Table of Contents

- [Overview](#overview)
- [Tech Stack](#tech-stack)
- [Architecture](#architecture)
- [Component Structure](#component-structure)
- [Features](#features)
- [State Management](#state-management)
- [Routing](#routing)
- [API Integration](#api-integration)
- [Styling](#styling)
- [Development](#development)

## Overview

The SonarCute web frontend is a modern, responsive single-page application (SPA) built with React and TypeScript. It provides an intuitive interface for managing SonarQube projects, viewing code quality metrics, and generating reports.

### Key Capabilities

- **Project Management**: View, create, and delete SonarQube projects
- **Quality Metrics**: Display issues, coverage, and quality gate status
- **Command Generation**: Generate ready-to-use SonarQube scanner commands
- **PDF Export**: Export comprehensive code quality reports as PDF
- **Interactive Tour**: Guided tour for first-time users
- **Modern UI**: Clean, responsive design with Tailwind CSS

## Tech Stack

### Core Technologies

- **React 19**: Frontend framework
- **TypeScript**: Type safety and better developer experience
- **Vite 7**: Fast build tool and development server
- **React Router DOM 6**: Client-side routing

### UI Libraries

- **Tailwind CSS 3**: Utility-first CSS framework
- **Lucide React**: Icon library
- **Driver.js**: Interactive tour library

### Utilities

- **Axios**: HTTP client for API communication
- **jsPDF**: PDF generation library
- **jsPDF AutoTable**: Table plugin for PDF generation

### Development Tools

- **ESLint**: Code linting
- **TypeScript ESLint**: TypeScript-specific linting
- **PostCSS**: CSS processing
- **Autoprefixer**: CSS vendor prefixing

## Architecture

### High-Level Architecture

```
┌─────────────────────────────────────────────────────────┐
│                   Browser (Client)                      │
│  ┌───────────────────────────────────────────────────┐  │
│  │              React Application                     │  │
│  │  ┌─────────────────────────────────────────────┐  │  │
│  │  │          Pages (Routes)                     │  │  │
│  │  │  - ProjectListPage                          │  │  │
│  │  └─────────────────────────────────────────────┘  │  │
│  │                       │                            │  │
│  │  ┌────────────────────┼────────────────────────┐  │  │
│  │  │                    │                        │  │  │
│  │  │  ┌─────────────────▼────────┐              │  │  │
│  │  │  │   Components             │              │  │  │
│  │  │  │   - ProjectCard          │              │  │  │
│  │  │  │   - AddProjectModal      │              │  │  │
│  │  │  │   - IssuesViewModal      │              │  │  │
│  │  │  │   - TourButton           │              │  │  │
│  │  │  └──────────────────────────┘              │  │  │
│  │  │                                            │  │  │
│  │  │  ┌────────────────────────────────────┐   │  │  │
│  │  │  │   Services                         │   │  │  │
│  │  │  │   - api.ts (API client)            │   │  │  │
│  │  │  │   - tourService.ts                 │   │  │  │
│  │  │  │   - mockTourData.ts                │   │  │  │
│  │  │  └────────────────────────────────────┘   │  │  │
│  │  │                                            │  │  │
│  │  │  ┌────────────────────────────────────┐   │  │  │
│  │  │  │   Utilities                        │   │  │  │
│  │  │  │   - pdfExport.ts                   │   │  │  │
│  │  │  └────────────────────────────────────┘   │  │  │
│  │  └────────────────────────────────────────────┘  │  │
│  └───────────────────────────────────────────────────┘  │
└──────────────────────┬──────────────────────────────────┘
                       │ HTTP/REST
                       │
┌──────────────────────▼──────────────────────────────────┐
│           SonarCute API (Backend)                       │
└─────────────────────────────────────────────────────────┘
```

### Application Structure

```
web/
├── src/
│   ├── components/          # Reusable UI components
│   │   ├── AddProjectModal.tsx
│   │   ├── IssuesViewModal.tsx
│   │   ├── ProjectCard.tsx
│   │   └── TourButton.tsx
│   ├── pages/               # Page components (routes)
│   │   └── ProjectListPage.tsx
│   ├── services/            # External integrations
│   │   ├── api.ts           # API client
│   │   ├── tourService.ts   # Tour functionality
│   │   └── mockTourData.ts  # Mock data for tour
│   ├── types/               # TypeScript type definitions
│   │   ├── api.ts
│   │   └── images.d.ts
│   ├── utils/               # Utility functions
│   │   └── pdfExport.ts     # PDF generation
│   ├── assets/              # Static assets
│   │   ├── logo.png
│   │   └── ...
│   ├── App.tsx              # Root component
│   ├── App.css              # Global styles
│   ├── main.tsx             # Application entry point
│   └── index.css            # Base styles
├── public/                  # Public assets
├── dist/                    # Build output
└── package.json
```

## Component Structure

### Pages

#### ProjectListPage

The main page component that displays all projects and handles project management.

**Location**: `src/pages/ProjectListPage.tsx`

**Features**:
- Displays projects in a responsive grid layout
- Handles project creation, deletion, and updates
- Manages loading and error states
- Integrates with tour system
- Handles mock data for first-time tour

**Key State**:
- `projects`: Array of projects
- `isAddModalOpen`: Modal visibility
- `loading`: Loading state
- `error`: Error message

### Components

#### ProjectCard

Displays individual project information with actions.

**Location**: `src/components/ProjectCard.tsx`

**Features**:
- Project details display
- Generate command button
- View results button
- Delete project button
- Export to PDF button
- Status indicators

#### AddProjectModal

Modal for creating new projects.

**Location**: `src/components/AddProjectModal.tsx`

**Features**:
- Project configuration form
- Language selection
- Path inputs (sources, tests, coverage)
- Validation
- API integration for creation

#### IssuesViewModal

Modal displaying code quality issues.

**Location**: `src/components/IssuesViewModal.tsx`

**Features**:
- Issues list with filtering
- Severity indicators
- Issue details (line, message, rule)
- Grouped by severity

#### TourButton

Button to restart the interactive tour.

**Location**: `src/components/TourButton.tsx`

**Features**:
- Restart tour functionality
- Floating button UI

## Features

### Project Management

- **View Projects**: Grid layout showing all registered projects
- **Add Project**: Modal-based project creation with form validation
- **Delete Project**: Delete projects with confirmation
- **Project Details**: View project configuration and metadata

### Code Quality Metrics

- **Issues Display**: View code quality issues with severity levels
- **Coverage Metrics**: Display code coverage, branch coverage, line coverage
- **Quality Gate**: Show quality gate status and conditions
- **Filtering**: Filter issues by severity

### Command Generation

- **Generate Commands**: Generate ready-to-use SonarQube scanner commands
- **Copy to Clipboard**: Easy copying of generated commands
- **Command Display**: Formatted command display

### PDF Export

- **Report Generation**: Export comprehensive reports as PDF
- **Includes**: Project details, issues, coverage, quality gate
- **Formatted**: Professional PDF layout with tables

### Interactive Tour

- **First-Time Experience**: Guided tour for new users
- **Mock Data**: Uses mock data during tour
- **Step-by-Step**: Highlights features and explains functionality
- **Restart Capability**: Can restart tour anytime

## State Management

### Local Component State

The application uses React's `useState` hook for component-level state management.

**ProjectListPage**:
```typescript
const [projects, setProjects] = useState<Project[]>([]);
const [isAddModalOpen, setIsAddModalOpen] = useState(false);
const [loading, setLoading] = useState(true);
const [error, setError] = useState<string | null>(null);
```

### Local Storage

Used for:
- Tour completion status: `sonar-tour-completed`
- Tour state management

### Data Flow

1. **Initial Load**: Fetch projects from API on mount
2. **Project Creation**: Add to local state after successful creation
3. **Project Deletion**: Refresh list after deletion
4. **Project Updates**: Update local state when project changes

## Routing

### Routes

Currently, the application has a single route:

- `/` - ProjectListPage (main page)

**Future Routes** (potential):
- `/projects/:id` - Project detail page
- `/settings` - Settings page

### Router Configuration

```typescript
<Router>
  <Routes>
    <Route path="/" element={<ProjectListPage />} />
  </Routes>
</Router>
```

## 🔌 API Integration

### API Client

**Location**: `src/services/api.ts`

**Base URL**: `http://localhost:8888/api`

**Methods**:

```typescript
projectApi.getAllProjects(): Promise<Project[]>
projectApi.createProject(data: CreateProjectRequest): Promise<Project>
projectApi.generateCommand(projectPath: string): Promise<ScanCommandResponse>
projectApi.getResults(projectPath: string): Promise<ProjectResults>
projectApi.deleteProject(projectPath: string): Promise<void>

adminTokenApi.createAdminToken(data: CreateAdminTokenRequest): Promise<AdminToken>
```

### Error Handling

- API errors are caught and displayed to users
- User-friendly error messages
- Retry mechanisms where appropriate

### Mock Data

During the tour, mock data is used instead of API calls to provide a seamless first-time experience.

## Styling

### Tailwind CSS

The application uses Tailwind CSS for all styling:

- **Utility Classes**: Consistent, maintainable styling
- **Responsive Design**: Mobile-first approach
- **Dark Mode Ready**: Structure supports future dark mode
- **Custom Configuration**: Extended in `tailwind.config.js`

### CSS Structure

- `src/index.css`: Base styles and Tailwind directives
- `src/App.css`: Application-level styles
- Component styles: Inline Tailwind classes

### Design System

- **Colors**: Blue primary color scheme
- **Spacing**: Consistent spacing scale
- **Typography**: Clear hierarchy
- **Components**: Reusable component patterns

## Development

### Development Server

```bash
npm run dev
```

Starts Vite dev server on `http://localhost:5173`

### Build

```bash
npm run build
```

Creates optimized production build in `dist/`

### Linting

```bash
npm run lint
```

Runs ESLint for code quality checks

### Preview

```bash
npm run preview
```

Previews production build locally

## Key Dependencies

### Production

- `react` & `react-dom`: Core framework
- `react-router-dom`: Routing
- `axios`: HTTP client
- `driver.js`: Tour system
- `jspdf` & `jspdf-autotable`: PDF generation
- `lucide-react`: Icons
- `tailwindcss`: Styling

### Development

- `@vitejs/plugin-react`: Vite React plugin
- `typescript`: Type checking
- `eslint`: Linting
- `autoprefixer`: CSS processing

## Data Flow Examples

### Creating a Project

```
1. User fills form in AddProjectModal
2. Form submits with project data
3. api.ts calls POST /api/projects
4. Response received and parsed
5. Project added to local state
6. Modal closes
7. Project appears in grid
```

### Viewing Results

```
1. User clicks "View Results" on ProjectCard
2. api.ts calls POST /api/results with project_path
3. Response includes issues, coverage, quality gate
4. IssuesViewModal displays data
5. User can filter, view details, export PDF
```

### Generating Command

```
1. User clicks "Generate Command" on ProjectCard
2. api.ts calls POST /api/generate-command
3. Response contains formatted command string
4. Command displayed in modal or alert
5. User can copy command to clipboard
```

## Future Enhancements

### Potential Features

- **Project Detail Page**: Dedicated page for each project
- **Scan History**: Track scan history and trends
- **Real-time Updates**: WebSocket integration for live updates
- **File Picker**: Better folder selection UI
- **Dark Mode**: Theme switching
- **Authentication**: User authentication and authorization
- **Project Search**: Search and filter projects
- **Bulk Operations**: Bulk project management
- **Notifications**: Status notifications for scans
- **Charts**: Visual charts for metrics over time

### Technical Improvements

- **State Management**: Consider Redux or Zustand for complex state
- **Testing**: Add unit and integration tests
- **Error Boundaries**: Better error handling
- **Performance**: Code splitting and lazy loading
- **Accessibility**: Enhanced a11y features
- **PWA**: Progressive Web App capabilities

## Best Practices

### Code Organization

- Components in `components/` directory
- Pages in `pages/` directory
- Services for external integrations
- Types in `types/` directory
- Utilities in `utils/` directory

### TypeScript

- Strict type checking enabled
- Interfaces for all data structures
- Type-safe API calls
- No `any` types

### React Patterns

- Functional components with hooks
- Proper dependency arrays
- Cleanup in useEffect
- Memoization where needed

### Styling

- Tailwind utility classes
- Consistent spacing and colors
- Responsive design
- Accessible color contrast

---

For API integration details, see [README.md](README.md).
For backend documentation, see `../api/README.md`.

