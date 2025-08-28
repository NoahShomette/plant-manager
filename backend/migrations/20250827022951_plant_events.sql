CREATE TABLE IF NOT EXISTS plant_events
(
    id          UUID PRIMARY KEY NOT NULL,
    plant_type_id        UUID        NOT NULL,
    data      JSON             NOT NULL,
    date_created timestamp NOT NULL
);
