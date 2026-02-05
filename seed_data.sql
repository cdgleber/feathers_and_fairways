-- Sample Seed Data for Fantasy Golf
-- This file contains sample data to get started quickly

-- Note: Run this AFTER the migrations have been applied

-- Insert sample golfers across all 6 groups
-- Group 1 (Highest win probability)
INSERT INTO golfers (name, win_probability_group) VALUES
('Scottie Scheffler', 1),
('Jon Rahm', 1),
('Rory McIlroy', 1),
('Viktor Hovland', 1);

-- Group 2
INSERT INTO golfers (name, win_probability_group) VALUES
('Xander Schauffele', 2),
('Patrick Cantlay', 2),
('Wyndham Clark', 2),
('Collin Morikawa', 2);

-- Group 3
INSERT INTO golfers (name, win_probability_group) VALUES
('Brooks Koepka', 3),
('Max Homa', 3),
('Tommy Fleetwood', 3),
('Jordan Spieth', 3);

-- Group 4
INSERT INTO golfers (name, win_probability_group) VALUES
('Tony Finau', 4),
('Cameron Young', 4),
('Hideki Matsuyama', 4),
('Tyrrell Hatton', 4);

-- Group 5
INSERT INTO golfers (name, win_probability_group) VALUES
('Russell Henley', 5),
('Keegan Bradley', 5),
('Brian Harman', 5),
('Sepp Straka', 5);

-- Group 6 (Lowest win probability but higher risk/reward)
INSERT INTO golfers (name, win_probability_group) VALUES
('Tom Kim', 6),
('Ludvig Aberg', 6),
('Nick Dunlap', 6),
('Akshay Bhatia', 6);

-- Create a sample season
INSERT INTO seasons (name, year, start_date, end_date, is_active) VALUES
('PGA Tour 2025 Season', 2025, '2025-01-01', '2025-12-31', true);

-- Example: Create a tournament (requires the season_id from above)
-- INSERT INTO tournaments (season_id, name, start_date, end_date, is_active) VALUES
-- ((SELECT id FROM seasons WHERE year = 2025 LIMIT 1), 'The Masters', '2025-04-10', '2025-04-13', true);

-- You can generate access keys via the admin panel or with:
-- Example access key generation (requires season_id)
-- The application will generate these via the API, but here's an example:
-- INSERT INTO access_keys (key_code, season_id) VALUES
-- ('ABCD-EFGH-IJKL', (SELECT id FROM seasons WHERE year = 2025 LIMIT 1));

COMMIT;