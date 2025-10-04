CREATE TABLE IF NOT EXISTS plants
(
    id          UUID PRIMARY KEY    NOT NULL,
    date_created       timestamp    NOT NULL,
    event_modified       timestamp    NOT NULL
);
