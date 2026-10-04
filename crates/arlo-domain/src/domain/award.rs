use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AwardRecipientKind {
    Player,
    Team,
    Manager,
    Referee,
    Person,
    Federation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AwardScopeKind {
    Global,
    Competition,
    Team,
    Player,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum AwardResultKind {
    #[default]
    SingleWinner,
    Roster,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AwardOrganizerPolicy {
    DefinedOrganization,
    LeagueCommittee,
    FederationCommittee,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AwardTrigger {
    MatchCompleted,
    SeasonCompleted,
    CalendarInterval { days: u32 },
    CalendarDate { month: u32, day: u32 },
    FixedAnnouncementDate,
    CompetitionMatchCountMultiple { count: u32 },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AwardEvaluationWindow {
    CurrentMatch,
    EntireSeason,
    PreviousCalendarInterval { days: u32 },
    PreviousSeasonCycle,
    PreviousSeasonThroughAnnouncement,
    PreviousNCompetitionMatches { count: u32 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AwardNormalization {
    Global,
    Position,
    PositionFamily,
    CandidatePool,
    Competition,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AwardCriterion {
    pub key: String,
    pub weight: f64,
    pub normalization: AwardNormalization,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AwardTieDirection {
    Descending,
    Ascending,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AwardTieBreak {
    pub metric_key: String,
    pub direction: AwardTieDirection,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AwardVoterGroup {
    pub code: String,
    pub voter_count: u32,
    pub result_weight: f64,
    pub criterion_preferences: Vec<AwardCriterionPreference>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AwardCriterionPreference {
    pub criterion_key: String,
    pub multiplier: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AwardSelectionPolicy {
    Utility {
        temperature: f64,
    },
    RankedVoting {
        ballot_points: Vec<u32>,
        groups: Vec<AwardVoterGroup>,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AwardDefinition {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub short_name: Option<String>,
    pub organizer_id: Option<Uuid>,
    pub organizer_policy: AwardOrganizerPolicy,
    pub recipient_kind: AwardRecipientKind,
    pub prestige: f64,
    pub scope: AwardScopeKind,
    #[serde(default)]
    pub result_kind: AwardResultKind,
    pub trigger: AwardTrigger,
    pub evaluation_window: AwardEvaluationWindow,
    #[serde(default)]
    pub announcement_month_order_index: Option<u32>,
    #[serde(default)]
    pub announcement_day_of_month: Option<u32>,
    pub eligible_positions: Vec<String>,
    #[serde(default)]
    pub minimum_age: Option<u32>,
    #[serde(default)]
    pub maximum_age: Option<u32>,
    pub eligible_countries: Vec<Uuid>,
    pub eligible_continents: Vec<Uuid>,
    pub eligible_competitions: Vec<Uuid>,
    pub minimum_matches: u32,
    pub nomination_limit: Option<usize>,
    pub criteria: Vec<AwardCriterion>,
    #[serde(default)]
    pub roster_slots: Vec<AwardRosterSlot>,
    #[serde(default)]
    pub tie_breaks: Vec<AwardTieBreak>,
    pub selection: AwardSelectionPolicy,
    pub active: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AwardRosterSlot {
    pub slot_index: u32,
    pub position_code: String,
    pub selection_group: Option<String>,
    pub slot_role: Option<String>,
    pub criteria: Vec<AwardCriterion>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AwardOrganization {
    pub id: Uuid,
    pub code: String,
    pub name: Option<String>,
    pub country_id: Option<Uuid>,
    pub continent_id: Option<Uuid>,
    pub league_id: Option<Uuid>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AwardCandidateEvidence {
    pub subject_kind: AwardRecipientKind,
    pub subject_id: Uuid,
    pub position: Option<String>,
    #[serde(default)]
    pub positions: Vec<String>,
    pub position_family: Option<String>,
    pub country_id: Option<Uuid>,
    pub continent_id: Option<Uuid>,
    pub competition_id: Option<Uuid>,
    #[serde(default)]
    pub competition_ids: Vec<Uuid>,
    pub team_id: Option<Uuid>,
    pub matches_played: u32,
    #[serde(default)]
    pub age_years: Option<u32>,
    pub metrics: Vec<AwardMetric>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AwardMetric {
    pub key: String,
    pub value: f64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AwardInstanceContext {
    pub period_key: String,
    pub scope_id: Option<Uuid>,
    pub selection_model_version: u32,
}

pub fn award_position_family(position: &str) -> Option<&'static str> {
    match position {
        "CenterOffense" | "WingOffense" | "Midcenter" | "TightWing" | "CenterTight"
        | "Corridor" => Some("OffensiveLine"),
        "Artrine" | "Passer" | "PassRusher" | "WideEnd" | "RunningEnd" | "Lineback"
        | "Fullback" => Some("BackLine"),
        "Centerback" | "DefensiveEnd" | "Rougieback" | "DefensiveBlocker" | "WideBlocker"
        | "OutsideZonerback" | "MiddleZonerback" => Some("DefenseLine"),
        "Goalguard" => Some("Goalguard"),
        _ => None,
    }
}
