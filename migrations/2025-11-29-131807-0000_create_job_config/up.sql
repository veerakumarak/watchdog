CREATE TABLE job_configs (
    -- Primary Key components (Composite Key)
                             application VARCHAR(255) NOT NULL,
                             job_name VARCHAR(255) NOT NULL,

    -- Core configuration fields
                             schedule VARCHAR(50), -- e.g., "0 5 * * *"
                             zone_id VARCHAR(50),
                             enabled BOOLEAN NOT NULL DEFAULT TRUE,

    -- Nested structure: Stores the stages as a JSON document
--                              stages JSONB NOT NULL,

    -- Timestamps
                             created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
                             updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),

    -- Define the Composite Primary Key
                             PRIMARY KEY (application, job_name)
);

-- 3. Apply the Diesel Trigger

-- This calls the helper function to automatically manage the 'updated_at' column
SELECT diesel_manage_updated_at('job_configs');

-- 4. Indexing for Query Performance

-- Index for efficient lookups by job name across applications
CREATE INDEX idx_job_configs_job_name ON job_configs (job_name);

-- Index for querying enabled/disabled jobs
CREATE INDEX idx_job_configs_enabled ON job_configs (enabled);