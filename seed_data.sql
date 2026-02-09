-- Seed Data for Fantasy Golf (SQLite)
-- Run AFTER migrations have been applied
-- Usage: sqlite3 feathers_and_fairways.db < seed_data.sql

-- =============================================
-- GOLFERS (24 golfers, 4 per group)
-- =============================================

-- Group 1 (Elite)
INSERT OR IGNORE INTO golfers (id, name, win_probability_group) VALUES
('g-001-scheffler', 'Scottie Scheffler', 1),
('g-002-rahm', 'Jon Rahm', 1),
('g-003-mcilroy', 'Rory McIlroy', 1),
('g-004-hovland', 'Viktor Hovland', 1);

-- Group 2
INSERT OR IGNORE INTO golfers (id, name, win_probability_group) VALUES
('g-005-schauffele', 'Xander Schauffele', 2),
('g-006-cantlay', 'Patrick Cantlay', 2),
('g-007-clark', 'Wyndham Clark', 2),
('g-008-morikawa', 'Collin Morikawa', 2);

-- Group 3
INSERT OR IGNORE INTO golfers (id, name, win_probability_group) VALUES
('g-009-koepka', 'Brooks Koepka', 3),
('g-010-homa', 'Max Homa', 3),
('g-011-fleetwood', 'Tommy Fleetwood', 3),
('g-012-spieth', 'Jordan Spieth', 3);

-- Group 4
INSERT OR IGNORE INTO golfers (id, name, win_probability_group) VALUES
('g-013-finau', 'Tony Finau', 4),
('g-014-young', 'Cameron Young', 4),
('g-015-matsuyama', 'Hideki Matsuyama', 4),
('g-016-hatton', 'Tyrrell Hatton', 4);

-- Group 5
INSERT OR IGNORE INTO golfers (id, name, win_probability_group) VALUES
('g-017-henley', 'Russell Henley', 5),
('g-018-bradley', 'Keegan Bradley', 5),
('g-019-harman', 'Brian Harman', 5),
('g-020-straka', 'Sepp Straka', 5);

-- Group 6 (Dark horses)
INSERT OR IGNORE INTO golfers (id, name, win_probability_group) VALUES
('g-021-kim', 'Tom Kim', 6),
('g-022-aberg', 'Ludvig Aberg', 6),
('g-023-dunlap', 'Nick Dunlap', 6),
('g-024-bhatia', 'Akshay Bhatia', 6);

-- =============================================
-- SEASON
-- =============================================
INSERT OR IGNORE INTO seasons (id, name, year, start_date, end_date, is_active) VALUES
('s-001-2025', 'PGA Tour 2025 Season', 2025, '2025-01-01', '2025-12-31', 1);

-- =============================================
-- TOURNAMENTS (3 tournaments, 1 completed, 1 active, 1 upcoming)
-- =============================================
INSERT OR IGNORE INTO tournaments (id, season_id, name, start_date, end_date, is_active) VALUES
('t-001-masters', 's-001-2025', 'The Masters', '2025-04-10', '2025-04-13', 0),
('t-002-pga', 's-001-2025', 'PGA Championship', '2025-05-15', '2025-05-18', 1),
('t-003-usopen', 's-001-2025', 'US Open', '2025-06-12', '2025-06-15', 0);

-- =============================================
-- ACCESS KEYS (6 keys, 4 used by players, 2 unclaimed)
-- =============================================
INSERT OR IGNORE INTO access_keys (id, key_code, season_id, player_name, is_used, used_at) VALUES
('ak-001', 'SEED-KEY-AA01', 's-001-2025', 'Alice Johnson', 1, '2025-03-01 10:00:00'),
('ak-002', 'SEED-KEY-BB02', 's-001-2025', 'Bob Smith', 1, '2025-03-02 14:30:00'),
('ak-003', 'SEED-KEY-CC03', 's-001-2025', 'Charlie Davis', 1, '2025-03-05 09:15:00'),
('ak-004', 'SEED-KEY-DD04', 's-001-2025', 'Dana Wilson', 1, '2025-03-10 16:45:00'),
('ak-005', 'SEED-KEY-EE05', 's-001-2025', NULL, 0, NULL),
('ak-006', 'SEED-KEY-FF06', 's-001-2025', NULL, 0, NULL);

-- =============================================
-- TEAMS (4 players x 2 tournaments = 8 teams)
-- Each team picks 1 golfer per group (6 golfers)
-- =============================================

-- Masters teams
INSERT OR IGNORE INTO teams (id, season_id, tournament_id, player_name, access_key_id) VALUES
('team-001-alice-masters', 's-001-2025', 't-001-masters', 'Alice Johnson', 'ak-001'),
('team-002-bob-masters', 's-001-2025', 't-001-masters', 'Bob Smith', 'ak-002'),
('team-003-charlie-masters', 's-001-2025', 't-001-masters', 'Charlie Davis', 'ak-003'),
('team-004-dana-masters', 's-001-2025', 't-001-masters', 'Dana Wilson', 'ak-004');

-- PGA Championship teams
INSERT OR IGNORE INTO teams (id, season_id, tournament_id, player_name, access_key_id) VALUES
('team-005-alice-pga', 's-001-2025', 't-002-pga', 'Alice Johnson', 'ak-001'),
('team-006-bob-pga', 's-001-2025', 't-002-pga', 'Bob Smith', 'ak-002'),
('team-007-charlie-pga', 's-001-2025', 't-002-pga', 'Charlie Davis', 'ak-003'),
('team-008-dana-pga', 's-001-2025', 't-002-pga', 'Dana Wilson', 'ak-004');

-- =============================================
-- TEAM GOLFERS (6 golfers per team)
-- =============================================

-- Alice's Masters team: Scheffler(1), Schauffele(2), Koepka(3), Finau(4), Henley(5), Kim(6)
INSERT OR IGNORE INTO team_golfers (id, team_id, golfer_id) VALUES
('tg-001', 'team-001-alice-masters', 'g-001-scheffler'),
('tg-002', 'team-001-alice-masters', 'g-005-schauffele'),
('tg-003', 'team-001-alice-masters', 'g-009-koepka'),
('tg-004', 'team-001-alice-masters', 'g-013-finau'),
('tg-005', 'team-001-alice-masters', 'g-017-henley'),
('tg-006', 'team-001-alice-masters', 'g-021-kim');

-- Bob's Masters team: Rahm(1), Cantlay(2), Homa(3), Young(4), Bradley(5), Aberg(6)
INSERT OR IGNORE INTO team_golfers (id, team_id, golfer_id) VALUES
('tg-007', 'team-002-bob-masters', 'g-002-rahm'),
('tg-008', 'team-002-bob-masters', 'g-006-cantlay'),
('tg-009', 'team-002-bob-masters', 'g-010-homa'),
('tg-010', 'team-002-bob-masters', 'g-014-young'),
('tg-011', 'team-002-bob-masters', 'g-018-bradley'),
('tg-012', 'team-002-bob-masters', 'g-022-aberg');

-- Charlie's Masters team: McIlroy(1), Clark(2), Fleetwood(3), Matsuyama(4), Harman(5), Dunlap(6)
INSERT OR IGNORE INTO team_golfers (id, team_id, golfer_id) VALUES
('tg-013', 'team-003-charlie-masters', 'g-003-mcilroy'),
('tg-014', 'team-003-charlie-masters', 'g-007-clark'),
('tg-015', 'team-003-charlie-masters', 'g-011-fleetwood'),
('tg-016', 'team-003-charlie-masters', 'g-015-matsuyama'),
('tg-017', 'team-003-charlie-masters', 'g-019-harman'),
('tg-018', 'team-003-charlie-masters', 'g-023-dunlap');

