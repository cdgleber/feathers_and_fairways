/#!/bin/bash

sudo docker compose exec postgres psql -U postgres -d feathers_and_fairways -f /tmp/migrations/01_create_seasons.sql
sudo docker compose exec postgres psql -U postgres -d feathers_and_fairways -f /tmp/migrations/02_create_golfers.sql
sudo docker compose exec postgres psql -U postgres -d feathers_and_fairways -f /tmp/migrations/03_create_access_keys.sql
sudo docker compose exec postgres psql -U postgres -d feathers_and_fairways -f /tmp/migrations/04_create_teams.sql
sudo docker compose exec postgres psql -U postgres -d feathers_and_fairways -f /tmp/migrations/05_create_team_golfers.sql
sudo docker compose exec postgres psql -U postgres -d feathers_and_fairways -f /tmp/migrations/06_create_tournaments.sql
sudo docker compose exec postgres psql -U postgres -d feathers_and_fairways -f /tmp/migrations/07_create_hole_scores.sql
sudo docker-compose exec postgres psql -U postgres -d feathers_and_fairways -f /tmp/seed_data.sql
