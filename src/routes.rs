use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sqlx::SqlitePool;
use validator::Validate;

use crate::models::*;
use crate::db::generate_access_key;
use crate::auth::*;

fn new_id() -> String {
    uuid::Uuid::new_v4().to_string()
}

fn calculate_fantasy_points(score_to_par: i32) -> i32 {
    match score_to_par {
        s if s <= -2 => 2,  // Eagle or better
        -1 => 1,            // Birdie
        0 => 0,             // Par
        _ => -1,            // Bogey or worse
    }
}

// Season routes
pub async fn create_season(
    State(pool): State<SqlitePool>,
    Json(payload): Json<CreateSeasonRequest>,
) -> Result<(StatusCode, Json<Season>), (StatusCode, Json<ApiError>)> {
    payload.validate()
        .map_err(|e| (StatusCode::BAD_REQUEST, Json(ApiError::new(e.to_string()))))?;

    // Deactivate all other seasons
    sqlx::query("UPDATE seasons SET is_active = 0")
        .execute(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

    let id = new_id();
    sqlx::query(
        "INSERT INTO seasons (id, name, year, start_date, end_date, is_active) VALUES (?, ?, ?, ?, ?, 1)"
    )
    .bind(&id)
    .bind(&payload.name)
    .bind(payload.year)
    .bind(&payload.start_date)
    .bind(&payload.end_date)
    .execute(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

    let season = sqlx::query_as::<_, Season>(
        "SELECT id, name, year, start_date, end_date, is_active, created_at FROM seasons WHERE id = ?"
    )
    .bind(&id)
    .fetch_one(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

    Ok((StatusCode::CREATED, Json(season)))
}

pub async fn list_seasons(
    State(pool): State<SqlitePool>,
) -> Result<Json<Vec<Season>>, (StatusCode, Json<ApiError>)> {
    let seasons = sqlx::query_as::<_, Season>(
        "SELECT id, name, year, start_date, end_date, is_active, created_at FROM seasons ORDER BY year DESC"
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

    Ok(Json(seasons))
}

pub async fn get_active_season(
    State(pool): State<SqlitePool>,
) -> Result<Json<Season>, (StatusCode, Json<ApiError>)> {
    let season = sqlx::query_as::<_, Season>(
        "SELECT id, name, year, start_date, end_date, is_active, created_at FROM seasons WHERE is_active = 1"
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?
    .ok_or((StatusCode::NOT_FOUND, Json(ApiError::new("No active season found"))))?;

    Ok(Json(season))
}

// Access key routes
pub async fn create_access_keys(
    State(pool): State<SqlitePool>,
    Json(payload): Json<CreateAccessKeysRequest>,
) -> Result<(StatusCode, Json<Vec<AccessKey>>), (StatusCode, Json<ApiError>)> {
    if payload.count < 1 || payload.count > 50 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ApiError::new("Count must be between 1 and 50")),
        ));
    }

    let mut keys = Vec::new();

    for _ in 0..payload.count {
        let id = new_id();
        let key_code = generate_access_key();

        sqlx::query(
            "INSERT INTO access_keys (id, key_code, season_id) VALUES (?, ?, ?)"
        )
        .bind(&id)
        .bind(&key_code)
        .bind(&payload.season_id)
        .execute(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

        let access_key = sqlx::query_as::<_, AccessKey>(
            "SELECT id, key_code, season_id, player_name, is_used, used_at, created_at FROM access_keys WHERE id = ?"
        )
        .bind(&id)
        .fetch_one(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

        keys.push(access_key);
    }

    Ok((StatusCode::CREATED, Json(keys)))
}

pub async fn validate_access_key(
    State(pool): State<SqlitePool>,
    Json(payload): Json<ValidateAccessKeyRequest>,
) -> Result<Json<AccessKeyValidationResponse>, (StatusCode, Json<ApiError>)> {
    let key = sqlx::query_as::<_, AccessKey>(
        "SELECT id, key_code, season_id, player_name, is_used, used_at, created_at FROM access_keys WHERE key_code = ?"
    )
    .bind(&payload.key_code)
    .fetch_optional(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

    match key {
        Some(k) => Ok(Json(AccessKeyValidationResponse {
            valid: true,
            season_id: Some(k.season_id),
            already_used: k.is_used,
        })),
        None => Ok(Json(AccessKeyValidationResponse {
            valid: false,
            season_id: None,
            already_used: false,
        })),
    }
}

// Golfer routes
pub async fn create_golfer(
    State(pool): State<SqlitePool>,
    Json(payload): Json<CreateGolferRequest>,
) -> Result<(StatusCode, Json<Golfer>), (StatusCode, Json<ApiError>)> {
    payload.validate()
        .map_err(|e| (StatusCode::BAD_REQUEST, Json(ApiError::new(e.to_string()))))?;

    let id = new_id();
    sqlx::query(
        "INSERT INTO golfers (id, name, win_probability_group) VALUES (?, ?, ?)"
    )
    .bind(&id)
    .bind(&payload.name)
    .bind(payload.win_probability_group)
    .execute(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

    let golfer = sqlx::query_as::<_, Golfer>(
        "SELECT id, name, win_probability_group, is_active, created_at FROM golfers WHERE id = ?"
    )
    .bind(&id)
    .fetch_one(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

    Ok((StatusCode::CREATED, Json(golfer)))
}

pub async fn list_golfers(
    State(pool): State<SqlitePool>,
) -> Result<Json<Vec<Golfer>>, (StatusCode, Json<ApiError>)> {
    let golfers = sqlx::query_as::<_, Golfer>(
        "SELECT id, name, win_probability_group, is_active, created_at FROM golfers WHERE is_active = 1 ORDER BY win_probability_group, name"
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

    Ok(Json(golfers))
}

pub async fn list_golfers_for_tournament(
    State(pool): State<SqlitePool>,
    Path(tournament_id): Path<String>,
) -> Result<Json<Vec<Golfer>>, (StatusCode, Json<ApiError>)> {
    let golfers = sqlx::query_as::<_, Golfer>(
        "SELECT g.id, g.name, \
         COALESCE(tgg.win_probability_group, g.win_probability_group) as win_probability_group, \
         g.is_active, g.created_at \
         FROM golfers g \
         LEFT JOIN tournament_golfer_groups tgg ON g.id = tgg.golfer_id AND tgg.tournament_id = ? \
         WHERE g.is_active = 1 \
         ORDER BY win_probability_group, name"
    )
    .bind(&tournament_id)
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

    Ok(Json(golfers))
}

// Team routes
pub async fn create_team(
    State(pool): State<SqlitePool>,
    Json(payload): Json<CreateTeamRequest>,
) -> Result<(StatusCode, Json<TeamWithGolfers>), (StatusCode, Json<ApiError>)> {
    payload.validate()
        .map_err(|e| (StatusCode::BAD_REQUEST, Json(ApiError::new(e.to_string()))))?;

    // Validate access key
    let access_key = sqlx::query_as::<_, AccessKey>(
        "SELECT id, key_code, season_id, player_name, is_used, used_at, created_at FROM access_keys WHERE key_code = ?"
    )
    .bind(&payload.key_code)
    .fetch_optional(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?
    .ok_or((StatusCode::NOT_FOUND, Json(ApiError::new("Invalid access key"))))?;

    if access_key.is_used {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ApiError::new("Access key has already been used")),
        ));
    }

    // Verify golfer selection (one from each group, using tournament-specific groups)
    let mut groups_used = std::collections::HashSet::new();

    for golfer_id in &payload.golfer_ids {
        let golfer = sqlx::query_as::<_, Golfer>(
            "SELECT g.id, g.name, \
             COALESCE(tgg.win_probability_group, g.win_probability_group) as win_probability_group, \
             g.is_active, g.created_at \
             FROM golfers g \
             LEFT JOIN tournament_golfer_groups tgg ON g.id = tgg.golfer_id AND tgg.tournament_id = ? \
             WHERE g.id = ?"
        )
        .bind(&payload.tournament_id)
        .bind(golfer_id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?
        .ok_or((StatusCode::BAD_REQUEST, Json(ApiError::new("Invalid golfer ID"))))?;

        if !groups_used.insert(golfer.win_probability_group) {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(ApiError::new("Cannot select multiple golfers from the same group")),
            ));
        }
    }

    if groups_used.len() != 6 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ApiError::new("Must select exactly one golfer from each of the 6 groups")),
        ));
    }

    // Start transaction
    let mut tx = pool.begin().await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

    // Create team
    let team_id = new_id();
    sqlx::query(
        "INSERT INTO teams (id, season_id, tournament_id, player_name, access_key_id) VALUES (?, ?, ?, ?, ?)"
    )
    .bind(&team_id)
    .bind(&access_key.season_id)
    .bind(&payload.tournament_id)
    .bind(&payload.player_name)
    .bind(&access_key.id)
    .execute(&mut *tx)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

    // Add golfers to team
    for golfer_id in &payload.golfer_ids {
        let tg_id = new_id();
        sqlx::query(
            "INSERT INTO team_golfers (id, team_id, golfer_id) VALUES (?, ?, ?)"
        )
        .bind(&tg_id)
        .bind(&team_id)
        .bind(golfer_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;
    }

    // Mark access key as used
    sqlx::query(
        "UPDATE access_keys SET is_used = 1, used_at = datetime('now'), player_name = ? WHERE id = ?"
    )
    .bind(&payload.player_name)
    .bind(&access_key.id)
    .execute(&mut *tx)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

    tx.commit().await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

    // Fetch the created team
    let team = sqlx::query_as::<_, Team>(
        "SELECT id, season_id, tournament_id, player_name, access_key_id, created_at FROM teams WHERE id = ?"
    )
    .bind(&team_id)
    .fetch_one(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

    // Fetch team golfers
    let golfers = sqlx::query_as::<_, Golfer>(
        "SELECT g.id, g.name, g.win_probability_group, g.is_active, g.created_at \
         FROM golfers g \
         INNER JOIN team_golfers tg ON g.id = tg.golfer_id \
         WHERE tg.team_id = ? \
         ORDER BY g.win_probability_group"
    )
    .bind(&team_id)
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

    Ok((StatusCode::CREATED, Json(TeamWithGolfers { team, golfers })))
}

pub async fn list_teams(
    State(pool): State<SqlitePool>,
    Path(season_id): Path<String>,
) -> Result<Json<Vec<Team>>, (StatusCode, Json<ApiError>)> {
    let teams = sqlx::query_as::<_, Team>(
        "SELECT id, season_id, tournament_id, player_name, access_key_id, created_at FROM teams WHERE season_id = ? ORDER BY player_name"
    )
    .bind(&season_id)
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

    Ok(Json(teams))
}

pub async fn get_team_golfers(
    State(pool): State<SqlitePool>,
    Path(team_id): Path<String>,
) -> Result<Json<Vec<Golfer>>, (StatusCode, Json<ApiError>)> {
    let golfers = sqlx::query_as::<_, Golfer>(
        "SELECT g.id, g.name, g.win_probability_group, g.is_active, g.created_at \
         FROM golfers g \
         INNER JOIN team_golfers tg ON g.id = tg.golfer_id \
         WHERE tg.team_id = ? \
         ORDER BY g.win_probability_group"
    )
    .bind(&team_id)
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

    Ok(Json(golfers))
}

// Tournament routes
pub async fn create_tournament(
    State(pool): State<SqlitePool>,
    Json(payload): Json<CreateTournamentRequest>,
) -> Result<(StatusCode, Json<Tournament>), (StatusCode, Json<ApiError>)> {
    payload.validate()
        .map_err(|e| (StatusCode::BAD_REQUEST, Json(ApiError::new(e.to_string()))))?;

    sqlx::query("UPDATE tournaments SET is_active = 0 WHERE season_id = ?")
        .bind(&payload.season_id)
        .execute(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

    let id = new_id();
    sqlx::query(
        "INSERT INTO tournaments (id, season_id, name, start_date, end_date, is_active) VALUES (?, ?, ?, ?, ?, 1)"
    )
    .bind(&id)
    .bind(&payload.season_id)
    .bind(&payload.name)
    .bind(&payload.start_date)
    .bind(&payload.end_date)
    .execute(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

    let tournament = sqlx::query_as::<_, Tournament>(
        "SELECT id, season_id, name, start_date, end_date, is_active, created_at FROM tournaments WHERE id = ?"
    )
    .bind(&id)
    .fetch_one(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

    Ok((StatusCode::CREATED, Json(tournament)))
}

pub async fn list_tournaments(
    State(pool): State<SqlitePool>,
    Path(season_id): Path<String>,
) -> Result<Json<Vec<Tournament>>, (StatusCode, Json<ApiError>)> {
    let tournaments = sqlx::query_as::<_, Tournament>(
        "SELECT id, season_id, name, start_date, end_date, is_active, created_at FROM tournaments WHERE season_id = ? ORDER BY start_date DESC"
    )
    .bind(&season_id)
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

    Ok(Json(tournaments))
}

pub async fn add_hole_scores(
    State(pool): State<SqlitePool>,
    Json(payload): Json<AddHoleScoresRequest>,
) -> Result<(StatusCode, Json<Vec<HoleScore>>), (StatusCode, Json<ApiError>)> {
    payload.validate()
        .map_err(|e| (StatusCode::BAD_REQUEST, Json(ApiError::new(e.to_string()))))?;

    let mut scores = Vec::new();

    for score_input in payload.scores {
        score_input.validate()
            .map_err(|e| (StatusCode::BAD_REQUEST, Json(ApiError::new(e.to_string()))))?;

        let fantasy_points = calculate_fantasy_points(score_input.score_to_par);
        let id = new_id();

        sqlx::query(
            "INSERT INTO hole_scores (id, tournament_id, golfer_id, day, hole, strokes, score_to_par, fantasy_points) \
             VALUES (?, ?, ?, ?, ?, ?, ?, ?) \
             ON CONFLICT (tournament_id, golfer_id, day, hole) \
             DO UPDATE SET strokes = excluded.strokes, score_to_par = excluded.score_to_par, fantasy_points = excluded.fantasy_points"
        )
        .bind(&id)
        .bind(&payload.tournament_id)
        .bind(&score_input.golfer_id)
        .bind(score_input.day)
        .bind(score_input.hole)
        .bind(score_input.strokes)
        .bind(score_input.score_to_par)
        .bind(fantasy_points)
        .execute(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

        // Fetch the upserted score
        let score = sqlx::query_as::<_, HoleScore>(
            "SELECT id, tournament_id, golfer_id, day, hole, strokes, score_to_par, fantasy_points, created_at \
             FROM hole_scores WHERE tournament_id = ? AND golfer_id = ? AND day = ? AND hole = ?"
        )
        .bind(&payload.tournament_id)
        .bind(&score_input.golfer_id)
        .bind(score_input.day)
        .bind(score_input.hole)
        .fetch_one(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

        scores.push(score);
    }

    Ok((StatusCode::CREATED, Json(scores)))
}

pub async fn get_tournament_scores(
    State(pool): State<SqlitePool>,
    Path(tournament_id): Path<String>,
) -> Result<Json<Vec<HoleScore>>, (StatusCode, Json<ApiError>)> {
    let scores = sqlx::query_as::<_, HoleScore>(
        "SELECT id, tournament_id, golfer_id, day, hole, strokes, score_to_par, fantasy_points, created_at \
         FROM hole_scores WHERE tournament_id = ? ORDER BY day, hole, golfer_id"
    )
    .bind(&tournament_id)
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

    Ok(Json(scores))
}

pub async fn get_season_leaderboard(
    State(pool): State<SqlitePool>,
    Path(season_id): Path<String>,
) -> Result<Json<Vec<LeaderboardEntry>>, (StatusCode, Json<ApiError>)> {
    let leaderboard = sqlx::query_as::<_, LeaderboardEntry>(
        "SELECT \
            t.player_name, \
            t.id as team_id, \
            COALESCE(SUM(hs.fantasy_points), 0) as total_points \
         FROM teams t \
         LEFT JOIN team_golfers tg ON t.id = tg.team_id \
         LEFT JOIN hole_scores hs ON tg.golfer_id = hs.golfer_id \
         LEFT JOIN tournaments tour ON hs.tournament_id = tour.id AND tour.season_id = ? \
         WHERE t.season_id = ? \
         GROUP BY t.id, t.player_name \
         ORDER BY total_points DESC"
    )
    .bind(&season_id)
    .bind(&season_id)
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

    Ok(Json(leaderboard))
}

pub async fn get_season_leaderboard_with_golfers(
    State(pool): State<SqlitePool>,
    Path(season_id): Path<String>,
) -> Result<Json<Vec<LeaderboardEntryWithGolfers>>, (StatusCode, Json<ApiError>)> {
    // Get leaderboard entries
    let leaderboard = sqlx::query_as::<_, LeaderboardEntry>(
        "SELECT \
            t.player_name, \
            t.id as team_id, \
            COALESCE(SUM(hs.fantasy_points), 0) as total_points \
         FROM teams t \
         LEFT JOIN team_golfers tg ON t.id = tg.team_id \
         LEFT JOIN hole_scores hs ON tg.golfer_id = hs.golfer_id \
         LEFT JOIN tournaments tour ON hs.tournament_id = tour.id AND tour.season_id = ? \
         WHERE t.season_id = ? \
         GROUP BY t.id, t.player_name \
         ORDER BY total_points DESC"
    )
    .bind(&season_id)
    .bind(&season_id)
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

    // Batch-fetch all team golfers for this season
    let team_golfers = sqlx::query_as::<_, TeamGolferRow>(
        "SELECT tg.team_id, g.id, g.name, g.win_probability_group \
         FROM team_golfers tg \
         INNER JOIN golfers g ON tg.golfer_id = g.id \
         INNER JOIN teams t ON tg.team_id = t.id \
         WHERE t.season_id = ? \
         ORDER BY g.win_probability_group"
    )
    .bind(&season_id)
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

    // Group golfers by team_id
    let mut golfers_by_team: std::collections::HashMap<String, Vec<GolferSummary>> = std::collections::HashMap::new();
    for row in team_golfers {
        golfers_by_team.entry(row.team_id).or_default().push(GolferSummary {
            id: row.id,
            name: row.name,
            win_probability_group: row.win_probability_group,
        });
    }

    // Combine leaderboard entries with golfers
    let result: Vec<LeaderboardEntryWithGolfers> = leaderboard
        .into_iter()
        .map(|entry| LeaderboardEntryWithGolfers {
            player_name: entry.player_name,
            team_id: entry.team_id.clone(),
            total_points: entry.total_points.unwrap_or(0),
            golfers: golfers_by_team.remove(&entry.team_id).unwrap_or_default(),
        })
        .collect();

    Ok(Json(result))
}

pub async fn get_tournament_leaderboard(
    State(pool): State<SqlitePool>,
    Path(tournament_id): Path<String>,
) -> Result<Json<Vec<TournamentScore>>, (StatusCode, Json<ApiError>)> {
    let leaderboard = sqlx::query_as::<_, TournamentScore>(
        "SELECT \
            g.name as golfer_name, \
            g.id as golfer_id, \
            COALESCE(SUM(hs.fantasy_points), 0) as total_points \
         FROM golfers g \
         LEFT JOIN hole_scores hs ON g.id = hs.golfer_id AND hs.tournament_id = ? \
         GROUP BY g.id, g.name \
         HAVING COALESCE(SUM(hs.fantasy_points), 0) > 0 \
         ORDER BY total_points DESC"
    )
    .bind(&tournament_id)
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

    Ok(Json(leaderboard))
}

// Upload golfers from JSON
pub async fn upload_golfers(
    State(pool): State<SqlitePool>,
    Json(payload): Json<GolferUploadRequest>,
) -> Result<Json<GolferUploadResponse>, (StatusCode, Json<ApiError>)> {
    if payload.golfers.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ApiError::new("golfers array is empty")),
        ));
    }

    let mut total_created: usize = 0;
    let mut total_updated: usize = 0;
    let mut errors: Vec<String> = Vec::new();

    for entry in &payload.golfers {
        if entry.group < 1 || entry.group > 6 {
            errors.push(format!("{}: group must be between 1 and 6", entry.name));
            continue;
        }

        if entry.name.trim().is_empty() {
            errors.push("Empty golfer name".to_string());
            continue;
        }

        // Check if golfer already exists
        #[derive(sqlx::FromRow)]
        struct ExistsRow {
            id: String,
        }

        let existing = sqlx::query_as::<_, ExistsRow>(
            "SELECT id FROM golfers WHERE LOWER(name) = LOWER(?)"
        )
        .bind(entry.name.trim())
        .fetch_optional(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

        let id = match &existing {
            Some(row) => row.id.clone(),
            None => new_id(),
        };

        sqlx::query(
            "INSERT INTO golfers (id, name, win_probability_group) VALUES (?, ?, ?) \
             ON CONFLICT(name) DO UPDATE SET win_probability_group = excluded.win_probability_group, is_active = 1"
        )
        .bind(&id)
        .bind(entry.name.trim())
        .bind(entry.group)
        .execute(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

        if existing.is_some() {
            total_updated += 1;
        } else {
            total_created += 1;
        }
    }

    Ok(Json(GolferUploadResponse {
        total_created,
        total_updated,
        errors,
    }))
}

// Upload tournament-specific golfer groups
pub async fn upload_tournament_golfer_groups(
    State(pool): State<SqlitePool>,
    Path(tournament_id): Path<String>,
    Json(payload): Json<TournamentGolferGroupUploadRequest>,
) -> Result<Json<TournamentGolferGroupUploadResponse>, (StatusCode, Json<ApiError>)> {
    // Verify tournament exists
    let _tournament = sqlx::query_as::<_, Tournament>(
        "SELECT id, season_id, name, start_date, end_date, is_active, created_at FROM tournaments WHERE id = ?"
    )
    .bind(&tournament_id)
    .fetch_optional(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?
    .ok_or((StatusCode::NOT_FOUND, Json(ApiError::new("Tournament not found"))))?;

    if payload.groups.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ApiError::new("groups array is empty")),
        ));
    }

    let mut total_processed: usize = 0;
    let mut errors: Vec<String> = Vec::new();

    for entry in &payload.groups {
        if entry.group < 1 || entry.group > 6 {
            errors.push(format!("{}: group must be between 1 and 6", entry.golfer));
            continue;
        }

        // Look up golfer by name (case-insensitive)
        #[derive(sqlx::FromRow)]
        struct GolferIdRow {
            id: String,
        }

        let golfer = sqlx::query_as::<_, GolferIdRow>(
            "SELECT id FROM golfers WHERE LOWER(name) = LOWER(?)"
        )
        .bind(&entry.golfer)
        .fetch_optional(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

        let golfer_id = match golfer {
            Some(g) => g.id,
            None => {
                errors.push(format!("{}: golfer not found", entry.golfer));
                continue;
            }
        };

        let id = new_id();
        sqlx::query(
            "INSERT INTO tournament_golfer_groups (id, tournament_id, golfer_id, win_probability_group) \
             VALUES (?, ?, ?, ?) \
             ON CONFLICT(tournament_id, golfer_id) \
             DO UPDATE SET win_probability_group = excluded.win_probability_group"
        )
        .bind(&id)
        .bind(&tournament_id)
        .bind(&golfer_id)
        .bind(entry.group)
        .execute(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

        total_processed += 1;
    }

    Ok(Json(TournamentGolferGroupUploadResponse {
        total_processed,
        errors,
    }))
}

// Admin authentication
pub async fn admin_login(
    Json(payload): Json<AdminLoginRequest>,
) -> Result<Json<AdminLoginResponse>, (StatusCode, Json<ApiError>)> {
    if verify_admin_password(&payload.password) {
        Ok(Json(AdminLoginResponse {
            success: true,
            token: Some(generate_admin_token()),
        }))
    } else {
        Ok(Json(AdminLoginResponse {
            success: false,
            token: None,
        }))
    }
}

// Update team (before tournament starts)
pub async fn update_team(
    State(pool): State<SqlitePool>,
    Json(payload): Json<UpdateTeamRequest>,
) -> Result<Json<TeamWithGolfers>, (StatusCode, Json<ApiError>)> {
    payload.validate()
        .map_err(|e| (StatusCode::BAD_REQUEST, Json(ApiError::new(e.to_string()))))?;

    // Validate access key
    let access_key = sqlx::query_as::<_, AccessKey>(
        "SELECT id, key_code, season_id, player_name, is_used, used_at, created_at FROM access_keys WHERE key_code = ?"
    )
    .bind(&payload.key_code)
    .fetch_optional(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?
    .ok_or((StatusCode::NOT_FOUND, Json(ApiError::new("Invalid access key"))))?;

    if !access_key.is_used {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ApiError::new("Access key has not been used yet. Create a team first.")),
        ));
    }

    // Check if tournament has started
    #[derive(sqlx::FromRow)]
    struct TournamentStartDate {
        start_date: String,
    }

    let tournament = sqlx::query_as::<_, TournamentStartDate>(
        "SELECT start_date FROM tournaments WHERE id = ?"
    )
    .bind(&payload.tournament_id)
    .fetch_optional(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?
    .ok_or((StatusCode::NOT_FOUND, Json(ApiError::new("Tournament not found"))))?;

    let now = chrono::Utc::now().date_naive();
    let start_date = chrono::NaiveDate::parse_from_str(&tournament.start_date, "%Y-%m-%d")
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(format!("Invalid date format: {}", e)))))?;

    if start_date <= now {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ApiError::new("Cannot edit team after tournament has started")),
        ));
    }

    // Find existing team
    let team = sqlx::query_as::<_, Team>(
        "SELECT id, season_id, tournament_id, player_name, access_key_id, created_at FROM teams WHERE access_key_id = ? AND tournament_id = ?"
    )
    .bind(&access_key.id)
    .bind(&payload.tournament_id)
    .fetch_optional(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?
    .ok_or((StatusCode::NOT_FOUND, Json(ApiError::new("Team not found for this tournament"))))?;

    // Verify golfer selection (one from each group, using tournament-specific groups)
    let mut groups_used = std::collections::HashSet::new();

    for golfer_id in &payload.golfer_ids {
        let golfer = sqlx::query_as::<_, Golfer>(
            "SELECT g.id, g.name, \
             COALESCE(tgg.win_probability_group, g.win_probability_group) as win_probability_group, \
             g.is_active, g.created_at \
             FROM golfers g \
             LEFT JOIN tournament_golfer_groups tgg ON g.id = tgg.golfer_id AND tgg.tournament_id = ? \
             WHERE g.id = ?"
        )
        .bind(&payload.tournament_id)
        .bind(golfer_id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?
        .ok_or((StatusCode::BAD_REQUEST, Json(ApiError::new("Invalid golfer ID"))))?;

        if !groups_used.insert(golfer.win_probability_group) {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(ApiError::new("Cannot select multiple golfers from the same group")),
            ));
        }
    }

    if groups_used.len() != 6 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ApiError::new("Must select exactly one golfer from each of the 6 groups")),
        ));
    }

    // Start transaction
    let mut tx = pool.begin().await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

    // Remove old golfer selections
    sqlx::query("DELETE FROM team_golfers WHERE team_id = ?")
        .bind(&team.id)
        .execute(&mut *tx)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

    // Add new golfer selections
    for golfer_id in &payload.golfer_ids {
        let tg_id = new_id();
        sqlx::query("INSERT INTO team_golfers (id, team_id, golfer_id) VALUES (?, ?, ?)")
            .bind(&tg_id)
            .bind(&team.id)
            .bind(golfer_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;
    }

    tx.commit().await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

    // Fetch updated team golfers
    let golfers = sqlx::query_as::<_, Golfer>(
        "SELECT g.id, g.name, g.win_probability_group, g.is_active, g.created_at \
         FROM golfers g \
         INNER JOIN team_golfers tg ON g.id = tg.golfer_id \
         WHERE tg.team_id = ? \
         ORDER BY g.win_probability_group"
    )
    .bind(&team.id)
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

    Ok(Json(TeamWithGolfers { team, golfers }))
}

