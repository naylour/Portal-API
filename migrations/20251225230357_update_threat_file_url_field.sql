-- Add migration script here
ALTER TABLE threat_source
DROP COLUMN file_url;

ALTER TABLE threat_source
ADD COLUMN files_urls TEXT[];
