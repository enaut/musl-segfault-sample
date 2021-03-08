-- Your SQL goes here

CREATE TABLE users
(
    id INTEGER PRIMARY KEY NOT NULL,
    username VARCHAR NOT NULL,
    email VARCHAR NOT NULL,
    password VARCHAR NOT NULL,

    UNIQUE(username, email)
);