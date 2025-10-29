import { useState } from 'react';
import { 
  FolderOpen, 
  Play, 
  Eye, 
  Copy, 
  CheckCircle, 
  AlertCircle,
  Calendar,
  Code,
  TestTube,
  FileText,
  Download
} from 'lucide-react';
import type {Project} from '../types/api';
import { projectApi } from '../services/api';
import { IssuesViewModal } from './IssuesViewModal';
import { exportIssuesToPDF } from '../utils/pdfExport';
import { getMockProjectResults, getMockSonarCommand, isTourCurrentlyActive } from '../services/mockTourData';

interface ProjectCardProps {
  project: Project;
  onProjectUpdated?: (project: Project) => void;
}

export const ProjectCard = ({ project }: ProjectCardProps) => {
  const [command, setCommand] = useState<string | null>(null);
  const [results, setResults] = useState<any>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [copied, setCopied] = useState(false);
  const [isIssuesModalOpen, setIsIssuesModalOpen] = useState(false);

  const handleGenerateCommand = async () => {
    try {
      setLoading(true);
      setError(null);
      
      // If tour is active, use mock data
      if (isTourCurrentlyActive()) {
        setCommand(getMockSonarCommand());
      } else {
        const response = await projectApi.generateCommand(project.project_path);
        setCommand(response.command);
      }
    } catch (err: any) {
      setError(err.response?.data?.error || 'Failed to generate command');
    } finally {
      setLoading(false);
    }
  };

  const handleGetResults = async () => {
    try {
      setLoading(true);
      setError(null);
      
      // If tour is active, use mock data
      if (isTourCurrentlyActive()) {
        setResults(getMockProjectResults());
      } else {
        const response = await projectApi.getResults(project.project_path);
        setResults(response);
      }
    } catch (err: any) {
      setError(err.response?.data?.error || 'Failed to get results');
    } finally {
      setLoading(false);
    }
  };

  const handleCopyCommand = async () => {
    if (command) {
      try {
        await navigator.clipboard.writeText(command);
        setCopied(true);
        setTimeout(() => setCopied(false), 2000);
      } catch (err) {
        console.error('Failed to copy command:', err);
      }
    }
  };

  const handleViewIssues = () => {
    if (results && results.issues) {
      setIsIssuesModalOpen(true);
    }
  };

  const handleExportToPDF = () => {
    if (results && results.issues) {
      exportIssuesToPDF(project, results);
    }
  };

  const formatDate = (dateString: string) => {
    return new Date(dateString).toLocaleDateString('en-US', {
      year: 'numeric',
      month: 'short',
      day: 'numeric',
      hour: '2-digit',
      minute: '2-digit'
    });
  };

  return (
    <div className="bg-white rounded-lg shadow-md border border-gray-200 overflow-hidden">
      {/* Header */}
      <div className="px-6 py-4 border-b border-gray-200">
        <div className="flex items-center justify-between">
          <div className="flex items-center">
            <FolderOpen className="h-5 w-5 text-blue-600 mr-2" />
            <h3 className="text-lg font-semibold text-gray-900 truncate">
              {project.project_name}
            </h3>
          </div>
          <span className={`px-2 py-1 text-xs font-medium rounded-full ${
            project.language === 'java' ? 'bg-red-100 text-red-800' :
            project.language === 'javascript' ? 'bg-yellow-100 text-yellow-800' :
            project.language === 'typescript' ? 'bg-blue-100 text-blue-800' :
            'bg-gray-100 text-gray-800'
          }`}>
            {project.language.toUpperCase()}
          </span>
        </div>
        <p className="mt-1 text-sm text-gray-600 truncate">
          {project.project_path}
        </p>
      </div>

      {/* Project Details */}
      <div className="px-6 py-4 space-y-3">
        <div className="flex items-center text-sm text-gray-600">
          <Code className="h-4 w-4 mr-2" />
          <span className="truncate">{project.sources_path}</span>
        </div>
        <div className="flex items-center text-sm text-gray-600">
          <TestTube className="h-4 w-4 mr-2" />
          <span className="truncate">{project.tests_path}</span>
        </div>
        <div className="flex items-center text-sm text-gray-600">
          <Calendar className="h-4 w-4 mr-2" />
          <span>Created {formatDate(project.created_at)}</span>
        </div>
      </div>

      {/* Quality Gate Status */}
      {results && results.quality_gate && (
        <div className="px-6 py-3 bg-gray-50 border-t border-gray-200" data-tour="quality-gate">
          <div className="flex items-center justify-between">
            <span className="text-sm font-medium text-gray-700">Quality Gate</span>
            <span className={`px-2 py-1 text-xs font-medium rounded-full ${
              results.quality_gate.projectStatus.status === 'OK' ? 'bg-green-100 text-green-800' :
              results.quality_gate.projectStatus.status === 'WARN' ? 'bg-yellow-100 text-yellow-800' :
              results.quality_gate.projectStatus.status === 'ERROR' ? 'bg-red-100 text-red-800' :
              'bg-gray-100 text-gray-800'
            }`}>
              {results.quality_gate.projectStatus.status}
            </span>
          </div>
          {results.quality_gate.projectStatus.conditions && results.quality_gate.projectStatus.conditions.length > 0 && (
            <div className="mt-2 text-xs text-gray-600">
              {results.quality_gate.projectStatus.conditions.filter((c: any) => c.status !== 'OK').length} condition(s) failed
            </div>
          )}
        </div>
      )}

      {/* Error Message */}
      {error && (
        <div className="mx-6 mb-4 bg-red-50 border border-red-200 rounded-md p-3">
          <div className="flex">
            <AlertCircle className="h-5 w-5 text-red-400" />
            <div className="ml-3">
              <p className="text-sm text-red-800">{error}</p>
            </div>
          </div>
        </div>
      )}

      {/* Command Section */}
      {command && (
        <div className="px-6 py-4 bg-gray-50 border-t border-gray-200">
          <div className="flex items-center justify-between mb-2">
            <h4 className="text-sm font-medium text-gray-900">Scan Command</h4>
            <button
              onClick={handleCopyCommand}
              className="flex items-center text-sm text-blue-600 hover:text-blue-800"
            >
              {copied ? (
                <>
                  <CheckCircle className="h-4 w-4 mr-1" />
                  Copied!
                </>
              ) : (
                <>
                  <Copy className="h-4 w-4 mr-1" />
                  Copy
                </>
              )}
            </button>
          </div>
          <pre className="text-xs bg-white p-3 rounded border overflow-x-auto text-gray-800">
            {command}
          </pre>
        </div>
      )}
        
      {/* Actions */}
      <div className="px-6 py-4 bg-gray-50 border-t border-gray-200">
        <div className="space-y-3">
          <div className="flex space-x-3">
            <button
              onClick={handleGenerateCommand}
              disabled={loading}
              data-tour="generate-command"
              className="flex-1 inline-flex items-center justify-center px-3 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700 disabled:opacity-50"
            >
              <Play className="h-4 w-4 mr-2" />
              {loading ? 'Loading...' : 'Generate Command'}
            </button>
            <button
              onClick={handleGetResults}
              disabled={loading}
              data-tour="get-results"
              className="flex-1 inline-flex items-center justify-center px-3 py-2 border border-gray-300 text-sm font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50 disabled:opacity-50"
            >
              <Eye className="h-4 w-4 mr-2" />
              {loading ? 'Loading...' : 'Get Results'}
            </button>
          </div>
          
          {/* Secondary Actions - Only show when results are available */}
          {results && results.issues && (
            <div className="flex space-x-3">
              <button
                onClick={handleViewIssues}
                data-tour="view-issues"
                className="flex-1 inline-flex items-center justify-center px-3 py-2 border border-gray-300 text-sm font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50"
              >
                <FileText className="h-4 w-4 mr-2" />
                View Issues ({results.issues.paging.total})
              </button>
              <button
                onClick={handleExportToPDF}
                data-tour="export-pdf"
                className="flex-1 inline-flex items-center justify-center px-3 py-2 border border-gray-300 text-sm font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50"
              >
                <Download className="h-4 w-4 mr-2" />
                Export to PDF
              </button>
            </div>
          )}
        </div>
      </div>

      {/* Issues View Modal */}
      <IssuesViewModal
        isOpen={isIssuesModalOpen}
        onClose={() => setIsIssuesModalOpen(false)}
        project={project}
        results={results}
      />
    </div>
  );
};
