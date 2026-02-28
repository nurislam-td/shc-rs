-- This file should undo anything in `up.sql`
ALTER TABLE connection
    RENAME TO new_connection;

CREATE TABLE connection
(
    id       INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name     TEXT                              NOT NULL,
    password TEXT                              NOT NULL,
    host     TEXT                              NOT NULL
);

INSERT INTO connection (id, name, password, host)
SELECT id, name, CAST(password AS TEXT), host
FROM new_connection;

DROP TABLE new_connection;
