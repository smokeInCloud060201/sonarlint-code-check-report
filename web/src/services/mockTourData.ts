import type { Project, ProjectResults } from '../types/api';

export const mockProject: Project = {
  id: 999999,
  project_name: 'Sample Java Project',
  project_path: '/path/to/sample-project',
  project_key: 'sample-project-key',
  language: 'java',
  sonar_host_url: 'http://localhost:9000',
  sonar_token: 'mock-token-for-tour',
  sources_path: 'src/main/java',
  tests_path: 'src/test/java',
  coverage_report_path: 'target/site/jacoco/jacoco.xml',
  created_at: new Date().toISOString(),
  updated_at: new Date().toISOString()
};

export const mockProjectResults: ProjectResults = {
  project: mockProject,
  issues: {
    issues: [
      {
        key: 'mock-issue-1',
        rule: 'java:S1068',
        severity: 'MAJOR',
        component: 'sample-project-key:src/main/java/com/example/Service.java',
        project: 'sample-project-key',
        line: 15,
        message: 'Unused private field should be removed',
        status: 'OPEN',
        type: 'CODE_SMELL'
      },
      {
        key: 'mock-issue-2',
        rule: 'java:S1186',
        severity: 'MINOR',
        component: 'sample-project-key:src/main/java/com/example/Controller.java',
        project: 'sample-project-key',
        line: 23,
        message: 'Empty methods should be removed',
        status: 'OPEN',
        type: 'CODE_SMELL'
      },
      {
        key: 'mock-issue-3',
        rule: 'java:S1144',
        severity: 'CRITICAL',
        component: 'sample-project-key:src/main/java/com/example/Utils.java',
        project: 'sample-project-key',
        line: 8,
        message: 'Unused private methods should be removed',
        status: 'OPEN',
        type: 'CODE_SMELL'
      },
      {
        key: 'mock-issue-4',
        rule: 'java:S1481',
        severity: 'INFO',
        component: 'sample-project-key:src/main/java/com/example/Helper.java',
        project: 'sample-project-key',
        line: 12,
        message: 'Unused local variables should be removed',
        status: 'OPEN',
        type: 'CODE_SMELL'
      }
    ],
    paging: {
      pageIndex: 1,
      pageSize: 500,
      total: 4
    }
  },
  coverage: {
    component: {
      measures: [
        {
          metric: 'coverage',
          value: '85.5'
        },
        {
          metric: 'line_coverage',
          value: '82.3'
        },
        {
          metric: 'branch_coverage',
          value: '78.9'
        },
        {
          metric: 'lines_to_cover',
          value: '1200'
        },
        {
          metric: 'uncovered_lines',
          value: '177'
        }
      ]
    }
  },
  quality_gate: {
    projectStatus: {
      status: 'WARN',
      conditions: [
        {
          status: 'OK',
          metricKey: 'new_coverage',
          comparator: 'LT',
          errorThreshold: '80',
          actualValue: '75'
        },
        {
          status: 'WARN',
          metricKey: 'new_duplicated_lines_density',
          comparator: 'GT',
          errorThreshold: '3',
          actualValue: '3.2'
        },
        {
          status: 'ERROR',
          metricKey: 'new_maintainability_rating',
          comparator: 'GT',
          errorThreshold: '1',
          actualValue: '2'
        },
        {
          status: 'OK',
          metricKey: 'new_reliability_rating',
          comparator: 'GT',
          errorThreshold: '1',
          actualValue: '1'
        },
        {
          status: 'OK',
          metricKey: 'new_security_rating',
          comparator: 'GT',
          errorThreshold: '1',
          actualValue: '1'
        }
      ],
      ignoredConditions: false,
      period: {
        mode: 'previous_version',
        date: '2024-01-15T10:30:00+0000'
      },
      caycStatus: 'compliant'
    }
  }
};

export const mockSonarCommand = `./gradlew sonar -Dsonar.token=mock-token-for-tour -Dsonar.host.url=http://localhost:9000 -Dsonar.projectKey=sample-project-key -Dsonar.projectName=Sample Java Project -Dsonar.coverage.jacoco.xmlReportPaths=target/site/jacoco/jacoco.xml -Dsonar.language=java -Dsonar.sources=src/main/java -Dsonar.tests=src/test/java`;

let isTourActive = false;

export const setTourActive = (active: boolean) => {
  isTourActive = active;
};

export const isTourCurrentlyActive = (): boolean => {
  return isTourActive;
};

export const getMockProjects = (): Project[] => {
  return isTourActive ? [mockProject] : [];
};

export const getMockProjectResults = (): ProjectResults => {
  return mockProjectResults;
};

export const getMockSonarCommand = (): string => {
  return mockSonarCommand;
};

export const injectMockDataForTour = () => {
  isTourActive = true;
};

export const removeMockDataAfterTour = () => {
  isTourActive = false;
};
