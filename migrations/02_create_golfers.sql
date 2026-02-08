CREATE TABLE IF NOT EXISTS golfers (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    win_probability_group INTEGER NOT NULL CHECK (win_probability_group >= 1 AND win_probability_group <= 6),
    is_active INTEGER NOT NULL DEFAULT 1,
    created_at TEXT DEFAULT (datetime('now')),
    UNIQUE(name)
);

CREATE INDEX IF NOT EXISTS idx_golfers_group ON golfers(win_probability_group);
CREATE INDEX IF NOT EXISTS idx_golfers_active ON golfers(is_active);
