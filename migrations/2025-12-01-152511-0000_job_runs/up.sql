CREATE EXTENSION IF NOT EXISTS "pgcrypto";

CREATE TYPE job_run_status AS ENUM ('in_progress', 'complete', 'failed');

CREATE TABLE job_runs (
                          id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                          application VARCHAR NOT NULL,       -- Maps to String
                          job_name VARCHAR NOT NULL,          -- Maps to String
                          triggered_at TIMESTAMPTZ NOT NULL,         -- Maps to u64 (Unix timestamp)
                          status job_run_status NOT NULL DEFAULT 'in_progress',
                          stages JSONB NOT NULL DEFAULT '[]'::jsonb, -- Maps to Vec<JobRunStage>
                          created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                          updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

SELECT diesel_manage_updated_at('job_runs');

CREATE INDEX idx_job_runs_app_job ON job_runs(application, job_name);
