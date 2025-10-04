CREATE TABLE IF NOT EXISTS event_types
(
    id          UUID PRIMARY KEY    NOT NULL,
    name        VARCHAR(250)        NOT NULL,
    event_type        JSON             NOT NULL,
    deletable           BOOL            NOT NULL,
    modifiable           BOOL            NOT NULL,
    is_unique           BOOL            NOT NULL
);
