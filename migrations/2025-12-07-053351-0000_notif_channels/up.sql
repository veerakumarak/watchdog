
CREATE TYPE provider_type AS ENUM ('gchat_webhook', 'email_smtp', 'slack_webhook');

CREATE TABLE channels (
                          id VARCHAR PRIMARY KEY DEFAULT gen_random_uuid(),
                          name VARCHAR NOT NULL,
                          provider_type provider_type NOT NULL,
                          configuration JSONB NOT NULL,
                          created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                          updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

SELECT diesel_manage_updated_at('channels');
