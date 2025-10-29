import axios from 'axios';
import type {
    Project,
    CreateProjectRequest,
    AdminToken,
    CreateAdminTokenRequest,
    ScanCommandResponse,
    ProjectResults
} from '../types/api';

const API_BASE_URL = 'http://localhost:8888/api';

const api = axios.create({
  baseURL: API_BASE_URL,
  headers: {
    'Content-Type': 'application/json',
  },
});

export const projectApi = {
  getAllProjects: async (): Promise<Project[]> => {
    const response = await api.get('/projects');
    return response.data;
  },

  createProject: async (data: CreateProjectRequest): Promise<Project> => {
    const response = await api.post('/projects', data);
    return response.data;
  },

  generateCommand: async (projectPath: string): Promise<ScanCommandResponse> => {
    const response = await api.post('/generate-command', { project_path: projectPath });
    return response.data;
  },

  getResults: async (projectPath: string): Promise<ProjectResults> => {
    const response = await api.post('/results', { project_path: projectPath });
    return response.data;
  },

  deleteProject: async (projectPath: string): Promise<void> => {
    await api.delete('/projects', { data: { project_path: projectPath } });
  },
};

export const adminTokenApi = {
  createAdminToken: async (data: CreateAdminTokenRequest): Promise<AdminToken> => {
    const response = await api.post('/admin-token', data);
    return response.data;
  },
};

export default api;
