import { useState } from 'react';
import { X, AlertCircle, FileText, MapPin } from 'lucide-react';
import type { ProjectResults } from '../types/api';

interface IssuesViewModalProps {
  isOpen: boolean;
  onClose: () => void;
  project: any;
  results: ProjectResults;
}

export const IssuesViewModal = ({ isOpen, onClose, project, results }: IssuesViewModalProps) => {
  const [selectedSeverity, setSelectedSeverity] = useState<string>('all');

  if (!isOpen || !results.issues) return null;

  const getSeverityColor = (severity: string) => {
    switch (severity.toLowerCase()) {
      case 'blocker':
      case 'critical':
        return 'text-red-600 bg-red-100';
      case 'major':
        return 'text-orange-600 bg-orange-100';
      case 'minor':
        return 'text-yellow-600 bg-yellow-100';
      case 'info':
        return 'text-blue-600 bg-blue-100';
      default:
        return 'text-gray-600 bg-gray-100';
    }
  };

  const getIssueTypeColor = (type: string) => {
    switch (type.toLowerCase()) {
      case 'bug':
        return 'text-red-600 bg-red-100';
      case 'vulnerability':
        return 'text-purple-600 bg-purple-100';
      case 'code_smell':
        return 'text-yellow-600 bg-yellow-100';
      default:
        return 'text-gray-600 bg-gray-100';
    }
  };

  const filteredIssues = selectedSeverity === 'all' 
    ? results.issues.issues 
    : results.issues.issues.filter(issue => issue.severity.toLowerCase() === selectedSeverity);

  const severityCounts = results.issues.issues.reduce((acc, issue) => {
    const severity = issue.severity.toLowerCase();
    acc[severity] = (acc[severity] || 0) + 1;
    return acc;
  }, {} as Record<string, number>);

  const formatDate = (dateString: string) => {
    return new Date(dateString).toLocaleString('en-US', {
      year: 'numeric',
      month: 'long',
      day: 'numeric',
      hour: '2-digit',
      minute: '2-digit'
    });
  };

  return (
    <div className="fixed inset-0 bg-gray-600 bg-opacity-50 h-full w-full z-50">
      <div className="flex items-center justify-center min-h-screen p-4">
        <div className="relative w-11/12 max-w-10xl max-h-[90vh] shadow-lg rounded-md bg-white flex flex-col">
          <div className="flex-shrink-0 p-6 border-b border-gray-200">
            {/* Header */}
            <div className="flex items-center justify-between">
              <div>
                <h3 className="text-xl font-semibold text-gray-900">
                  Issues Report - {project.project_name}
                </h3>
                <p className="text-sm text-gray-600 mt-1">
                  Scan completed on {formatDate(new Date().toISOString())}
                </p>
                {/* Quality Gate Status */}
                {results.quality_gate && (
                  <div className="mt-2 flex items-center">
                    <span className="text-sm text-gray-600 mr-2">Quality Gate:</span>
                    <span className={`px-2 py-1 text-xs font-medium rounded-full ${
                      results.quality_gate.projectStatus.status === 'OK' ? 'bg-green-100 text-green-800' :
                      results.quality_gate.projectStatus.status === 'WARN' ? 'bg-yellow-100 text-yellow-800' :
                      results.quality_gate.projectStatus.status === 'ERROR' ? 'bg-red-100 text-red-800' :
                      'bg-gray-100 text-gray-800'
                    }`}>
                      {results.quality_gate.projectStatus.status}
                    </span>
                  </div>
                )}
              </div>
              <button
                onClick={onClose}
                className="text-gray-400 hover:text-gray-600"
              >
                <X className="h-6 w-6" />
              </button>
            </div>
          </div>

          {/* Scrollable Content */}
          <div className="flex-1 overflow-y-auto p-6 [scrollbar-color:white_gray]">
            {/* Project Overview Section */}
            <div className="mb-6">
              <h4 className="text-lg font-semibold text-gray-900 mb-4">Project Overview</h4>
              
              {/* Coverage and Quality Gate Status */}
              <div className="grid grid-cols-1 md:grid-cols-2 gap-4 mb-4">
                {/* Coverage Information */}
                {results.coverage && results.coverage.component && (
                  <div className="bg-blue-50 border border-blue-200 rounded-lg p-4">
                    <h5 className="text-sm font-medium text-blue-900 mb-2">Code Coverage</h5>
                    <div className="space-y-1">
                      {results.coverage.component.measures.map((measure, index) => (
                        <div key={index} className="flex justify-between text-sm">
                          <span className="text-blue-700 capitalize">
                            {measure.metric.replace(/_/g, ' ')}:
                          </span>
                          <span className="font-medium text-blue-900">
                            {measure.value}%
                          </span>
                        </div>
                      ))}
                    </div>
                  </div>
                )}

                {/* Quality Gate Conditions */}
                {results.quality_gate && results.quality_gate.projectStatus && (
                  <div className="bg-gray-50 border border-gray-200 rounded-lg p-4">
                    <h5 className="text-sm font-medium text-gray-900 mb-2">Quality Gate Conditions</h5>
                    <div className="space-y-2">
                      {results.quality_gate.projectStatus.conditions.map((condition, index) => (
                        <div key={index} className="text-sm">
                          <div className="flex items-center justify-between mb-1">
                            <span className="text-gray-700 font-medium">
                              {condition.metricKey.replace(/_/g, ' ').replace(/\b\w/g, l => l.toUpperCase())}
                            </span>
                            <span className={`px-2 py-1 text-xs font-medium rounded-full ${
                              condition.status === 'OK' ? 'bg-green-100 text-green-800' :
                              condition.status === 'WARN' ? 'bg-yellow-100 text-yellow-800' :
                              condition.status === 'ERROR' ? 'bg-red-100 text-red-800' :
                              'bg-gray-100 text-gray-800'
                            }`}>
                              {condition.status}
                            </span>
                          </div>
                          <div className="text-xs text-gray-500">
                            {condition.actualValue && condition.errorThreshold && (
                              <span>
                                Actual: {condition.actualValue} | Threshold: {condition.errorThreshold} | 
                                Operator: {condition.comparator}
                              </span>
                            )}
                          </div>
                        </div>
                      ))}
                    </div>
                  </div>
                )}
              </div>

              {/* Quality Gate Period Information */}
              {results.quality_gate && results.quality_gate.projectStatus.period && (
                <div className="bg-gray-50 border border-gray-200 rounded-lg p-3 mb-4">
                  <div className="flex items-center justify-between text-sm">
                    <span className="text-gray-600">Analysis Period:</span>
                    <span className="font-medium text-gray-900">
                      {results.quality_gate.projectStatus.period.mode.replace(/_/g, ' ')} - 
                      {new Date(results.quality_gate.projectStatus.period.date).toLocaleDateString()}
                    </span>
                  </div>
                  {results.quality_gate.projectStatus.caycStatus && (
                    <div className="flex items-center justify-between text-sm mt-1">
                      <span className="text-gray-600">CAYC Status:</span>
                      <span className={`px-2 py-1 text-xs font-medium rounded-full ${
                        results.quality_gate.projectStatus.caycStatus === 'compliant' 
                          ? 'bg-green-100 text-green-800' 
                          : 'bg-yellow-100 text-yellow-800'
                      }`}>
                        {results.quality_gate.projectStatus.caycStatus}
                      </span>
                    </div>
                  )}
                </div>
              )}
            </div>

            {/* Issues Summary Stats */}
            <div className="mb-6">
              <h4 className="text-lg font-semibold text-gray-900 mb-4">Issues Summary</h4>
              <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
                <div className="bg-gray-50 rounded-lg p-4 text-center">
                  <div className="text-2xl font-bold text-gray-900">{results.issues.paging.total}</div>
                  <div className="text-sm text-gray-600">Total Issues</div>
                </div>
                {Object.entries(severityCounts).map(([severity, count]) => (
                  <div key={severity} className="bg-gray-50 rounded-lg p-4 text-center">
                    <div className={`text-2xl font-bold ${getSeverityColor(severity).split(' ')[0]}`}>
                      {count}
                    </div>
                    <div className="text-sm text-gray-600 capitalize">{severity}</div>
                  </div>
                ))}
              </div>
            </div>

          {/* Severity Filter */}
          <div className="mb-4">
            <label className="block text-sm font-medium text-gray-700 mb-2">
              Filter by Severity
            </label>
            <div className="flex flex-wrap gap-2">
              <button
                onClick={() => setSelectedSeverity('all')}
                className={`px-3 py-1 rounded-full text-sm font-medium ${
                  selectedSeverity === 'all'
                    ? 'bg-blue-100 text-blue-800'
                    : 'bg-gray-100 text-gray-700 hover:bg-gray-200'
                }`}
              >
                All ({results.issues.paging.total})
              </button>
              {Object.entries(severityCounts).map(([severity, count]) => (
                <button
                  key={severity}
                  onClick={() => setSelectedSeverity(severity)}
                  className={`px-3 py-1 rounded-full text-sm font-medium ${
                    selectedSeverity === severity
                      ? `${getSeverityColor(severity)}`
                      : 'bg-gray-100 text-gray-700 hover:bg-gray-200'
                  }`}
                >
                  {severity.charAt(0).toUpperCase() + severity.slice(1)} ({count})
                </button>
              ))}
            </div>
          </div>

          {/* Issues Table */}
          <div className="border border-gray-200 rounded-lg overflow-hidden">
            <div className="overflow-x-auto">
              <table className="min-w-full divide-y divide-gray-200">
                <thead className="bg-gray-50">
                  <tr>
                    <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                      Type
                    </th>
                    <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                      Severity
                    </th>
                    <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                      Issue Details
                    </th>
                    <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                      Location
                    </th>
                    <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                      Line
                    </th>
                  </tr>
                </thead>
                <tbody className="bg-white divide-y divide-gray-200">
                  {filteredIssues.map((issue) => (
                    <tr key={issue.key} className="hover:bg-gray-50">
                      <td className="px-6 py-4 whitespace-nowrap">
                        <span className={`inline-flex px-2 py-1 text-xs font-semibold rounded-full ${getIssueTypeColor(issue.type)}`}>
                          {issue.type.replace('_', ' ').toUpperCase()}
                        </span>
                      </td>
                      <td className="px-6 py-4 whitespace-nowrap">
                        <span className={`inline-flex px-2 py-1 text-xs font-semibold rounded-full ${getSeverityColor(issue.severity)}`}>
                          {issue.severity.toUpperCase()}
                        </span>
                      </td>
                      <td className="px-6 py-4">
                        <div className="text-sm text-gray-900 max-w-md">
                          {issue.message}
                        </div>
                        <div className="text-xs text-gray-500 mt-1">
                          Rule: {issue.rule}
                        </div>
                      </td>
                      <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                        <div className="flex items-center">
                          <FileText className="h-4 w-4 mr-1 text-gray-400" />
                          <span className="truncate max-w-xs">
                            {issue.component.replace(project.project_key + ':', '')}
                          </span>
                        </div>
                      </td>
                      <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                        {issue.line ? (
                          <div className="flex items-center">
                            <MapPin className="h-4 w-4 mr-1 text-gray-400" />
                            {issue.line}
                          </div>
                        ) : (
                          <span className="text-gray-400">-</span>
                        )}
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          </div>

            {filteredIssues.length === 0 && (
              <div className="text-center py-8">
                <AlertCircle className="mx-auto h-12 w-12 text-gray-400" />
                <h3 className="mt-2 text-sm font-medium text-gray-900">No issues found</h3>
                <p className="mt-1 text-sm text-gray-500">
                  No issues match the selected severity filter.
                </p>
              </div>
            )}
          </div>

          {/* Footer */}
          <div className="flex-shrink-0 flex justify-end p-6 border-t border-gray-200">
            <button
              onClick={onClose}
              className="px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50"
            >
              Close
            </button>
          </div>
        </div>
      </div>
    </div>
  );
};
