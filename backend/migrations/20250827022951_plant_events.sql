CREATE TABLE IF NOT EXISTS plant_events
(
    id          UUID PRIMARY KEY NOT NULL,
    event_type_id        UUID        NOT NULL,
    plant_id        UUID        NOT NULL,
    data      JSON             NOT NULL,
    event_date timestamp NOT NULL
);
