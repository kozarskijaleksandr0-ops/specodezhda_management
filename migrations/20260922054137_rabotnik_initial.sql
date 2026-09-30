-- Add migration script here
CREATE TABLE IF NOT EXISTS rabotnik (
    id              BLOB PRIMARY KEY NOT NULL,
    fio             TEXT NOT NULL,
    dolzhnost       TEXT NOT NULL,
    skidka          REAL NOT NULL,
    ceh_id          BLOB NOT NULL,
    created_at      TEXT NOT NULL,
    updated_at      TEXT,
    deleted_at      TEXT,

    CONSTRAINT ceh_fk
        FOREIGN KEY (ceh_id) REFERENCES ceh(id)
);