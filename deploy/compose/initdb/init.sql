CREATE DATABASE sonarcute OWNER sonar;

\connect sonarcute;

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


-- Create admin_tokens table
CREATE TABLE IF NOT EXISTS admin_tokens (
                                            id SERIAL PRIMARY KEY,
                                            username VARCHAR(255) NOT NULL,
    token_name VARCHAR(255) NOT NULL,
    token_value TEXT NOT NULL,
    sonar_host_url VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
    );

-- Create indexes for better performance
CREATE INDEX IF NOT EXISTS idx_admin_tokens_sonar_host_url ON admin_tokens(sonar_host_url);
CREATE INDEX IF NOT EXISTS idx_admin_tokens_username ON admin_tokens(username);


-- Add token_type column to admin_tokens table
ALTER TABLE admin_tokens ADD COLUMN IF NOT EXISTS token_type VARCHAR(50) NOT NULL DEFAULT 'USER_TOKEN';

-- Create index for token_type
CREATE INDEX IF NOT EXISTS idx_admin_tokens_token_type ON admin_tokens(token_type);

-- Update existing tokens to be USER_TOKEN (if any exist)
UPDATE admin_tokens SET token_type = 'USER_TOKEN' WHERE token_type IS NULL OR token_type = '';


