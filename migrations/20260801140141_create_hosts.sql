CREATE TABLE hosts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL UNIQUE,
    type TEXT NOT NULL,
    docker_endpoint TEXT NOT NULL,
    status TEXT NOT NULL CHECK (status IN ('online', 'offline', 'unknown')),
    last_seen_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL
);
