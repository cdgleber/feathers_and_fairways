CREATE TABLE IF NOT EXISTS teams (
    id TEXT PRIMARY KEY NOT NULL,
    season_id TEXT NOT NULL REFERENCES seasons(id) ON DELETE CASCADE,
    tournament_id TEXT NOT NULL REFERENCES tournaments(id) ON DELETE CASCADE,
    player_name TEXT NOT NULL,
    access_key_id TEXT NOT NULL REFERENCES access_keys(id),
    created_at TEXT DEFAULT (datetime('now')),
    UNIQUE(tournament_id, player_name)
);

CREATE INDEX IF NOT EXISTS idx_teams_tournament ON teams(tournament_id);
CREATE INDEX IF NOT EXISTS idx_teams_player ON teams(player_name);
