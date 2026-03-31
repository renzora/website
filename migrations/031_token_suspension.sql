-- Add suspended flag to API tokens and app tokens so admins can disable without deleting
ALTER TABLE api_tokens ADD COLUMN IF NOT EXISTS suspended BOOLEAN NOT NULL DEFAULT false;
ALTER TABLE app_tokens ADD COLUMN IF NOT EXISTS suspended BOOLEAN NOT NULL DEFAULT false;

-- Add suspended flag to developer apps
ALTER TABLE developer_apps ADD COLUMN IF NOT EXISTS suspended BOOLEAN NOT NULL DEFAULT false;
