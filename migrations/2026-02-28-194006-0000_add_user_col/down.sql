-- This file should undo anything in `up.sql`


ALTER TABLE connection
    DROP COLUMN username;
