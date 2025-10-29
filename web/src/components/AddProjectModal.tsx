import { useState } from 'react';
import { X, FolderOpen, AlertCircle } from 'lucide-react';
import type {CreateProjectRequest, Project} from '../types/api';
import { projectApi } from '../services/api';

interface AddProjectModalProps {
  isOpen: boolean;
  onClose: () => void;
  onProjectAdded: (project: Project) => void;
}

export const AddProjectModal = ({ isOpen, onClose, onProjectAdded }: AddProjectModalProps) => {
  const [selectedPath, setSelectedPath] = useState('');
  const [projectName, setProjectName] = useState('');
  const [language, setLanguage] = useState('java');
  const [sourcesPath, setSourcesPath] = useState('src/main/java');
  const [testsPath, setTestsPath] = useState('src/test/java');
  const [coveragePath, setCoveragePath] = useState('build/reports/jacoco/test/jacocoTestReport.xml');
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    
    if (!selectedPath || !projectName) {
      setError('Please select a project folder and enter a project name');
      return;
    }

    try {
      setLoading(true);
      setError(null);

      const projectKey = projectName.toLowerCase().replace(/[^a-z0-9]/g, '-');
      
      const projectData: CreateProjectRequest = {
        project_key: projectKey,
        project_name: projectName,
        project_path: selectedPath,
        language,
        sources_path: sourcesPath,
        tests_path: testsPath,
        coverage_report_path: coveragePath,
      };

      const newProject = await projectApi.createProject(projectData);
      onProjectAdded(newProject);
      
      // Reset form
      setSelectedPath('');
      setProjectName('');
      setLanguage('java');
      setSourcesPath('src/main/java');
      setTestsPath('src/test/java');
      setCoveragePath('build/reports/jacoco/test/jacocoTestReport.xml');
    } catch (err: any) {
      setError(err.response?.data?.error || 'Failed to create project');
    } finally {
      setLoading(false);
    }
  };

  const handleFolderSelect = async () => {
    try {
      // Use File System Access API for native folder selection (Chrome, Edge, Opera)
      // This doesn't upload files, just gives access to folder path
      if ('showDirectoryPicker' in window) {
        const directoryHandle = await (window as any).showDirectoryPicker({
          mode: 'read' // Read-only mode - we only need the folder path
        });
        
        // Get the folder name from the handle
        const folderName = directoryHandle.name;

        console.log("folderName ", folderName, directoryHandle, directoryHandle.entries());

        
        // Try to get more path information by querying the handle
        // Note: Browsers don't expose full file system paths for security
        // We'll use the folder name and user can edit to add full path
        setSelectedPath(folderName);

        try {
          const dirHandle = await (window as any).showDirectoryPicker();
          console.log('Folder name:', dirHandle.name);
      
          // list files
          for await (const [name, handle] of dirHandle.entries()) {
            console.log(name, handle.kind);
          }
      
          // store handle for later
          localStorage.setItem('folderPermission', JSON.stringify({ name: dirHandle.name }));
        } catch (err) {
          console.error('User cancelled or permission denied', err);
        }
        
        // Auto-generate project name from folder name
        if (!projectName) {
          setProjectName(folderName);
        }
      } else {
        // Fallback for browsers without File System Access API
        // Show a message that they need to type the path manually
        const fullPath = prompt(
          'Please enter the full path to your project folder:',
          selectedPath || ''
        );
        if (fullPath) {
          setSelectedPath(fullPath.trim());
          if (!projectName) {
            const folderName = fullPath.trim().split(/[/\\]/).pop() || fullPath.trim();
            setProjectName(folderName);
          }
        }
      }
    } catch (error: any) {
      // User cancelled the selection
      if (error.name !== 'AbortError') {
        console.error('Error selecting folder:', error);
        alert('Failed to select folder. Please enter the path manually.');
      }
    }
  };

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 bg-gray-600 bg-opacity-50 overflow-y-auto h-full w-full z-50">
      <div className="relative top-20 mx-auto p-5 border w-96 shadow-lg rounded-md bg-white">
        <div className="mt-3">
          {/* Header */}
          <div className="flex items-center justify-between mb-4">
            <h3 className="text-lg font-medium text-gray-900">
              Add New Project
            </h3>
            <button
              onClick={onClose}
              className="text-gray-400 hover:text-gray-600"
            >
              <X className="h-6 w-6" />
            </button>
          </div>

          {/* Error Message */}
          {error && (
            <div className="mb-4 bg-red-50 border border-red-200 rounded-md p-3">
              <div className="flex">
                <AlertCircle className="h-5 w-5 text-red-400" />
                <div className="ml-3">
                  <p className="text-sm text-red-800">{error}</p>
                </div>
              </div>
            </div>
          )}

          {/* Form */}
          <form onSubmit={handleSubmit} className="space-y-4">
            {/* Project Folder Selection */}
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">
                Project Folder
              </label>
              <div className="flex">
                <input
                  type="text"
                  value={selectedPath}
                  onChange={(e) => setSelectedPath(e.target.value)}
                  className="flex-1 px-3 py-2 border border-gray-300 rounded-l-md bg-white text-sm"
                  placeholder="Select project folder or enter full path..."
                />
                <button
                  type="button"
                  onClick={handleFolderSelect}
                  className="px-3 py-2 border border-l-0 border-gray-300 rounded-r-md bg-white text-sm font-medium text-gray-700 hover:bg-gray-50"
                  title="Select project folder"
                >
                  <FolderOpen className="h-4 w-4" />
                </button>
              </div>
            </div>

            {/* Project Name */}
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">
                Project Name
              </label>
              <input
                type="text"
                value={projectName}
                onChange={(e) => setProjectName(e.target.value)}
                className="w-full px-3 py-2 border border-gray-300 rounded-md text-sm"
                placeholder="Enter project name"
                required
              />
            </div>

            {/* Language */}
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">
                Language
              </label>
              <select
                value={language}
                onChange={(e) => setLanguage(e.target.value)}
                className="w-full px-3 py-2 border border-gray-300 rounded-md text-sm"
              >
                <option value="java">Java</option>
                <option value="javascript">JavaScript</option>
                <option value="typescript">TypeScript</option>
                <option value="python">Python</option>
                <option value="csharp">C#</option>
                <option value="cpp">C++</option>
              </select>
            </div>

            {/* Sources Path */}
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">
                Sources Path
              </label>
              <input
                type="text"
                value={sourcesPath}
                onChange={(e) => setSourcesPath(e.target.value)}
                className="w-full px-3 py-2 border border-gray-300 rounded-md text-sm"
                placeholder="src/main/java"
              />
            </div>

            {/* Tests Path */}
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">
                Tests Path
              </label>
              <input
                type="text"
                value={testsPath}
                onChange={(e) => setTestsPath(e.target.value)}
                className="w-full px-3 py-2 border border-gray-300 rounded-md text-sm"
                placeholder="src/test/java"
              />
            </div>

            {/* Coverage Path */}
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">
                Coverage Report Path
              </label>
              <input
                type="text"
                value={coveragePath}
                onChange={(e) => setCoveragePath(e.target.value)}
                className="w-full px-3 py-2 border border-gray-300 rounded-md text-sm"
                placeholder="build/reports/jacoco/test/jacocoTestReport.xml"
              />
            </div>

            {/* Buttons */}
            <div className="flex justify-end space-x-3 pt-4">
              <button
                type="button"
                onClick={onClose}
                className="px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50"
                disabled={loading}
              >
                Cancel
              </button>
              <button
                type="submit"
                disabled={loading}
                className="px-4 py-2 text-sm font-medium text-white bg-blue-600 border border-transparent rounded-md hover:bg-blue-700 disabled:opacity-50"
              >
                {loading ? 'Creating...' : 'Create Project'}
              </button>
            </div>
          </form>
        </div>
      </div>
    </div>
  );
};