-- Dana's Masters team: Hovland(1), Morikawa(2), Spieth(3), Hatton(4), Straka(5), Bhatia(6)
INSERT OR IGNORE INTO team_golfers (id, team_id, golfer_id) VALUES
('tg-019', 'team-004-dana-masters', 'g-004-hovland'),
('tg-020', 'team-004-dana-masters', 'g-008-morikawa'),
('tg-021', 'team-004-dana-masters', 'g-012-spieth'),
('tg-022', 'team-004-dana-masters', 'g-016-hatton'),
('tg-023', 'team-004-dana-masters', 'g-020-straka'),
('tg-024', 'team-004-dana-masters', 'g-024-bhatia');

-- Alice's PGA team: McIlroy(1), Morikawa(2), Spieth(3), Matsuyama(4), Harman(5), Aberg(6)
INSERT OR IGNORE INTO team_golfers (id, team_id, golfer_id) VALUES
('tg-025', 'team-005-alice-pga', 'g-003-mcilroy'),
('tg-026', 'team-005-alice-pga', 'g-008-morikawa'),
('tg-027', 'team-005-alice-pga', 'g-012-spieth'),
('tg-028', 'team-005-alice-pga', 'g-015-matsuyama'),
('tg-029', 'team-005-alice-pga', 'g-019-harman'),
('tg-030', 'team-005-alice-pga', 'g-022-aberg');

-- Bob's PGA team: Scheffler(1), Schauffele(2), Fleetwood(3), Hatton(4), Straka(5), Kim(6)
INSERT OR IGNORE INTO team_golfers (id, team_id, golfer_id) VALUES
('tg-031', 'team-006-bob-pga', 'g-001-scheffler'),
('tg-032', 'team-006-bob-pga', 'g-005-schauffele'),
('tg-033', 'team-006-bob-pga', 'g-011-fleetwood'),
('tg-034', 'team-006-bob-pga', 'g-016-hatton'),
('tg-035', 'team-006-bob-pga', 'g-020-straka'),
('tg-036', 'team-006-bob-pga', 'g-021-kim');

-- Charlie's PGA team: Rahm(1), Clark(2), Koepka(3), Finau(4), Bradley(5), Dunlap(6)
INSERT OR IGNORE INTO team_golfers (id, team_id, golfer_id) VALUES
('tg-037', 'team-007-charlie-pga', 'g-002-rahm'),
('tg-038', 'team-007-charlie-pga', 'g-007-clark'),
('tg-039', 'team-007-charlie-pga', 'g-009-koepka'),
('tg-040', 'team-007-charlie-pga', 'g-013-finau'),
('tg-041', 'team-007-charlie-pga', 'g-018-bradley'),
('tg-042', 'team-007-charlie-pga', 'g-023-dunlap');

-- Dana's PGA team: Hovland(1), Cantlay(2), Homa(3), Young(4), Henley(5), Bhatia(6)
INSERT OR IGNORE INTO team_golfers (id, team_id, golfer_id) VALUES
('tg-043', 'team-008-dana-pga', 'g-004-hovland'),
('tg-044', 'team-008-dana-pga', 'g-006-cantlay'),
('tg-045', 'team-008-dana-pga', 'g-010-homa'),
('tg-046', 'team-008-dana-pga', 'g-014-young'),
('tg-047', 'team-008-dana-pga', 'g-017-henley'),
('tg-048', 'team-008-dana-pga', 'g-024-bhatia');

-- =============================================
-- HOLE SCORES for The Masters (completed tournament)
-- Day 1 only (18 holes) for 8 golfers to keep it manageable
-- Par layout: 4,5,4,3,4,3,4,5,4,4,4,3,5,4,5,3,4,4
-- Fantasy points: eagle_or_better=+2, birdie=+1, par=0, bogey_or_worse=-1
-- =============================================

-- Scheffler Day 1: Strong round (-5). 4 birdies, 1 eagle, 13 pars = 4+2+0 = 6 pts
INSERT OR IGNORE INTO hole_scores (id, tournament_id, golfer_id, day, hole, strokes, score_to_par, fantasy_points) VALUES
('hs-001', 't-001-masters', 'g-001-scheffler', 1, 1, 3, -1, 1),
('hs-002', 't-001-masters', 'g-001-scheffler', 1, 2, 5, 0, 0),
('hs-003', 't-001-masters', 'g-001-scheffler', 1, 3, 4, 0, 0),
('hs-004', 't-001-masters', 'g-001-scheffler', 1, 4, 3, 0, 0),
('hs-005', 't-001-masters', 'g-001-scheffler', 1, 5, 3, -1, 1),
('hs-006', 't-001-masters', 'g-001-scheffler', 1, 6, 3, 0, 0),
('hs-007', 't-001-masters', 'g-001-scheffler', 1, 7, 4, 0, 0),
('hs-008', 't-001-masters', 'g-001-scheffler', 1, 8, 3, -2, 2),
('hs-009', 't-001-masters', 'g-001-scheffler', 1, 9, 4, 0, 0),
('hs-010', 't-001-masters', 'g-001-scheffler', 1, 10, 4, 0, 0),
('hs-011', 't-001-masters', 'g-001-scheffler', 1, 11, 4, 0, 0),
('hs-012', 't-001-masters', 'g-001-scheffler', 1, 12, 3, 0, 0),
('hs-013', 't-001-masters', 'g-001-scheffler', 1, 13, 4, -1, 1),
('hs-014', 't-001-masters', 'g-001-scheffler', 1, 14, 4, 0, 0),
('hs-015', 't-001-masters', 'g-001-scheffler', 1, 15, 5, 0, 0),
('hs-016', 't-001-masters', 'g-001-scheffler', 1, 16, 3, 0, 0),
('hs-017', 't-001-masters', 'g-001-scheffler', 1, 17, 4, 0, 0),
('hs-018', 't-001-masters', 'g-001-scheffler', 1, 18, 3, -1, 1);

-- Rahm Day 1: Good round (-3). 3 birdies, 15 pars = 3 pts
INSERT OR IGNORE INTO hole_scores (id, tournament_id, golfer_id, day, hole, strokes, score_to_par, fantasy_points) VALUES
('hs-019', 't-001-masters', 'g-002-rahm', 1, 1, 4, 0, 0),
('hs-020', 't-001-masters', 'g-002-rahm', 1, 2, 4, -1, 1),
('hs-021', 't-001-masters', 'g-002-rahm', 1, 3, 4, 0, 0),
('hs-022', 't-001-masters', 'g-002-rahm', 1, 4, 3, 0, 0),
('hs-023', 't-001-masters', 'g-002-rahm', 1, 5, 4, 0, 0),
('hs-024', 't-001-masters', 'g-002-rahm', 1, 6, 3, 0, 0),
('hs-025', 't-001-masters', 'g-002-rahm', 1, 7, 3, -1, 1),
('hs-026', 't-001-masters', 'g-002-rahm', 1, 8, 5, 0, 0),
('hs-027', 't-001-masters', 'g-002-rahm', 1, 9, 4, 0, 0),
('hs-028', 't-001-masters', 'g-002-rahm', 1, 10, 4, 0, 0),
('hs-029', 't-001-masters', 'g-002-rahm', 1, 11, 4, 0, 0),
('hs-030', 't-001-masters', 'g-002-rahm', 1, 12, 3, 0, 0),
('hs-031', 't-001-masters', 'g-002-rahm', 1, 13, 5, 0, 0),
('hs-032', 't-001-masters', 'g-002-rahm', 1, 14, 4, 0, 0),
('hs-033', 't-001-masters', 'g-002-rahm', 1, 15, 4, -1, 1),
('hs-034', 't-001-masters', 'g-002-rahm', 1, 16, 3, 0, 0),
('hs-035', 't-001-masters', 'g-002-rahm', 1, 17, 4, 0, 0),
('hs-036', 't-001-masters', 'g-002-rahm', 1, 18, 4, 0, 0);

