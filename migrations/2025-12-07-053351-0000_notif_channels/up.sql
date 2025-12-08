
CREATE TABLE channels (
                          id VARCHAR PRIMARY KEY DEFAULT gen_random_uuid(),
                          name VARCHAR NOT NULL,
                          provider_type VARCHAR NOT NULL,
                          configuration JSONB NOT NULL DEFAULT '[]'::jsonb,
                          created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                          updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

SELECT diesel_manage_updated_at('channels');
