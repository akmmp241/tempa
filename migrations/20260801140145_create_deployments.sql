CREATE TABLE deployments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    environment_id UUID NOT NULL REFERENCES environments(id),
    revision_id UUID NOT NULL REFERENCES compose_revisions(id),
    status TEXT NOT NULL,
    trigger_type TEXT NOT NULL CHECK (trigger_type IN ('manual', 'rollback', 'redeploy')),
    requested_at TIMESTAMPTZ NOT NULL,
    started_at TIMESTAMPTZ,
    finished_at TIMESTAMPTZ,
    failure_code TEXT,
    failure_message TEXT,
    previous_deployment_id UUID REFERENCES deployments(id)
);
