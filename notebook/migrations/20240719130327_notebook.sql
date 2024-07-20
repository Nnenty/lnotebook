CREATE TABLE IF NOT EXISTS notebook (
    --ID        int  PRIMARY KEY--
    note_name TEXT UNIQUE NOT NULL,
    note      TEXT 
)
