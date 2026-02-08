CREATE TABLE IF NOT EXISTS team_golfers (
    id TEXT PRIMARY KEY NOT NULL,
    team_id TEXT NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
    golfer_id TEXT NOT NULL REFERENCES golfers(id) ON DELETE CASCADE,
    created_at TEXT DEFAULT (datetime('now')),
    UNIQUE(team_id, golfer_id)
);

CREATE INDEX IF NOT EXISTS idx_team_golfers_team ON team_golfers(team_id);
CREATE INDEX IF NOT EXISTS idx_team_golfers_golfer ON team_golfers(golfer_id);
