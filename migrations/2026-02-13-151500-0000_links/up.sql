-- Table for the nodes
CREATE TABLE node (
  id SERIAL PRIMARY KEY,
  name VARCHAR(255) NOT NULL DEFAULT 'Unnamed Node',
  in_use BOOLEAN NOT NULL DEFAULT FALSE,
  measurements BIGINT NOT NULL DEFAULT 0, -- start with 0 measurements
  node_type VARCHAR(255) NOT NULL DEFAULT 'client'
);

-- Table for the measurements
CREATE TABLE measurements (
  id SERIAL PRIMARY KEY,
  node_id INT NOT NULL REFERENCES node(id),
  measurement_id BIGINT,
  value SMALLINT,
  consumed BOOLEAN
);

-- Table for the pending measurements
CREATE TABLE pending_measurements (
  id SERIAL PRIMARY KEY,
  node_id INT NOT NULL REFERENCES node(id),
  measurement_id BIGINT,
  value SMALLINT,
  consumed BOOLEAN
);

-- Table for the links
CREATE TABLE links (
  id SERIAL PRIMARY KEY,
  length BIGINT,
  attenuation REAL,
  error REAL,
  nodea BIGINT REFERENCES node(id),
  nodeb BIGINT REFERENCES node(id),
  next_available_time BIGINT
);
