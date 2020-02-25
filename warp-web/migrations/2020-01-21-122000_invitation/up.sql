-- Your SQL goes here
CREATE TABLE session_tb (
  id UUID NOT NULL PRIMARY KEY,
  email VARCHAR(100) NOT NULL,
  expired_at TIMESTAMP NOT NULL
);