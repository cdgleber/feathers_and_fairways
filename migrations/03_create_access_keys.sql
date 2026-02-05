CREATE TABLE IF NOT EXISTS access_keys (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    key_code VARCHAR(50) NOT NULL UNIQUE,
    season_id UUID NOT NULL REFERENCES seasons(id) ON DELETE CASCADE,
    player_name VARCHAR(255),
    is_used BOOLEAN NOT NULL DEFAULT false,
    used_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_access_keys_season ON access_keys(season_id);
CREATE INDEX idx_access_keys_code ON access_keys(key_code);
