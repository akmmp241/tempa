CREATE TABLE docker_networks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    docker_network_id TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL UNIQUE,
    driver TEXT NOT NULL,
    scope TEXT,
    is_external BOOLEAN NOT NULL DEFAULT TRUE,
    labels JSONB NOT NULL DEFAULT '{}',
    last_seen_at TIMESTAMPTZ NOT NULL
);
