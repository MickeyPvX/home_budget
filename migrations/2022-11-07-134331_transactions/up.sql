-- Your SQL goes here
CREATE TABLE transactions (
    id          SERIAL          PRIMARY KEY,
    date        date            NOT NULL,
    description varchar(128),
    category    varchar(32)     NOT NULL,
    amount      float           NOT NULL
)
