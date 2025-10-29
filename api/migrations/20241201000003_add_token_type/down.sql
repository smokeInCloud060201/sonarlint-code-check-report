-- Remove token_type column
ALTER TABLE admin_tokens DROP COLUMN IF EXISTS token_type;

