-- Create projects table
CREATE TABLE IF NOT EXISTS projects (
    id SERIAL PRIMARY KEY,
    project_key VARCHAR(255) NOT NULL UNIQUE,
    project_name VARCHAR(255) NOT NULL,
    project_path VARCHAR(500) NOT NULL UNIQUE,
    sonar_token TEXT NOT NULL,
    sonar_host_url VARCHAR(255) NOT NULL DEFAULT 'http://localhost:9000',
    language VARCHAR(50) NOT NULL,
    sources_path VARCHAR(500) NOT NULL,
    tests_path VARCHAR(500) NOT NULL,
    coverage_report_path VARCHAR(500),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create indexes for better performance
CREATE INDEX IF NOT EXISTS idx_projects_project_key ON projects(project_key);
CREATE INDEX IF NOT EXISTS idx_projects_project_path ON projects(project_path);
