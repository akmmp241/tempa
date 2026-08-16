CREATE TABLE runtime_resources
(
    id                 UUID PRIMARY KEY      DEFAULT gen_random_uuid(),
    project_id         UUID         NOT NULL REFERENCES projects (id),
    deployment_id      UUID REFERENCES deployments (id),
    resource_type      TEXT         NOT NULL CHECK (
        resource_type IN (
                          'container',
                          'image',
                          'volume',
                          'network'
            )
        ),
    docker_resource_id TEXT         NOT NULL,
    name               VARCHAR(255) NOT NULL,
    status             TEXT,
    metadata           JSONB        NOT NULL DEFAULT '{}',
    discovered_at      TIMESTAMPTZ  NOT NULL,
    last_seen_at       TIMESTAMPTZ  NOT NULL,

    UNIQUE (resource_type, docker_resource_id)
);