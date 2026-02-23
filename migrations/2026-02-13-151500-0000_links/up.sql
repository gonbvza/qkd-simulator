-- Table for the nodes
CREATE TABLE nodes (
  id SERIAL PRIMARY KEY,
  name VARCHAR(255) UNIQUE NOT NULL DEFAULT 'Unnamed Node',
  in_use BOOLEAN NOT NULL DEFAULT FALSE,
  measurements BIGINT NOT NULL DEFAULT 0,
  node_type VARCHAR(255) NOT NULL DEFAULT 'client'
);

-- Table for the measurements
CREATE TABLE measurements (
  id SERIAL PRIMARY KEY,
  node_id INT NOT NULL REFERENCES nodes(id),
  measurement_id BIGINT NOT NULL,
  value SMALLINT NOT NULL,
  consumed BOOLEAN NOT NULL
);

-- Table for the pending measurements
CREATE TABLE pending_measurements (
  id SERIAL PRIMARY KEY,
  node_id INT NOT NULL REFERENCES nodes(id),
  measurement_id BIGINT NOT NULL,
  value SMALLINT NOT NULL,
  consumed BOOLEAN NOT NULL
);

-- Table for the links
CREATE TABLE links (
  id SERIAL PRIMARY KEY,
  length BIGINT NOT NULL,
  attenuation REAL NOT NULL,
  error_rate REAL NOT NULL,
  node_a INT NOT NULL REFERENCES nodes(id),
  node_b INT NOT NULL REFERENCES nodes(id),
  next_available_time BIGINT NOT NULL
);
