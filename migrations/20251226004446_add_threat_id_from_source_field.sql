-- Add migration script here
ALTER TABLE threat
ADD COLUMN id_from_source TEXT NOT NULL DEFAULT '';
