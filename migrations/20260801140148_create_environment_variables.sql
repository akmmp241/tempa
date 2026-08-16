CREATE TABLE environment_variables
(
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    project_id      UUID        NOT NULL REFERENCES projects (id),
    key             TEXT        NOT NULL,
    encrypted_value BYTEA       NOT NULL,
    value_nonce     BYTEA       NOT NULL,
    is_secret       BOOLEAN     NOT NULL DEFAULT TRUE,
    created_at      TIMESTAMPTZ NOT NULL,
    updated_at      TIMESTAMPTZ NOT NULL,

    UNIQUE (project_id, key)
);