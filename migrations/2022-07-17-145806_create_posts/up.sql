-- Your SQL goes here
CREATE TABLE posts (
  datetime timestamp with time zone PRIMARY KEY,
  temperature real,
  relative_humidity real,
  atmospheric_pressure real
)