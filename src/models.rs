use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc, NaiveDate};
use validator::Validate;

#[derive(Debug, Serialize, FromRow)]
pub struct Season {
    pub id: Uuid,
    pub name: String,
    pub year: i32,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub is_active: bool,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateSeasonRequest {
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    pub year: i32,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
}

#[derive(Debug, Serialize, FromRow)]
pub struct Golfer {
    pub id: Uuid,
    pub name: String,
    pub win_probability_group: i32,
    pub is_active: bool,
    pub created_at: Option<DateTime<Utc>>,
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
    pub id: Uuid,
    pub key_code: String,
    pub season_id: Uuid,
    pub player_name: Option<String>,
    pub is_used: bool,
    pub used_at: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct CreateAccessKeysRequest {
    pub season_id: Uuid,
    pub count: i32,
}

#[derive(Debug, Deserialize)]
pub struct ValidateAccessKeyRequest {
    pub key_code: String,
}

#[derive(Debug, Serialize)]
pub struct AccessKeyValidationResponse {
    pub valid: bool,
    pub season_id: Option<Uuid>,
    pub already_used: bool,
}

#[derive(Debug, Serialize, FromRow)]
pub struct Team {
    pub id: Uuid,
    pub season_id: Uuid,
    pub player_name: String,
    pub access_key_id: Uuid,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateTeamRequest {
    #[validate(length(min = 1))]
    pub key_code: String,
    #[validate(length(min = 1, max = 255))]
    pub player_name: String,
    #[validate(length(min = 6, max = 6))]
    pub golfer_ids: Vec<Uuid>,
}

#[derive(Debug, Serialize)]
pub struct TeamWithGolfers {
    pub team: Team,
    pub golfers: Vec<Golfer>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct Tournament {
    pub id: Uuid,
    pub season_id: Uuid,
    pub name: String,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub is_active: bool,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateTournamentRequest {
    pub season_id: Uuid,
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
}

#[derive(Debug, Serialize, FromRow)]
pub struct HoleScore {
    pub id: Uuid,
    pub tournament_id: Uuid,
    pub golfer_id: Uuid,
    pub day: i32,
    pub hole: i32,
    pub strokes: i32,
    pub score_to_par: i32,
    pub fantasy_points: i32,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct HoleScoreInput {
    pub golfer_id: Uuid,
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
    pub tournament_id: Uuid,
    #[validate(length(min = 1))]
    pub scores: Vec<HoleScoreInput>,
}

#[derive(Debug, Serialize, FromRow, Validate)]
pub struct LeaderboardEntry {
    pub player_name: String,
    pub team_id: Uuid,
    #[validate(range(min = 0))]
    pub total_points: Option<i64>,
}

#[derive(Debug, Serialize, FromRow, Validate)]
pub struct TournamentScore {
    pub golfer_name: String,
    pub golfer_id: Uuid,
    #[validate(range(min = 0))]
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