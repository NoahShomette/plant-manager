CREATE TABLE IF NOT EXISTS plants
(
    id          UUID PRIMARY KEY    NOT NULL,
    name        VARCHAR(250)        NOT NULL,
    state       JSON                NOT NULL
);
