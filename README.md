# Feathers & Fairways

A fantasy golf league management application. Commissioners manage seasons and tournaments, players create teams using access keys, and the system tracks scores and leaderboards.

## Features

### Scoring System

- **Eagle or better**: +2 points
- **Birdie**: +1 point
- **Par**: 0 points
- **Bogey or worse**: -1 point (amateurs score 0 instead — they cannot receive negative fantasy points)

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

