CREATE TABLE IF NOT EXISTS seasons (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    year INTEGER NOT NULL,
    start_date TEXT NOT NULL,
    end_date TEXT NOT NULL,
    is_active INTEGER NOT NULL DEFAULT 0,
    created_at TEXT DEFAULT (datetime('now')),
    UNIQUE(year)
);

CREATE INDEX IF NOT EXISTS idx_seasons_active ON seasons(is_active);
