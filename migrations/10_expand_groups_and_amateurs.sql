-- Expand win_probability_group from 1-6 to 1-9 and add is_amateur flag to golfers

-- Rebuild golfers table with expanded CHECK constraint and is_amateur column
CREATE TABLE golfers_new (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    win_probability_group INTEGER NOT NULL CHECK (win_probability_group >= 1 AND win_probability_group <= 9),
    is_amateur INTEGER NOT NULL DEFAULT 0,
    is_active INTEGER NOT NULL DEFAULT 1,
    created_at TEXT DEFAULT (datetime('now')),
    UNIQUE(name)
);

INSERT INTO golfers_new (id, name, win_probability_group, is_amateur, is_active, created_at)
    SELECT id, name, win_probability_group, 0, is_active, created_at FROM golfers;

DROP TABLE golfers;
ALTER TABLE golfers_new RENAME TO golfers;

CREATE INDEX IF NOT EXISTS idx_golfers_group ON golfers(win_probability_group);
CREATE INDEX IF NOT EXISTS idx_golfers_active ON golfers(is_active);

-- Rebuild tournament_golfer_groups table with expanded CHECK constraint
CREATE TABLE tournament_golfer_groups_new (
    id TEXT PRIMARY KEY NOT NULL,
    tournament_id TEXT NOT NULL REFERENCES tournaments(id) ON DELETE CASCADE,
    golfer_id TEXT NOT NULL REFERENCES golfers(id) ON DELETE CASCADE,
    win_probability_group INTEGER NOT NULL CHECK (win_probability_group >= 1 AND win_probability_group <= 9),
    created_at TEXT DEFAULT (datetime('now')),
    UNIQUE(tournament_id, golfer_id)
);

INSERT INTO tournament_golfer_groups_new (id, tournament_id, golfer_id, win_probability_group, created_at)
    SELECT id, tournament_id, golfer_id, win_probability_group, created_at FROM tournament_golfer_groups;

DROP TABLE tournament_golfer_groups;
ALTER TABLE tournament_golfer_groups_new RENAME TO tournament_golfer_groups;

CREATE INDEX IF NOT EXISTS idx_tgg_tournament ON tournament_golfer_groups(tournament_id);
CREATE INDEX IF NOT EXISTS idx_tgg_golfer ON tournament_golfer_groups(golfer_id);
