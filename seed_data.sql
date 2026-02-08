-- Sample Seed Data for Fantasy Golf (SQLite)
-- This file contains sample data to get started quickly

-- Note: Run this AFTER the migrations have been applied
-- Usage: sqlite3 feathers_and_fairways.db < seed_data.sql

-- Insert sample golfers across all 6 groups
-- Group 1 (Highest win probability)
INSERT INTO golfers (id, name, win_probability_group) VALUES
('g-001-scheffler', 'Scottie Scheffler', 1),
('g-002-rahm', 'Jon Rahm', 1),
('g-003-mcilroy', 'Rory McIlroy', 1),
('g-004-hovland', 'Viktor Hovland', 1);

-- Group 2
INSERT INTO golfers (id, name, win_probability_group) VALUES
('g-005-schauffele', 'Xander Schauffele', 2),
('g-006-cantlay', 'Patrick Cantlay', 2),
('g-007-clark', 'Wyndham Clark', 2),
('g-008-morikawa', 'Collin Morikawa', 2);

-- Group 3
INSERT INTO golfers (id, name, win_probability_group) VALUES
('g-009-koepka', 'Brooks Koepka', 3),
('g-010-homa', 'Max Homa', 3),
('g-011-fleetwood', 'Tommy Fleetwood', 3),
('g-012-spieth', 'Jordan Spieth', 3);

-- Group 4
INSERT INTO golfers (id, name, win_probability_group) VALUES
('g-013-finau', 'Tony Finau', 4),
('g-014-young', 'Cameron Young', 4),
('g-015-matsuyama', 'Hideki Matsuyama', 4),
('g-016-hatton', 'Tyrrell Hatton', 4);

-- Group 5
INSERT INTO golfers (id, name, win_probability_group) VALUES
('g-017-henley', 'Russell Henley', 5),
('g-018-bradley', 'Keegan Bradley', 5),
('g-019-harman', 'Brian Harman', 5),
('g-020-straka', 'Sepp Straka', 5);

-- Group 6 (Lowest win probability but higher risk/reward)
INSERT INTO golfers (id, name, win_probability_group) VALUES
('g-021-kim', 'Tom Kim', 6),
('g-022-aberg', 'Ludvig Aberg', 6),
('g-023-dunlap', 'Nick Dunlap', 6),
('g-024-bhatia', 'Akshay Bhatia', 6);

-- Create a sample season
INSERT INTO seasons (id, name, year, start_date, end_date, is_active) VALUES
('s-001-2025', 'PGA Tour 2025 Season', 2025, '2025-01-01', '2025-12-31', 1);
