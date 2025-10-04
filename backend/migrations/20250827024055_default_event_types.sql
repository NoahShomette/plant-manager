-- Add migration script here
INSERT INTO event_types (id, name, event_type, deletable, modifiable, is_unique) VALUES
  ('9c8c6cfc-e111-44c2-9b5c-f5d84ae2da7a', 'Watered', '"DateTime"', false, false, false),
  ('1e7c1c14-dddd-4658-be0a-5c20726b4d16', 'Repotted', '"DateTime"', false, false, false),
    ('700866fd-a8b8-4cef-af5b-1752a1434129', 'Birthday', '"DateTime"', false, false, true),
    ('a501afa2-1959-4f1e-9706-abe97eb85263', 'Name', '"String"', false, false, false),  
    ('1a5c53bb-18c2-4789-8ba4-9bbfc4bc2371', 'State', '{"CustomEnum":{"options":["Alive","Retired","Gifted"],"selected":0}}', false, true, false)

ON CONFLICT (id) DO NOTHING;
