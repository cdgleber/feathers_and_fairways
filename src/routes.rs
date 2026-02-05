use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

use crate::models::*;
use crate::db::generate_access_key;

// Season routes
pub async fn create_season(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateSeasonRequest>,
) -> Result<(StatusCode, Json<Season>), (StatusCode, Json<ApiError>)> {
    payload.validate()
        .map_err(|e| (StatusCode::BAD_REQUEST, Json(ApiError::new(e.to_string()))))?;

    // Deactivate all other seasons
    sqlx::query!("UPDATE seasons SET is_active = false")
        .execute(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

    let season = sqlx::query_as!(
        Season,
        r#"
        INSERT INTO seasons (name, year, start_date, end_date, is_active)
        VALUES ($1, $2, $3, $4, true)
        RETURNING id, name, year, start_date, end_date, is_active, created_at
        "#,
        payload.name,
        payload.year,
        payload.start_date,
        payload.end_date
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

    Ok((StatusCode::CREATED, Json(season)))
}

pub async fn list_seasons(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<Season>>, (StatusCode, Json<ApiError>)> {
    let seasons = sqlx::query_as!(
        Season,
        "SELECT id, name, year, start_date, end_date, is_active, created_at FROM seasons ORDER BY year DESC"
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

    Ok(Json(seasons))
}

pub async fn get_active_season(
    State(pool): State<PgPool>,
) -> Result<Json<Season>, (StatusCode, Json<ApiError>)> {
    let season = sqlx::query_as!(
        Season,
        "SELECT id, name, year, start_date, end_date, is_active, created_at FROM seasons WHERE is_active = true"
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?
    .ok_or((StatusCode::NOT_FOUND, Json(ApiError::new("No active season found"))))?;

    Ok(Json(season))
}

// Access key routes
pub async fn create_access_keys(
    State(pool): State<PgPool>,
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
        let key_code = generate_access_key();
        
        let access_key = sqlx::query_as!(
            AccessKey,
            r#"
            INSERT INTO access_keys (key_code, season_id)
            VALUES ($1, $2)
            RETURNING id, key_code, season_id, player_name, is_used, used_at, created_at
            "#,
            key_code,
            payload.season_id
        )
        .fetch_one(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;
        
        keys.push(access_key);
    }

    Ok((StatusCode::CREATED, Json(keys)))
}

pub async fn validate_access_key(
    State(pool): State<PgPool>,
    Json(payload): Json<ValidateAccessKeyRequest>,
) -> Result<Json<AccessKeyValidationResponse>, (StatusCode, Json<ApiError>)> {
    let key = sqlx::query_as!(
        AccessKey,
        "SELECT id, key_code, season_id, player_name, is_used, used_at, created_at FROM access_keys WHERE key_code = $1",
        payload.key_code
    )
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
    State(pool): State<PgPool>,
    Json(payload): Json<CreateGolferRequest>,
) -> Result<(StatusCode, Json<Golfer>), (StatusCode, Json<ApiError>)> {
    payload.validate()
        .map_err(|e| (StatusCode::BAD_REQUEST, Json(ApiError::new(e.to_string()))))?;

    let golfer = sqlx::query_as!(
        Golfer,
        r#"
        INSERT INTO golfers (name, win_probability_group)
        VALUES ($1, $2)
        RETURNING id, name, win_probability_group, is_active, created_at
        "#,
        payload.name,
        payload.win_probability_group
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

    Ok((StatusCode::CREATED, Json(golfer)))
}

pub async fn list_golfers(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<Golfer>>, (StatusCode, Json<ApiError>)> {
    let golfers = sqlx::query_as!(
        Golfer,
        "SELECT id, name, win_probability_group, is_active, created_at FROM golfers WHERE is_active = true ORDER BY win_probability_group, name"
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

    Ok(Json(golfers))
}

// Team routes
pub async fn create_team(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateTeamRequest>,
) -> Result<(StatusCode, Json<TeamWithGolfers>), (StatusCode, Json<ApiError>)> {
    payload.validate()
        .map_err(|e| (StatusCode::BAD_REQUEST, Json(ApiError::new(e.to_string()))))?;

    // Validate access key
    let access_key = sqlx::query_as!(
        AccessKey,
        "SELECT id, key_code, season_id, player_name, is_used, used_at, created_at FROM access_keys WHERE key_code = $1",
        payload.key_code
    )
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

    // Verify golfer selection (one from each group)
    let mut groups_used = std::collections::HashSet::new();
    
    for golfer_id in &payload.golfer_ids {
        let golfer = sqlx::query_as!(
            Golfer,
            "SELECT id, name, win_probability_group, is_active, created_at FROM golfers WHERE id = $1",
            golfer_id
        )
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
    let team = sqlx::query_as!(
        Team,
        r#"
        INSERT INTO teams (season_id, player_name, access_key_id)
        VALUES ($1, $2, $3)
        RETURNING id, season_id, player_name, access_key_id, created_at
        "#,
        access_key.season_id,
        payload.player_name,
        access_key.id
    )
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

    // Add golfers to team
    for golfer_id in &payload.golfer_ids {
        sqlx::query!(
            "INSERT INTO team_golfers (team_id, golfer_id) VALUES ($1, $2)",
            team.id,
            golfer_id
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;
    }

    // Mark access key as used
    sqlx::query!(
        "UPDATE access_keys SET is_used = true, used_at = CURRENT_TIMESTAMP, player_name = $1 WHERE id = $2",
        payload.player_name,
        access_key.id
    )
    .execute(&mut *tx)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

    tx.commit().await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

    // Fetch team golfers
    let golfers = sqlx::query_as!(
        Golfer,
        r#"
        SELECT g.id, g.name, g.win_probability_group, g.is_active, g.created_at
        FROM golfers g
        INNER JOIN team_golfers tg ON g.id = tg.golfer_id
        WHERE tg.team_id = $1
        ORDER BY g.win_probability_group
        "#,
        team.id
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

    Ok((StatusCode::CREATED, Json(TeamWithGolfers { team, golfers })))
}

pub async fn list_teams(
    State(pool): State<PgPool>,
    Path(season_id): Path<Uuid>,
) -> Result<Json<Vec<Team>>, (StatusCode, Json<ApiError>)> {
    let teams = sqlx::query_as!(
        Team,
        "SELECT id, season_id, player_name, access_key_id, created_at FROM teams WHERE season_id = $1 ORDER BY player_name",
        season_id
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

    Ok(Json(teams))
}

pub async fn get_team_golfers(
    State(pool): State<PgPool>,
    Path(team_id): Path<Uuid>,
) -> Result<Json<Vec<Golfer>>, (StatusCode, Json<ApiError>)> {
    let golfers = sqlx::query_as!(
        Golfer,
        r#"
        SELECT g.id, g.name, g.win_probability_group, g.is_active, g.created_at
        FROM golfers g
        INNER JOIN team_golfers tg ON g.id = tg.golfer_id
        WHERE tg.team_id = $1
        ORDER BY g.win_probability_group
        "#,
        team_id
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

    Ok(Json(golfers))
}

// Tournament routes
pub async fn create_tournament(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateTournamentRequest>,
) -> Result<(StatusCode, Json<Tournament>), (StatusCode, Json<ApiError>)> {
    payload.validate()
        .map_err(|e| (StatusCode::BAD_REQUEST, Json(ApiError::new(e.to_string()))))?;

    sqlx::query!(
        "UPDATE tournaments SET is_active = false WHERE season_id = $1",
        payload.season_id
    )
    .execute(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

    let tournament = sqlx::query_as!(
        Tournament,
        r#"
        INSERT INTO tournaments (season_id, name, start_date, end_date, is_active)
        VALUES ($1, $2, $3, $4, true)
        RETURNING id, season_id, name, start_date, end_date, is_active, created_at
        "#,
        payload.season_id,
        payload.name,
        payload.start_date,
        payload.end_date
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

    Ok((StatusCode::CREATED, Json(tournament)))
}

pub async fn list_tournaments(
    State(pool): State<PgPool>,
    Path(season_id): Path<Uuid>,
) -> Result<Json<Vec<Tournament>>, (StatusCode, Json<ApiError>)> {
    let tournaments = sqlx::query_as!(
        Tournament,
        "SELECT id, season_id, name, start_date, end_date, is_active, created_at FROM tournaments WHERE season_id = $1 ORDER BY start_date DESC",
        season_id
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

    Ok(Json(tournaments))
}

pub async fn add_hole_scores(
    State(pool): State<PgPool>,
    Json(payload): Json<AddHoleScoresRequest>,
) -> Result<(StatusCode, Json<Vec<HoleScore>>), (StatusCode, Json<ApiError>)> {
    payload.validate()
        .map_err(|e| (StatusCode::BAD_REQUEST, Json(ApiError::new(e.to_string()))))?;

    let mut scores = Vec::new();

    for score_input in payload.scores {
        score_input.validate()
            .map_err(|e| (StatusCode::BAD_REQUEST, Json(ApiError::new(e.to_string()))))?;

        let score = sqlx::query_as!(
            HoleScore,
            r#"
            INSERT INTO hole_scores (tournament_id, golfer_id, day, hole, strokes, score_to_par, fantasy_points)
            VALUES ($1, $2, $3, $4, $5, $6, 0)
            ON CONFLICT (tournament_id, golfer_id, day, hole)
            DO UPDATE SET strokes = $5, score_to_par = $6
            RETURNING id, tournament_id, golfer_id, day, hole, strokes, score_to_par, fantasy_points, created_at
            "#,
            payload.tournament_id,
            score_input.golfer_id,
            score_input.day,
            score_input.hole,
            score_input.strokes,
            score_input.score_to_par
        )
        .fetch_one(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

        scores.push(score);
    }

    Ok((StatusCode::CREATED, Json(scores)))
}

pub async fn get_tournament_scores(
    State(pool): State<PgPool>,
    Path(tournament_id): Path<Uuid>,
) -> Result<Json<Vec<HoleScore>>, (StatusCode, Json<ApiError>)> {
    let scores = sqlx::query_as!(
        HoleScore,
        "SELECT id, tournament_id, golfer_id, day, hole, strokes, score_to_par, fantasy_points, created_at FROM hole_scores WHERE tournament_id = $1 ORDER BY day, hole, golfer_id",
        tournament_id
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

    Ok(Json(scores))
}

pub async fn get_season_leaderboard(
    State(pool): State<PgPool>,
    Path(season_id): Path<Uuid>,
) -> Result<Json<Vec<LeaderboardEntry>>, (StatusCode, Json<ApiError>)> {
    let leaderboard = sqlx::query_as!(
        LeaderboardEntry,
        r#"
        SELECT 
            t.player_name,
            t.id as team_id,
            COALESCE(SUM(hs.fantasy_points), 0) as "total_points"
        FROM teams t
        LEFT JOIN team_golfers tg ON t.id = tg.team_id
        LEFT JOIN hole_scores hs ON tg.golfer_id = hs.golfer_id
        LEFT JOIN tournaments tour ON hs.tournament_id = tour.id AND tour.season_id = $1
        WHERE t.season_id = $1
        GROUP BY t.id, t.player_name
        ORDER BY total_points DESC
        "#,
        season_id
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

    Ok(Json(leaderboard))
}

pub async fn get_tournament_leaderboard(
    State(pool): State<PgPool>,
    Path(tournament_id): Path<Uuid>,
) -> Result<Json<Vec<TournamentScore>>, (StatusCode, Json<ApiError>)> {
    let leaderboard = sqlx::query_as!(
        TournamentScore,
        r#"
        SELECT 
            g.name as golfer_name,
            g.id as golfer_id,
            COALESCE(SUM(hs.fantasy_points), 0) as "total_points"
        FROM golfers g
        LEFT JOIN hole_scores hs ON g.id = hs.golfer_id AND hs.tournament_id = $1
        GROUP BY g.id, g.name
        HAVING COALESCE(SUM(hs.fantasy_points), 0) > 0
        ORDER BY total_points DESC
        "#,
        tournament_id
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

    Ok(Json(leaderboard))
}