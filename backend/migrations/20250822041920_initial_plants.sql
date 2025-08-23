CREATE TABLE IF NOT EXISTS plants
(
    id          UUID PRIMARY KEY    NOT NULL,
    name        JSON                NOT NULL,
    state       JSON                NOT NULL
);
