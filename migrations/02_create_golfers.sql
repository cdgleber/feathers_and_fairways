CREATE TABLE IF NOT EXISTS golfers (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    win_probability_group INTEGER NOT NULL CHECK (win_probability_group >= 1 AND win_probability_group <= 6),
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(name)
);

CREATE INDEX idx_golfers_group ON golfers(win_probability_group);
CREATE INDEX idx_golfers_active ON golfers(is_active);
