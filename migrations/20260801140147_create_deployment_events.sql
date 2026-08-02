CREATE TABLE deployment_events (
    id BIGSERIAL PRIMARY KEY,
    deployment_id UUID NOT NULL REFERENCES deployments(id),
    sequence_number BIGINT NOT NULL,
    event_type TEXT NOT NULL,
    level TEXT NOT NULL,
    message TEXT NOT NULL,
    metadata JSONB,
    created_at TIMESTAMPTZ NOT NULL,

    UNIQUE (deployment_id, sequence_number)
);
