# Feathers & Fairways

## Features

### For Players
- **Team Creation**: Join the league with a unique access key provided by the commissioner
- **Strategic Selection**: Choose 6 golfers - one from each skill tier (based on win probability)
- **Real-time Leaderboards**: Track season standings and tournament-specific scores
- **Responsive Design**: Beautiful Material Design interface that works on all devices

### For Commissioners
- **Season Management**: Create and manage golf seasons
- **Access Key Generation**: Generate unique keys for players to join
- **Golfer Database**: Add golfers grouped by win probability (1-6)
- **Tournament Creation**: Set up tournaments throughout the season
- **Score Entry**: Record hole-by-hole results for accurate fantasy points

### Scoring System
- **Eagle or better**: +2 points
- **Birdie**: +1 point
- **Par**: 0 points
- **Bogey or worse**: -1 point

## Tech Stack

### Backend
- **Rust** with Axum web framework
- **SQLx** for compile-time checked database queries
- **PostgreSQL** 16 for data persistence
- **Tower** for middleware and services

### Frontend
- **Vanilla JavaScript** (ES6+)
- **Material Design** principles
- **Google Fonts** (Inter) and Material Icons
- **Responsive CSS Grid/Flexbox**

## Project Structure

```
fantasy-golf/
├── src/
│   ├── main.rs           # Application entry point and server setup
│   ├── models.rs         # Data structures and types
│   ├── routes.rs         # API endpoint handlers
│   └── db.rs             # Database utilities
├── migrations/           # SQLx database migrations
│   ├── 20240101000001_create_seasons.sql
│   ├── 20240101000002_create_golfers.sql
│   ├── 20240101000003_create_access_keys.sql
│   ├── 20240101000004_create_teams.sql
│   ├── 20240101000005_create_team_golfers.sql
│   ├── 20240101000006_create_tournaments.sql
│   └── 20240101000007_create_hole_scores.sql
├── dist/                 # Frontend static files
│   ├── index.html
│   ├── css/
│   │   └── styles.css
│   └── js/
│       └── app.js
├── Cargo.toml           # Rust dependencies
├── Dockerfile           # Docker build configuration
├── docker-compose.yml   # Docker Compose orchestration
└── .env                 # Environment variables (create from .env.example)
```

## Prerequisites

- **Rust** 1.75+ (with Cargo)
- **PostgreSQL** 16+
- **Docker** and **Docker Compose** (for containerized deployment)
- **SQLx CLI** (for migrations): `cargo install sqlx-cli --no-default-features --features postgres`

## Setup Instructions

### Option 1: Local Development

1. **Clone the repository**
   ```bash
   cd fantasy-golf
   ```

2. **Set up PostgreSQL**
   ```bash
   # Start PostgreSQL on localhost:5432
   # Create database
   createdb fantasy_golf
   ```

3. **Configure environment variables**
   ```bash
   # .env file is already created with defaults
   # Modify if needed:
   DATABASE_URL=postgres://postgres:postgres@localhost:5432/fantasy_golf
   RUST_LOG=debug
   HOST=0.0.0.0
   PORT=3000
   ```

4. **Run database migrations**
   ```bash
   sqlx migrate run
   ```

5. **Build and run the application**
   ```bash
   # Development mode
   cargo run

   # Production mode (optimized)
   cargo build --release
   ./target/release/fantasy-golf
   ```

6. **Access the application**
   - Open your browser to `http://localhost:3000`

### Option 2: Docker Deployment (Recommended)

1. **Build and start all services**
   ```bash
   docker-compose up -d
   ```

   This will:
   - Start PostgreSQL on port 5432
   - Build the Rust application
   - Run database migrations automatically
   - Start the web server on port 3000

2. **View logs**
   ```bash
   docker-compose logs -f app
   ```

3. **Stop services**
   ```bash
   docker-compose down
   ```

4. **Complete teardown (including data)**
   ```bash
   docker-compose down -v
   ```

## API Endpoints

### Seasons
- `POST /api/seasons` - Create a new season
- `GET /api/seasons` - List all seasons
- `GET /api/seasons/active` - Get the active season

