-- Your SQL goes here
CREATE TABLE user_tb (
  email VARCHAR(100) NOT NULL PRIMARY KEY,
  passwd VARCHAR(200), --bcrypt
  username VARCHAR(20),
  nickname VARCHAR(20),
  created_at TIMESTAMP NOT NULL
);