use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

#[derive(Debug, Serialize, FromRow)]
pub struct Golfer {
    pub id: String,
    pub name: String,
    pub win_probability_group: i32,
    pub is_amateur: bool,
    pub is_active: bool,
    pub espn_id: Option<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateGolferRequest {
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    #[validate(range(min = 1, max = 9))]
    pub win_probability_group: i32,
    pub is_amateur: Option<bool>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct AccessKey {
    pub id: String,
    pub key_code: String,
    pub tournament_id: String,
    pub player_name: Option<String>,
    pub is_used: bool,
    pub used_at: Option<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateAccessKeysRequest {
    pub tournament_id: String,
    pub count: i32,
}

#[derive(Debug, Deserialize)]
pub struct ValidateAccessKeyRequest {
    pub key_code: String,
}

#[derive(Debug, Serialize)]
pub struct AccessKeyValidationResponse {
    pub valid: bool,
    pub tournament_id: Option<String>,
    pub already_used: bool,
}

#[derive(Debug, Serialize, FromRow)]
pub struct Team {
    pub id: String,
    pub tournament_id: String,
    pub player_name: String,
    pub access_key_id: Option<String>,
    pub email: Option<String>,
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
    pub name: String,
    pub start_date: String,
    pub end_date: String,
    pub is_active: bool,
    pub espn_tournament_id: Option<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateTournamentRequest {
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
    #[validate(length(min = 9, max = 9))]
    pub golfer_ids: Vec<String>,
    #[validate(email)]
    pub email: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct GolferSummary {
    pub id: String,
    pub name: String,
    pub win_probability_group: i32,
    pub is_amateur: bool,
}

#[derive(Debug, FromRow)]
pub struct TeamGolferRow {
    pub team_id: String,
    pub id: String,
    pub name: String,
    pub win_probability_group: i32,
    pub is_amateur: bool,
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
    #[validate(length(min = 9, max = 9))]
    pub golfer_ids: Vec<String>,
}

// Admin Stats
#[derive(Debug, Serialize)]
pub struct ScoreDistribution {
    pub eagles_or_better: i64,
    pub birdies: i64,
    pub pars: i64,
    pub bogeys_or_worse: i64,
}

#[derive(Debug, Serialize, FromRow)]
pub struct PopularGolfer {
    pub golfer_name: String,
    pub times_selected: i64,
}

#[derive(Debug, Serialize)]
pub struct AdminStats {
    pub total_tournaments: i64,
    pub total_teams: i64,
    pub total_golfers: i64,
    pub total_scores: i64,
    pub access_keys_total: i64,
    pub access_keys_used: i64,
    pub access_keys_unused: i64,
    pub score_distribution: ScoreDistribution,
    pub popular_golfers: Vec<PopularGolfer>,
}

// Admin Team Golfer Update
#[derive(Debug, Deserialize, Validate)]
pub struct AdminUpdateTeamGolfersRequest {
    pub tournament_id: String,
    #[validate(length(min = 9, max = 9))]
    pub golfer_ids: Vec<String>,
}

// Completed Tournament History
#[derive(Debug, Serialize, FromRow)]
pub struct CompletedTournament {
    pub id: String,
    pub name: String,
    pub start_date: String,
    pub end_date: String,
}

#[derive(Debug, Serialize, FromRow)]
pub struct TournamentTeamLeaderboardEntry {
    pub player_name: String,
    pub team_id: String,
    pub total_points: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct TournamentTeamLeaderboardEntryWithGolfers {
    pub player_name: String,
    pub team_id: String,
    pub total_points: i64,
    pub golfers: Vec<GolferSummary>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct TournamentStats {
    pub total_holes_played: i64,
    pub total_fantasy_points: i64,
    pub eagles_or_better: i64,
    pub birdies: i64,
    pub pars: i64,
    pub bogeys_or_worse: i64,
    pub best_round_golfer: Option<String>,
    pub best_round_points: Option<i64>,
}

// Tournament Import (from get-golf-scores tournament.json)
#[derive(Debug, Deserialize)]
pub struct ImportTournamentJson {
    pub tournament: ImportTournamentInfo,
    pub players: Vec<ImportPlayer>,
}

#[derive(Debug, Deserialize)]
pub struct ImportTournamentInfo {
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ImportPlayer {
    pub slug: String,
    pub name: String,
    pub espn_athlete_id: Option<String>,
    pub rounds: Vec<ImportRound>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ImportRound {
    pub round_number: i32,
    pub holes: Vec<ImportHole>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ImportHole {
    pub hole: i32,
    pub par: i32,
    pub score: i32,
}

#[derive(Debug, Serialize)]
pub struct ImportPreviewResponse {
    pub tournament_name: String,
    pub matched: Vec<ImportMatchedGolfer>,
    pub unmatched: Vec<ImportUnmatchedGolfer>,
}

#[derive(Debug, Serialize)]
pub struct ImportMatchedGolfer {
    pub json_name: String,
    pub slug: String,
    pub golfer_id: String,
    pub golfer_name: String,
    pub is_amateur: bool,
    pub rounds_available: Vec<i32>,
}

#[derive(Debug, Serialize)]
pub struct ImportUnmatchedGolfer {
    pub json_name: String,
    pub slug: String,
    pub candidates: Vec<ImportGolferCandidate>,
    pub rounds_available: Vec<i32>,
}

#[derive(Debug, Serialize)]
pub struct ImportGolferCandidate {
    pub golfer_id: String,
    pub golfer_name: String,
}

#[derive(Debug, Deserialize)]
pub struct ImportCommitRequest {
    pub tournament_id: String,
    pub espn_tournament_id: Option<String>,
    pub player_scores: Vec<ImportPlayerScore>,
    #[serde(default)]
    pub new_golfers: Vec<NewGolferImport>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct NewGolferImport {
    pub name: String,
    pub slug: String,
    pub win_probability_group: i32,
    #[serde(default)]
    pub is_amateur: bool,
    pub espn_athlete_id: Option<String>,
    pub rounds: Vec<ImportCommitRound>,
}

#[derive(Debug, Deserialize)]
pub struct ImportPlayerScore {
    pub golfer_id: String,
    pub espn_athlete_id: Option<String>,
    pub rounds: Vec<ImportCommitRound>,
}

#[derive(Debug, Deserialize)]
pub struct ImportCommitRound {
    pub round_number: i32,
    pub holes: Vec<ImportCommitHole>,
}

#[derive(Debug, Deserialize)]
pub struct ImportCommitHole {
    pub hole: i32,
    pub strokes: i32,
    pub par: i32,
}

#[derive(Debug, Serialize)]
pub struct ImportCommitResponse {
    pub total_scores_processed: usize,
    pub errors: Vec<String>,
}

// ESPN Import
#[derive(Debug, Deserialize)]
pub struct EspnImportRequest {
    pub espn_tournament_id: String,
}

#[derive(Debug, Deserialize)]
pub struct EspnEvent {
    pub name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EspnCompetitorPage {
    pub page_count: i32,
    pub items: Vec<EspnCompetitorRef>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EspnCompetitorRef {
    #[serde(rename = "$ref")]
    pub href: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct EspnCompetitor {
    pub id: String,
    pub score: Option<EspnScoreRef>,
    pub linescores: Option<EspnLinescoresRef>,
    pub athlete: Option<EspnAthleteRef>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct EspnScoreRef {
    #[serde(rename = "$ref")]
    pub href: String,
}

#[derive(Debug, Deserialize)]
pub struct EspnLinescoresRef {
    #[serde(rename = "$ref")]
    pub href: String,
}

#[derive(Debug, Deserialize)]
pub struct EspnAthleteRef {
    #[serde(rename = "$ref")]
    pub href: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EspnAthlete {
    pub id: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub short_name: Option<String>,
    pub display_name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct EspnLinescores {
    pub items: Vec<EspnRound>,
}

#[derive(Debug, Deserialize)]
pub struct EspnRound {
    #[serde(default)]
    pub period: Option<i32>,
    #[serde(default)]
    pub linescores: Option<EspnRoundLinescores>,
}

#[derive(Debug, Deserialize)]
pub struct EspnRoundLinescores {
    pub items: Vec<EspnHoleScore>,
}

#[derive(Debug, Deserialize)]
pub struct EspnHoleScore {
    #[serde(default)]
    pub period: Option<i32>,
    #[serde(default)]
    pub par: Option<i32>,
    #[serde(default)]
    pub value: Option<i32>,
}

// Internal struct for transformed ESPN player data
#[derive(Debug, Serialize, Clone)]
pub struct EspnPlayerData {
    pub display_name: String,
    pub slug: String,
    pub espn_athlete_id: Option<String>,
    pub rounds: Vec<ImportRound>,
}

// ESPN preview response includes both preview and raw player data for the commit step
#[derive(Debug, Serialize)]
pub struct EspnImportPreviewResponse {
    pub tournament_name: String,
    pub matched: Vec<ImportMatchedGolfer>,
    pub unmatched: Vec<ImportUnmatchedGolfer>,
    pub players: Vec<ImportPlayer>,
}

#[derive(Debug, Serialize)]
pub struct RefreshScoresResponse {
    pub total_scores_processed: usize,
    pub golfers_updated: usize,
    pub golfers_skipped: usize,
    pub errors: Vec<String>,
}

// Paste golfers from a list of names
#[derive(Debug, Deserialize)]
pub struct PasteGolfersRequest {
    pub names: Vec<String>,
    pub is_amateur: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct PasteGolferResult {
    pub name: String,
    pub id: String,
    pub created: bool,
}

#[derive(Debug, Serialize)]
pub struct PasteGolfersResponse {
    pub results: Vec<PasteGolferResult>,
    pub errors: Vec<String>,
}

// ESPN field fetch and group assignment
#[derive(Debug, Serialize)]
pub struct EspnFieldGolfer {
    pub golfer_id: String,
    pub name: String,
    pub espn_id: Option<String>,
    pub created: bool,
}

#[derive(Debug, Serialize)]
pub struct EspnFieldGroup {
    pub group: i32,
    pub golfers: Vec<EspnFieldGolfer>,
}

#[derive(Debug, Serialize)]
pub struct EspnFieldPreviewResponse {
    pub groups: Vec<EspnFieldGroup>,
}

#[derive(Debug, Deserialize)]
pub struct GolferGroupAssignment {
    pub golfer_id: String,
    pub group: i32,
}

#[derive(Debug, Deserialize)]
pub struct SaveGroupsRequest {
    pub assignments: Vec<GolferGroupAssignment>,
}

#[derive(Debug, Serialize)]
pub struct SaveGroupsResponse {
    pub total_processed: usize,
}

// Public query param for listing teams by tournament
#[derive(Debug, Deserialize)]
pub struct TournamentIdQuery {
    pub tournament_id: Option<String>,
}
