CREATE TABLE IF NOT EXISTS takeoffs
(
    id          SERIAL PRIMARY KEY,
    body        VARCHAR(2048) NOT NULL,
    picture     BYTEA,
    latitude    DOUBLE PRECISION NOT NULL,
    longitude   DOUBLE PRECISION NOT NULL,
    creation    BIGINT NOT NULL
);