### Access Keys
- `POST /api/access-keys` - Generate access keys (commissioner only)
- `POST /api/access-keys/validate` - Validate an access key

### Golfers
- `POST /api/golfers` - Add a new golfer
- `GET /api/golfers` - List all active golfers

### Teams
- `POST /api/teams` - Create a team with golfer selection
- `GET /api/teams/:season_id` - List teams in a season
- `GET /api/teams/:team_id/golfers` - Get team's golfers

### Tournaments
- `POST /api/tournaments` - Create a tournament
- `GET /api/tournaments/:season_id` - List tournaments in a season

### Scores
- `POST /api/scores` - Add hole scores
- `GET /api/scores/tournament/:tournament_id` - Get tournament scores

### Leaderboards
- `GET /api/leaderboard/:season_id` - Season leaderboard
- `GET /api/leaderboard/tournament/:tournament_id` - Tournament leaderboard

## Usage Guide

### For Commissioners

1. **Initial Setup**
   - Navigate to the Admin panel
   - Create a new season with name, year, and date range
   - Add golfers to the database (organized in 6 groups by win probability)

2. **Invite Players**
   - Generate access keys in the Admin panel
   - Distribute keys to players via email or messaging
   - Each key can be used once to create a team

3. **Manage Tournaments**
   - Create tournaments throughout the season
   - Enter hole scores after each round
   - Scores are automatically converted to fantasy points

### For Players

1. **Join the League**
   - Click "Join League" in the navigation
   - Enter your unique access key
   - Enter your name

2. **Build Your Team**
   - Select one golfer from each of the 6 skill groups
   - Consider balancing high-risk, high-reward picks with consistent performers
   - Submit your team (teams cannot be changed once created)

3. **Track Your Progress**
   - View the leaderboard to see season standings
   - Check tournament-specific scores
   - Watch your position change as tournaments progress

## Database Schema

### Core Tables
- **seasons**: Golf seasons with date ranges
- **golfers**: Professional golfers grouped by win probability (1-6)
- **access_keys**: Unique keys for player registration
- **teams**: Player teams linked to a season
- **team_golfers**: Junction table (team ↔ golfers)
- **tournaments**: Individual tournaments within a season
- **hole_scores**: Per-hole scoring data

### Automatic Features
- Fantasy points calculated automatically via PostgreSQL triggers
- Team validation ensures exactly 6 golfers (one per group)
- Unique constraints prevent duplicate selections

## Development

### Running Tests
```bash
cargo test
```

### Database Migrations

Create a new migration:
```bash
sqlx migrate add <migration_name>
```

Run migrations:
```bash
sqlx migrate run
```

Revert last migration:
```bash
sqlx migrate revert
```

### SQLx Offline Mode

For CI/CD or offline compilation:
```bash
# Generate query metadata
cargo sqlx prepare

# Build with offline mode
cargo build --release
```

## Performance Considerations

- Connection pooling configured for 5 concurrent connections
- Database indexes on frequently queried columns
- Compiled queries via SQLx macros (zero runtime overhead)
- Optimized production build with LTO

## Security Notes

- Use strong passwords for PostgreSQL in production
- Consider implementing authentication for admin routes
- Access keys provide basic authorization
- HTTPS recommended for production deployment
- Environment variables should not be committed to version control

## Troubleshooting

### Database Connection Issues
```bash
# Check PostgreSQL is running
pg_isready -h localhost -p 5432

# Test connection
psql -h localhost -U postgres -d fantasy_golf
```

### Migration Errors
```bash
# Check migration status
sqlx migrate info

# Force reset (WARNING: destroys data)
sqlx database reset
```

### Docker Issues
```bash
# Rebuild containers
docker-compose up --build

# Check container logs
docker-compose logs app
docker-compose logs postgres
```

## Future Enhancements

- [ ] Player authentication system
- [ ] Real-time score updates via WebSockets
- [ ] Advanced analytics and statistics
- [ ] Email notifications for tournaments
- [ ] Integration with live golf APIs
