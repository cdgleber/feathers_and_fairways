# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Feathers & Fairways is a fantasy golf league management application. Commissioners manage seasons and tournaments, players create 9-golfer teams using access keys, and the system tracks scores and leaderboards. Fantasy points are calculated in Rust application code based on score-to-par. Amateur golfers cannot receive negative fantasy points (bogeys or worse score 0 instead of -1).

## Tech Stack

- **Backend**: Rust (edition 2021), Axum 0.7, Tokio, SQLx 0.7 (runtime queries), reqwest 0.12 (ESPN API)
- **Database**: SQLite (WAL mode, foreign keys enabled)
- **Frontend**: Vanilla HTML/CSS/JS served as static files from `dist/`
- **Auth**: JWT (jsonwebtoken) + Basic auth middleware for admin routes

## Build & Run Commands

```bash
cargo build                    # Debug build
cargo build --release          # Release build (LTO enabled)
cargo run                      # Run dev server (port 41549)
cargo test                     # Run tests
```

The app requires a `.env` file with `DATABASE_URL=sqlite:feathers_and_fairways.db`, `RUST_LOG`, `HOST`, `PORT`, and `ADMIN_PASSWORD`. The SQLite database is created automatically on first run.

## Architecture

### Backend (`src/`)

- **main.rs** — App entry point: SQLite pool creation with WAL/FK pragmas, migration runner, Axum router setup with CORS/tracing middleware, static file serving from `dist/`
- **routes.rs** — All API handlers. Contains business logic inline (validation, transactions, queries). Uses runtime `sqlx::query_as::<_, Model>()` queries with `?` placeholders and `.bind()` chains. UUIDs generated in Rust via `uuid::Uuid::new_v4().to_string()`. Fantasy points calculated in Rust via `calculate_fantasy_points()` helper.
- **models.rs** — Structs for DB entities (`#[derive(FromRow)]`) and request types (`#[derive(Deserialize, Validate)]`). All IDs are `String` (UUID text). Dates stored as `String` in `YYYY-MM-DD` format.
- **auth.rs** — Tower middleware for `/api/admin/*` routes. Supports JWT Bearer tokens (HS256, 60-min expiry) and Basic auth against `ADMIN_PASSWORD` env var. Login endpoint returns a JWT.
- **db.rs** — Utility for generating 12-char alphanumeric access keys.

### Database (`migrations/`)

Ten sequential SQL migrations (`01` through `10`). Key design decisions:
- UUIDs as TEXT primary keys (generated in Rust, not DB)
- Booleans stored as INTEGER (0/1)
- Dates stored as TEXT in ISO format
- Fantasy points: eagle+ = +2, birdie = +1, par = 0, bogey+ = -1 (amateurs get 0 for bogey+) (calculated in Rust before INSERT)
- Team golfer limit (max 9, one per group) enforced in application code
- Golfers have an `is_amateur` flag (migration 10); amateurs cannot receive negative fantasy points
- Only one active season and one active tournament per season at a time (enforced in route handlers)
- Per-tournament golfer groups with override support (`tournament_golfer_groups` table, migration 08)
- Optional email field on teams (migration 09)

### Frontend (`dist/`)

Single-page app with view switching via JS DOM manipulation. No build step — edit files directly. JS files in `dist/js/`, CSS in `dist/css/`. Handles API calls, form submission, leaderboard rendering, admin panel, and tournament history tab. Material Design styling with dark mode support.

## API Routes

Public: `/api/seasons`, `/api/seasons/active`, `/api/golfers`, `/api/golfers/tournament/:tournament_id`, `/api/teams`, `/api/teams/update`, `/api/teams/:season_id`, `/api/teams/:team_id/golfers`, `/api/tournaments/:season_id`, `/api/tournaments/:season_id/completed`, `/api/tournaments/:tournament_id/stats`, `/api/scores/tournament/:tournament_id`, `/api/leaderboard/:season_id`, `/api/leaderboard/:season_id/detailed`, `/api/leaderboard/tournament/:tournament_id`, `/api/leaderboard/tournament/:tournament_id/teams`, `/api/access-keys/validate`

Login (unprotected): `/api/admin/login` — returns JWT token

Protected (admin auth middleware via `/api/admin` nest): `/api/admin/seasons`, `/api/admin/access-keys`, `/api/admin/golfers`, `/api/admin/golfers/upload`, `/api/admin/tournaments`, `/api/admin/tournaments/:tournament_id/scores/upload`, `/api/admin/tournaments/:tournament_id/groups/upload`, `/api/admin/tournaments/:tournament_id/teams`, `/api/admin/teams/:team_id/golfers` (PUT), `/api/admin/scores`, `/api/admin/stats`, `/api/admin/tournaments/import/preview`, `/api/admin/tournaments/import/espn-preview`, `/api/admin/tournaments/import/commit`

## Key Business Rules

- Golfers belong to groups 1-9 (skill tiers). Teams must select exactly one golfer per group.
- Access keys are single-use; player name is recorded when key is claimed.
- Teams cannot be updated after a tournament's start_date.
- Leaderboard sums fantasy_points across all tournaments in a season using LEFT JOINs.
- Admin login returns a JWT (60-min expiry); `ADMIN_PASSWORD` env var is required (no default fallback).
- Bulk upload endpoints support JSON arrays for golfers, scores, and tournament golfer groups.
- ESPN tournament import: backend fetches data from ESPN's core API (`sports.core.api.espn.com`), transforms it into the standard import format, and feeds it through the existing preview/commit flow. Uses `reqwest` with `tokio::sync::Semaphore(10)` for concurrency-limited parallel fetching.
- ESPN field import (`fetch_espn_field`): auto-assigns golfers to groups 1-9 using equal-count binning (quantiles) based on ESPN's `order` field from competitor page items. Golfers are sorted by order ascending (best first), then divided evenly into 9 groups. The `order` field reflects ESPN's field ordering (typically OWGR-based pre-tournament). Falls back to `sortOrder` on individual competitor objects if present, then original page position.

## Additional Files

- **seed_data.sql** — SQL seed data for development/testing
- **example_jsons/** — Example JSON request bodies for all admin API endpoints

## SQLx Notes

This project uses SQLx runtime query checking (not compile-time macros). No running database is needed at build time. The SQLite database file is created automatically when the server starts.
