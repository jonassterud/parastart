/* Takeoffs */

CREATE TABLE IF NOT EXISTS "takeoffs" (
    "id"            SERIAL PRIMARY KEY,
    "image"         BYTEA,
    "description"   VARCHAR(2048) NOT NULL,
    "latitude"      DOUBLE PRECISION NOT NULL,
    "longitude"     DOUBLE PRECISION NOT NULL,
    "creation"      BIGINT NOT NULL
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
