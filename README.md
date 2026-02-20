# Feathers & Fairways

A fantasy golf league management application. Commissioners manage seasons and tournaments, players create teams using access keys, and the system tracks scores and leaderboards.

## Features

### For Players

- **Team Creation**: Join the league with a unique access key provided by the commissioner
- **Strategic Selection**: Choose 9 golfers — one from each skill tier (groups 1–9 based on win probability)
- **Team Updates**: Modify your team roster before a tournament's start date
- **Real-time Leaderboards**: Track season standings, tournament-specific scores, and detailed breakdowns
- **Tournament History**: View completed tournaments with team leaderboards and stats
- **Responsive Design**: Material Design interface with dark mode support

### For Commissioners

- **Season Management**: Create and manage golf seasons (one active at a time)
- **Access Key Generation**: Generate unique single-use keys for players to join
- **Golfer Database**: Add golfers grouped by win probability (groups 1–9), with amateur flag support
- **Tournament Creation**: Set up tournaments throughout the season
- **Score Entry**: Record hole-by-hole results via manual entry or bulk JSON upload
- **Tournament Import**: Preview and import tournament data from a JSON file or directly from ESPN by tournament ID
- **Per-Tournament Groups**: Override default golfer groups on a per-tournament basis
- **Team Editor**: View and edit team golfer selections from the admin panel
- **Database Stats**: View database statistics from the admin panel
- **Tabbed Admin Panel**: Organized interface with tabs for scores, teams, golfer management, and tournament import

### Scoring System

- **Eagle or better**: +2 points
- **Birdie**: +1 point
- **Par**: 0 points
- **Bogey or worse**: -1 point (amateurs score 0 instead — they cannot receive negative fantasy points)

## Tech Stack

### Backend

- **Rust** (edition 2021) with **Axum** 0.7 web framework
- **reqwest** 0.12 for ESPN API integration
- **SQLx** 0.7 for runtime-checked database queries (not compile-time macros)
- **SQLite** with WAL mode and foreign keys enabled
- **Tower** middleware for auth and tracing
- **JWT** (jsonwebtoken, HS256, 60-min expiry) + Basic auth for admin routes

### Frontend

- **Vanilla JavaScript** (ES6+) — single-page app with view switching via DOM manipulation
- **Material Design** styling with **Google Fonts** (Inter) and Material Icons
- **Dark mode** support
- **Responsive CSS** Grid/Flexbox
- No build step — edit files in `dist/` directly

## Project Structure

```
feathers_and_fairways/
├── src/
│   ├── main.rs           # Entry point, SQLite pool, migrations, Axum router
│   ├── routes.rs         # All API handlers with inline business logic
│   ├── models.rs         # DB entities (FromRow) and request types (Deserialize, Validate)
│   ├── auth.rs           # JWT/Basic auth middleware for admin routes
│   └── db.rs             # Access key generation utility
├── migrations/           # 10 sequential SQLite migrations (01–10)
├── dist/                 # Frontend static files
│   ├── index.html
│   ├── css/styles.css
│   ├── js/app.js
│   └── favicons/
├── example_jsons/        # Example JSON request bodies for admin API endpoints
├── seed_data.sql         # SQL seed data for development/testing
├── Cargo.toml
├── Dockerfile
├── docker-compose.yml
└── .env                  # Environment variables (not committed)
```

## Prerequisites

- **Rust** 1.75+ (with Cargo)

## Setup

1. **Clone the repository**

   ```bash
   git clone <repo-url>
   cd feathers_and_fairways
   ```

2. **Configure environment variables**

   Create a `.env` file:

   ```bash
   DATABASE_URL=sqlite:feathers_and_fairways.db
   RUST_LOG=debug
   HOST=0.0.0.0
   PORT=41549
   ADMIN_PASSWORD=your_secure_password_here
   ```

   `ADMIN_PASSWORD` is required — there is no default fallback.

3. **Build and run**

   ```bash
   # Development
   cargo run

   # Production (optimized with LTO)
   cargo build --release
   ./target/release/feathers_and_fairways
   ```

   The SQLite database is created automatically on first run. Migrations run automatically at startup.

4. **Access the application**
   - Open your browser to `http://localhost:41549`

## API Endpoints

