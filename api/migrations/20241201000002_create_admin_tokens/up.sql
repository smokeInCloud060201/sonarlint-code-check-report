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
