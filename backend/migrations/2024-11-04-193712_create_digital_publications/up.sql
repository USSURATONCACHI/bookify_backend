CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE digital_publications (
    uuid UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    source UUID NULL REFERENCES sources(uuid) ON DELETE SET NULL,
    
    name        VARCHAR NOT NULL,
    description VARCHAR NOT NULL,
    cover_url   VARCHAR NOT NULL,
    links       TEXT[] NOT NULL
);