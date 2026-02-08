CREATE TABLE IF NOT EXISTS tournament_golfer_groups (
    id TEXT PRIMARY KEY NOT NULL,
    tournament_id TEXT NOT NULL REFERENCES tournaments(id) ON DELETE CASCADE,
    golfer_id TEXT NOT NULL REFERENCES golfers(id) ON DELETE CASCADE,
    win_probability_group INTEGER NOT NULL CHECK (win_probability_group >= 1 AND win_probability_group <= 6),
    created_at TEXT DEFAULT (datetime('now')),
    UNIQUE(tournament_id, golfer_id)
);

CREATE INDEX IF NOT EXISTS idx_tgg_tournament ON tournament_golfer_groups(tournament_id);
CREATE INDEX IF NOT EXISTS idx_tgg_golfer ON tournament_golfer_groups(golfer_id);
