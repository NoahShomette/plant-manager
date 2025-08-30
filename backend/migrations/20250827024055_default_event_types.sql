-- Add migration script here
INSERT INTO plant_event_types (id, name, event_type) VALUES
  ('9c8c6cfc-e111-44c2-9b5c-f5d84ae2da7a', 'Watered', '"Day"'),
  ('1e7c1c14-dddd-4658-be0a-5c20726b4d16', 'Repotted', '"Day"')
ON CONFLICT (id) DO NOTHING;
