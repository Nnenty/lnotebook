CREATE TABLE IF NOT EXISTS notebook (
    id SERIAL,
    note_name TEXT UNIQUE NOT NULL,
    note TEXT,
    PRIMARY KEY (id)
)
