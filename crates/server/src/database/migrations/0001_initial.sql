/* Takeoffs */

CREATE TABLE IF NOT EXISTS "takeoffs" (
    "id"                SERIAL PRIMARY KEY,
    "name"              TEXT NOT NULL,
    "description"       TEXT NOT NULL,
    "image"             BYTEA,
    "region"            TEXT NOT NULL,
    "altitude"          INTEGER,
    "altitude_diff"     INTEGER,
    "latitude"          DOUBLE PRECISION NOT NULL,
    "longitude"         DOUBLE PRECISION NOT NULL,
    "wind_dirs"         TEXT[] NOT NULL,
    "info_url"          TEXT,
    "source_url"        TEXT UNIQUE,
    "created"           TEXT NOT NULL,
    "updated"           TEXT NOT NULL
);

/* Users, roles and sessions */

CREATE TABLE IF NOT EXISTS "users" (
    "id"        SERIAL PRIMARY KEY,
    "username"  VARCHAR(30) NOT NULL UNIQUE,
    "password"  TEXT NOT NULL   
);


CREATE TABLE IF NOT EXISTS "sessions" (
    "id"        SERIAL PRIMARY KEY,
    "user_id"   INTEGER REFERENCES "users" ON DELETE CASCADE NOT NULL,
    "token"     TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS "roles" (
    "id"    SERIAL PRIMARY KEY,
    "name"  TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS "users_roles" (
    "id"        SERIAL PRIMARY KEY,
    "user_id"   INTEGER REFERENCES "users" ON DELETE CASCADE NOT NULL,
    "role_id"   INTEGER REFERENCES "roles" ON DELETE CASCADE NOT NULL
);
