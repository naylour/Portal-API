-- Add migration script here
CREATE TYPE threat_status AS ENUM (
  'NEW',
  'IN_PROGRESS',
  'DONE',
  'ARCHIVED',
  'UNKNOWN'
);

ALTER TABLE threat
ADD COLUMN adding_in_source TIMESTAMPTZ;

ALTER TABLE threat
ADD COLUMN last_update_in_source TIMESTAMPTZ;

ALTER TABLE threat
ADD COLUMN comment TEXT;

ALTER TABLE threat
ADD COLUMN status threat_status NOT NULL DEFAULT 'UNKNOWN';


CREATE TABLE IF NOT EXISTS object_interaction (
    id UUID PRIMARY KEY DEFAULT uuidv7(),

    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

ALTER TABLE threat
ADD COLUMN object_interaction_id UUID REFERENCES object_interaction(id) ON DELETE SET NULL;