-- McIlroy Day 1: Decent round (-2). 4 birdies, 2 bogeys, 12 pars = 4-2+0 = 2 pts
INSERT OR IGNORE INTO hole_scores (id, tournament_id, golfer_id, day, hole, strokes, score_to_par, fantasy_points) VALUES
('hs-037', 't-001-masters', 'g-003-mcilroy', 1, 1, 3, -1, 1),
('hs-038', 't-001-masters', 'g-003-mcilroy', 1, 2, 5, 0, 0),
('hs-039', 't-001-masters', 'g-003-mcilroy', 1, 3, 5, 1, -1),
('hs-040', 't-001-masters', 'g-003-mcilroy', 1, 4, 3, 0, 0),
('hs-041', 't-001-masters', 'g-003-mcilroy', 1, 5, 3, -1, 1),
('hs-042', 't-001-masters', 'g-003-mcilroy', 1, 6, 3, 0, 0),
('hs-043', 't-001-masters', 'g-003-mcilroy', 1, 7, 4, 0, 0),
('hs-044', 't-001-masters', 'g-003-mcilroy', 1, 8, 5, 0, 0),
('hs-045', 't-001-masters', 'g-003-mcilroy', 1, 9, 4, 0, 0),
('hs-046', 't-001-masters', 'g-003-mcilroy', 1, 10, 3, -1, 1),
('hs-047', 't-001-masters', 'g-003-mcilroy', 1, 11, 4, 0, 0),
('hs-048', 't-001-masters', 'g-003-mcilroy', 1, 12, 3, 0, 0),
('hs-049', 't-001-masters', 'g-003-mcilroy', 1, 13, 5, 0, 0),
('hs-050', 't-001-masters', 'g-003-mcilroy', 1, 14, 5, 1, -1),
('hs-051', 't-001-masters', 'g-003-mcilroy', 1, 15, 4, -1, 1),
('hs-052', 't-001-masters', 'g-003-mcilroy', 1, 16, 3, 0, 0),
('hs-053', 't-001-masters', 'g-003-mcilroy', 1, 17, 4, 0, 0),
('hs-054', 't-001-masters', 'g-003-mcilroy', 1, 18, 4, 0, 0);

-- Hovland Day 1: Even par. 2 birdies, 2 bogeys, 14 pars = 0 pts
INSERT OR IGNORE INTO hole_scores (id, tournament_id, golfer_id, day, hole, strokes, score_to_par, fantasy_points) VALUES
('hs-055', 't-001-masters', 'g-004-hovland', 1, 1, 4, 0, 0),
('hs-056', 't-001-masters', 'g-004-hovland', 1, 2, 5, 0, 0),
('hs-057', 't-001-masters', 'g-004-hovland', 1, 3, 4, 0, 0),
('hs-058', 't-001-masters', 'g-004-hovland', 1, 4, 2, -1, 1),
('hs-059', 't-001-masters', 'g-004-hovland', 1, 5, 5, 1, -1),
('hs-060', 't-001-masters', 'g-004-hovland', 1, 6, 3, 0, 0),
('hs-061', 't-001-masters', 'g-004-hovland', 1, 7, 4, 0, 0),
('hs-062', 't-001-masters', 'g-004-hovland', 1, 8, 5, 0, 0),
('hs-063', 't-001-masters', 'g-004-hovland', 1, 9, 4, 0, 0),
('hs-064', 't-001-masters', 'g-004-hovland', 1, 10, 4, 0, 0),
('hs-065', 't-001-masters', 'g-004-hovland', 1, 11, 5, 1, -1),
('hs-066', 't-001-masters', 'g-004-hovland', 1, 12, 3, 0, 0),
('hs-067', 't-001-masters', 'g-004-hovland', 1, 13, 5, 0, 0),
('hs-068', 't-001-masters', 'g-004-hovland', 1, 14, 3, -1, 1),
('hs-069', 't-001-masters', 'g-004-hovland', 1, 15, 5, 0, 0),
('hs-070', 't-001-masters', 'g-004-hovland', 1, 16, 3, 0, 0),
('hs-071', 't-001-masters', 'g-004-hovland', 1, 17, 4, 0, 0),
('hs-072', 't-001-masters', 'g-004-hovland', 1, 18, 4, 0, 0);

-- Schauffele Day 1: Great round (-4). 4 birdies, 14 pars = 4 pts
INSERT OR IGNORE INTO hole_scores (id, tournament_id, golfer_id, day, hole, strokes, score_to_par, fantasy_points) VALUES
('hs-073', 't-001-masters', 'g-005-schauffele', 1, 1, 4, 0, 0),
('hs-074', 't-001-masters', 'g-005-schauffele', 1, 2, 4, -1, 1),
('hs-075', 't-001-masters', 'g-005-schauffele', 1, 3, 4, 0, 0),
('hs-076', 't-001-masters', 'g-005-schauffele', 1, 4, 3, 0, 0),
('hs-077', 't-001-masters', 'g-005-schauffele', 1, 5, 4, 0, 0),
('hs-078', 't-001-masters', 'g-005-schauffele', 1, 6, 2, -1, 1),
('hs-079', 't-001-masters', 'g-005-schauffele', 1, 7, 4, 0, 0),
('hs-080', 't-001-masters', 'g-005-schauffele', 1, 8, 5, 0, 0),
('hs-081', 't-001-masters', 'g-005-schauffele', 1, 9, 4, 0, 0),
('hs-082', 't-001-masters', 'g-005-schauffele', 1, 10, 4, 0, 0),
('hs-083', 't-001-masters', 'g-005-schauffele', 1, 11, 4, 0, 0),
('hs-084', 't-001-masters', 'g-005-schauffele', 1, 12, 3, 0, 0),
('hs-085', 't-001-masters', 'g-005-schauffele', 1, 13, 4, -1, 1),
('hs-086', 't-001-masters', 'g-005-schauffele', 1, 14, 4, 0, 0),
('hs-087', 't-001-masters', 'g-005-schauffele', 1, 15, 5, 0, 0),
('hs-088', 't-001-masters', 'g-005-schauffele', 1, 16, 2, -1, 1),
('hs-089', 't-001-masters', 'g-005-schauffele', 1, 17, 4, 0, 0),
('hs-090', 't-001-masters', 'g-005-schauffele', 1, 18, 4, 0, 0);

-- Cantlay Day 1: Solid round (-1). 2 birdies, 1 bogey, 15 pars = 1 pt
INSERT OR IGNORE INTO hole_scores (id, tournament_id, golfer_id, day, hole, strokes, score_to_par, fantasy_points) VALUES
('hs-091', 't-001-masters', 'g-006-cantlay', 1, 1, 4, 0, 0),
('hs-092', 't-001-masters', 'g-006-cantlay', 1, 2, 5, 0, 0),
('hs-093', 't-001-masters', 'g-006-cantlay', 1, 3, 3, -1, 1),
('hs-094', 't-001-masters', 'g-006-cantlay', 1, 4, 3, 0, 0),
('hs-095', 't-001-masters', 'g-006-cantlay', 1, 5, 4, 0, 0),
('hs-096', 't-001-masters', 'g-006-cantlay', 1, 6, 3, 0, 0),
('hs-097', 't-001-masters', 'g-006-cantlay', 1, 7, 4, 0, 0),
('hs-098', 't-001-masters', 'g-006-cantlay', 1, 8, 5, 0, 0),
('hs-099', 't-001-masters', 'g-006-cantlay', 1, 9, 5, 1, -1),
('hs-100', 't-001-masters', 'g-006-cantlay', 1, 10, 4, 0, 0),
('hs-101', 't-001-masters', 'g-006-cantlay', 1, 11, 4, 0, 0),
('hs-102', 't-001-masters', 'g-006-cantlay', 1, 12, 3, 0, 0),
('hs-103', 't-001-masters', 'g-006-cantlay', 1, 13, 5, 0, 0),
('hs-104', 't-001-masters', 'g-006-cantlay', 1, 14, 4, 0, 0),
('hs-105', 't-001-masters', 'g-006-cantlay', 1, 15, 4, -1, 1),
('hs-106', 't-001-masters', 'g-006-cantlay', 1, 16, 3, 0, 0),
('hs-107', 't-001-masters', 'g-006-cantlay', 1, 17, 4, 0, 0),
('hs-108', 't-001-masters', 'g-006-cantlay', 1, 18, 4, 0, 0);

