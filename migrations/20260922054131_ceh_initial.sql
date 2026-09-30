-- Add migration script here
CREATE TABLE IF NOT EXISTS ceh (
    id              BLOB PRIMARY KEY NOT NULL,
    name            TEXT NOT NULL,
    nachalnik       TEXT NOT NULL,
    created_at      TEXT NOT NULL,
    updated_at      TEXT,
    deleted_at      TEXT
);