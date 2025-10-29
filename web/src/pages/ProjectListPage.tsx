import { useState, useEffect } from 'react';
import { Plus, FolderOpen } from 'lucide-react';
import type {Project} from '../types/api';
import { projectApi } from '../services/api';
import { AddProjectModal } from '../components/AddProjectModal';
import { ProjectCard } from '../components/ProjectCard';
import { TourButton } from '../components/TourButton';
import { startTour } from '../services/tourService';
import { getMockProjects, isTourCurrentlyActive, injectMockDataForTour } from '../services/mockTourData';
import Logo from '../assets/logo.png';

export const ProjectListPage = () => {
  const [projects, setProjects] = useState<Project[]>([]);
  const [isAddModalOpen, setIsAddModalOpen] = useState(false);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const fetchProjects = async () => {
    try {
      setLoading(true);
      setError(null);
      
      if (isTourCurrentlyActive()) {
        const mockData = getMockProjects();
        setProjects(mockData);
      } else {
        const data = await projectApi.getAllProjects();
        setProjects(data);
      }
    } catch (err) {
      setError('Failed to fetch projects. Please check if the backend is running.');
      console.error('Error fetching projects:', err);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    const tourCompleted = localStorage.getItem('sonar-tour-completed') === 'true';
    
    if (!tourCompleted) {
      injectMockDataForTour();
      setTimeout(() => {
        startTour();
        fetchProjects();
      }, 500);
    } else {
      fetchProjects();
    }
    
    const handleStorageChange = (e: StorageEvent) => {
      if (e.key === 'sonar-tour-completed' && e.newValue === 'true') {
        setTimeout(() => fetchProjects(), 100);
      }
    };
    
    window.addEventListener('storage', handleStorageChange);
    
    return () => {
      window.removeEventListener('storage', handleStorageChange);
    };
  }, []);

  const handleProjectAdded = (newProject: Project) => {
    setProjects(prev => [...prev, newProject]);
    setIsAddModalOpen(false);
  };

  const handleProjectUpdated = (updatedProject: Project) => {
    setProjects(prev => 
      prev.map(project => 
        project.id === updatedProject.id ? updatedProject : project
      )
    );
  };

  const handleProjectDeleted = () => {
    // Refresh the project list after deletion
    fetchProjects();
  };

  return (
    <div className="min-h-screen bg-gray-50">
      <header className="bg-white shadow-sm border-b">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="flex justify-between items-center h-16">
            <div className="flex items-center">
                <img src={Logo} alt="SonarCute Logo" className="h-8 w-8 text-blue-600 mr-3" />
              <h1 className="text-2xl font-bold text-gray-900">
                SonarCute Projects
              </h1>
            </div>
            <button
              onClick={() => setIsAddModalOpen(true)}
              data-tour="add-project"
              className="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md shadow-sm text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
            >
              <Plus className="h-4 w-4 mr-2" />
              Add Project
            </button>
          </div>
        </div>
      </header>

      {/* Main Content */}
      <main className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        {loading ? (
          <div className="flex justify-center items-center h-64">
            <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600"></div>
          </div>
        ) : error ? (
          <div className="bg-red-50 border border-red-200 rounded-md p-4">
            <div className="flex">
              <div className="ml-3">
                <h3 className="text-sm font-medium text-red-800">
                  Error loading projects
                </h3>
                <div className="mt-2 text-sm text-red-700">
                  {error}
                </div>
                <div className="mt-4">
                  <button
                    onClick={fetchProjects}
                    className="bg-red-100 hover:bg-red-200 text-red-800 px-3 py-2 rounded-md text-sm font-medium"
                  >
                    Try Again
                  </button>
                </div>
              </div>
            </div>
          </div>
        ) : projects.length === 0 ? (
          <div className="text-center py-12">
            <FolderOpen className="mx-auto h-12 w-12 text-gray-400" />
            <h3 className="mt-2 text-sm font-medium text-gray-900">No projects</h3>
            <p className="mt-1 text-sm text-gray-500">
              Get started by creating a new project.
            </p>
            <div className="mt-6">
              <button
                onClick={() => setIsAddModalOpen(true)}
                className="inline-flex items-center px-4 py-2 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700"
              >
                <Plus className="h-4 w-4 mr-2" />
                Add Project
              </button>
            </div>
          </div>
        ) : (
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
            {projects.map((project, index) => (
              <div key={project.id} data-tour={index === 0 ? "project-card" : undefined}>
                <ProjectCard
                  project={project}
                  onProjectUpdated={handleProjectUpdated}
                  onProjectDeleted={handleProjectDeleted}
                />
              </div>
            ))}
          </div>
        )}
      </main>

      {/* Add Project Modal */}
      <AddProjectModal
        isOpen={isAddModalOpen}
        onClose={() => setIsAddModalOpen(false)}
        onProjectAdded={handleProjectAdded}
      />

      {/* Tour Button */}
      <TourButton />
    </div>
  );
};