-- Koepka Day 1: Rough round (+2). 1 birdie, 3 bogeys, 14 pars = -2 pts
INSERT OR IGNORE INTO hole_scores (id, tournament_id, golfer_id, day, hole, strokes, score_to_par, fantasy_points) VALUES
('hs-109', 't-001-masters', 'g-009-koepka', 1, 1, 5, 1, -1),
('hs-110', 't-001-masters', 'g-009-koepka', 1, 2, 5, 0, 0),
('hs-111', 't-001-masters', 'g-009-koepka', 1, 3, 4, 0, 0),
('hs-112', 't-001-masters', 'g-009-koepka', 1, 4, 3, 0, 0),
('hs-113', 't-001-masters', 'g-009-koepka', 1, 5, 4, 0, 0),
('hs-114', 't-001-masters', 'g-009-koepka', 1, 6, 3, 0, 0),
('hs-115', 't-001-masters', 'g-009-koepka', 1, 7, 4, 0, 0),
('hs-116', 't-001-masters', 'g-009-koepka', 1, 8, 5, 0, 0),
('hs-117', 't-001-masters', 'g-009-koepka', 1, 9, 4, 0, 0),
('hs-118', 't-001-masters', 'g-009-koepka', 1, 10, 5, 1, -1),
('hs-119', 't-001-masters', 'g-009-koepka', 1, 11, 4, 0, 0),
('hs-120', 't-001-masters', 'g-009-koepka', 1, 12, 3, 0, 0),
('hs-121', 't-001-masters', 'g-009-koepka', 1, 13, 4, -1, 1),
('hs-122', 't-001-masters', 'g-009-koepka', 1, 14, 4, 0, 0),
('hs-123', 't-001-masters', 'g-009-koepka', 1, 15, 5, 0, 0),
('hs-124', 't-001-masters', 'g-009-koepka', 1, 16, 4, 1, -1),
('hs-125', 't-001-masters', 'g-009-koepka', 1, 17, 4, 0, 0),
('hs-126', 't-001-masters', 'g-009-koepka', 1, 18, 4, 0, 0);

-- Fleetwood Day 1: Par round (E). 1 birdie, 1 bogey, 16 pars = 0 pts
INSERT OR IGNORE INTO hole_scores (id, tournament_id, golfer_id, day, hole, strokes, score_to_par, fantasy_points) VALUES
('hs-127', 't-001-masters', 'g-011-fleetwood', 1, 1, 4, 0, 0),
('hs-128', 't-001-masters', 'g-011-fleetwood', 1, 2, 5, 0, 0),
('hs-129', 't-001-masters', 'g-011-fleetwood', 1, 3, 4, 0, 0),
('hs-130', 't-001-masters', 'g-011-fleetwood', 1, 4, 3, 0, 0),
('hs-131', 't-001-masters', 'g-011-fleetwood', 1, 5, 3, -1, 1),
('hs-132', 't-001-masters', 'g-011-fleetwood', 1, 6, 3, 0, 0),
('hs-133', 't-001-masters', 'g-011-fleetwood', 1, 7, 4, 0, 0),
('hs-134', 't-001-masters', 'g-011-fleetwood', 1, 8, 5, 0, 0),
('hs-135', 't-001-masters', 'g-011-fleetwood', 1, 9, 4, 0, 0),
('hs-136', 't-001-masters', 'g-011-fleetwood', 1, 10, 4, 0, 0),
('hs-137', 't-001-masters', 'g-011-fleetwood', 1, 11, 4, 0, 0),
('hs-138', 't-001-masters', 'g-011-fleetwood', 1, 12, 4, 1, -1),
('hs-139', 't-001-masters', 'g-011-fleetwood', 1, 13, 5, 0, 0),
('hs-140', 't-001-masters', 'g-011-fleetwood', 1, 14, 4, 0, 0),
('hs-141', 't-001-masters', 'g-011-fleetwood', 1, 15, 5, 0, 0),
('hs-142', 't-001-masters', 'g-011-fleetwood', 1, 16, 3, 0, 0),
('hs-143', 't-001-masters', 'g-011-fleetwood', 1, 17, 4, 0, 0),
('hs-144', 't-001-masters', 'g-011-fleetwood', 1, 18, 4, 0, 0);

-- Finau Day 1: Decent (-1). 3 birdies, 2 bogeys, 13 pars = 1 pt
INSERT OR IGNORE INTO hole_scores (id, tournament_id, golfer_id, day, hole, strokes, score_to_par, fantasy_points) VALUES
('hs-145', 't-001-masters', 'g-013-finau', 1, 1, 4, 0, 0),
('hs-146', 't-001-masters', 'g-013-finau', 1, 2, 4, -1, 1),
('hs-147', 't-001-masters', 'g-013-finau', 1, 3, 4, 0, 0),
('hs-148', 't-001-masters', 'g-013-finau', 1, 4, 4, 1, -1),
('hs-149', 't-001-masters', 'g-013-finau', 1, 5, 4, 0, 0),
('hs-150', 't-001-masters', 'g-013-finau', 1, 6, 2, -1, 1),
('hs-151', 't-001-masters', 'g-013-finau', 1, 7, 4, 0, 0),
('hs-152', 't-001-masters', 'g-013-finau', 1, 8, 5, 0, 0),
('hs-153', 't-001-masters', 'g-013-finau', 1, 9, 4, 0, 0),
('hs-154', 't-001-masters', 'g-013-finau', 1, 10, 4, 0, 0),
('hs-155', 't-001-masters', 'g-013-finau', 1, 11, 5, 1, -1),
('hs-156', 't-001-masters', 'g-013-finau', 1, 12, 3, 0, 0),
('hs-157', 't-001-masters', 'g-013-finau', 1, 13, 5, 0, 0),
('hs-158', 't-001-masters', 'g-013-finau', 1, 14, 4, 0, 0),
('hs-159', 't-001-masters', 'g-013-finau', 1, 15, 4, -1, 1),
('hs-160', 't-001-masters', 'g-013-finau', 1, 16, 3, 0, 0),
('hs-161', 't-001-masters', 'g-013-finau', 1, 17, 4, 0, 0),
('hs-162', 't-001-masters', 'g-013-finau', 1, 18, 4, 0, 0);

