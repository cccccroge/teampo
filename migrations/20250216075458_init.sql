-- Add migration script here
CREATE TABLE entity_types (type_id TEXT PRIMARY KEY);

CREATE TABLE metric_definitions (
  metric_id TEXT PRIMARY KEY,
  name_key TEXT NOT NULL,
  description_key TEXT NOT NULL,
  category TEXT NOT NULL,
  FOREIGN KEY (category) REFERENCES entity_types (type_id)
);

CREATE TABLE metric_values (
  entity_id TEXT NOT NULL,
  entity_type TEXT NOT NULL,
  metric_id TEXT NOT NULL,
  value1 REAL NOT NULL,
  value2 REAL, -- nullable
  value3 REAL, -- nullable
  period_start TIMESTAMP NOT NULL,
  PRIMARY KEY (entity_id, entity_type, metric_id),
  FOREIGN KEY (metric_id) REFERENCES metric_definitions (metric_id),
  FOREIGN KEY (entity_type) REFERENCES entity_types (type_id)
);

-- Insert initial entity types
INSERT INTO
  entity_types (type_id)
VALUES
  ('user'),
  ('thread');
