CREATE TABLE compose_revisions
(
    id                 UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    project_id         UUID        NOT NULL REFERENCES projects (id),
    revision_number    BIGINT      NOT NULL,
    raw_content        TEXT        NOT NULL,
    normalized_content TEXT,
    content_checksum   TEXT        NOT NULL,
    validation_status  TEXT        NOT NULL CHECK (
        validation_status IN ('pending', 'valid', 'invalid')
        ),
    validation_error   TEXT,
    created_at         TIMESTAMPTZ NOT NULL,

    UNIQUE (project_id, revision_number),
    UNIQUE (project_id, content_checksum)
);