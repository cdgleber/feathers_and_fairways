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
pub struct AccessKeyDetail {
    pub id: String,
    pub key_code: String,
    pub is_used: bool,
    pub created_at: Option<String>,
    pub tournament_name: String,
    pub team_name: Option<String>,
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
    /// ESPN field seeding position (typically OWGR for PGA events)
    pub sort_order: Option<i64>,
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
    // ESPN returns linescores as a raw array, not {items:[...]}
    #[serde(default)]
    pub linescores: Option<Vec<EspnHoleScore>>,
}

#[derive(Debug, Deserialize)]
pub struct EspnHoleScore {
    #[serde(default)]
    pub period: Option<i32>,
    // ESPN returns numeric fields as floats (e.g. 4.0)
    #[serde(default)]
    pub par: Option<f64>,
    #[serde(default)]
    pub value: Option<f64>,
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

#[cfg(test)]
mod espn_parsing_tests {
    use super::*;

    /// The outer linescores response from ESPN — a paginated list of rounds.
    /// Each round has `period` (round number) and `linescores` which is a **raw array**
    /// of hole score objects (not `{items:[...]}`).
    #[test]
    fn test_espn_linescores_parses_raw_array_format() {
        let json = r#"{
            "count": 2,
            "pageIndex": 1,
            "pageSize": 25,
            "pageCount": 1,
            "items": [
                {
                    "value": 77.0,
                    "displayValue": "+6",
                    "period": 1,
                    "linescores": [
                        {"value": 4.0, "displayValue": "4", "period": 10, "scoreType": {"name": "BIRDIE", "displayName": "Birdie"}},
                        {"value": 5.0, "displayValue": "5", "period": 11, "scoreType": {"name": "PAR", "displayName": "Par"}}
                    ]
                },
                {
                    "value": 71.0,
                    "displayValue": "-1",
                    "period": 2,
                    "linescores": [
                        {"value": 3.0, "displayValue": "3", "period": 1, "scoreType": {"name": "BIRDIE", "displayName": "Birdie"}}
                    ]
                }
            ]
        }"#;

        let parsed: EspnLinescores = serde_json::from_str(json).expect("should parse linescores");
        assert_eq!(parsed.items.len(), 2);

        let round1 = &parsed.items[0];
        assert_eq!(round1.period, Some(1));
        let holes1 = round1.linescores.as_ref().expect("round 1 should have linescores");
        assert_eq!(holes1.len(), 2);
        assert_eq!(holes1[0].value, Some(4.0));
        assert_eq!(holes1[0].period, Some(10));
        assert_eq!(holes1[1].value, Some(5.0));
        assert_eq!(holes1[1].period, Some(11));

        let round2 = &parsed.items[1];
        assert_eq!(round2.period, Some(2));
        let holes2 = round2.linescores.as_ref().expect("round 2 should have linescores");
        assert_eq!(holes2.len(), 1);
        assert_eq!(holes2[0].value, Some(3.0));
    }

    /// ESPN returns numeric values as floats even for whole numbers (4.0 not 4).
    #[test]
    fn test_espn_hole_score_parses_float_values() {
        let json = r#"{"value": 4.0, "period": 7, "par": 4.0}"#;
        let score: EspnHoleScore = serde_json::from_str(json).expect("should parse hole score");
        assert_eq!(score.value, Some(4.0_f64));
        assert_eq!(score.par, Some(4.0_f64));
        assert_eq!(score.period, Some(7));

        // Conversion to i32 used in route handler should work correctly
        assert_eq!(score.value.map(|v| v as i32), Some(4));
        assert_eq!(score.par.map(|v| v as i32), Some(4));
    }

    /// Rounds missing linescores (player withdrew / not yet played) should parse cleanly.
    #[test]
    fn test_espn_round_without_linescores() {
        let json = r#"{"value": 77.0, "displayValue": "+6", "period": 1}"#;
        let round: EspnRound = serde_json::from_str(json).expect("should parse round without linescores");
        assert_eq!(round.period, Some(1));
        assert!(round.linescores.is_none());
    }

    /// Rounds with an empty linescores array should parse as Some([]).
    #[test]
    fn test_espn_round_with_empty_linescores() {
        let json = r#"{"period": 1, "linescores": []}"#;
        let round: EspnRound = serde_json::from_str(json).expect("should parse round with empty linescores");
        let holes = round.linescores.expect("should have linescores field");
        assert!(holes.is_empty());
    }

