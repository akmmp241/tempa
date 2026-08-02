CREATE TABLE runtime_resources (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    environment_id UUID NOT NULL REFERENCES environments(id),
    deployment_id UUID REFERENCES deployments(id),
    resource_type TEXT NOT NULL,
    docker_resource_id TEXT NOT NULL,
    name TEXT NOT NULL,
    status TEXT,
    metadata JSONB NOT NULL DEFAULT '{}',
    discovered_at TIMESTAMPTZ NOT NULL,
    last_seen_at TIMESTAMPTZ NOT NULL,

    UNIQUE (resource_type, docker_resource_id)
);