-- Matsuyama Day 1: Hot round (-6). 1 eagle, 4 birdies, 13 pars = 6 pts
INSERT OR IGNORE INTO hole_scores (id, tournament_id, golfer_id, day, hole, strokes, score_to_par, fantasy_points) VALUES
('hs-163', 't-001-masters', 'g-015-matsuyama', 1, 1, 3, -1, 1),
('hs-164', 't-001-masters', 'g-015-matsuyama', 1, 2, 3, -2, 2),
('hs-165', 't-001-masters', 'g-015-matsuyama', 1, 3, 4, 0, 0),
('hs-166', 't-001-masters', 'g-015-matsuyama', 1, 4, 3, 0, 0),
('hs-167', 't-001-masters', 'g-015-matsuyama', 1, 5, 3, -1, 1),
('hs-168', 't-001-masters', 'g-015-matsuyama', 1, 6, 3, 0, 0),
('hs-169', 't-001-masters', 'g-015-matsuyama', 1, 7, 4, 0, 0),
('hs-170', 't-001-masters', 'g-015-matsuyama', 1, 8, 5, 0, 0),
('hs-171', 't-001-masters', 'g-015-matsuyama', 1, 9, 4, 0, 0),
('hs-172', 't-001-masters', 'g-015-matsuyama', 1, 10, 4, 0, 0),
('hs-173', 't-001-masters', 'g-015-matsuyama', 1, 11, 4, 0, 0),
('hs-174', 't-001-masters', 'g-015-matsuyama', 1, 12, 3, 0, 0),
('hs-175', 't-001-masters', 'g-015-matsuyama', 1, 13, 4, -1, 1),
('hs-176', 't-001-masters', 'g-015-matsuyama', 1, 14, 4, 0, 0),
('hs-177', 't-001-masters', 'g-015-matsuyama', 1, 15, 4, -1, 1),
('hs-178', 't-001-masters', 'g-015-matsuyama', 1, 16, 3, 0, 0),
('hs-179', 't-001-masters', 'g-015-matsuyama', 1, 17, 4, 0, 0),
('hs-180', 't-001-masters', 'g-015-matsuyama', 1, 18, 4, 0, 0);

-- Henley Day 1: Solid (-2). 2 birdies, 16 pars = 2 pts
INSERT OR IGNORE INTO hole_scores (id, tournament_id, golfer_id, day, hole, strokes, score_to_par, fantasy_points) VALUES
('hs-181', 't-001-masters', 'g-017-henley', 1, 1, 4, 0, 0),
('hs-182', 't-001-masters', 'g-017-henley', 1, 2, 5, 0, 0),
('hs-183', 't-001-masters', 'g-017-henley', 1, 3, 4, 0, 0),
('hs-184', 't-001-masters', 'g-017-henley', 1, 4, 3, 0, 0),
('hs-185', 't-001-masters', 'g-017-henley', 1, 5, 4, 0, 0),
('hs-186', 't-001-masters', 'g-017-henley', 1, 6, 3, 0, 0),
('hs-187', 't-001-masters', 'g-017-henley', 1, 7, 3, -1, 1),
('hs-188', 't-001-masters', 'g-017-henley', 1, 8, 5, 0, 0),
('hs-189', 't-001-masters', 'g-017-henley', 1, 9, 4, 0, 0),
('hs-190', 't-001-masters', 'g-017-henley', 1, 10, 4, 0, 0),
('hs-191', 't-001-masters', 'g-017-henley', 1, 11, 4, 0, 0),
('hs-192', 't-001-masters', 'g-017-henley', 1, 12, 3, 0, 0),
('hs-193', 't-001-masters', 'g-017-henley', 1, 13, 5, 0, 0),
('hs-194', 't-001-masters', 'g-017-henley', 1, 14, 3, -1, 1),
('hs-195', 't-001-masters', 'g-017-henley', 1, 15, 5, 0, 0),
('hs-196', 't-001-masters', 'g-017-henley', 1, 16, 3, 0, 0),
('hs-197', 't-001-masters', 'g-017-henley', 1, 17, 4, 0, 0),
('hs-198', 't-001-masters', 'g-017-henley', 1, 18, 4, 0, 0);

-- Kim Day 1: Tough round (+3). 1 birdie, 4 bogeys, 13 pars = -3 pts
INSERT OR IGNORE INTO hole_scores (id, tournament_id, golfer_id, day, hole, strokes, score_to_par, fantasy_points) VALUES
('hs-199', 't-001-masters', 'g-021-kim', 1, 1, 5, 1, -1),
('hs-200', 't-001-masters', 'g-021-kim', 1, 2, 5, 0, 0),
('hs-201', 't-001-masters', 'g-021-kim', 1, 3, 5, 1, -1),
('hs-202', 't-001-masters', 'g-021-kim', 1, 4, 3, 0, 0),
('hs-203', 't-001-masters', 'g-021-kim', 1, 5, 4, 0, 0),
('hs-204', 't-001-masters', 'g-021-kim', 1, 6, 3, 0, 0),
('hs-205', 't-001-masters', 'g-021-kim', 1, 7, 4, 0, 0),
('hs-206', 't-001-masters', 'g-021-kim', 1, 8, 5, 0, 0),
('hs-207', 't-001-masters', 'g-021-kim', 1, 9, 4, 0, 0),
('hs-208', 't-001-masters', 'g-021-kim', 1, 10, 4, 0, 0),
('hs-209', 't-001-masters', 'g-021-kim', 1, 11, 5, 1, -1),
('hs-210', 't-001-masters', 'g-021-kim', 1, 12, 3, 0, 0),
('hs-211', 't-001-masters', 'g-021-kim', 1, 13, 4, -1, 1),
('hs-212', 't-001-masters', 'g-021-kim', 1, 14, 4, 0, 0),
('hs-213', 't-001-masters', 'g-021-kim', 1, 15, 5, 0, 0),
('hs-214', 't-001-masters', 'g-021-kim', 1, 16, 4, 1, -1),
('hs-215', 't-001-masters', 'g-021-kim', 1, 17, 4, 0, 0),
('hs-216', 't-001-masters', 'g-021-kim', 1, 18, 4, 0, 0);

-- Aberg Day 1: Good round (-3). 3 birdies, 15 pars = 3 pts
INSERT OR IGNORE INTO hole_scores (id, tournament_id, golfer_id, day, hole, strokes, score_to_par, fantasy_points) VALUES
('hs-217', 't-001-masters', 'g-022-aberg', 1, 1, 3, -1, 1),
('hs-218', 't-001-masters', 'g-022-aberg', 1, 2, 5, 0, 0),
('hs-219', 't-001-masters', 'g-022-aberg', 1, 3, 4, 0, 0),
('hs-220', 't-001-masters', 'g-022-aberg', 1, 4, 3, 0, 0),
('hs-221', 't-001-masters', 'g-022-aberg', 1, 5, 4, 0, 0),
('hs-222', 't-001-masters', 'g-022-aberg', 1, 6, 3, 0, 0),
('hs-223', 't-001-masters', 'g-022-aberg', 1, 7, 4, 0, 0),
('hs-224', 't-001-masters', 'g-022-aberg', 1, 8, 4, -1, 1),
('hs-225', 't-001-masters', 'g-022-aberg', 1, 9, 4, 0, 0),
('hs-226', 't-001-masters', 'g-022-aberg', 1, 10, 4, 0, 0),
('hs-227', 't-001-masters', 'g-022-aberg', 1, 11, 4, 0, 0),
('hs-228', 't-001-masters', 'g-022-aberg', 1, 12, 3, 0, 0),
('hs-229', 't-001-masters', 'g-022-aberg', 1, 13, 5, 0, 0),
('hs-230', 't-001-masters', 'g-022-aberg', 1, 14, 4, 0, 0),
('hs-231', 't-001-masters', 'g-022-aberg', 1, 15, 4, -1, 1),
('hs-232', 't-001-masters', 'g-022-aberg', 1, 16, 3, 0, 0),
('hs-233', 't-001-masters', 'g-022-aberg', 1, 17, 4, 0, 0),
('hs-234', 't-001-masters', 'g-022-aberg', 1, 18, 4, 0, 0);

