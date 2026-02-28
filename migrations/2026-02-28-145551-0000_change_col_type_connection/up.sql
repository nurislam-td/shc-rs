-- Your SQL goes here
ALTER TABLE connection
    RENAME TO old_connection;


CREATE TABLE connection
(
    id       INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name     TEXT                              NOT NULL,
    password BLOB                              NOT NULL,           -- Теперь это байты (BLOB)
    host     TEXT                              NOT NULL,
    port     INTEGER                           NOT NULL DEFAULT 22 -- Новое поле
);

INSERT INTO connection (id, name, password, host, port)
SELECT id, name, CAST(password AS BLOB), host, 5432
FROM old_connection;

DROP TABLE old_connection;
