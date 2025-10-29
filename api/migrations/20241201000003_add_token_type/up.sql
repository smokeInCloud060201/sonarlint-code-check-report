-- Add token_type column to admin_tokens table
ALTER TABLE admin_tokens ADD COLUMN IF NOT EXISTS token_type VARCHAR(50) NOT NULL DEFAULT 'USER_TOKEN';

-- Create index for token_type
CREATE INDEX IF NOT EXISTS idx_admin_tokens_token_type ON admin_tokens(token_type);

-- Update existing tokens to be USER_TOKEN (if any exist)
UPDATE admin_tokens SET token_type = 'USER_TOKEN' WHERE token_type IS NULL OR token_type = '';

