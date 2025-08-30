CREATE TABLE IF NOT EXISTS plant_event_types
(
    id          UUID PRIMARY KEY    NOT NULL,
    name        VARCHAR(250)        NOT NULL,
    event_type        JSON             NOT NULL
);
