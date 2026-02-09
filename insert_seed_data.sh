#!/usr/bin/env bash
set -euo pipefail

DB="feathers_and_fairways.db"
SEED_FILE="seed_data.sql"
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"

cd "$SCRIPT_DIR"

if [ ! -f "$SEED_FILE" ]; then
    echo "Error: $SEED_FILE not found in $SCRIPT_DIR"
    exit 1
fi

if ! command -v sqlite3 &>/dev/null; then
    echo "Error: sqlite3 is not installed"
    exit 1
fi

# If the DB doesn't exist, run the app briefly to create it with migrations,
# or apply migrations manually
if [ ! -f "$DB" ]; then
    echo "Database $DB not found. Creating and applying migrations..."
    # Create the DB and run migrations via sqlite3
    for migration in migrations/*.sql; do
        echo "  Applying $migration..."
        sqlite3 "$DB" < "$migration"
    done
    # Enable WAL mode and foreign keys
    sqlite3 "$DB" "PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;"
    echo "Migrations applied."
fi

echo "Inserting seed data into $DB..."
sqlite3 "$DB" < "$SEED_FILE"

echo ""
echo "Seed data inserted. Summary:"
echo "----------------------------"
sqlite3 -header -column "$DB" "
SELECT 'golfers' AS 'table', COUNT(*) AS count FROM golfers
UNION ALL SELECT 'seasons', COUNT(*) FROM seasons
UNION ALL SELECT 'tournaments', COUNT(*) FROM tournaments
UNION ALL SELECT 'access_keys', COUNT(*) FROM access_keys
UNION ALL SELECT 'teams', COUNT(*) FROM teams
UNION ALL SELECT 'team_golfers', COUNT(*) FROM team_golfers
UNION ALL SELECT 'hole_scores', COUNT(*) FROM hole_scores;
"

echo ""
echo "Leaderboard (Masters - The completed tournament):"
echo "--------------------------------------------------"
sqlite3 -header -column "$DB" "
SELECT t.player_name, SUM(hs.fantasy_points) AS total_points
FROM teams t
JOIN team_golfers tg ON tg.team_id = t.id
JOIN hole_scores hs ON hs.golfer_id = tg.golfer_id AND hs.tournament_id = t.tournament_id
WHERE t.tournament_id = 't-001-masters'
GROUP BY t.id
ORDER BY total_points DESC;
"

echo ""
echo "Unclaimed access keys: $(sqlite3 "$DB" "SELECT GROUP_CONCAT(key_code, ', ') FROM access_keys WHERE is_used = 0;")"
echo ""
echo "Done! Start the server with 'cargo run' to test."