// Upload tournament scores from JSON
pub async fn upload_tournament_scores(
    State(pool): State<SqlitePool>,
    Path(tournament_id): Path<String>,
    Json(payload): Json<ScoreUploadRequest>,
) -> Result<Json<ScoreUploadResponse>, (StatusCode, Json<ApiError>)> {
    // Validate pars array
    if payload.pars.len() != 18 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ApiError::new("pars array must have exactly 18 elements")),
        ));
    }

    // Verify tournament exists
    let _tournament = sqlx::query_as::<_, Tournament>(
        "SELECT id, season_id, name, start_date, end_date, is_active, created_at FROM tournaments WHERE id = ?"
    )
    .bind(&tournament_id)
    .fetch_optional(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?
    .ok_or((StatusCode::NOT_FOUND, Json(ApiError::new("Tournament not found"))))?;

    let mut total_processed: usize = 0;
    let mut errors: Vec<String> = Vec::new();

    for entry in &payload.scores {
        // Validate holes array
        if entry.holes.len() != 18 {
            errors.push(format!("{}: holes array must have exactly 18 elements", entry.golfer));
            continue;
        }

        // Validate day
        if entry.day < 1 || entry.day > 4 {
            errors.push(format!("{}: day must be between 1 and 4", entry.golfer));
            continue;
        }

        // Look up golfer by name (case-insensitive)
        #[derive(sqlx::FromRow)]
        struct GolferIdRow {
            id: String,
        }

        let golfer = sqlx::query_as::<_, GolferIdRow>(
            "SELECT id FROM golfers WHERE LOWER(name) = LOWER(?)"
        )
        .bind(&entry.golfer)
        .fetch_optional(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

        let golfer_id = match golfer {
            Some(g) => g.id,
            None => {
                errors.push(format!("{}: golfer not found", entry.golfer));
                continue;
            }
        };

        // Insert scores for each hole
        for (hole_index, &strokes) in entry.holes.iter().enumerate() {
            let hole_num = (hole_index + 1) as i32;
            let par = payload.pars[hole_index];
            let score_to_par = strokes - par;
            let fantasy_points = calculate_fantasy_points(score_to_par);
            let id = new_id();

            sqlx::query(
                "INSERT INTO hole_scores (id, tournament_id, golfer_id, day, hole, strokes, score_to_par, fantasy_points) \
                 VALUES (?, ?, ?, ?, ?, ?, ?, ?) \
                 ON CONFLICT (tournament_id, golfer_id, day, hole) \
                 DO UPDATE SET strokes = excluded.strokes, score_to_par = excluded.score_to_par, fantasy_points = excluded.fantasy_points"
            )
            .bind(&id)
            .bind(&tournament_id)
            .bind(&golfer_id)
            .bind(entry.day)
            .bind(hole_num)
            .bind(strokes)
            .bind(score_to_par)
            .bind(fantasy_points)
            .execute(&pool)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

            total_processed += 1;
        }
    }

    Ok(Json(ScoreUploadResponse {
        total_scores_processed: total_processed,
        errors,
    }))
}