    /// The competitor page response (used for pagination) should parse correctly.
    #[test]
    fn test_espn_competitor_page_parses() {
        let json = r#"{
            "count": 156,
            "pageIndex": 1,
            "pageSize": 25,
            "pageCount": 7,
            "items": [
                {"$ref": "http://sports.core.api.espn.com/v2/sports/golf/leagues/pga/events/401811934/competitions/401811934/competitors/4848?lang=en&region=us"},
                {"$ref": "http://sports.core.api.espn.com/v2/sports/golf/leagues/pga/events/401811934/competitions/401811934/competitors/780?lang=en&region=us"}
            ]
        }"#;

        let page: EspnCompetitorPage = serde_json::from_str(json).expect("should parse competitor page");
        assert_eq!(page.page_count, 7);
        assert_eq!(page.items.len(), 2);
        assert!(page.items[0].href.contains("401811934"));
    }

    /// The competitor object links to athlete and linescores via $ref.
    #[test]
    fn test_espn_competitor_parses_refs() {
        let json = r#"{
            "id": "4848",
            "score": {"$ref": "http://sports.core.api.espn.com/v2/sports/golf/leagues/pga/events/401811934/competitions/401811934/competitors/4848/score?lang=en&region=us"},
            "linescores": {"$ref": "http://sports.core.api.espn.com/v2/sports/golf/leagues/pga/events/401811934/competitions/401811934/competitors/4848/linescores?lang=en&region=us"},
            "athlete": {"$ref": "http://sports.core.api.espn.com/v2/sports/golf/athletes/4848?lang=en&region=us"}
        }"#;

        let competitor: EspnCompetitor = serde_json::from_str(json).expect("should parse competitor");
        assert_eq!(competitor.id, "4848");
        assert!(competitor.linescores.is_some());
        assert!(competitor.athlete.is_some());
        assert!(competitor.linescores.unwrap().href.contains("linescores"));
    }

    /// Competitor with no linescores ref (player withdrawn before play) should have None.
    #[test]
    fn test_espn_competitor_without_linescores_ref() {
        let json = r#"{
            "id": "9999",
            "athlete": {"$ref": "http://sports.core.api.espn.com/v2/sports/golf/athletes/9999?lang=en&region=us"}
        }"#;

        let competitor: EspnCompetitor = serde_json::from_str(json).expect("should parse competitor");
        assert!(competitor.linescores.is_none());
        assert!(competitor.score.is_none());
    }

    /// During a live tournament, a player's round may only have partial holes (e.g. 5 of 18).
    #[test]
    fn test_espn_partial_round_during_live_tournament() {
        let json = r#"{
            "count": 1,
            "pageIndex": 1,
            "pageSize": 25,
            "pageCount": 1,
            "items": [
                {
                    "value": 18.0,
                    "displayValue": "-2",
                    "period": 1,
                    "linescores": [
                        {"value": 4.0, "period": 1},
                        {"value": 3.0, "period": 2},
                        {"value": 5.0, "period": 3},
                        {"value": 4.0, "period": 4},
                        {"value": 2.0, "period": 5}
                    ]
                }
            ]
        }"#;

        let parsed: EspnLinescores = serde_json::from_str(json).expect("should parse partial round");
        assert_eq!(parsed.items.len(), 1);
        let holes = parsed.items[0].linescores.as_ref().unwrap();
        assert_eq!(holes.len(), 5, "partial round has only 5 holes played");
        assert_eq!(holes[4].period, Some(5));
        assert_eq!(holes[4].value, Some(2.0));
    }

    /// A golfer missing the cut has round 1 and 2 scores but no round 3 or 4.
    #[test]
    fn test_espn_cut_player_has_two_rounds() {
        let json = r#"{
            "count": 2,
            "pageIndex": 1,
            "pageSize": 25,
            "pageCount": 1,
            "items": [
                {
                    "value": 74.0,
                    "displayValue": "+3",
                    "period": 1,
                    "linescores": [
                        {"value": 4.0, "period": 1},
                        {"value": 5.0, "period": 2}
                    ]
                },
                {
                    "value": 75.0,
                    "displayValue": "+4",
                    "period": 2,
                    "linescores": [
                        {"value": 5.0, "period": 1},
                        {"value": 4.0, "period": 2}
                    ]
                }
            ]
        }"#;

        let parsed: EspnLinescores = serde_json::from_str(json).expect("should parse cut player rounds");
        assert_eq!(parsed.items.len(), 2);
        assert_eq!(parsed.items[0].period, Some(1));
        assert_eq!(parsed.items[1].period, Some(2));
    }
}
