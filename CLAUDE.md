# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Feathers & Fairways is a fantasy golf league management application. Commissioners manage seasons and tournaments, players create 6-golfer teams using access keys, and the system tracks scores and leaderboards. Fantasy points are calculated via database triggers based on score-to-par.

## Tech Stack

- **Backend**: Rust (edition 2021), Axum 0.7, Tokio, SQLx 0.7 (compile-time checked queries)
- **Database**: PostgreSQL 16
- **Frontend**: Vanilla HTML/CSS/JS served as static files from `dist/`
- **Auth**: Basic/Bearer auth middleware for admin routes
- **Containerization**: Docker multi-stage build, Docker Compose

## Build & Run Commands

```bash
cargo build                    # Debug build
cargo build --release          # Release build (LTO enabled)
cargo run                      # Run dev server (port 3000)
cargo test                     # Run tests
sqlx migrate run               # Run database migrations
cargo sqlx prepare             # Generate offline query metadata

docker-compose up -d           # Start PostgreSQL
docker-compose down -v         # Stop and remove volumes
```

The app requires a running PostgreSQL instance and a `.env` file with `DATABASE_URL`, `RUST_LOG`, `HOST`, `PORT`, and `ADMIN_PASSWORD`.

## Architecture

### Backend (`src/`)

- **main.rs** — App entry point: DB pool creation with retry logic, migration runner, Axum router setup with CORS/tracing middleware, static file serving from `dist/`
- **routes.rs** — All API handlers (~650 lines). Contains business logic inline (validation, transactions, queries). This is the largest file and where most feature work happens.
- **models.rs** — Structs for DB entities (`#[derive(FromRow)]`) and request types (`#[derive(Deserialize, Validate)]`). Separate structs for DB rows vs API requests.
- **auth.rs** — Tower middleware for `/api/admin/*` routes. Supports both Basic auth and Bearer token against `ADMIN_PASSWORD` env var.
- **db.rs** — Utility for generating 12-char alphanumeric access keys.

### Database (`migrations/`)

Seven sequential SQL migrations. Key design decisions:
- UUIDs as primary keys (`gen_random_uuid()`)
- Database triggers: `auto_calculate_fantasy_points` on hole_scores, `enforce_team_golfer_limit` (max 6 per team)
- Fantasy points: eagle+ = +2, birdie = +1, par = 0, bogey+ = -1
- Only one active season and one active tournament per season at a time (enforced in route handlers, not DB constraints)

### Frontend (`dist/`)

Single-page app with view switching via JS DOM manipulation. No build step — edit files directly. `app.js` handles API calls, form submission, leaderboard rendering. Material Design styling.

## API Routes

Public: `/api/seasons`, `/api/golfers`, `/api/teams`, `/api/tournaments/:season_id`, `/api/scores/tournament/:tournament_id`, `/api/leaderboard/:season_id`, `/api/leaderboard/tournament/:tournament_id`

Protected (admin auth middleware): `/api/admin/seasons`, `/api/admin/access-keys`, `/api/admin/golfers`, `/api/admin/tournaments`, `/api/admin/scores`, `/api/admin/login`

## Key Business Rules

- Golfers belong to groups 1-6 (skill tiers). Teams must select exactly one golfer per group.
- Access keys are single-use; player name is recorded when key is claimed.
- Teams cannot be updated after a tournament's start_date.
- Leaderboard sums fantasy_points across all tournaments in a season using LEFT JOINs.

## SQLx Notes

This project uses SQLx compile-time query checking. The database must be running and `DATABASE_URL` set for `cargo build` to succeed, unless offline mode is prepared with `cargo sqlx prepare`.
