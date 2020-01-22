-- Your SQL goes here
CREATE TABLE user_tb (
  email VARCHAR(100) NOT NULL PRIMARY KEY,
  hash VARCHAR(122) NOT NULL, --argon hash
  passwd VARCHAR(200), --bcrypt
  username VARCHAR(20),
  nickname VARCHAR(20),
  created_at TIMESTAMP NOT NULL
);