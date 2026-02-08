CREATE TABLE IF NOT EXISTS hole_scores (
    id TEXT PRIMARY KEY NOT NULL,
    tournament_id TEXT NOT NULL REFERENCES tournaments(id) ON DELETE CASCADE,
    golfer_id TEXT NOT NULL REFERENCES golfers(id) ON DELETE CASCADE,
    day INTEGER NOT NULL CHECK (day >= 1 AND day <= 4),
    hole INTEGER NOT NULL CHECK (hole >= 1 AND hole <= 18),
    strokes INTEGER NOT NULL CHECK (strokes > 0),
    score_to_par INTEGER NOT NULL,
    fantasy_points INTEGER NOT NULL,
    created_at TEXT DEFAULT (datetime('now')),
    UNIQUE(tournament_id, golfer_id, day, hole)
);

CREATE INDEX IF NOT EXISTS idx_hole_scores_tournament ON hole_scores(tournament_id);
CREATE INDEX IF NOT EXISTS idx_hole_scores_golfer ON hole_scores(golfer_id);
CREATE INDEX IF NOT EXISTS idx_hole_scores_day ON hole_scores(day);
