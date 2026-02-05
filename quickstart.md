# Feathers & Fairways - Quick Start Guide

## 🚀 Get Started in 5 Minutes

### Prerequisites
- Docker and Docker Compose installed on your system
- Port 3000 and 5432 available

### Step 2: Add Sample Data (Optional)
```bash
# Connect to the database
docker-compose exec postgres psql -U postgres -d feathers_and_fairways

# In the psql prompt, run:
\i /docker-entrypoint-initdb.d/seed_data.sql

# Or copy the file and run it:
docker cp seed_data.sql feathers_and_fairways-db:/tmp/
docker-compose exec postgres psql -U postgres -d feathers_and_fairways -f /tmp/seed_data.sql
```

### Step 3: Access the Application
Open your browser to: **http://localhost:3000**

### Step 4: Commissioner Setup

1. **Navigate to Admin Panel**
   - Click "Admin" in the navigation

2. **Create Season** (if you didn't load seed data)
   - Season Name: "PGA Tour 2025"
   - Year: 2025
   - Start/End dates
   - Click "Create Season"

3. **Add Golfers** (if you didn't load seed data)
   - Add golfers across all 6 groups
   - Group 1 = Highest win probability
   - Group 6 = Lowest win probability

4. **Generate Access Keys**
   - Enter number of keys (1-50)
   - Click "Generate Keys"
   - Copy and share keys with players

5. **Create Tournament**
   - Tournament name (e.g., "The Masters")
   - Start and end dates
   - Click "Create Tournament"

### Step 5: Player Experience

1. **Join League**
   - Click "Join League"
   - Enter access key provided by commissioner
   - Click "Validate Key"

2. **Build Team**
   - Enter your name
   - Select one golfer from each group (6 total)
   - Click "Create Team"

3. **View Leaderboard**
   - Click "Leaderboard"
   - See season standings or tournament scores

## 📊 Sample Workflow

### Before Tournament Starts
- Commissioner creates tournament
- Players have their teams ready

### During Tournament
- Commissioner enters hole scores as they complete
- Points automatically calculated:
  - Eagle or better: +2
  - Birdie: +1
  - Par: 0
  - Bogey or worse: -1

### After Tournament
- View leaderboard to see standings
- Season leaderboard accumulates points across all tournaments

## 🛠️ Common Commands

```bash
# View application logs
docker-compose logs -f app

# View database logs
docker-compose logs -f postgres

# Restart services
docker-compose restart

# Stop services
docker-compose down

# Stop and remove all data
docker-compose down -v

# Rebuild from scratch
docker-compose down -v
docker-compose up --build -d
```

## 📝 Sample Data Included

The seed_data.sql file includes:
- 24 professional golfers (4 per group)
- A 2025 season
- Ready for you to create tournaments and generate access keys

## 🔍 Troubleshooting

### Port Already in Use
```bash
# Change ports in docker-compose.yml
# Modify the ports section under each service
```

### Database Connection Failed
```bash
# Check if PostgreSQL is running
docker-compose ps

# Restart the database
docker-compose restart postgres

# Check logs
docker-compose logs postgres
```

### Application Won't Start
```bash
# Check logs for errors
docker-compose logs app

# Rebuild the application
docker-compose up --build app
```

### Reset Everything
```bash
# Nuclear option - removes all data
docker-compose down -v
./setup.sh
```

## 🎯 Next Steps

1. Customize golfers based on current PGA Tour rankings
2. Set up your season schedule
3. Invite players with access keys
4. Start tracking tournaments!

## 💡 Pro Tips

- **Golfer Groups**: Organize by current world rankings or win probability
- **Access Keys**: Generate extras - unused keys don't hurt
- **Team Strategy**: Balance high-risk picks with consistent performers
- **Scoring**: Enter scores after each round for real-time updates
- **Season Long**: Points accumulate across all tournaments

## 📧 Need Help?

Check the main README.md for:
- Detailed API documentation
- Development setup
- Architecture overview
- Advanced configuration

---

**Ready to play? Let's go! ⛳**