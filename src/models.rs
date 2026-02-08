use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

#[derive(Debug, Serialize, FromRow)]
pub struct Season {
    pub id: String,
    pub name: String,
    pub year: i32,
    pub start_date: String,
    pub end_date: String,
    pub is_active: bool,
    pub created_at: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateSeasonRequest {
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    pub year: i32,
    pub start_date: String,
    pub end_date: String,
}

#[derive(Debug, Serialize, FromRow)]
pub struct Golfer {
    pub id: String,
    pub name: String,
    pub win_probability_group: i32,
    pub is_active: bool,
    pub created_at: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateGolferRequest {
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    #[validate(range(min = 1, max = 6))]
    pub win_probability_group: i32,
}

#[derive(Debug, Serialize, FromRow)]
pub struct AccessKey {
    pub id: String,
    pub key_code: String,
    pub season_id: String,
    pub player_name: Option<String>,
    pub is_used: bool,
    pub used_at: Option<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateAccessKeysRequest {
    pub season_id: String,
    pub count: i32,
}

#[derive(Debug, Deserialize)]
pub struct ValidateAccessKeyRequest {
    pub key_code: String,
}

#[derive(Debug, Serialize)]
pub struct AccessKeyValidationResponse {
    pub valid: bool,
    pub season_id: Option<String>,
    pub already_used: bool,
}

#[derive(Debug, Serialize, FromRow)]
pub struct Team {
    pub id: String,
    pub season_id: String,
    pub tournament_id: Option<String>,
    pub player_name: String,
    pub access_key_id: String,
    pub created_at: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct TeamWithGolfers {
    pub team: Team,
    pub golfers: Vec<Golfer>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct Tournament {
    pub id: String,
    pub season_id: String,
    pub name: String,
    pub start_date: String,
    pub end_date: String,
    pub is_active: bool,
    pub created_at: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateTournamentRequest {
    pub season_id: String,
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    pub start_date: String,
    pub end_date: String,
}

#[derive(Debug, Serialize, FromRow)]
pub struct HoleScore {
    pub id: String,
    pub tournament_id: String,
    pub golfer_id: String,
    pub day: i32,
    pub hole: i32,
    pub strokes: i32,
    pub score_to_par: i32,
    pub fantasy_points: i32,
    pub created_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct HoleScoreInput {
    pub golfer_id: String,
    #[validate(range(min = 1, max = 4))]
    pub day: i32,
    #[validate(range(min = 1, max = 18))]
    pub hole: i32,
    #[validate(range(min = 1))]
    pub strokes: i32,
    pub score_to_par: i32,
}

#[derive(Debug, Deserialize, Validate)]
pub struct AddHoleScoresRequest {
    pub tournament_id: String,
    #[validate(length(min = 1))]
    pub scores: Vec<HoleScoreInput>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateTeamRequest {
    #[validate(length(min = 1))]
    pub key_code: String,
    #[validate(length(min = 1, max = 255))]
    pub player_name: String,
    pub tournament_id: String,
    #[validate(length(min = 6, max = 6))]
    pub golfer_ids: Vec<String>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct LeaderboardEntry {
    pub player_name: String,
    pub team_id: String,
    pub total_points: Option<i64>,
}

#[derive(Debug, Serialize, Clone)]
pub struct GolferSummary {
    pub id: String,
    pub name: String,
    pub win_probability_group: i32,
}

#[derive(Debug, Serialize)]
pub struct LeaderboardEntryWithGolfers {
    pub player_name: String,
    pub team_id: String,
    pub total_points: i64,
    pub golfers: Vec<GolferSummary>,
}

#[derive(Debug, FromRow)]
pub struct TeamGolferRow {
    pub team_id: String,
    pub id: String,
    pub name: String,
    pub win_probability_group: i32,
}

#[derive(Debug, Serialize, FromRow)]
pub struct TournamentScore {
    pub golfer_name: String,
    pub golfer_id: String,
    pub total_points: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct ApiError {
    pub message: String,
}

impl ApiError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct AdminLoginRequest {
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AdminLoginResponse {
    pub success: bool,
    pub token: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateTeamRequest {
    pub key_code: String,
    pub tournament_id: String,
    #[validate(length(min = 6, max = 6))]
    pub golfer_ids: Vec<String>,
}

// JSON Score Upload
#[derive(Debug, Deserialize)]
pub struct ScoreUploadEntry {
    pub golfer: String,
    pub day: i32,
    pub holes: Vec<i32>,
}

#[derive(Debug, Deserialize)]
pub struct ScoreUploadRequest {
    pub pars: Vec<i32>,
    pub scores: Vec<ScoreUploadEntry>,
}

#[derive(Debug, Serialize)]
pub struct ScoreUploadResponse {
    pub total_scores_processed: usize,
    pub errors: Vec<String>,
}
