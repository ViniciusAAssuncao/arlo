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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AwardOrganizerPolicy {
    DefinedOrganization,
    LeagueCommittee,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AwardTrigger {
    MatchCompleted,
    SeasonCompleted,
    CalendarInterval { days: u32 },
    CalendarDate { month: u32, day: u32 },
    CompetitionMatchCountMultiple { count: u32 },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AwardEvaluationWindow {
    CurrentMatch,
    EntireSeason,
    PreviousCalendarInterval { days: u32 },
    PreviousSeasonCycle,
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
    pub trigger: AwardTrigger,
    pub evaluation_window: AwardEvaluationWindow,
    pub eligible_positions: Vec<String>,
    pub eligible_countries: Vec<Uuid>,
    pub eligible_continents: Vec<Uuid>,
    pub eligible_competitions: Vec<Uuid>,
    pub minimum_matches: u32,
    pub nomination_limit: Option<usize>,
    pub criteria: Vec<AwardCriterion>,
    pub selection: AwardSelectionPolicy,
    pub active: bool,
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
    pub position_family: Option<String>,
    pub country_id: Option<Uuid>,
    pub continent_id: Option<Uuid>,
    pub competition_id: Option<Uuid>,
    pub team_id: Option<Uuid>,
    pub matches_played: u32,
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
