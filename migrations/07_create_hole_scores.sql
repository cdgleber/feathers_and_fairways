CREATE TABLE IF NOT EXISTS hole_scores (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tournament_id UUID NOT NULL REFERENCES tournaments(id) ON DELETE CASCADE,
    golfer_id UUID NOT NULL REFERENCES golfers(id) ON DELETE CASCADE,
    day INTEGER NOT NULL CHECK (day >= 1 AND day <= 4),
    hole INTEGER NOT NULL CHECK (hole >= 1 AND hole <= 18),
    strokes INTEGER NOT NULL CHECK (strokes > 0),
    score_to_par INTEGER NOT NULL,
    fantasy_points INTEGER NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(tournament_id, golfer_id, day, hole)
);

CREATE INDEX IF NOT EXISTS idx_hole_scores_tournament ON hole_scores(tournament_id);
CREATE INDEX IF NOT EXISTS idx_hole_scores_golfer ON hole_scores(golfer_id);
CREATE INDEX IF NOT EXISTS idx_hole_scores_day ON hole_scores(day);

-- Function to calculate fantasy points based on score to par
CREATE OR REPLACE FUNCTION calculate_fantasy_points(par_score INTEGER)
RETURNS INTEGER AS $$
BEGIN
    RETURN CASE
        WHEN par_score <= -2 THEN 2  -- Eagle or better
        WHEN par_score = -1 THEN 1   -- Birdie
        WHEN par_score = 0 THEN 0    -- Par
        WHEN par_score >= 1 THEN -1  -- Bogey or worse
    END;
END;
$$ LANGUAGE plpgsql IMMUTABLE;

-- Automatically calculate fantasy points on insert/update
CREATE OR REPLACE FUNCTION set_fantasy_points()
RETURNS TRIGGER AS $$
BEGIN
    NEW.fantasy_points = calculate_fantasy_points(NEW.score_to_par);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS auto_calculate_fantasy_points ON hole_scores;

CREATE TRIGGER auto_calculate_fantasy_points
    BEFORE INSERT OR UPDATE ON hole_scores
    FOR EACH ROW
    EXECUTE FUNCTION set_fantasy_points();
