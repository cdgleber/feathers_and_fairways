-- Add ESPN ID columns for score refresh support
ALTER TABLE tournaments ADD COLUMN espn_tournament_id TEXT;
ALTER TABLE golfers ADD COLUMN espn_id TEXT;

CREATE INDEX idx_tournaments_espn_id ON tournaments(espn_tournament_id);
CREATE INDEX idx_golfers_espn_id ON golfers(espn_id);
