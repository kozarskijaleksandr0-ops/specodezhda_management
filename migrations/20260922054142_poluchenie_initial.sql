-- Add migration script here
CREATE TABLE IF NOT EXISTS poluchenie (
    rabotnik_id         BLOB NOT NULL,
    specodezhda_id      BLOB NOT NULL,
    data_polucheniya    TEXT NOT NULL,
    podpis              TEXT NOT NULL,
    created_at          TEXT NOT NULL,

    PRIMARY KEY (rabotnik_id, specodezhda_id, data_polucheniya),

    CONSTRAINT rabotnik_fk
        FOREIGN KEY (rabotnik_id) REFERENCES rabotnik(id),

    CONSTRAINT specodezhda_fk
        FOREIGN KEY (specodezhda_id) REFERENCES specodezhda(id)
);