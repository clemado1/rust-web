-- Your SQL goes here
CREATE TABLE user_tb (
  email VARCHAR(100) NOT NULL PRIMARY KEY,
  passwd VARCHAR(200) NOT NULL, --bcrypt
  username VARCHAR(20) NOT NULL,
  nickname VARCHAR(20) NOT NULL,
  created_at TIMESTAMP NOT NULL
);