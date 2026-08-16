CREATE TABLE hosts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL UNIQUE,
    type VARCHAR(20) NOT NULL CHECK (type IN ('local', 'remote')),
    docker_endpoint VARCHAR(100) NOT NULL,
    status VARCHAR(20) NOT NULL CHECK (status IN ('online', 'offline', 'unknown')),
    last_seen_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL
);
