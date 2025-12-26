-- Add migration script here
ALTER TABLE threat
ADD CONSTRAINT threat_source_id_id_from_source_uniq
UNIQUE (source_id, id_from_source);

ALTER TABLE threat
ADD COLUMN link TEXT;
