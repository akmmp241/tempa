CREATE TABLE deployment_jobs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    deployment_id UUID NOT NULL UNIQUE REFERENCES deployments(id),
    status TEXT NOT NULL CHECK (
        status IN (
                   'pending',
                   'running',
                   'retry_wait',
                   'completed',
                   'failed',
                   'cancelled'
            )
        ),
    attempts INTEGER NOT NULL DEFAULT 0,
    available_at TIMESTAMPTZ NOT NULL,
    locked_at TIMESTAMPTZ,
    locked_by TEXT,
    created_at TIMESTAMPTZ NOT NULL
);

CREATE INDEX deployment_jobs_available_idx
    ON deployment_jobs (status, available_at)
    WHERE status IN ('pending', 'retry_wait');