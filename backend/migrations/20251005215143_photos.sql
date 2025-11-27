-- Add migration script here
CREATE TABLE IF NOT EXISTS photos
(
    id          UUID UNIQUE PRIMARY KEY NOT NULL,
    file_location        VARCHAR        NOT NULL,
    photo_date        timestamp        NOT NULL
);
