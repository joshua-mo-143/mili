CREATE type source as enum ('qrcode', 'link');

-- Add migration script here
CREATE TABLE IF NOT EXISTS links (
	id SERIAL PRIMARY KEY,
	uri VARCHAR NOT NULL,
	shortlink_id VARCHAR UNIQUE NOT NULL,
	created_at TIMESTAMPTZ NOT NULL DEFAULT current_timestamp
);

CREATE TABLE IF NOT EXISTS images (
	id SERIAL PRIMARY KEY,
	alias VARCHAR NOT NULL,
	bytedata BYTEA NOT NULL,
	is_default boolean DEFAULT true,
	CONSTRAINT is_default_true_or_null CHECK (is_default),
  	CONSTRAINT is_default_only_1_true UNIQUE (is_default)
);

CREATE TABLE IF NOT EXISTS stats (
	id VARCHAR PRIMARY KEY,
	link_id INT NOT NULL,
	visit_source source NOT NULL,
	visited_at TIMESTAMPTZ NOT NULL DEFAULT current_timestamp 
);