-- Spieth Day 1: Mediocre (+1). 2 birdies, 3 bogeys, 13 pars = -1 pt
INSERT OR IGNORE INTO hole_scores (id, tournament_id, golfer_id, day, hole, strokes, score_to_par, fantasy_points) VALUES
('hs-235', 't-001-masters', 'g-012-spieth', 1, 1, 4, 0, 0),
('hs-236', 't-001-masters', 'g-012-spieth', 1, 2, 5, 0, 0),
('hs-237', 't-001-masters', 'g-012-spieth', 1, 3, 5, 1, -1),
('hs-238', 't-001-masters', 'g-012-spieth', 1, 4, 3, 0, 0),
('hs-239', 't-001-masters', 'g-012-spieth', 1, 5, 3, -1, 1),
('hs-240', 't-001-masters', 'g-012-spieth', 1, 6, 4, 1, -1),
('hs-241', 't-001-masters', 'g-012-spieth', 1, 7, 4, 0, 0),
('hs-242', 't-001-masters', 'g-012-spieth', 1, 8, 5, 0, 0),
('hs-243', 't-001-masters', 'g-012-spieth', 1, 9, 4, 0, 0),
('hs-244', 't-001-masters', 'g-012-spieth', 1, 10, 4, 0, 0),
('hs-245', 't-001-masters', 'g-012-spieth', 1, 11, 4, 0, 0),
('hs-246', 't-001-masters', 'g-012-spieth', 1, 12, 3, 0, 0),
('hs-247', 't-001-masters', 'g-012-spieth', 1, 13, 4, -1, 1),
('hs-248', 't-001-masters', 'g-012-spieth', 1, 14, 5, 1, -1),
('hs-249', 't-001-masters', 'g-012-spieth', 1, 15, 5, 0, 0),
('hs-250', 't-001-masters', 'g-012-spieth', 1, 16, 3, 0, 0),
('hs-251', 't-001-masters', 'g-012-spieth', 1, 17, 4, 0, 0),
('hs-252', 't-001-masters', 'g-012-spieth', 1, 18, 4, 0, 0);

-- Hatton Day 1: Frustrating (+2). 1 birdie, 3 bogeys, 14 pars = -2 pts
INSERT OR IGNORE INTO hole_scores (id, tournament_id, golfer_id, day, hole, strokes, score_to_par, fantasy_points) VALUES
('hs-253', 't-001-masters', 'g-016-hatton', 1, 1, 4, 0, 0),
('hs-254', 't-001-masters', 'g-016-hatton', 1, 2, 6, 1, -1),
('hs-255', 't-001-masters', 'g-016-hatton', 1, 3, 4, 0, 0),
('hs-256', 't-001-masters', 'g-016-hatton', 1, 4, 3, 0, 0),
('hs-257', 't-001-masters', 'g-016-hatton', 1, 5, 4, 0, 0),
('hs-258', 't-001-masters', 'g-016-hatton', 1, 6, 3, 0, 0),
('hs-259', 't-001-masters', 'g-016-hatton', 1, 7, 4, 0, 0),
('hs-260', 't-001-masters', 'g-016-hatton', 1, 8, 5, 0, 0),
('hs-261', 't-001-masters', 'g-016-hatton', 1, 9, 5, 1, -1),
('hs-262', 't-001-masters', 'g-016-hatton', 1, 10, 3, -1, 1),
('hs-263', 't-001-masters', 'g-016-hatton', 1, 11, 4, 0, 0),
('hs-264', 't-001-masters', 'g-016-hatton', 1, 12, 3, 0, 0),
('hs-265', 't-001-masters', 'g-016-hatton', 1, 13, 5, 0, 0),
('hs-266', 't-001-masters', 'g-016-hatton', 1, 14, 4, 0, 0),
('hs-267', 't-001-masters', 'g-016-hatton', 1, 15, 5, 0, 0),
('hs-268', 't-001-masters', 'g-016-hatton', 1, 16, 4, 1, -1),
('hs-269', 't-001-masters', 'g-016-hatton', 1, 17, 4, 0, 0),
('hs-270', 't-001-masters', 'g-016-hatton', 1, 18, 4, 0, 0);

-- Straka Day 1: Quiet (-1). 1 birdie, 17 pars = 1 pt
INSERT OR IGNORE INTO hole_scores (id, tournament_id, golfer_id, day, hole, strokes, score_to_par, fantasy_points) VALUES
('hs-271', 't-001-masters', 'g-020-straka', 1, 1, 4, 0, 0),
('hs-272', 't-001-masters', 'g-020-straka', 1, 2, 5, 0, 0),
('hs-273', 't-001-masters', 'g-020-straka', 1, 3, 4, 0, 0),
('hs-274', 't-001-masters', 'g-020-straka', 1, 4, 3, 0, 0),
('hs-275', 't-001-masters', 'g-020-straka', 1, 5, 4, 0, 0),
('hs-276', 't-001-masters', 'g-020-straka', 1, 6, 3, 0, 0),
('hs-277', 't-001-masters', 'g-020-straka', 1, 7, 4, 0, 0),
('hs-278', 't-001-masters', 'g-020-straka', 1, 8, 5, 0, 0),
('hs-279', 't-001-masters', 'g-020-straka', 1, 9, 4, 0, 0),
('hs-280', 't-001-masters', 'g-020-straka', 1, 10, 3, -1, 1),
('hs-281', 't-001-masters', 'g-020-straka', 1, 11, 4, 0, 0),
('hs-282', 't-001-masters', 'g-020-straka', 1, 12, 3, 0, 0),
('hs-283', 't-001-masters', 'g-020-straka', 1, 13, 5, 0, 0),
('hs-284', 't-001-masters', 'g-020-straka', 1, 14, 4, 0, 0),
('hs-285', 't-001-masters', 'g-020-straka', 1, 15, 5, 0, 0),
('hs-286', 't-001-masters', 'g-020-straka', 1, 16, 3, 0, 0),
('hs-287', 't-001-masters', 'g-020-straka', 1, 17, 4, 0, 0),
('hs-288', 't-001-masters', 'g-020-straka', 1, 18, 4, 0, 0);

-- Bhatia Day 1: Disaster (+5). 0 birdies, 5 bogeys, 13 pars = -5 pts
INSERT OR IGNORE INTO hole_scores (id, tournament_id, golfer_id, day, hole, strokes, score_to_par, fantasy_points) VALUES
('hs-289', 't-001-masters', 'g-024-bhatia', 1, 1, 5, 1, -1),
('hs-290', 't-001-masters', 'g-024-bhatia', 1, 2, 6, 1, -1),
('hs-291', 't-001-masters', 'g-024-bhatia', 1, 3, 4, 0, 0),
('hs-292', 't-001-masters', 'g-024-bhatia', 1, 4, 3, 0, 0),
('hs-293', 't-001-masters', 'g-024-bhatia', 1, 5, 5, 1, -1),
('hs-294', 't-001-masters', 'g-024-bhatia', 1, 6, 3, 0, 0),
('hs-295', 't-001-masters', 'g-024-bhatia', 1, 7, 4, 0, 0),
('hs-296', 't-001-masters', 'g-024-bhatia', 1, 8, 5, 0, 0),
('hs-297', 't-001-masters', 'g-024-bhatia', 1, 9, 4, 0, 0),
('hs-298', 't-001-masters', 'g-024-bhatia', 1, 10, 4, 0, 0),
('hs-299', 't-001-masters', 'g-024-bhatia', 1, 11, 4, 0, 0),
('hs-300', 't-001-masters', 'g-024-bhatia', 1, 12, 3, 0, 0),
('hs-301', 't-001-masters', 'g-024-bhatia', 1, 13, 5, 0, 0),
('hs-302', 't-001-masters', 'g-024-bhatia', 1, 14, 5, 1, -1),
('hs-303', 't-001-masters', 'g-024-bhatia', 1, 15, 5, 0, 0),
('hs-304', 't-001-masters', 'g-024-bhatia', 1, 16, 4, 1, -1),
('hs-305', 't-001-masters', 'g-024-bhatia', 1, 17, 4, 0, 0),
('hs-306', 't-001-masters', 'g-024-bhatia', 1, 18, 4, 0, 0);

