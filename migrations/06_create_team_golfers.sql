CREATE TABLE IF NOT EXISTS team_golfers (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    team_id UUID NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
    golfer_id UUID NOT NULL REFERENCES golfers(id) ON DELETE CASCADE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(team_id, golfer_id)
);

CREATE INDEX IF NOT EXISTS idx_team_golfers_team ON team_golfers(team_id);
CREATE INDEX IF NOT EXISTS idx_team_golfers_golfer ON team_golfers(golfer_id);

-- Ensure each team has exactly 6 golfers
CREATE OR REPLACE FUNCTION check_team_golfer_count()
RETURNS TRIGGER AS $$
BEGIN
    IF (SELECT COUNT(*) FROM team_golfers WHERE team_id = NEW.team_id) > 6 THEN
        RAISE EXCEPTION 'A team cannot have more than 6 golfers';
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS enforce_team_golfer_limit ON team_golfers;
CREATE TRIGGER enforce_team_golfer_limit
    BEFORE INSERT ON team_golfers
    FOR EACH ROW
    EXECUTE FUNCTION check_team_golfer_count();