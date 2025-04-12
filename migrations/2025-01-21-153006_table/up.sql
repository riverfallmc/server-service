CREATE TABLE server (
  id SERIAL PRIMARY KEY,
  name TEXT NOT NULL,
  enabled BOOLEAN NOT NULL,
  client TEXT NOT NULL,
  online JSONB NOT NULL,
  ip TEXT NOT NULL,
  icon TEXT NOT NULL,
  background TEXT NOT NULL,
  CONSTRAINT unique_server_name UNIQUE (name),
  CONSTRAINT unique_server_ip UNIQUE (ip)
);

CREATE TABLE client (
  id SERIAL PRIMARY KEY,
  name TEXT NOT NULL,
  description TEXT NOT NULL,
  modloader TEXT NOT NULL,
  version TEXT NOT NULL,
  mods TEXT[] NOT NULL,
  CONSTRAINT unique_client_name UNIQUE (name)
);
