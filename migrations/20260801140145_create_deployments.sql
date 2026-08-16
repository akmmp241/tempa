CREATE TABLE deployments
(
    id                     UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    project_id             UUID        NOT NULL REFERENCES projects (id),
    revision_id            UUID        NOT NULL REFERENCES compose_revisions (id),
    status                 TEXT        NOT NULL CHECK (
        status IN (
                   'queued',
                   'preparing',
                   'building',
                   'deploying',
                   'verifying',
                   'succeeded',
                   'failed',
                   'cancelled'
            )
        ),
    trigger_type           TEXT        NOT NULL CHECK (
        trigger_type IN ('manual', 'rollback', 'redeploy')
        ),
    requested_at           TIMESTAMPTZ NOT NULL,
    started_at             TIMESTAMPTZ,
    finished_at            TIMESTAMPTZ,
    failure_code           TEXT,
    failure_message        TEXT,
    previous_deployment_id UUID REFERENCES deployments (id)
);