-- Dunlap Day 1: Birdie fest (-4). 5 birdies, 1 bogey, 12 pars = 4 pts
INSERT OR IGNORE INTO hole_scores (id, tournament_id, golfer_id, day, hole, strokes, score_to_par, fantasy_points) VALUES
('hs-307', 't-001-masters', 'g-023-dunlap', 1, 1, 3, -1, 1),
('hs-308', 't-001-masters', 'g-023-dunlap', 1, 2, 4, -1, 1),
('hs-309', 't-001-masters', 'g-023-dunlap', 1, 3, 4, 0, 0),
('hs-310', 't-001-masters', 'g-023-dunlap', 1, 4, 3, 0, 0),
('hs-311', 't-001-masters', 'g-023-dunlap', 1, 5, 4, 0, 0),
('hs-312', 't-001-masters', 'g-023-dunlap', 1, 6, 3, 0, 0),
('hs-313', 't-001-masters', 'g-023-dunlap', 1, 7, 3, -1, 1),
('hs-314', 't-001-masters', 'g-023-dunlap', 1, 8, 5, 0, 0),
('hs-315', 't-001-masters', 'g-023-dunlap', 1, 9, 4, 0, 0),
('hs-316', 't-001-masters', 'g-023-dunlap', 1, 10, 4, 0, 0),
('hs-317', 't-001-masters', 'g-023-dunlap', 1, 11, 5, 1, -1),
('hs-318', 't-001-masters', 'g-023-dunlap', 1, 12, 3, 0, 0),
('hs-319', 't-001-masters', 'g-023-dunlap', 1, 13, 4, -1, 1),
('hs-320', 't-001-masters', 'g-023-dunlap', 1, 14, 4, 0, 0),
('hs-321', 't-001-masters', 'g-023-dunlap', 1, 15, 4, -1, 1),
('hs-322', 't-001-masters', 'g-023-dunlap', 1, 16, 3, 0, 0),
('hs-323', 't-001-masters', 'g-023-dunlap', 1, 17, 4, 0, 0),
('hs-324', 't-001-masters', 'g-023-dunlap', 1, 18, 4, 0, 0);

-- Bradley Day 1: Steady (-2). 2 birdies, 16 pars = 2 pts
INSERT OR IGNORE INTO hole_scores (id, tournament_id, golfer_id, day, hole, strokes, score_to_par, fantasy_points) VALUES
('hs-325', 't-001-masters', 'g-018-bradley', 1, 1, 4, 0, 0),
('hs-326', 't-001-masters', 'g-018-bradley', 1, 2, 5, 0, 0),
('hs-327', 't-001-masters', 'g-018-bradley', 1, 3, 3, -1, 1),
('hs-328', 't-001-masters', 'g-018-bradley', 1, 4, 3, 0, 0),
('hs-329', 't-001-masters', 'g-018-bradley', 1, 5, 4, 0, 0),
('hs-330', 't-001-masters', 'g-018-bradley', 1, 6, 3, 0, 0),
('hs-331', 't-001-masters', 'g-018-bradley', 1, 7, 4, 0, 0),
('hs-332', 't-001-masters', 'g-018-bradley', 1, 8, 5, 0, 0),
('hs-333', 't-001-masters', 'g-018-bradley', 1, 9, 4, 0, 0),
('hs-334', 't-001-masters', 'g-018-bradley', 1, 10, 4, 0, 0),
('hs-335', 't-001-masters', 'g-018-bradley', 1, 11, 4, 0, 0),
('hs-336', 't-001-masters', 'g-018-bradley', 1, 12, 3, 0, 0),
('hs-337', 't-001-masters', 'g-018-bradley', 1, 13, 5, 0, 0),
('hs-338', 't-001-masters', 'g-018-bradley', 1, 14, 4, 0, 0),
('hs-339', 't-001-masters', 'g-018-bradley', 1, 15, 4, -1, 1),
('hs-340', 't-001-masters', 'g-018-bradley', 1, 16, 3, 0, 0),
('hs-341', 't-001-masters', 'g-018-bradley', 1, 17, 4, 0, 0),
('hs-342', 't-001-masters', 'g-018-bradley', 1, 18, 4, 0, 0);

-- Harman Day 1: Solid (-1). 2 birdies, 1 bogey, 15 pars = 1 pt
INSERT OR IGNORE INTO hole_scores (id, tournament_id, golfer_id, day, hole, strokes, score_to_par, fantasy_points) VALUES
('hs-343', 't-001-masters', 'g-019-harman', 1, 1, 4, 0, 0),
('hs-344', 't-001-masters', 'g-019-harman', 1, 2, 5, 0, 0),
('hs-345', 't-001-masters', 'g-019-harman', 1, 3, 4, 0, 0),
('hs-346', 't-001-masters', 'g-019-harman', 1, 4, 2, -1, 1),
('hs-347', 't-001-masters', 'g-019-harman', 1, 5, 4, 0, 0),
('hs-348', 't-001-masters', 'g-019-harman', 1, 6, 3, 0, 0),
('hs-349', 't-001-masters', 'g-019-harman', 1, 7, 4, 0, 0),
('hs-350', 't-001-masters', 'g-019-harman', 1, 8, 5, 0, 0),
('hs-351', 't-001-masters', 'g-019-harman', 1, 9, 4, 0, 0),
('hs-352', 't-001-masters', 'g-019-harman', 1, 10, 5, 1, -1),
('hs-353', 't-001-masters', 'g-019-harman', 1, 11, 4, 0, 0),
('hs-354', 't-001-masters', 'g-019-harman', 1, 12, 3, 0, 0),
('hs-355', 't-001-masters', 'g-019-harman', 1, 13, 4, -1, 1),
('hs-356', 't-001-masters', 'g-019-harman', 1, 14, 4, 0, 0),
('hs-357', 't-001-masters', 'g-019-harman', 1, 15, 5, 0, 0),
('hs-358', 't-001-masters', 'g-019-harman', 1, 16, 3, 0, 0),
('hs-359', 't-001-masters', 'g-019-harman', 1, 17, 4, 0, 0),
('hs-360', 't-001-masters', 'g-019-harman', 1, 18, 4, 0, 0);

-- Homa Day 1: Mixed bag (E). 3 birdies, 3 bogeys, 12 pars = 0 pts
INSERT OR IGNORE INTO hole_scores (id, tournament_id, golfer_id, day, hole, strokes, score_to_par, fantasy_points) VALUES
('hs-361', 't-001-masters', 'g-010-homa', 1, 1, 4, 0, 0),
('hs-362', 't-001-masters', 'g-010-homa', 1, 2, 4, -1, 1),
('hs-363', 't-001-masters', 'g-010-homa', 1, 3, 5, 1, -1),
('hs-364', 't-001-masters', 'g-010-homa', 1, 4, 3, 0, 0),
('hs-365', 't-001-masters', 'g-010-homa', 1, 5, 4, 0, 0),
('hs-366', 't-001-masters', 'g-010-homa', 1, 6, 2, -1, 1),
('hs-367', 't-001-masters', 'g-010-homa', 1, 7, 4, 0, 0),
('hs-368', 't-001-masters', 'g-010-homa', 1, 8, 6, 1, -1),
('hs-369', 't-001-masters', 'g-010-homa', 1, 9, 4, 0, 0),
('hs-370', 't-001-masters', 'g-010-homa', 1, 10, 4, 0, 0),
('hs-371', 't-001-masters', 'g-010-homa', 1, 11, 4, 0, 0),
('hs-372', 't-001-masters', 'g-010-homa', 1, 12, 3, 0, 0),
('hs-373', 't-001-masters', 'g-010-homa', 1, 13, 5, 0, 0),
('hs-374', 't-001-masters', 'g-010-homa', 1, 14, 4, 0, 0),
('hs-375', 't-001-masters', 'g-010-homa', 1, 15, 4, -1, 1),
('hs-376', 't-001-masters', 'g-010-homa', 1, 16, 4, 1, -1),
('hs-377', 't-001-masters', 'g-010-homa', 1, 17, 4, 0, 0),
('hs-378', 't-001-masters', 'g-010-homa', 1, 18, 4, 0, 0);

