-- Add migration script here
CREATE TABLE IF NOT EXISTS events_unique
(
    id          UUID UNIQUE PRIMARY KEY NOT NULL,
    event_type_id        UUID        NOT NULL,
    plant_id        UUID        NOT NULL,
    data      JSON             NOT NULL,
    event_date timestamp NOT NULL,
    UNIQUE (event_type_id, plant_id)
);
