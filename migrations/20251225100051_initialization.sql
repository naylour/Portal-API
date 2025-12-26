
CREATE TABLE IF NOT EXISTS country (
    id UUID PRIMARY KEY DEFAULT uuidv7(),

    code VARCHAR(2) NOT NULL UNIQUE,
    name TEXT NOT NULL,

    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS threat_type (
    id UUID PRIMARY KEY DEFAULT uuidv7(),

    code TEXT NOT NULL UNIQUE,
    description TEXT,

    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS threat_source (
    id UUID PRIMARY KEY DEFAULT uuidv7(),

    name TEXT NOT NULL UNIQUE,
    url TEXT,
    file_url TEXT NOT NULL,

    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS threat (
    id UUID PRIMARY KEY DEFAULT uuidv7(),

    title       TEXT NOT NULL,

    source_id   UUID NOT NULL REFERENCES threat_source(id) ON DELETE CASCADE,
    type_id     UUID NOT NULL REFERENCES threat_type(id)   ON DELETE RESTRICT,
    country_id  UUID NOT NULL REFERENCES country(id)      ON DELETE RESTRICT,

    description TEXT,

    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),

    CONSTRAINT threat_source_unique UNIQUE (source_id, type_id)
);

CREATE TABLE IF NOT EXISTS threat_knowledge (
    id UUID PRIMARY KEY DEFAULT uuidv7(),

    threat_id UUID NOT NULL REFERENCES threat(id) ON DELETE CASCADE,
    summary TEXT NOT NULL,

    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);


CREATE INDEX idx_threat_type       ON threat (type_id)   ;
CREATE INDEX idx_threat_country    ON threat (country_id);
CREATE INDEX idx_threat_source     ON threat (source_id) ;
CREATE INDEX idx_threat_created_at ON threat (created_at);



CREATE OR REPLACE FUNCTION set_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = now();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER threat_set_updated_at
BEFORE UPDATE ON threat
FOR EACH ROW
EXECUTE FUNCTION set_updated_at();

CREATE TRIGGER threat_source_set_updated_at
BEFORE UPDATE ON threat_source
FOR EACH ROW
EXECUTE FUNCTION set_updated_at();

CREATE TRIGGER threat_knowledge_set_updated_at
BEFORE UPDATE ON threat_knowledge
FOR EACH ROW
EXECUTE FUNCTION set_updated_at();
