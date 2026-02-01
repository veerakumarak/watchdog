CREATE TABLE global_settings (
                                 id INTEGER PRIMARY KEY DEFAULT 1,
                                 success_retention_days INT NOT NULL,
                                 failure_retention_days INT NOT NULL,
                                 maintenance_mode BOOLEAN NOT NULL,
                                 default_channels TEXT NOT NULL,
                                 max_stage_duration_hours INT NOT NULL
);

-- Initialize with default values
INSERT INTO global_settings (id, success_retention_days, failure_retention_days, maintenance_mode, default_channels, max_stage_duration_hours)
VALUES (1, 30, 90, false, 'eng_slack', 12)
    ON CONFLICT (id) DO NOTHING;-- Your SQL goes here
