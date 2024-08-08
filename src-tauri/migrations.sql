CREATE TABLE IF NOT EXISTS window_state (
    id INTEGER PRIMARY KEY,
    w INTEGER,
    h INTEGER,
    x INTEGER,
    y INTEGER
);

CREATE TABLE IF NOT EXISTS anime (
    id INTEGER PRIMARY KEY,
    image_url TEXT NOT NULL,
    small_image_url TEXT NOT NULL,
    large_image_url TEXT NOT NULL,
    title TEXT NOT NULL,
    title_english TEXT,
    title_japanese TEXT,
    rating TEXT,
    synopsis TEXT,
    score REAL DEFAULT 0,
    scored_by INTEGER,
    rank INTEGER DEFAULT 9999,
    year INTEGER DEFAULT 9999,
    to_watch BOOLEAN DEFAULT FALSE,
    watched BOOLEAN DEFAULT FALSE,
    watched_date TEXT DEFAULT '12-9999',
    my_score INTEGER DEFAULT 0
);

