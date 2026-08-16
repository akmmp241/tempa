CREATE TABLE projects
(
    id                   UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    host_id              UUID         NOT NULL REFERENCES hosts (id),
    name                 VARCHAR(255) NOT NULL,
    slug                 VARCHAR(255) NOT NULL,
    description          TEXT,
    compose_project_name VARCHAR(255) NOT NULL,
    active_deployment_id UUID,
    created_at           TIMESTAMPTZ  NOT NULL,
    updated_at           TIMESTAMPTZ  NOT NULL,

    UNIQUE (host_id, slug),
    UNIQUE (host_id, compose_project_name)
);
CREATE INDEX projects_host_id_idx ON projects (host_id);
