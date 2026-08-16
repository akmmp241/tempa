CREATE TABLE deployment_events (
    id BIGSERIAL PRIMARY KEY,
    deployment_id UUID NOT NULL REFERENCES deployments(id),
    sequence_number BIGINT NOT NULL,
    event_type TEXT NOT NULL,
    level TEXT NOT NULL CHECK (level IN ('debug', 'info', 'warn', 'error')),
    message TEXT NOT NULL,
    metadata JSONB,
    created_at TIMESTAMPTZ NOT NULL,

    UNIQUE (deployment_id, sequence_number)
);

CREATE INDEX deployment_events_deployment_sequence_idx
    ON deployment_events (deployment_id, sequence_number);