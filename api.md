# Fantasy Golf API Documentation

Base URL: `http://localhost:3000/api`

## Table of Contents
- [Seasons](#seasons)
- [Access Keys](#access-keys)
- [Golfers](#golfers)
- [Teams](#teams)
- [Tournaments](#tournaments)
- [Scores](#scores)
- [Leaderboards](#leaderboards)

---

## Seasons

### Create Season
Create a new golf season and set it as active.

**Endpoint:** `POST /api/seasons`

**Request Body:**
```json
{
  "name": "PGA Tour 2025",
  "year": 2025,
  "start_date": "2025-01-01",
  "end_date": "2025-12-31"
}
```

**Response:** `201 Created`
```json
{
  "id": "uuid",
  "name": "PGA Tour 2025",
  "year": 2025,
  "start_date": "2025-01-01",
  "end_date": "2025-12-31",
  "is_active": true,
  "created_at": "2025-01-01T00:00:00Z"
}
```

### List Seasons
Get all seasons ordered by year.

**Endpoint:** `GET /api/seasons`

**Response:** `200 OK`
```json
[
  {
    "id": "uuid",
    "name": "PGA Tour 2025",
    "year": 2025,
    "start_date": "2025-01-01",
    "end_date": "2025-12-31",
    "is_active": true,
    "created_at": "2025-01-01T00:00:00Z"
  }
]
```

### Get Active Season
Get the currently active season.

**Endpoint:** `GET /api/seasons/active`

**Response:** `200 OK`
```json
{
  "id": "uuid",
  "name": "PGA Tour 2025",
  "year": 2025,
  "start_date": "2025-01-01",
  "end_date": "2025-12-31",
  "is_active": true,
  "created_at": "2025-01-01T00:00:00Z"
}
```

---

## Access Keys

### Generate Access Keys
Generate unique access keys for players to join the league.

**Endpoint:** `POST /api/access-keys`

**Request Body:**
```json
{
  "season_id": "uuid",
  "count": 10
}
```

**Constraints:**
- `count`: 1-50

**Response:** `201 Created`
```json
[
  {
    "id": "uuid",
    "key_code": "ABCD-EFGH-IJKL",
    "season_id": "uuid",
    "player_name": null,
    "is_used": false,
    "used_at": null,
    "created_at": "2025-01-01T00:00:00Z"
  }
]
```

### Validate Access Key
Check if an access key is valid and available.

**Endpoint:** `POST /api/access-keys/validate`

**Request Body:**
```json
{
  "key_code": "ABCD-EFGH-IJKL"
}
```

**Response:** `200 OK`
```json
{
  "valid": true,
  "season_id": "uuid",
  "already_used": false
}
```

---

## Golfers

### Add Golfer
Add a new golfer to the database.

**Endpoint:** `POST /api/golfers`

**Request Body:**
```json
{
  "name": "Tiger Woods",
  "win_probability_group": 1
}
```

**Constraints:**
- `name`: 1-255 characters
- `win_probability_group`: 1-6

**Response:** `201 Created`
```json
{
  "id": "uuid",
  "name": "Tiger Woods",
  "win_probability_group": 1,
  "is_active": true,
  "created_at": "2025-01-01T00:00:00Z"
}
```

### List Golfers
Get all active golfers grouped by win probability.

**Endpoint:** `GET /api/golfers`

**Response:** `200 OK`
```json
[
  {
    "id": "uuid",
    "name": "Tiger Woods",
    "win_probability_group": 1,
    "is_active": true,
    "created_at": "2025-01-01T00:00:00Z"
  }
]
```

---

## Teams

### Create Team
Create a new team with golfer selections.

**Endpoint:** `POST /api/teams`

**Request Body:**
```json
{
  "key_code": "ABCD-EFGH-IJKL",
  "player_name": "John Doe",
  "golfer_ids": [
    "uuid1",
    "uuid2",
    "uuid3",
    "uuid4",
    "uuid5",
    "uuid6"
  ]
}
```

**Constraints:**
- Must select exactly 6 golfers
- One golfer from each win probability group (1-6)
- Access key must be valid and unused

**Response:** `201 Created`
```json
{
  "team": {
    "id": "uuid",
    "season_id": "uuid",
    "player_name": "John Doe",
    "access_key_id": "uuid",
    "created_at": "2025-01-01T00:00:00Z"
  },
  "golfers": [
    {
      "id": "uuid",
      "name": "Tiger Woods",
      "win_probability_group": 1,
      "is_active": true,
      "created_at": "2025-01-01T00:00:00Z"
    }
  ]
}
```

### List Teams
Get all teams in a season.

**Endpoint:** `GET /api/teams/:season_id`

**Response:** `200 OK`
```json
[
  {
    "id": "uuid",
    "season_id": "uuid",
    "player_name": "John Doe",
    "access_key_id": "uuid",
    "created_at": "2025-01-01T00:00:00Z"
  }
]
```

### Get Team Golfers
Get the golfers on a specific team.

**Endpoint:** `GET /api/teams/:team_id/golfers`

**Response:** `200 OK`
```json
[
  {
    "id": "uuid",
    "name": "Tiger Woods",
    "win_probability_group": 1,
    "is_active": true,
    "created_at": "2025-01-01T00:00:00Z"
  }
]
```

---

## Tournaments

### Create Tournament
Create a new tournament in the current season.

**Endpoint:** `POST /api/tournaments`

**Request Body:**
```json
{
  "season_id": "uuid",
  "name": "The Masters",
  "start_date": "2025-04-10",
  "end_date": "2025-04-13"
}
```

**Response:** `201 Created`
```json
{
  "id": "uuid",
  "season_id": "uuid",
  "name": "The Masters",
  "start_date": "2025-04-10",
  "end_date": "2025-04-13",
  "is_active": true,
  "created_at": "2025-01-01T00:00:00Z"
}
```

### List Tournaments
Get all tournaments in a season.

**Endpoint:** `GET /api/tournaments/:season_id`

**Response:** `200 OK`
```json
[
  {
    "id": "uuid",
    "season_id": "uuid",
    "name": "The Masters",
    "start_date": "2025-04-10",
    "end_date": "2025-04-13",
    "is_active": true,
    "created_at": "2025-01-01T00:00:00Z"
  }
]
```

---

## Scores

### Add Hole Scores
Record hole-by-hole scores for golfers.

**Endpoint:** `POST /api/scores`

**Request Body:**
```json
{
  "tournament_id": "uuid",
  "scores": [
    {
      "golfer_id": "uuid",
      "day": 1,
      "hole": 1,
      "strokes": 4,
      "score_to_par": 0
    },
    {
      "golfer_id": "uuid",
      "day": 1,
      "hole": 2,
      "strokes": 3,
      "score_to_par": -1
    }
  ]
}
```

**Constraints:**
- `day`: 1-4
- `hole`: 1-18
- `strokes`: > 0

**Fantasy Points Calculation:**
- Score to par ≤ -2 (Eagle or better): +2 points
- Score to par = -1 (Birdie): +1 point
- Score to par = 0 (Par): 0 points
- Score to par ≥ 1 (Bogey or worse): -1 point

**Response:** `201 Created`
```json
[
  {
    "id": "uuid",
    "tournament_id": "uuid",
    "golfer_id": "uuid",
    "day": 1,
    "hole": 1,
    "strokes": 4,
    "score_to_par": 0,
    "fantasy_points": 0,
    "created_at": "2025-04-10T12:00:00Z"
  }
]
```

### Get Tournament Scores
Get all scores for a tournament.

**Endpoint:** `GET /api/scores/tournament/:tournament_id`

**Response:** `200 OK`
```json
[
  {
    "id": "uuid",
    "tournament_id": "uuid",
    "golfer_id": "uuid",
    "day": 1,
    "hole": 1,
    "strokes": 4,
    "score_to_par": 0,
    "fantasy_points": 0,
    "created_at": "2025-04-10T12:00:00Z"
  }
]
```

---

## Leaderboards

### Season Leaderboard
Get cumulative standings for the entire season.

**Endpoint:** `GET /api/leaderboard/:season_id`

**Response:** `200 OK`
```json
[
  {
    "player_name": "John Doe",
    "team_id": "uuid",
    "total_points": 150
  },
  {
    "player_name": "Jane Smith",
    "team_id": "uuid",
    "total_points": 142
  }
]
```

### Tournament Leaderboard
Get golfer standings for a specific tournament.

**Endpoint:** `GET /api/leaderboard/tournament/:tournament_id`

**Response:** `200 OK`
```json
[
  {
    "golfer_name": "Tiger Woods",
    "golfer_id": "uuid",
    "total_points": 25
  },
  {
    "golfer_name": "Phil Mickelson",
    "golfer_id": "uuid",
    "total_points": 22
  }
]
```

---

## Error Responses

All endpoints may return error responses:

**400 Bad Request**
```json
{
  "message": "Validation error description"
}
```

**404 Not Found**
```json
{
  "message": "Resource not found"
}
```

**500 Internal Server Error**
```json
{
  "message": "Internal server error"
}
```

---

## Rate Limiting

Currently no rate limiting is implemented. Consider adding rate limiting for production use.

## Authentication

The API currently does not require authentication. For production:
- Implement JWT or session-based auth
- Protect admin endpoints
- Add role-based access control

## CORS

CORS is currently configured to allow all origins (`CorsLayer::permissive()`). 
For production, configure specific allowed origins.