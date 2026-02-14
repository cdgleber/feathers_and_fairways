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

fn calculate_fantasy_points(score_to_par: i32, is_amateur: bool) -> i32 {
    match score_to_par {
        s if s <= -2 => 2,  // Eagle or better
        -1 => 1,            // Birdie
        0 => 0,             // Par
        _ => if is_amateur { 0 } else { -1 },  // Bogey or worse (amateurs get 0)
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
    let is_amateur = payload.is_amateur.unwrap_or(false);
    sqlx::query(
        "INSERT INTO golfers (id, name, win_probability_group, is_amateur) VALUES (?, ?, ?, ?)"
    )
    .bind(&id)
    .bind(&payload.name)
    .bind(payload.win_probability_group)
    .bind(is_amateur)
    .execute(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

    let golfer = sqlx::query_as::<_, Golfer>(
        "SELECT id, name, win_probability_group, is_amateur, is_active, created_at FROM golfers WHERE id = ?"
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
        "SELECT id, name, win_probability_group, is_amateur, is_active, created_at FROM golfers WHERE is_active = 1 ORDER BY win_probability_group, name"
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
         g.is_amateur, g.is_active, g.created_at \
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
             g.is_amateur, g.is_active, g.created_at \
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

    if groups_used.len() != 9 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ApiError::new("Must select exactly one golfer from each of the 9 groups")),
        ));
    }

    // Start transaction
    let mut tx = pool.begin().await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

    // Create team
    let team_id = new_id();
    sqlx::query(
        "INSERT INTO teams (id, season_id, tournament_id, player_name, access_key_id, email) VALUES (?, ?, ?, ?, ?, ?)"
    )
    .bind(&team_id)
    .bind(&access_key.season_id)
    .bind(&payload.tournament_id)
    .bind(&payload.player_name)
    .bind(&access_key.id)
    .bind(&payload.email)
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
        "SELECT id, season_id, tournament_id, player_name, access_key_id, email, created_at FROM teams WHERE id = ?"
    )
    .bind(&team_id)
    .fetch_one(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

    // Fetch team golfers
    let golfers = sqlx::query_as::<_, Golfer>(
        "SELECT g.id, g.name, g.win_probability_group, g.is_amateur, g.is_active, g.created_at \
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
        "SELECT id, season_id, tournament_id, player_name, access_key_id, email, created_at FROM teams WHERE season_id = ? ORDER BY player_name"
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
        "SELECT g.id, g.name, g.win_probability_group, g.is_amateur, g.is_active, g.created_at \
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

        #[derive(sqlx::FromRow)]
        struct AmateurRow { is_amateur: bool }
        let golfer_row = sqlx::query_as::<_, AmateurRow>(
            "SELECT is_amateur FROM golfers WHERE id = ?"
        )
        .bind(&score_input.golfer_id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?
        .ok_or((StatusCode::BAD_REQUEST, Json(ApiError::new("Golfer not found"))))?;

        let fantasy_points = calculate_fantasy_points(score_input.score_to_par, golfer_row.is_amateur);
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
        "SELECT tg.team_id, g.id, g.name, g.win_probability_group, g.is_amateur \
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
            is_amateur: row.is_amateur,
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
        if entry.group < 1 || entry.group > 9 {
            errors.push(format!("{}: group must be between 1 and 9", entry.name));
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

        let is_amateur = entry.amateur.unwrap_or(false);

        if let Some(row) = existing {
            // Update existing golfer (avoids case-sensitivity mismatch with ON CONFLICT)
            sqlx::query(
                "UPDATE golfers SET win_probability_group = ?, is_amateur = ?, is_active = 1 WHERE id = ?"
            )
            .bind(entry.group)
            .bind(is_amateur)
            .bind(&row.id)
            .execute(&pool)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

            total_updated += 1;
        } else {
            // Insert new golfer
            let id = new_id();
            sqlx::query(
                "INSERT INTO golfers (id, name, win_probability_group, is_amateur) VALUES (?, ?, ?, ?)"
            )
            .bind(&id)
            .bind(entry.name.trim())
            .bind(entry.group)
            .bind(is_amateur)
            .execute(&pool)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

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
        if entry.group < 1 || entry.group > 9 {
            errors.push(format!("{}: group must be between 1 and 9", entry.golfer));
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
    let password_valid = verify_admin_password(&payload.password)
        .map_err(|_| (StatusCode::UNAUTHORIZED, Json(ApiError::new("Admin authentication is not configured"))))?;

    if password_valid {
        let token = generate_admin_token()
            .map_err(|_| (StatusCode::UNAUTHORIZED, Json(ApiError::new("Admin authentication is not configured"))))?;
        Ok(Json(AdminLoginResponse {
            success: true,
            token: Some(token),
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
        "SELECT id, season_id, tournament_id, player_name, access_key_id, email, created_at FROM teams WHERE access_key_id = ? AND tournament_id = ?"
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
             g.is_amateur, g.is_active, g.created_at \
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

    if groups_used.len() != 9 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ApiError::new("Must select exactly one golfer from each of the 9 groups")),
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
        "SELECT g.id, g.name, g.win_probability_group, g.is_amateur, g.is_active, g.created_at \
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

// List teams for a tournament (admin)
pub async fn list_teams_for_tournament(
    State(pool): State<SqlitePool>,
    Path(tournament_id): Path<String>,
) -> Result<Json<Vec<Team>>, (StatusCode, Json<ApiError>)> {
    let teams = sqlx::query_as::<_, Team>(
        "SELECT id, season_id, tournament_id, player_name, access_key_id, email, created_at \
         FROM teams WHERE tournament_id = ? ORDER BY player_name"
    )
    .bind(&tournament_id)
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

    Ok(Json(teams))
}

// Admin update team golfers (bypasses start-date check)
pub async fn admin_update_team_golfers(
    State(pool): State<SqlitePool>,
    Path(team_id): Path<String>,
    Json(payload): Json<AdminUpdateTeamGolfersRequest>,
) -> Result<Json<TeamWithGolfers>, (StatusCode, Json<ApiError>)> {
    payload.validate()
        .map_err(|e| (StatusCode::BAD_REQUEST, Json(ApiError::new(e.to_string()))))?;

    // Validate team exists
    let team = sqlx::query_as::<_, Team>(
        "SELECT id, season_id, tournament_id, player_name, access_key_id, email, created_at FROM teams WHERE id = ?"
    )
    .bind(&team_id)
    .fetch_optional(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?
    .ok_or((StatusCode::NOT_FOUND, Json(ApiError::new("Team not found"))))?;

    // Verify golfer selection (one from each group, using tournament-specific groups)
    let mut groups_used = std::collections::HashSet::new();

    for golfer_id in &payload.golfer_ids {
        let golfer = sqlx::query_as::<_, Golfer>(
            "SELECT g.id, g.name, \
             COALESCE(tgg.win_probability_group, g.win_probability_group) as win_probability_group, \
             g.is_amateur, g.is_active, g.created_at \
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

    if groups_used.len() != 9 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ApiError::new("Must select exactly one golfer from each of the 9 groups")),
        ));
    }

    // Transaction: delete old, insert new
    let mut tx = pool.begin().await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

    sqlx::query("DELETE FROM team_golfers WHERE team_id = ?")
        .bind(&team_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

    for golfer_id in &payload.golfer_ids {
        let tg_id = new_id();
        sqlx::query("INSERT INTO team_golfers (id, team_id, golfer_id) VALUES (?, ?, ?)")
            .bind(&tg_id)
            .bind(&team_id)
            .bind(golfer_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;
    }

    tx.commit().await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

    // Fetch updated team golfers
    let golfers = sqlx::query_as::<_, Golfer>(
        "SELECT g.id, g.name, g.win_probability_group, g.is_amateur, g.is_active, g.created_at \
         FROM golfers g \
         INNER JOIN team_golfers tg ON g.id = tg.golfer_id \
         WHERE tg.team_id = ? \
         ORDER BY g.win_probability_group"
    )
    .bind(&team_id)
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
            is_amateur: bool,
        }

        let golfer = sqlx::query_as::<_, GolferIdRow>(
            "SELECT id, is_amateur FROM golfers WHERE LOWER(name) = LOWER(?)"
        )
        .bind(&entry.golfer)
        .fetch_optional(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

        let (golfer_id, is_amateur) = match golfer {
            Some(g) => (g.id, g.is_amateur),
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
            let fantasy_points = calculate_fantasy_points(score_to_par, is_amateur);
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

// Admin stats
pub async fn get_admin_stats(
    State(pool): State<SqlitePool>,
) -> Result<Json<AdminStats>, (StatusCode, Json<ApiError>)> {
    #[derive(sqlx::FromRow)]
    struct CountRow { count: i64 }

    let total_seasons = sqlx::query_as::<_, CountRow>("SELECT COUNT(*) as count FROM seasons")
        .fetch_one(&pool).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

    let total_tournaments = sqlx::query_as::<_, CountRow>("SELECT COUNT(*) as count FROM tournaments")
        .fetch_one(&pool).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

    let total_teams = sqlx::query_as::<_, CountRow>("SELECT COUNT(*) as count FROM teams")
        .fetch_one(&pool).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

    let total_golfers = sqlx::query_as::<_, CountRow>("SELECT COUNT(*) as count FROM golfers WHERE is_active = 1")
        .fetch_one(&pool).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

    let total_scores = sqlx::query_as::<_, CountRow>("SELECT COUNT(*) as count FROM hole_scores")
        .fetch_one(&pool).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

    let keys_total = sqlx::query_as::<_, CountRow>("SELECT COUNT(*) as count FROM access_keys")
        .fetch_one(&pool).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

    let keys_used = sqlx::query_as::<_, CountRow>("SELECT COUNT(*) as count FROM access_keys WHERE is_used = 1")
        .fetch_one(&pool).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

    // Score distribution
    let eagles = sqlx::query_as::<_, CountRow>("SELECT COUNT(*) as count FROM hole_scores WHERE score_to_par <= -2")
        .fetch_one(&pool).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

    let birdies = sqlx::query_as::<_, CountRow>("SELECT COUNT(*) as count FROM hole_scores WHERE score_to_par = -1")
        .fetch_one(&pool).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

    let pars = sqlx::query_as::<_, CountRow>("SELECT COUNT(*) as count FROM hole_scores WHERE score_to_par = 0")
        .fetch_one(&pool).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

    let bogeys = sqlx::query_as::<_, CountRow>("SELECT COUNT(*) as count FROM hole_scores WHERE score_to_par >= 1")
        .fetch_one(&pool).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

    let score_distribution = ScoreDistribution {
        eagles_or_better: eagles.count,
        birdies: birdies.count,
        pars: pars.count,
        bogeys_or_worse: bogeys.count,
    };

    // Season breakdown
    let season_breakdown = sqlx::query_as::<_, SeasonBreakdown>(
        "SELECT s.name as season_name, s.year as season_year, \
         (SELECT COUNT(*) FROM tournaments t WHERE t.season_id = s.id) as tournament_count, \
         (SELECT COUNT(*) FROM teams tm WHERE tm.season_id = s.id) as team_count, \
         (SELECT COUNT(*) FROM hole_scores hs INNER JOIN tournaments t ON hs.tournament_id = t.id WHERE t.season_id = s.id) as score_count \
         FROM seasons s ORDER BY s.year DESC"
    )
    .fetch_all(&pool).await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

    // Popular golfers (top 10 most selected)
    let popular_golfers = sqlx::query_as::<_, PopularGolfer>(
        "SELECT g.name as golfer_name, COUNT(tg.id) as times_selected \
         FROM golfers g \
         INNER JOIN team_golfers tg ON g.id = tg.golfer_id \
         GROUP BY g.id, g.name \
         ORDER BY times_selected DESC \
         LIMIT 10"
    )
    .fetch_all(&pool).await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

    Ok(Json(AdminStats {
        total_seasons: total_seasons.count,
        total_tournaments: total_tournaments.count,
        total_teams: total_teams.count,
        total_golfers: total_golfers.count,
        total_scores: total_scores.count,
        access_keys_total: keys_total.count,
        access_keys_used: keys_used.count,
        access_keys_unused: keys_total.count - keys_used.count,
        score_distribution,
        season_breakdown,
        popular_golfers,
    }))
}

// Completed tournaments for a season
pub async fn get_completed_tournaments(
    State(pool): State<SqlitePool>,
    Path(season_id): Path<String>,
) -> Result<Json<Vec<CompletedTournament>>, (StatusCode, Json<ApiError>)> {
    let tournaments = sqlx::query_as::<_, CompletedTournament>(
        "SELECT id, name, start_date, end_date FROM tournaments \
         WHERE season_id = ? AND end_date < date('now') \
         ORDER BY start_date DESC"
    )
    .bind(&season_id)
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

    Ok(Json(tournaments))
}

// Team leaderboard for a specific tournament
pub async fn get_tournament_team_leaderboard(
    State(pool): State<SqlitePool>,
    Path(tournament_id): Path<String>,
) -> Result<Json<Vec<TournamentTeamLeaderboardEntryWithGolfers>>, (StatusCode, Json<ApiError>)> {
    let leaderboard = sqlx::query_as::<_, TournamentTeamLeaderboardEntry>(
        "SELECT \
            t.player_name, \
            t.id as team_id, \
            COALESCE(SUM(hs.fantasy_points), 0) as total_points \
         FROM teams t \
         LEFT JOIN team_golfers tg ON t.id = tg.team_id \
         LEFT JOIN hole_scores hs ON tg.golfer_id = hs.golfer_id AND hs.tournament_id = ? \
         WHERE t.tournament_id = ? \
         GROUP BY t.id, t.player_name \
         ORDER BY total_points DESC"
    )
    .bind(&tournament_id)
    .bind(&tournament_id)
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

    // Batch-fetch all team golfers for this tournament
    let team_golfers = sqlx::query_as::<_, TeamGolferRow>(
        "SELECT tg.team_id, g.id, g.name, g.win_probability_group, g.is_amateur \
         FROM team_golfers tg \
         INNER JOIN golfers g ON tg.golfer_id = g.id \
         INNER JOIN teams t ON tg.team_id = t.id \
         WHERE t.tournament_id = ? \
         ORDER BY g.win_probability_group"
    )
    .bind(&tournament_id)
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

    let mut golfers_by_team: std::collections::HashMap<String, Vec<GolferSummary>> = std::collections::HashMap::new();
    for row in team_golfers {
        golfers_by_team.entry(row.team_id).or_default().push(GolferSummary {
            id: row.id,
            name: row.name,
            win_probability_group: row.win_probability_group,
            is_amateur: row.is_amateur,
        });
    }

    let result: Vec<TournamentTeamLeaderboardEntryWithGolfers> = leaderboard
        .into_iter()
        .map(|entry| TournamentTeamLeaderboardEntryWithGolfers {
            player_name: entry.player_name,
            team_id: entry.team_id.clone(),
            total_points: entry.total_points.unwrap_or(0),
            golfers: golfers_by_team.remove(&entry.team_id).unwrap_or_default(),
        })
        .collect();

    Ok(Json(result))
}

// Tournament stats
pub async fn get_tournament_stats(
    State(pool): State<SqlitePool>,
    Path(tournament_id): Path<String>,
) -> Result<Json<TournamentStats>, (StatusCode, Json<ApiError>)> {
    #[derive(sqlx::FromRow)]
    struct StatsRow {
        total_holes_played: i64,
        total_fantasy_points: i64,
        eagles_or_better: i64,
        birdies: i64,
        pars: i64,
        bogeys_or_worse: i64,
    }

    let stats = sqlx::query_as::<_, StatsRow>(
        "SELECT \
            COUNT(*) as total_holes_played, \
            COALESCE(SUM(fantasy_points), 0) as total_fantasy_points, \
            SUM(CASE WHEN score_to_par <= -2 THEN 1 ELSE 0 END) as eagles_or_better, \
            SUM(CASE WHEN score_to_par = -1 THEN 1 ELSE 0 END) as birdies, \
            SUM(CASE WHEN score_to_par = 0 THEN 1 ELSE 0 END) as pars, \
            SUM(CASE WHEN score_to_par >= 1 THEN 1 ELSE 0 END) as bogeys_or_worse \
         FROM hole_scores WHERE tournament_id = ?"
    )
    .bind(&tournament_id)
    .fetch_one(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

    #[derive(sqlx::FromRow)]
    struct BestRoundRow {
        golfer_name: String,
        round_points: i64,
    }

    let best_round = sqlx::query_as::<_, BestRoundRow>(
        "SELECT g.name as golfer_name, SUM(hs.fantasy_points) as round_points \
         FROM hole_scores hs \
         INNER JOIN golfers g ON hs.golfer_id = g.id \
         WHERE hs.tournament_id = ? \
         GROUP BY hs.golfer_id, hs.day \
         ORDER BY round_points DESC \
         LIMIT 1"
    )
    .bind(&tournament_id)
    .fetch_optional(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

    Ok(Json(TournamentStats {
        total_holes_played: stats.total_holes_played,
        total_fantasy_points: stats.total_fantasy_points,
        eagles_or_better: stats.eagles_or_better,
        birdies: stats.birdies,
        pars: stats.pars,
        bogeys_or_worse: stats.bogeys_or_worse,
        best_round_golfer: best_round.as_ref().map(|r| r.golfer_name.clone()),
        best_round_points: best_round.map(|r| r.round_points),
    }))
}
