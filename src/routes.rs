use axum::{
    extract::{Path, Query, State},
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
    let points = match score_to_par {
        s if s <= -3 => 8,   // Better than eagle (albatross or better)
        -2 => 5,             // Eagle
        -1 => 2,             // Birdie
        0 => 1,              // Par
        1 => -1,             // Bogey
        _ => -3,             // Double bogey or worse
    };
    if is_amateur && points < 0 { 0 } else { points }
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

    // Verify tournament exists
    let exists = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM tournaments WHERE id = ?")
        .bind(&payload.tournament_id)
        .fetch_one(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

    if exists == 0 {
        return Err((StatusCode::NOT_FOUND, Json(ApiError::new("Tournament not found"))));
    }

    let mut keys = Vec::new();

    for _ in 0..payload.count {
        let id = new_id();
        let key_code = generate_access_key();

        sqlx::query(
            "INSERT INTO access_keys (id, key_code, tournament_id) VALUES (?, ?, ?)"
        )
        .bind(&id)
        .bind(&key_code)
        .bind(&payload.tournament_id)
        .execute(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

        let access_key = sqlx::query_as::<_, AccessKey>(
            "SELECT id, key_code, tournament_id, player_name, is_used, used_at, created_at FROM access_keys WHERE id = ?"
        )
        .bind(&id)
        .fetch_one(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

        keys.push(access_key);
    }

    Ok((StatusCode::CREATED, Json(keys)))
}

pub async fn list_access_keys(
    State(pool): State<SqlitePool>,
) -> Result<Json<Vec<AccessKeyDetail>>, (StatusCode, Json<ApiError>)> {
    let keys = sqlx::query_as::<_, AccessKeyDetail>(
        "SELECT ak.id, ak.key_code, ak.is_used, ak.created_at, \
         t.name as tournament_name, te.player_name as team_name \
         FROM access_keys ak \
         JOIN tournaments t ON ak.tournament_id = t.id \
         LEFT JOIN teams te ON te.access_key_id = ak.id \
         ORDER BY ak.created_at DESC"
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

    Ok(Json(keys))
}

pub async fn validate_access_key(
    State(pool): State<SqlitePool>,
    Json(payload): Json<ValidateAccessKeyRequest>,
) -> Result<Json<AccessKeyValidationResponse>, (StatusCode, Json<ApiError>)> {
    let key = sqlx::query_as::<_, AccessKey>(
        "SELECT id, key_code, tournament_id, player_name, is_used, used_at, created_at FROM access_keys WHERE key_code = ?"
    )
    .bind(&payload.key_code)
    .fetch_optional(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

    match key {
        Some(k) => {
            // When the key is already used, look up the team so the frontend can load it directly
            let team_id = if k.is_used {
                #[derive(sqlx::FromRow)]
                struct TeamIdRow { id: String }
                sqlx::query_as::<_, TeamIdRow>(
                    "SELECT id FROM teams WHERE access_key_id = ? AND tournament_id = ?"
                )
                .bind(&k.id)
                .bind(&k.tournament_id)
                .fetch_optional(&pool)
                .await
                .ok()
                .flatten()
                .map(|r| r.id)
            } else {
                None
            };

            Ok(Json(AccessKeyValidationResponse {
                valid: true,
                tournament_id: Some(k.tournament_id),
                already_used: k.is_used,
                team_id,
            }))
        }
        None => Ok(Json(AccessKeyValidationResponse {
            valid: false,
            tournament_id: None,
            already_used: false,
            team_id: None,
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
        "SELECT id, name, win_probability_group, is_amateur, is_active, espn_id, created_at FROM golfers WHERE id = ?"
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
        "SELECT id, name, win_probability_group, is_amateur, is_active, espn_id, created_at FROM golfers WHERE is_active = 1 ORDER BY win_probability_group, name"
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
         tgg.win_probability_group, \
         g.is_amateur, g.is_active, g.espn_id, g.created_at \
         FROM golfers g \
         JOIN tournament_golfer_groups tgg ON g.id = tgg.golfer_id AND tgg.tournament_id = ? \
         WHERE g.is_active = 1 \
         ORDER BY tgg.win_probability_group, g.name"
    )
    .bind(&tournament_id)
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

    Ok(Json(golfers))
}

// Update golfer amateur status
pub async fn update_golfer_amateur(
    State(pool): State<SqlitePool>,
    Path(golfer_id): Path<String>,
    Json(payload): Json<UpdateAmateurRequest>,
) -> Result<Json<Golfer>, (StatusCode, Json<ApiError>)> {
    sqlx::query("UPDATE golfers SET is_amateur = ? WHERE id = ?")
        .bind(payload.is_amateur)
        .bind(&golfer_id)
        .execute(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

    let golfer = sqlx::query_as::<_, Golfer>(
        "SELECT id, name, win_probability_group, is_amateur, is_active, espn_id, created_at FROM golfers WHERE id = ?"
    )
    .bind(&golfer_id)
    .fetch_optional(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?
    .ok_or((StatusCode::NOT_FOUND, Json(ApiError::new("Golfer not found"))))?;

    Ok(Json(golfer))
}

// Paste golfers from a list of names
pub async fn paste_golfers(
    State(pool): State<SqlitePool>,
    Json(payload): Json<PasteGolfersRequest>,
) -> Result<Json<PasteGolfersResponse>, (StatusCode, Json<ApiError>)> {
    let is_amateur = payload.is_amateur.unwrap_or(false);
    let mut results = Vec::new();
    let mut errors = Vec::new();

    for raw_name in &payload.names {
        let name = raw_name.trim();
        if name.is_empty() {
            continue;
        }

        #[derive(sqlx::FromRow)]
        struct ExistsRow { id: String }

        let existing = sqlx::query_as::<_, ExistsRow>(
            "SELECT id FROM golfers WHERE LOWER(name) = LOWER(?)"
        )
        .bind(name)
        .fetch_optional(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

        if let Some(row) = existing {
            results.push(PasteGolferResult {
                name: name.to_string(),
                id: row.id,
                created: false,
            });
        } else {
            let id = new_id();
            match sqlx::query(
                "INSERT INTO golfers (id, name, win_probability_group, is_amateur) VALUES (?, ?, 5, ?)"
            )
            .bind(&id)
            .bind(name)
            .bind(is_amateur)
            .execute(&pool)
            .await {
                Ok(_) => {
                    results.push(PasteGolferResult {
                        name: name.to_string(),
                        id,
                        created: true,
                    });
                }
                Err(e) => {
                    errors.push(format!("{}: {}", name, e));
                }
            }
        }
    }

    Ok(Json(PasteGolfersResponse { results, errors }))
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
        "SELECT id, key_code, tournament_id, player_name, is_used, used_at, created_at FROM access_keys WHERE key_code = ?"
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
             tgg.win_probability_group, \
             g.is_amateur, g.is_active, g.espn_id, g.created_at \
             FROM golfers g \
             JOIN tournament_golfer_groups tgg ON g.id = tgg.golfer_id AND tgg.tournament_id = ? \
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
        "INSERT INTO teams (id, tournament_id, player_name, access_key_id, email) VALUES (?, ?, ?, ?, ?)"
    )
    .bind(&team_id)
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
        "SELECT id, tournament_id, player_name, access_key_id, email, created_at FROM teams WHERE id = ?"
    )
    .bind(&team_id)
    .fetch_one(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

    // Fetch team golfers
    let golfers = sqlx::query_as::<_, Golfer>(
        "SELECT g.id, g.name, g.win_probability_group, g.is_amateur, g.is_active, g.espn_id, g.created_at \
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

// List teams by tournament_id query param (public)
pub async fn list_teams_by_tournament(
    State(pool): State<SqlitePool>,
    Query(params): Query<TournamentIdQuery>,
) -> Result<Json<Vec<Team>>, (StatusCode, Json<ApiError>)> {
    let tournament_id = params.tournament_id.ok_or((
        StatusCode::BAD_REQUEST,
        Json(ApiError::new("tournament_id query parameter is required")),
    ))?;

    let teams = sqlx::query_as::<_, Team>(
        "SELECT id, tournament_id, player_name, access_key_id, email, created_at FROM teams WHERE tournament_id = ? ORDER BY player_name"
    )
    .bind(&tournament_id)
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
        "SELECT g.id, g.name, g.win_probability_group, g.is_amateur, g.is_active, g.espn_id, g.created_at \
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

    // Deactivate all other tournaments globally
    sqlx::query("UPDATE tournaments SET is_active = 0")
        .execute(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

    let id = new_id();
    sqlx::query(
        "INSERT INTO tournaments (id, name, start_date, end_date, is_active) VALUES (?, ?, ?, ?, 1)"
    )
    .bind(&id)
    .bind(&payload.name)
    .bind(&payload.start_date)
    .bind(&payload.end_date)
    .execute(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

    let tournament = sqlx::query_as::<_, Tournament>(
        "SELECT id, name, start_date, end_date, is_active, espn_tournament_id, created_at FROM tournaments WHERE id = ?"
    )
    .bind(&id)
    .fetch_one(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

    Ok((StatusCode::CREATED, Json(tournament)))
}

pub async fn list_all_tournaments(
    State(pool): State<SqlitePool>,
) -> Result<Json<Vec<Tournament>>, (StatusCode, Json<ApiError>)> {
    let tournaments = sqlx::query_as::<_, Tournament>(
        "SELECT id, name, start_date, end_date, is_active, espn_tournament_id, created_at FROM tournaments ORDER BY start_date DESC"
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

    Ok(Json(tournaments))
}

pub async fn list_completed_tournaments(
    State(pool): State<SqlitePool>,
) -> Result<Json<Vec<CompletedTournament>>, (StatusCode, Json<ApiError>)> {
    let tournaments = sqlx::query_as::<_, CompletedTournament>(
        "SELECT id, name, start_date, end_date FROM tournaments \
         WHERE end_date < date('now') \
         ORDER BY start_date DESC"
    )
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
         INNER JOIN (SELECT DISTINCT tg.golfer_id FROM team_golfers tg \
            INNER JOIN teams t ON tg.team_id = t.id AND t.tournament_id = ?) dt \
            ON g.id = dt.golfer_id \
         LEFT JOIN hole_scores hs ON g.id = hs.golfer_id AND hs.tournament_id = ? \
         GROUP BY g.id, g.name \
         ORDER BY total_points DESC"
    )
    .bind(&tournament_id)
    .bind(&tournament_id)
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

    Ok(Json(leaderboard))
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
        "SELECT id, key_code, tournament_id, player_name, is_used, used_at, created_at FROM access_keys WHERE key_code = ?"
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
        "SELECT id, tournament_id, player_name, access_key_id, email, created_at FROM teams WHERE access_key_id = ? AND tournament_id = ?"
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
             tgg.win_probability_group, \
             g.is_amateur, g.is_active, g.espn_id, g.created_at \
             FROM golfers g \
             JOIN tournament_golfer_groups tgg ON g.id = tgg.golfer_id AND tgg.tournament_id = ? \
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
        "SELECT g.id, g.name, g.win_probability_group, g.is_amateur, g.is_active, g.espn_id, g.created_at \
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
        "SELECT id, tournament_id, player_name, access_key_id, email, created_at \
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
        "SELECT id, tournament_id, player_name, access_key_id, email, created_at FROM teams WHERE id = ?"
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
             tgg.win_probability_group, \
             g.is_amateur, g.is_active, g.espn_id, g.created_at \
             FROM golfers g \
             JOIN tournament_golfer_groups tgg ON g.id = tgg.golfer_id AND tgg.tournament_id = ? \
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
        "SELECT g.id, g.name, g.win_probability_group, g.is_amateur, g.is_active, g.espn_id, g.created_at \
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

// Admin stats
pub async fn get_admin_stats(
    State(pool): State<SqlitePool>,
) -> Result<Json<AdminStats>, (StatusCode, Json<ApiError>)> {
    #[derive(sqlx::FromRow)]
    struct CountRow { count: i64 }

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
        total_tournaments: total_tournaments.count,
        total_teams: total_teams.count,
        total_golfers: total_golfers.count,
        total_scores: total_scores.count,
        access_keys_total: keys_total.count,
        access_keys_used: keys_used.count,
        access_keys_unused: keys_total.count - keys_used.count,
        score_distribution,
        popular_golfers,
    }))
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
        "SELECT tg.team_id, g.id, g.name, \
         COALESCE(tgg.win_probability_group, g.win_probability_group) as win_probability_group, \
         g.is_amateur \
         FROM team_golfers tg \
         INNER JOIN golfers g ON tg.golfer_id = g.id \
         INNER JOIN teams t ON tg.team_id = t.id \
         LEFT JOIN tournament_golfer_groups tgg ON tgg.golfer_id = g.id AND tgg.tournament_id = t.tournament_id \
         WHERE t.tournament_id = ? \
         ORDER BY COALESCE(tgg.win_probability_group, g.win_probability_group)"
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

// Tournament Import - Preview
struct PlayerForPreview {
    name: String,
    slug: String,
    rounds_available: Vec<i32>,
}

async fn build_import_preview(
    pool: &SqlitePool,
    tournament_name: String,
    players: Vec<PlayerForPreview>,
) -> Result<ImportPreviewResponse, (StatusCode, Json<ApiError>)> {
    let mut matched = Vec::new();
    let mut unmatched = Vec::new();

    for player in &players {
        // Extract last name (text before first space, e.g. "McIlroy" from "McIlroy R.")
        let last_name = player.name.split_whitespace().next().unwrap_or(&player.name);

        #[derive(sqlx::FromRow)]
        struct GolferMatchRow {
            id: String,
            name: String,
            is_amateur: bool,
        }

        let candidates = sqlx::query_as::<_, GolferMatchRow>(
            "SELECT id, name, is_amateur FROM golfers WHERE LOWER(name) LIKE '%' || LOWER(?) || '%' AND is_active = 1"
        )
        .bind(last_name)
        .fetch_all(pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

        if candidates.len() == 1 {
            matched.push(ImportMatchedGolfer {
                json_name: player.name.clone(),
                slug: player.slug.clone(),
                golfer_id: candidates[0].id.clone(),
                golfer_name: candidates[0].name.clone(),
                is_amateur: candidates[0].is_amateur,
                rounds_available: player.rounds_available.clone(),
            });
        } else {
            unmatched.push(ImportUnmatchedGolfer {
                json_name: player.name.clone(),
                slug: player.slug.clone(),
                candidates: candidates.iter().map(|c| ImportGolferCandidate {
                    golfer_id: c.id.clone(),
                    golfer_name: c.name.clone(),
                }).collect(),
                rounds_available: player.rounds_available.clone(),
            });
        }
    }

    Ok(ImportPreviewResponse {
        tournament_name,
        matched,
        unmatched,
    })
}

pub async fn import_preview(
    State(pool): State<SqlitePool>,
    Json(payload): Json<ImportTournamentJson>,
) -> Result<Json<ImportPreviewResponse>, (StatusCode, Json<ApiError>)> {
    let players: Vec<PlayerForPreview> = payload.players.iter().map(|p| PlayerForPreview {
        name: p.name.clone(),
        slug: p.slug.clone(),
        rounds_available: p.rounds.iter().map(|r| r.round_number).collect(),
    }).collect();

    let preview = build_import_preview(&pool, payload.tournament.name.clone(), players).await?;
    Ok(Json(preview))
}

async fn fetch_espn_competitor(
    client: &reqwest::Client,
    competitor_url: &str,
) -> Result<EspnPlayerData, String> {
    // Fetch the competitor object (contains $ref links to athlete, linescores, etc.)
    let competitor: EspnCompetitor = client.get(competitor_url).send().await
        .map_err(|e| format!("competitor fetch: {}", e))?
        .json().await
        .map_err(|e| format!("competitor parse: {}", e))?;

    // Fetch athlete info
    let athlete_ref = competitor.athlete
        .ok_or_else(|| "no athlete ref".to_string())?;
    let athlete: EspnAthlete = client.get(&athlete_ref.href).send().await
        .map_err(|e| format!("athlete fetch: {}", e))?
        .json().await
        .map_err(|e| format!("athlete parse: {}", e))?;

    let espn_athlete_id = athlete.id.clone();

    let display_name = athlete.display_name
        .or(athlete.short_name)
        .unwrap_or_else(|| {
            format!("{} {}",
                athlete.first_name.as_deref().unwrap_or(""),
                athlete.last_name.as_deref().unwrap_or("")
            ).trim().to_string()
        });

    // Build a slug from the name (lowercase, spaces to hyphens)
    let slug = display_name.to_lowercase().replace(' ', "-");

    // Fetch linescores
    let linescores_ref = match competitor.linescores {
        Some(r) => r,
        None => return Ok(EspnPlayerData { display_name, slug, espn_athlete_id, rounds: vec![] }),
    };

    let linescores_response = client.get(&linescores_ref.href).send().await
        .map_err(|e| format!("linescores fetch: {}", e))?;
    let linescores_text = linescores_response.text().await
        .map_err(|e| format!("linescores read: {}", e))?;

    let linescores: EspnLinescores = match serde_json::from_str(&linescores_text) {
        Ok(ls) => ls,
        Err(e) => {
            let truncated = if linescores_text.len() > 200 { &linescores_text[..200] } else { &linescores_text };
            tracing::warn!("ESPN: linescores parse failed for {}: {} — body: {}", display_name, e, truncated);
            return Ok(EspnPlayerData { display_name, slug, espn_athlete_id, rounds: vec![] });
        }
    };

    // Transform ESPN rounds into ImportRound format
    let mut rounds = Vec::new();
    for espn_round in &linescores.items {
        let round_period = match espn_round.period {
            Some(p) if p >= 1 && p <= 4 => p,
            _ => continue,
        };

        let hole_scores = match &espn_round.linescores {
            Some(ls) => ls,
            None => continue,
        };

        // Skip rounds with no valid hole data
        if hole_scores.is_empty() {
            continue;
        }

        let holes: Vec<ImportHole> = hole_scores.iter()
            .filter_map(|h| {
                let par = h.par.map(|v| v as i32)?;
                let score = h.value.map(|v| v as i32)?;
                let hole_num = h.period?;
                Some(ImportHole {
                    hole: hole_num,
                    par,
                    score,
                })
            })
            .collect();

        if !holes.is_empty() {
            rounds.push(ImportRound {
                round_number: round_period,
                holes,
            });
        }
    }

    Ok(EspnPlayerData { display_name, slug, espn_athlete_id, rounds })
}

// Fetch ESPN field for a tournament and auto-assign groups
pub async fn fetch_espn_field(
    State(pool): State<SqlitePool>,
    Path(tournament_id): Path<String>,
    Json(payload): Json<EspnImportRequest>,
) -> Result<Json<EspnFieldPreviewResponse>, (StatusCode, Json<ApiError>)> {
    let espn_id = &payload.espn_tournament_id;
    let client = reqwest::Client::new();

    // Fetch all competitor pages (no linescores needed, just athlete info)
    let mut all_competitor_refs: Vec<(String, Option<i64>)> = Vec::new();
    let mut page = 1;
    loop {
        let comp_url = format!(
            "https://sports.core.api.espn.com/v2/sports/golf/leagues/pga/events/{}/competitions/{}/competitors?lang=en&region=us&page={}",
            espn_id, espn_id, page
        );
        let comp_page: EspnCompetitorPage = client.get(&comp_url).send().await
            .map_err(|e| (StatusCode::BAD_GATEWAY, Json(ApiError::new(format!("ESPN request failed: {}", e)))))?
            .json().await
            .map_err(|e| (StatusCode::BAD_GATEWAY, Json(ApiError::new(format!("ESPN parse error: {}", e)))))?;

        for item in &comp_page.items {
            all_competitor_refs.push((item.href.clone(), item.order));
        }

        if page >= comp_page.page_count {
            break;
        }
        page += 1;
    }

    tracing::info!("ESPN field: Found {} competitors for event {}", all_competitor_refs.len(), espn_id);

    // Fetch athlete info for each competitor concurrently (no linescores)
    let semaphore = std::sync::Arc::new(tokio::sync::Semaphore::new(10));
    let client = std::sync::Arc::new(client);
    let mut handles = Vec::new();

    for (orig_idx, (comp_ref, page_order)) in all_competitor_refs.into_iter().enumerate() {
        let sem = semaphore.clone();
        let client = client.clone();
        let handle = tokio::spawn(async move {
            let _permit = sem.acquire().await.unwrap();
            // Fetch competitor to get athlete ref, then fetch athlete
            let result: Result<(String, Option<String>, Option<i64>, usize), String> = async {
                let competitor: EspnCompetitor = client.get(&comp_ref).send().await
                    .map_err(|e| format!("competitor fetch: {}", e))?
                    .json().await
                    .map_err(|e| format!("competitor parse: {}", e))?;

                // Prefer sortOrder from competitor object, then page order, then original index
                let ranking = competitor.sort_order
                    .or(page_order);

                let athlete_ref = competitor.athlete
                    .ok_or_else(|| "no athlete ref".to_string())?;
                let athlete: EspnAthlete = client.get(&athlete_ref.href).send().await
                    .map_err(|e| format!("athlete fetch: {}", e))?
                    .json().await
                    .map_err(|e| format!("athlete parse: {}", e))?;

                let espn_id = athlete.id.clone();
                let name = athlete.display_name
                    .or(athlete.short_name)
                    .unwrap_or_else(|| {
                        format!("{} {}",
                            athlete.first_name.as_deref().unwrap_or(""),
                            athlete.last_name.as_deref().unwrap_or("")
                        ).trim().to_string()
                    });
                Ok((name, espn_id, ranking, orig_idx))
            }.await;
            result
        });
        handles.push(handle);
    }

    // Collect field; sort afterward so ordering is deterministic
    let mut field: Vec<(String, Option<String>, Option<i64>, usize)> = Vec::new();
    for handle in handles {
        match handle.await {
            Ok(Ok(data)) => field.push(data),
            Ok(Err(e)) => tracing::warn!("ESPN field: Failed to fetch competitor: {}", e),
            Err(e) => tracing::warn!("ESPN field: Task join error: {}", e),
        }
    }

    let total = field.len();
    if total == 0 {
        return Err((StatusCode::BAD_GATEWAY, Json(ApiError::new("No competitors found in ESPN field"))));
    }

    // Sort by ranking ascending (best = lowest number = Group 1).
    // Uses ESPN page order field, falling back to original page position.
    field.sort_by(|a, b| match (&a.2, &b.2) {
        (Some(ra), Some(rb)) => ra.cmp(rb),
        (Some(_), None) => std::cmp::Ordering::Less,
        (None, Some(_)) => std::cmp::Ordering::Greater,
        (None, None) => a.3.cmp(&b.3),
    });

    // Assign groups using equal-count binning (quantiles).
    // Group 1 = best-ranked players, Group 9 = worst-ranked.
    // Each group gets roughly total/9 players for balanced team selection.
    let mut groups: Vec<EspnFieldGroup> = (1..=9).map(|g| EspnFieldGroup { group: g, golfers: Vec::new() }).collect();

    for (idx, (name, espn_athlete_id, _, _)) in field.into_iter().enumerate() {
        let group_idx = (idx * 9 / total).min(8);

        // Upsert golfer: find by ESPN ID or name, create if not found
        #[derive(sqlx::FromRow)]
        struct GolferRow { id: String }

        let existing = if let Some(ref eid) = espn_athlete_id {
            sqlx::query_as::<_, GolferRow>("SELECT id FROM golfers WHERE espn_id = ?")
                .bind(eid)
                .fetch_optional(&pool)
                .await
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?
        } else {
            None
        };

        let (golfer_id, created) = if let Some(row) = existing {
            (row.id, false)
        } else {
            // Try to find by name
            let by_name = sqlx::query_as::<_, GolferRow>(
                "SELECT id FROM golfers WHERE LOWER(name) = LOWER(?)"
            )
            .bind(&name)
            .fetch_optional(&pool)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

            if let Some(row) = by_name {
                // Update ESPN ID if we have one
                if let Some(ref eid) = espn_athlete_id {
                    sqlx::query("UPDATE golfers SET espn_id = ? WHERE id = ? AND espn_id IS NULL")
                        .bind(eid)
                        .bind(&row.id)
                        .execute(&pool)
                        .await
                        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;
                }
                (row.id, false)
            } else {
                // Create new golfer
                let id = new_id();
                sqlx::query(
                    "INSERT INTO golfers (id, name, win_probability_group, is_amateur, espn_id) VALUES (?, ?, ?, 0, ?)"
                )
                .bind(&id)
                .bind(&name)
                .bind((group_idx as i32) + 1)
                .bind(&espn_athlete_id)
                .execute(&pool)
                .await
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;
                (id, true)
            }
        };

        groups[group_idx].golfers.push(EspnFieldGolfer {
            golfer_id,
            name,
            espn_id: espn_athlete_id,
            created,
        });
    }

    // Save ESPN tournament ID on tournament record
    sqlx::query("UPDATE tournaments SET espn_tournament_id = ? WHERE id = ?")
        .bind(espn_id)
        .bind(&tournament_id)
        .execute(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

    Ok(Json(EspnFieldPreviewResponse { groups }))
}

// Save admin-adjusted tournament group assignments
pub async fn save_tournament_groups(
    State(pool): State<SqlitePool>,
    Path(tournament_id): Path<String>,
    Json(payload): Json<SaveGroupsRequest>,
) -> Result<Json<SaveGroupsResponse>, (StatusCode, Json<ApiError>)> {
    let mut total_processed = 0;

    for assignment in &payload.assignments {
        if assignment.group < 1 || assignment.group > 9 {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(ApiError::new(format!("Group must be between 1 and 9, got {}", assignment.group))),
            ));
        }

        let id = new_id();
        sqlx::query(
            "INSERT INTO tournament_golfer_groups (id, tournament_id, golfer_id, win_probability_group) \
             VALUES (?, ?, ?, ?) \
             ON CONFLICT(tournament_id, golfer_id) \
             DO UPDATE SET win_probability_group = excluded.win_probability_group"
        )
        .bind(&id)
        .bind(&tournament_id)
        .bind(&assignment.golfer_id)
        .bind(assignment.group)
        .execute(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

        total_processed += 1;
    }

    Ok(Json(SaveGroupsResponse { total_processed }))
}

pub async fn import_espn_preview(
    State(pool): State<SqlitePool>,
    Json(payload): Json<EspnImportRequest>,
) -> Result<Json<EspnImportPreviewResponse>, (StatusCode, Json<ApiError>)> {
    let espn_id = &payload.espn_tournament_id;
    let client = reqwest::Client::new();

    // Fetch tournament event info
    let event_url = format!(
        "https://sports.core.api.espn.com/v2/sports/golf/leagues/pga/events/{}?lang=en&region=us",
        espn_id
    );
    let event: EspnEvent = client.get(&event_url).send().await
        .map_err(|e| (StatusCode::BAD_GATEWAY, Json(ApiError::new(format!("ESPN request failed: {}", e)))))?
        .json().await
        .map_err(|e| (StatusCode::BAD_GATEWAY, Json(ApiError::new(format!("ESPN parse error: {}", e)))))?;

    // Fetch all competitor pages
    let mut all_competitor_refs: Vec<String> = Vec::new();
    let mut page = 1;
    loop {
        let comp_url = format!(
            "https://sports.core.api.espn.com/v2/sports/golf/leagues/pga/events/{}/competitions/{}/competitors?lang=en&region=us&page={}",
            espn_id, espn_id, page
        );
        let comp_page: EspnCompetitorPage = client.get(&comp_url).send().await
            .map_err(|e| (StatusCode::BAD_GATEWAY, Json(ApiError::new(format!("ESPN request failed: {}", e)))))?
            .json().await
            .map_err(|e| (StatusCode::BAD_GATEWAY, Json(ApiError::new(format!("ESPN parse error: {}", e)))))?;

        for item in &comp_page.items {
            all_competitor_refs.push(item.href.clone());
        }

        if page >= comp_page.page_count {
            break;
        }
        page += 1;
    }

    tracing::info!("ESPN: Found {} competitors for event {}", all_competitor_refs.len(), espn_id);

    // Fetch each competitor's details concurrently with semaphore
    let semaphore = std::sync::Arc::new(tokio::sync::Semaphore::new(10));
    let client = std::sync::Arc::new(client);
    let mut handles = Vec::new();

    for comp_ref in all_competitor_refs {
        let sem = semaphore.clone();
        let client = client.clone();
        let handle = tokio::spawn(async move {
            let _permit = sem.acquire().await.unwrap();
            fetch_espn_competitor(&client, &comp_ref).await
        });
        handles.push(handle);
    }

    let mut players_for_preview: Vec<PlayerForPreview> = Vec::new();
    let mut import_players: Vec<ImportPlayer> = Vec::new();

    for handle in handles {
        match handle.await {
            Ok(Ok(data)) => {
                players_for_preview.push(PlayerForPreview {
                    name: data.display_name.clone(),
                    slug: data.slug.clone(),
                    rounds_available: data.rounds.iter().map(|r| r.round_number).collect(),
                });
                import_players.push(ImportPlayer {
                    name: data.display_name,
                    slug: data.slug,
                    espn_athlete_id: data.espn_athlete_id,
                    rounds: data.rounds,
                });
            }
            Ok(Err(e)) => {
                tracing::warn!("ESPN: Failed to fetch competitor: {}", e);
            }
            Err(e) => {
                tracing::warn!("ESPN: Task join error: {}", e);
            }
        }
    }

    let preview = build_import_preview(&pool, event.name.clone(), players_for_preview).await?;

    Ok(Json(EspnImportPreviewResponse {
        tournament_name: preview.tournament_name,
        matched: preview.matched,
        unmatched: preview.unmatched,
        players: import_players,
    }))
}

// Tournament Import - Commit
pub async fn import_commit(
    State(pool): State<SqlitePool>,
    Json(payload): Json<ImportCommitRequest>,
) -> Result<Json<ImportCommitResponse>, (StatusCode, Json<ApiError>)> {
    // Verify tournament exists
    let _tournament = sqlx::query_as::<_, Tournament>(
        "SELECT id, name, start_date, end_date, is_active, espn_tournament_id, created_at FROM tournaments WHERE id = ?"
    )
    .bind(&payload.tournament_id)
    .fetch_optional(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?
    .ok_or((StatusCode::NOT_FOUND, Json(ApiError::new("Tournament not found"))))?;

    // Store ESPN tournament ID if provided
    if let Some(ref espn_tid) = payload.espn_tournament_id {
        sqlx::query("UPDATE tournaments SET espn_tournament_id = ? WHERE id = ?")
            .bind(espn_tid)
            .bind(&payload.tournament_id)
            .execute(&pool)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;
    }

    let mut total_processed: usize = 0;
    let mut errors: Vec<String> = Vec::new();

    // Process new golfers first: create them in DB, then import their scores
    for new_golfer in &payload.new_golfers {
        if new_golfer.win_probability_group < 1 || new_golfer.win_probability_group > 9 {
            errors.push(format!("{}: group must be between 1 and 9", new_golfer.name));
            continue;
        }

        let golfer_id = new_id();
        sqlx::query(
            "INSERT INTO golfers (id, name, win_probability_group, is_amateur, espn_id) VALUES (?, ?, ?, ?, ?)"
        )
        .bind(&golfer_id)
        .bind(&new_golfer.name)
        .bind(new_golfer.win_probability_group)
        .bind(new_golfer.is_amateur)
        .bind(&new_golfer.espn_athlete_id)
        .execute(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

        // Import their rounds
        for round in &new_golfer.rounds {
            if round.round_number < 1 || round.round_number > 4 {
                errors.push(format!("New golfer {}: round_number {} out of range", new_golfer.name, round.round_number));
                continue;
            }

            for hole in &round.holes {
                let score_to_par = hole.strokes - hole.par;
                let fantasy_points = calculate_fantasy_points(score_to_par, new_golfer.is_amateur);
                let id = new_id();

                sqlx::query(
                    "INSERT INTO hole_scores (id, tournament_id, golfer_id, day, hole, strokes, score_to_par, fantasy_points) \
                     VALUES (?, ?, ?, ?, ?, ?, ?, ?) \
                     ON CONFLICT (tournament_id, golfer_id, day, hole) \
                     DO UPDATE SET strokes = excluded.strokes, score_to_par = excluded.score_to_par, fantasy_points = excluded.fantasy_points"
                )
                .bind(&id)
                .bind(&payload.tournament_id)
                .bind(&golfer_id)
                .bind(round.round_number)
                .bind(hole.hole)
                .bind(hole.strokes)
                .bind(score_to_par)
                .bind(fantasy_points)
                .execute(&pool)
                .await
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

                total_processed += 1;
            }
        }
    }

    for player_score in &payload.player_scores {
        // Store ESPN athlete ID on golfer if provided and not already set
        if let Some(ref espn_aid) = player_score.espn_athlete_id {
            sqlx::query("UPDATE golfers SET espn_id = ? WHERE id = ? AND espn_id IS NULL")
                .bind(espn_aid)
                .bind(&player_score.golfer_id)
                .execute(&pool)
                .await
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;
        }

        // Look up golfer's amateur status
        #[derive(sqlx::FromRow)]
        struct AmateurRow { is_amateur: bool }

        let golfer_row = match sqlx::query_as::<_, AmateurRow>(
            "SELECT is_amateur FROM golfers WHERE id = ?"
        )
        .bind(&player_score.golfer_id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))? {
            Some(row) => row,
            None => {
                errors.push(format!("Golfer {} not found", player_score.golfer_id));
                continue;
            }
        };

        for round in &player_score.rounds {
            if round.round_number < 1 || round.round_number > 4 {
                errors.push(format!("Golfer {}: round_number {} out of range", player_score.golfer_id, round.round_number));
                continue;
            }

            for hole in &round.holes {
                let score_to_par = hole.strokes - hole.par;
                let fantasy_points = calculate_fantasy_points(score_to_par, golfer_row.is_amateur);
                let id = new_id();

                sqlx::query(
                    "INSERT INTO hole_scores (id, tournament_id, golfer_id, day, hole, strokes, score_to_par, fantasy_points) \
                     VALUES (?, ?, ?, ?, ?, ?, ?, ?) \
                     ON CONFLICT (tournament_id, golfer_id, day, hole) \
                     DO UPDATE SET strokes = excluded.strokes, score_to_par = excluded.score_to_par, fantasy_points = excluded.fantasy_points"
                )
                .bind(&id)
                .bind(&payload.tournament_id)
                .bind(&player_score.golfer_id)
                .bind(round.round_number)
                .bind(hole.hole)
                .bind(hole.strokes)
                .bind(score_to_par)
                .bind(fantasy_points)
                .execute(&pool)
                .await
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

                total_processed += 1;
            }
        }
    }

    Ok(Json(ImportCommitResponse {
        total_scores_processed: total_processed,
        errors,
    }))
}

// Refresh scores from ESPN for a tournament
pub async fn refresh_scores(
    State(pool): State<SqlitePool>,
    Path(tournament_id): Path<String>,
) -> Result<Json<RefreshScoresResponse>, (StatusCode, Json<ApiError>)> {
    // Look up tournament and get espn_tournament_id
    let tournament = sqlx::query_as::<_, Tournament>(
        "SELECT id, name, start_date, end_date, is_active, espn_tournament_id, created_at FROM tournaments WHERE id = ?"
    )
    .bind(&tournament_id)
    .fetch_optional(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?
    .ok_or((StatusCode::NOT_FOUND, Json(ApiError::new("Tournament not found"))))?;

    let espn_id = tournament.espn_tournament_id
        .ok_or((StatusCode::BAD_REQUEST, Json(ApiError::new("Tournament has no ESPN tournament ID. Import via ESPN first."))))?;

    // Load all golfers with espn_id into a HashMap<espn_id, (golfer_id, is_amateur)>
    #[derive(sqlx::FromRow)]
    struct GolferEspnRow {
        id: String,
        espn_id: String,
        is_amateur: bool,
    }

    let golfer_rows = sqlx::query_as::<_, GolferEspnRow>(
        "SELECT id, espn_id, is_amateur FROM golfers WHERE espn_id IS NOT NULL"
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

    let golfer_map: std::collections::HashMap<String, (String, bool)> = golfer_rows
        .into_iter()
        .map(|r| (r.espn_id, (r.id, r.is_amateur)))
        .collect();

    // Fetch ESPN competitors (same pagination as import_espn_preview)
    let client = reqwest::Client::new();
    let mut all_competitor_refs: Vec<String> = Vec::new();
    let mut page = 1;
    loop {
        let comp_url = format!(
            "https://sports.core.api.espn.com/v2/sports/golf/leagues/pga/events/{}/competitions/{}/competitors?lang=en&region=us&page={}",
            espn_id, espn_id, page
        );
        let comp_page: EspnCompetitorPage = client.get(&comp_url).send().await
            .map_err(|e| (StatusCode::BAD_GATEWAY, Json(ApiError::new(format!("ESPN request failed: {}", e)))))?
            .json().await
            .map_err(|e| (StatusCode::BAD_GATEWAY, Json(ApiError::new(format!("ESPN parse error: {}", e)))))?;

        for item in &comp_page.items {
            all_competitor_refs.push(item.href.clone());
        }

        if page >= comp_page.page_count {
            break;
        }
        page += 1;
    }

    tracing::info!("ESPN refresh: Found {} competitors for event {}", all_competitor_refs.len(), espn_id);

    // Fetch each competitor concurrently
    let semaphore = std::sync::Arc::new(tokio::sync::Semaphore::new(10));
    let client = std::sync::Arc::new(client);
    let mut handles = Vec::new();

    for comp_ref in all_competitor_refs {
        let sem = semaphore.clone();
        let client = client.clone();
        let handle = tokio::spawn(async move {
            let _permit = sem.acquire().await.unwrap();
            fetch_espn_competitor(&client, &comp_ref).await
        });
        handles.push(handle);
    }

    let mut total_processed: usize = 0;
    let mut golfers_updated: usize = 0;
    let mut golfers_skipped: usize = 0;
    let mut errors: Vec<String> = Vec::new();

    for handle in handles {
        let data = match handle.await {
            Ok(Ok(data)) => data,
            Ok(Err(e)) => {
                errors.push(format!("ESPN fetch error: {}", e));
                continue;
            }
            Err(e) => {
                errors.push(format!("Task join error: {}", e));
                continue;
            }
        };

        // Match by ESPN athlete ID
        let espn_athlete_id = match &data.espn_athlete_id {
            Some(id) => id,
            None => {
                golfers_skipped += 1;
                continue;
            }
        };

        let (golfer_id, is_amateur) = match golfer_map.get(espn_athlete_id) {
            Some(entry) => entry.clone(),
            None => {
                golfers_skipped += 1;
                continue;
            }
        };

        if data.rounds.is_empty() {
            golfers_skipped += 1;
            continue;
        }

        // Upsert scores for this golfer
        for round in &data.rounds {
            for hole in &round.holes {
                let score_to_par = hole.score - hole.par;
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
                .bind(round.round_number)
                .bind(hole.hole)
                .bind(hole.score)
                .bind(score_to_par)
                .bind(fantasy_points)
                .execute(&pool)
                .await
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiError::new(e.to_string()))))?;

                total_processed += 1;
            }
        }
        golfers_updated += 1;
    }

    Ok(Json(RefreshScoresResponse {
        total_scores_processed: total_processed,
        golfers_updated,
        golfers_skipped,
        errors,
    }))
}
