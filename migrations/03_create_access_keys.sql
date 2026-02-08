CREATE TABLE IF NOT EXISTS access_keys (
    id TEXT PRIMARY KEY NOT NULL,
    key_code TEXT NOT NULL UNIQUE,
    season_id TEXT NOT NULL REFERENCES seasons(id) ON DELETE CASCADE,
    player_name TEXT,
    is_used INTEGER NOT NULL DEFAULT 0,
    used_at TEXT,
    created_at TEXT DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_access_keys_season ON access_keys(season_id);
CREATE INDEX IF NOT EXISTS idx_access_keys_code ON access_keys(key_code);
