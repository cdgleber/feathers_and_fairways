-- Migration 12: Remove seasons — make tournaments standalone
-- Rebuilds tournaments, access_keys, and teams without season_id references,
-- then drops the seasons table.

-- 1. Rebuild tournaments without season_id
CREATE TABLE tournaments_new (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    start_date TEXT NOT NULL,
    end_date TEXT NOT NULL,
    is_active INTEGER NOT NULL DEFAULT 0,
    espn_tournament_id TEXT,
    created_at TEXT DEFAULT (datetime('now'))
);
INSERT INTO tournaments_new SELECT id, name, start_date, end_date, is_active, espn_tournament_id, created_at FROM tournaments;
DROP TABLE tournaments;
ALTER TABLE tournaments_new RENAME TO tournaments;
CREATE INDEX idx_tournaments_is_active ON tournaments(is_active);

-- 2. Rebuild access_keys: replace season_id with tournament_id
CREATE TABLE access_keys_new (
    id TEXT PRIMARY KEY,
    key_code TEXT NOT NULL UNIQUE,
    tournament_id TEXT NOT NULL REFERENCES tournaments(id) ON DELETE CASCADE,
    player_name TEXT,
    is_used INTEGER NOT NULL DEFAULT 0,
    used_at TEXT,
    created_at TEXT DEFAULT (datetime('now'))
);
-- Best-effort migration: map existing keys to the first tournament.
-- In practice this is a dev database so this is acceptable.
INSERT INTO access_keys_new
    SELECT ak.id, ak.key_code, t.id AS tournament_id, ak.player_name, ak.is_used, ak.used_at, ak.created_at
    FROM access_keys ak
    JOIN (SELECT id FROM tournaments LIMIT 1) t;
DROP TABLE access_keys;
ALTER TABLE access_keys_new RENAME TO access_keys;

-- 3. Rebuild teams without season_id (tournament_id already existed)
CREATE TABLE teams_new (
    id TEXT PRIMARY KEY,
    tournament_id TEXT NOT NULL REFERENCES tournaments(id) ON DELETE CASCADE,
    player_name TEXT NOT NULL,
    access_key_id TEXT REFERENCES access_keys(id),
    email TEXT,
    created_at TEXT DEFAULT (datetime('now')),
    UNIQUE(tournament_id, player_name)
);
INSERT INTO teams_new SELECT id, tournament_id, player_name, access_key_id, email, created_at FROM teams WHERE tournament_id IS NOT NULL;
DROP TABLE teams;
ALTER TABLE teams_new RENAME TO teams;

-- 4. Drop seasons table
DROP TABLE IF EXISTS seasons;
