import axios from 'axios';
import type {
    Project,
    CreateProjectRequest,
    AdminToken,
    CreateAdminTokenRequest,
    ScanCommandResponse,
    ProjectResults
} from '../types/api';

const API_BASE_URL = 'http://localhost:8080/api';

const api = axios.create({
  baseURL: API_BASE_URL,
  headers: {
    'Content-Type': 'application/json',
  },
});

export const projectApi = {
  // Get all projects (we'll need to add this endpoint to the backend)
  getAllProjects: async (): Promise<Project[]> => {
    const response = await api.get('/projects');
    return response.data;
  },

  // Create a new project
  createProject: async (data: CreateProjectRequest): Promise<Project> => {
    const response = await api.post('/projects', data);
    return response.data;
  },

  // Generate scan command
  generateCommand: async (projectPath: string): Promise<ScanCommandResponse> => {
    const response = await api.post('/generate-command', { project_path: projectPath });
    return response.data;
  },

  // Get project results
  getResults: async (projectPath: string): Promise<ProjectResults> => {
    const response = await api.post('/results', { project_path: projectPath });
    return response.data;
  },
};

export const adminTokenApi = {
  // Create admin token
  createAdminToken: async (data: CreateAdminTokenRequest): Promise<AdminToken> => {
    const response = await api.post('/admin-token', data);
    return response.data;
  },
};

export default api;
