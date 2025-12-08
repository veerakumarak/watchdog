-- 1. Drop the table (cascades to the trigger and indexes)
DROP TABLE IF EXISTS job_runs;

-- 2. Drop the custom enum type
DROP TYPE IF EXISTS job_run_status;
