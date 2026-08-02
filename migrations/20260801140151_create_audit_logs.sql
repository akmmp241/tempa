CREATE TABLE audit_logs (
    id BIGSERIAL PRIMARY KEY,
    user_id UUID REFERENCES users(id),
    action TEXT NOT NULL,
    entity_type TEXT NOT NULL,
    entity_id UUID,
    metadata JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL
);

CREATE INDEX audit_logs_user_id_created_at_idx
    ON audit_logs(user_id, created_at DESC);

CREATE INDEX audit_logs_entity_idx
    ON audit_logs(entity_type, entity_id, created_at DESC);
