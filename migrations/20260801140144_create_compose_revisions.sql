CREATE TABLE compose_revisions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    environment_id UUID NOT NULL REFERENCES environments(id),
    revision_number BIGINT NOT NULL,
    raw_content TEXT NOT NULL,
    normalized_content TEXT,
    content_checksum TEXT NOT NULL,
    validation_status TEXT NOT NULL,
    validation_error TEXT,
    created_at TIMESTAMPTZ NOT NULL,

    UNIQUE (environment_id, revision_number),
    UNIQUE (environment_id, content_checksum)
);

ALTER TABLE environments
    ADD CONSTRAINT environments_active_revision_id_fkey
    FOREIGN KEY (active_revision_id) REFERENCES compose_revisions(id);