### Public

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/seasons` | List all seasons |
| GET | `/api/seasons/active` | Get the active season |
| GET | `/api/golfers` | List all active golfers |
| GET | `/api/golfers/tournament/:tournament_id` | List golfers for a tournament (with group overrides) |
| POST | `/api/teams` | Create a team |
| POST | `/api/teams/update` | Update team golfers |
| GET | `/api/teams/:season_id` | List teams in a season |
| GET | `/api/teams/:team_id/golfers` | Get team's golfers |
| GET | `/api/tournaments/:season_id` | List tournaments in a season |
| GET | `/api/tournaments/:season_id/completed` | List completed tournaments |
| GET | `/api/tournaments/:tournament_id/stats` | Get tournament statistics |
| GET | `/api/scores/tournament/:tournament_id` | Get tournament scores |
| GET | `/api/leaderboard/:season_id` | Season leaderboard |
| GET | `/api/leaderboard/:season_id/detailed` | Season leaderboard with golfer details |
| GET | `/api/leaderboard/tournament/:tournament_id` | Tournament leaderboard |
| GET | `/api/leaderboard/tournament/:tournament_id/teams` | Tournament team leaderboard |
| POST | `/api/access-keys/validate` | Validate an access key |

### Admin (JWT or Basic auth required)

| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | `/api/admin/login` | Login (returns JWT token) |
| POST | `/api/admin/seasons` | Create a season |
| POST | `/api/admin/access-keys` | Generate access keys |
| POST | `/api/admin/golfers` | Add a golfer |
| POST | `/api/admin/golfers/upload` | Bulk upload golfers (JSON array) |
| POST | `/api/admin/tournaments` | Create a tournament |
| POST | `/api/admin/scores` | Add hole scores |
| POST | `/api/admin/tournaments/:id/scores/upload` | Bulk upload tournament scores |
| POST | `/api/admin/tournaments/:id/groups/upload` | Upload per-tournament golfer groups |
| GET | `/api/admin/tournaments/:id/teams` | List teams for a tournament |
| PUT | `/api/admin/teams/:team_id/golfers` | Edit team golfer selections |
| GET | `/api/admin/stats` | Database statistics |
| POST | `/api/admin/tournaments/import/preview` | Preview tournament import (JSON file) |
| POST | `/api/admin/tournaments/import/espn-preview` | Preview tournament import (ESPN API) |
| POST | `/api/admin/tournaments/import/commit` | Commit tournament import |

## Usage Guide

### For Commissioners

1. **Initial Setup** — Navigate to the Admin panel, log in with your admin password, create a season, and add golfers (organized in 9 groups by win probability)
2. **Invite Players** — Generate access keys and distribute them to players. Each key is single-use.
3. **Manage Tournaments** — Create tournaments, enter scores (manually or via bulk upload), and optionally override golfer groups per tournament
4. **Import Tournaments** — Use the tournament import tab to preview and import tournament data in one step

### For Players

1. **Join the League** — Click "Join League", enter your access key and name
2. **Build Your Team** — Select one golfer from each of the 9 skill groups
3. **Track Progress** — View the season leaderboard, tournament scores, and completed tournament history

## Database Schema

### Tables

- **seasons** — Golf seasons with date ranges and active flag
- **golfers** — Golfers with group assignment (1–9) and amateur flag
- **access_keys** — Single-use keys for player registration
- **tournaments** — Tournaments within a season with start/end dates
- **teams** — Player teams linked to a season (optional email)
- **team_golfers** — Junction table (team ↔ golfers)
- **hole_scores** — Per-hole scoring data with fantasy points
- **tournament_golfer_groups** — Per-tournament golfer group overrides

### Key Design Decisions

- UUIDs as TEXT primary keys (generated in Rust, not DB)
- Fantasy points calculated in Rust application code (not DB triggers)
- Booleans stored as INTEGER (0/1)
- Dates stored as TEXT in ISO format
- Team golfer limit (max 9, one per group) enforced in application code

## Security

- Admin routes protected by JWT (60-min expiry) or Basic auth
- `ADMIN_PASSWORD` env var required — no default fallback
- Access keys provide player authorization
- HTTPS recommended for production deployment
- `.env` file is gitignored

## Troubleshooting

### Database Issues

The SQLite database file is created automatically. If you need to reset:

```bash
# Delete the database and restart — migrations will recreate it
rm feathers_and_fairways.db
cargo run
```

### Running Tests

```bash
cargo test
```
