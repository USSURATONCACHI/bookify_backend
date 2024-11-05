CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE sources (
    uuid UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR NOT NULL
);

