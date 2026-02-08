# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Feathers & Fairways is a fantasy golf league management application. Commissioners manage seasons and tournaments, players create 6-golfer teams using access keys, and the system tracks scores and leaderboards. Fantasy points are calculated in Rust application code based on score-to-par.

## Tech Stack

- **Backend**: Rust (edition 2021), Axum 0.7, Tokio, SQLx 0.7 (runtime queries)
- **Database**: SQLite (WAL mode, foreign keys enabled)
- **Frontend**: Vanilla HTML/CSS/JS served as static files from `dist/`
- **Auth**: Basic/Bearer auth middleware for admin routes

## Build & Run Commands

```bash
cargo build                    # Debug build
cargo build --release          # Release build (LTO enabled)
cargo run                      # Run dev server (port 3000)
cargo test                     # Run tests
```

The app requires a `.env` file with `DATABASE_URL=sqlite:feathers_and_fairways.db`, `RUST_LOG`, `HOST`, `PORT`, and `ADMIN_PASSWORD`. The SQLite database is created automatically on first run.

## Architecture

### Backend (`src/`)

- **main.rs** — App entry point: SQLite pool creation with WAL/FK pragmas, migration runner, Axum router setup with CORS/tracing middleware, static file serving from `dist/`
- **routes.rs** — All API handlers. Contains business logic inline (validation, transactions, queries). Uses runtime `sqlx::query_as::<_, Model>()` queries with `?` placeholders and `.bind()` chains. UUIDs generated in Rust via `uuid::Uuid::new_v4().to_string()`. Fantasy points calculated in Rust via `calculate_fantasy_points()` helper.
- **models.rs** — Structs for DB entities (`#[derive(FromRow)]`) and request types (`#[derive(Deserialize, Validate)]`). All IDs are `String` (UUID text). Dates stored as `String` in `YYYY-MM-DD` format.
- **auth.rs** — Tower middleware for `/api/admin/*` routes. Supports both Basic auth and Bearer token against `ADMIN_PASSWORD` env var.
- **db.rs** — Utility for generating 12-char alphanumeric access keys.

### Database (`migrations/`)

Seven sequential SQL migrations. Key design decisions:
- UUIDs as TEXT primary keys (generated in Rust, not DB)
- Booleans stored as INTEGER (0/1)
- Dates stored as TEXT in ISO format
- Fantasy points: eagle+ = +2, birdie = +1, par = 0, bogey+ = -1 (calculated in Rust before INSERT)
- Team golfer limit (max 6) enforced in application code
- Only one active season and one active tournament per season at a time (enforced in route handlers)

### Frontend (`dist/`)

Single-page app with view switching via JS DOM manipulation. No build step — edit files directly. `app.js` handles API calls, form submission, leaderboard rendering. Material Design styling with dark mode support.

## API Routes

Public: `/api/seasons`, `/api/golfers`, `/api/teams`, `/api/tournaments/:season_id`, `/api/scores/tournament/:tournament_id`, `/api/leaderboard/:season_id`, `/api/leaderboard/tournament/:tournament_id`

Protected (admin auth middleware): `/api/admin/seasons`, `/api/admin/access-keys`, `/api/admin/golfers`, `/api/admin/tournaments`, `/api/admin/scores`, `/api/admin/login`

## Key Business Rules

- Golfers belong to groups 1-6 (skill tiers). Teams must select exactly one golfer per group.
- Access keys are single-use; player name is recorded when key is claimed.
- Teams cannot be updated after a tournament's start_date.
- Leaderboard sums fantasy_points across all tournaments in a season using LEFT JOINs.

## SQLx Notes

This project uses SQLx runtime query checking (not compile-time macros). No running database is needed at build time. The SQLite database file is created automatically when the server starts.
