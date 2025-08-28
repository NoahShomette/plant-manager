-- Add migration script here
CREATE TABLE IF NOT EXISTS deleted_plants
(
    id          UUID PRIMARY KEY    NOT NULL,
    date_deleted       timestamp    NOT NULL
);
