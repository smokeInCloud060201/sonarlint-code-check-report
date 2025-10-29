export interface Project {
  id: number;
  project_key: string;
  project_name: string;
  project_path: string;
  sonar_token: string;
  sonar_host_url: string;
  language: string;
  sources_path: string;
  tests_path: string;
  coverage_report_path?: string;
  created_at: string;
  updated_at: string;
}

export interface CreateProjectRequest {
  project_key: string;
  project_name: string;
  project_path: string;
  language: string;
  sources_path: string;
  tests_path: string;
  coverage_report_path?: string;
}

export interface AdminToken {
  id: number;
  username: string;
  token_name: string;
  token_value: string;
  sonar_host_url: string;
  created_at: string;
  updated_at: string;
}

export interface CreateAdminTokenRequest {
  username: string;
  password: string;
  token_name: string;
  sonar_host_url: string;
}

export interface ScanCommandResponse {
  command: string;
  project_path: string;
}

export interface ProjectResults {
  project: Project;
  issues?: {
    issues: Array<{
      key: string;
      rule: string;
      severity: string;
      component: string;
      project: string;
      line?: number;
      message: string;
      status: string;
      type: string;
    }>;
    paging: {
      pageIndex: number;
      pageSize: number;
      total: number;
    };
  };
  coverage?: {
    component: {
      measures: Array<{
        metric: string;
        value: string;
      }>;
    };
  };
  quality_gate?: {
    projectStatus: {
      status: string; // "OK", "WARN", "ERROR"
      conditions: Array<{
        status: string; // "OK", "WARN", "ERROR"
        metricKey: string;
        comparator: string; // "GT", "LT", "EQ"
        errorThreshold?: string;
        actualValue?: string;
        periodIndex?: number;
      }>;
      ignoredConditions?: boolean;
      period?: {
        mode: string;
        date: string;
      };
      caycStatus?: string;
    };
  };
}