-- Young Day 1: Roller coaster (-1). 4 birdies, 3 bogeys, 11 pars = 1 pt
INSERT OR IGNORE INTO hole_scores (id, tournament_id, golfer_id, day, hole, strokes, score_to_par, fantasy_points) VALUES
('hs-379', 't-001-masters', 'g-014-young', 1, 1, 3, -1, 1),
('hs-380', 't-001-masters', 'g-014-young', 1, 2, 5, 0, 0),
('hs-381', 't-001-masters', 'g-014-young', 1, 3, 5, 1, -1),
('hs-382', 't-001-masters', 'g-014-young', 1, 4, 2, -1, 1),
('hs-383', 't-001-masters', 'g-014-young', 1, 5, 4, 0, 0),
('hs-384', 't-001-masters', 'g-014-young', 1, 6, 4, 1, -1),
('hs-385', 't-001-masters', 'g-014-young', 1, 7, 4, 0, 0),
('hs-386', 't-001-masters', 'g-014-young', 1, 8, 4, -1, 1),
('hs-387', 't-001-masters', 'g-014-young', 1, 9, 4, 0, 0),
('hs-388', 't-001-masters', 'g-014-young', 1, 10, 4, 0, 0),
('hs-389', 't-001-masters', 'g-014-young', 1, 11, 4, 0, 0),
('hs-390', 't-001-masters', 'g-014-young', 1, 12, 3, 0, 0),
('hs-391', 't-001-masters', 'g-014-young', 1, 13, 5, 0, 0),
('hs-392', 't-001-masters', 'g-014-young', 1, 14, 4, 0, 0),
('hs-393', 't-001-masters', 'g-014-young', 1, 15, 4, -1, 1),
('hs-394', 't-001-masters', 'g-014-young', 1, 16, 4, 1, -1),
('hs-395', 't-001-masters', 'g-014-young', 1, 17, 4, 0, 0),
('hs-396', 't-001-masters', 'g-014-young', 1, 18, 4, 0, 0);

-- Clark Day 1: Birdie machine (-3). 3 birdies, 15 pars = 3 pts
INSERT OR IGNORE INTO hole_scores (id, tournament_id, golfer_id, day, hole, strokes, score_to_par, fantasy_points) VALUES
('hs-397', 't-001-masters', 'g-007-clark', 1, 1, 4, 0, 0),
('hs-398', 't-001-masters', 'g-007-clark', 1, 2, 5, 0, 0),
('hs-399', 't-001-masters', 'g-007-clark', 1, 3, 3, -1, 1),
('hs-400', 't-001-masters', 'g-007-clark', 1, 4, 3, 0, 0),
('hs-401', 't-001-masters', 'g-007-clark', 1, 5, 4, 0, 0),
('hs-402', 't-001-masters', 'g-007-clark', 1, 6, 3, 0, 0),
('hs-403', 't-001-masters', 'g-007-clark', 1, 7, 4, 0, 0),
('hs-404', 't-001-masters', 'g-007-clark', 1, 8, 4, -1, 1),
('hs-405', 't-001-masters', 'g-007-clark', 1, 9, 4, 0, 0),
('hs-406', 't-001-masters', 'g-007-clark', 1, 10, 4, 0, 0),
('hs-407', 't-001-masters', 'g-007-clark', 1, 11, 4, 0, 0),
('hs-408', 't-001-masters', 'g-007-clark', 1, 12, 3, 0, 0),
('hs-409', 't-001-masters', 'g-007-clark', 1, 13, 5, 0, 0),
('hs-410', 't-001-masters', 'g-007-clark', 1, 14, 4, 0, 0),
('hs-411', 't-001-masters', 'g-007-clark', 1, 15, 5, 0, 0),
('hs-412', 't-001-masters', 'g-007-clark', 1, 16, 2, -1, 1),
('hs-413', 't-001-masters', 'g-007-clark', 1, 17, 4, 0, 0),
('hs-414', 't-001-masters', 'g-007-clark', 1, 18, 4, 0, 0);

-- Morikawa Day 1: Consistent (-2). 2 birdies, 16 pars = 2 pts
INSERT OR IGNORE INTO hole_scores (id, tournament_id, golfer_id, day, hole, strokes, score_to_par, fantasy_points) VALUES
('hs-415', 't-001-masters', 'g-008-morikawa', 1, 1, 4, 0, 0),
('hs-416', 't-001-masters', 'g-008-morikawa', 1, 2, 5, 0, 0),
('hs-417', 't-001-masters', 'g-008-morikawa', 1, 3, 4, 0, 0),
('hs-418', 't-001-masters', 'g-008-morikawa', 1, 4, 2, -1, 1),
('hs-419', 't-001-masters', 'g-008-morikawa', 1, 5, 4, 0, 0),
('hs-420', 't-001-masters', 'g-008-morikawa', 1, 6, 3, 0, 0),
('hs-421', 't-001-masters', 'g-008-morikawa', 1, 7, 4, 0, 0),
('hs-422', 't-001-masters', 'g-008-morikawa', 1, 8, 5, 0, 0),
('hs-423', 't-001-masters', 'g-008-morikawa', 1, 9, 4, 0, 0),
('hs-424', 't-001-masters', 'g-008-morikawa', 1, 10, 4, 0, 0),
('hs-425', 't-001-masters', 'g-008-morikawa', 1, 11, 4, 0, 0),
('hs-426', 't-001-masters', 'g-008-morikawa', 1, 12, 3, 0, 0),
('hs-427', 't-001-masters', 'g-008-morikawa', 1, 13, 5, 0, 0),
('hs-428', 't-001-masters', 'g-008-morikawa', 1, 14, 4, 0, 0),
('hs-429', 't-001-masters', 'g-008-morikawa', 1, 15, 4, -1, 1),
('hs-430', 't-001-masters', 'g-008-morikawa', 1, 16, 3, 0, 0),
('hs-431', 't-001-masters', 'g-008-morikawa', 1, 17, 4, 0, 0),
('hs-432', 't-001-masters', 'g-008-morikawa', 1, 18, 4, 0, 0);

-- =============================================
-- EXPECTED LEADERBOARD SUMMARY (Masters Day 1)
-- =============================================
-- Alice's team: Scheffler(6) + Schauffele(4) + Koepka(-2) + Finau(1) + Henley(2) + Kim(-3) = 8 pts
-- Bob's team: Rahm(3) + Cantlay(1) + Homa(0) + Young(1) + Bradley(2) + Aberg(3) = 10 pts
-- Charlie's team: McIlroy(2) + Clark(3) + Fleetwood(0) + Matsuyama(6) + Harman(1) + Dunlap(4) = 16 pts
-- Dana's team: Hovland(0) + Morikawa(2) + Spieth(-1) + Hatton(-2) + Straka(1) + Bhatia(-5) = -5 pts
