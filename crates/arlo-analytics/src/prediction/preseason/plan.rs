use crate::error::{AnalyticsError, AnalyticsResult};
use crate::power_ranking::TeamPowerSeed;
use crate::prediction::ForecastParameters;
use arlo_domain::{
    CompetitionGroup, QtaWeightingPolicy, ScheduleAlgorithmKind, SpaScoringPolicy, StageDefinition,
    StageType, TieBreakCriterion,
};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

pub const PRESEASON_MODEL_VERSION: u32 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PreseasonConfig {
    pub simulation_count: u32,
    pub historical_rating_sigma: f64,
    pub new_team_rating_sigma: f64,
    pub knockout_margin_mean: f64,
}

impl Default for PreseasonConfig {
    fn default() -> Self {
        Self {
            simulation_count: 4096,
            historical_rating_sigma: 40.0,
            new_team_rating_sigma: 90.0,
            knockout_margin_mean: 12.0,
        }
    }
}

impl PreseasonConfig {
    pub fn validate(self) -> AnalyticsResult<()> {
        if self.simulation_count == 0 {
            return Err(AnalyticsError::InvalidData(
                "simulation_count must be positive".into(),
            ));
        }
        for (field, value) in [
            ("historical_rating_sigma", self.historical_rating_sigma),
            ("new_team_rating_sigma", self.new_team_rating_sigma),
            ("knockout_margin_mean", self.knockout_margin_mean),
        ] {
            if !value.is_finite() || value <= 0.0 {
                return Err(AnalyticsError::InvalidData(format!(
                    "{field} must be finite and positive"
                )));
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct GoalPointModel {
    pub mean_per_team: f64,
    pub rating_slope: f64,
}

impl GoalPointModel {
    pub fn validate(self) -> AnalyticsResult<()> {
        if !self.mean_per_team.is_finite()
            || self.mean_per_team <= 0.0
            || !self.rating_slope.is_finite()
        {
            return Err(AnalyticsError::InvalidData(
                "invalid Goal Point model".into(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ProjectionTeam {
    pub seed: TeamPowerSeed,
    pub has_rating_history: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectionFixture {
    pub round_index: u32,
    pub home_team_id: Uuid,
    pub away_team_id: Uuid,
    pub neutral_venue: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectionTie {
    pub high_team_id: Uuid,
    pub low_team_id: Uuid,
    pub high_seed_number: u32,
    pub low_seed_number: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectionStage {
    pub definition: StageDefinition,
    pub known_fixtures: Vec<ProjectionFixture>,
    pub known_ties: Vec<ProjectionTie>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CompetitionProjectionPlan {
    pub season_instance_id: Uuid,
    pub teams: Vec<ProjectionTeam>,
    pub initial_team_ids: Vec<Uuid>,
    pub stages: Vec<ProjectionStage>,
    pub groups: Vec<CompetitionGroup>,
    pub external_winners: HashMap<Uuid, Uuid>,
    pub algorithm: ScheduleAlgorithmKind,
    pub spa_policy: SpaScoringPolicy,
    pub qta_policy: QtaWeightingPolicy,
    pub tie_break_criteria: Vec<TieBreakCriterion>,
    pub forecast_parameters: ForecastParameters,
    pub goal_point_model: Option<GoalPointModel>,
    pub config: PreseasonConfig,
}

#[derive(Debug, Clone, Copy)]
pub(super) struct IndexedFixture {
    pub round: u32,
    pub home: usize,
    pub away: usize,
    pub neutral: bool,
}

#[derive(Debug, Clone, Copy)]
pub(super) struct IndexedTie {
    pub high: usize,
    pub low: usize,
    pub high_seed: u32,
    pub low_seed: u32,
}

pub(super) struct PreparedPlan {
    pub plan: CompetitionProjectionPlan,
    pub ids: Vec<Uuid>,
    pub index: HashMap<Uuid, usize>,
    pub known_fixtures: Vec<Vec<IndexedFixture>>,
    pub known_ties: Vec<IndexedTie>,
    pub initial_participants: Vec<usize>,
}

impl PreparedPlan {
    pub fn new(plan: CompetitionProjectionPlan) -> AnalyticsResult<Self> {
        plan.config.validate()?;
        plan.forecast_parameters.validate()?;
        if let Some(model) = plan.goal_point_model {
            model.validate()?;
        }
        if plan.teams.len() < 2 || plan.stages.is_empty() {
            return Err(AnalyticsError::InvalidData(
                "projection requires teams and stages".into(),
            ));
        }
        let mut ids = Vec::with_capacity(plan.teams.len());
        let mut index = HashMap::with_capacity(plan.teams.len());
        for team in &plan.teams {
            let id = team.seed.team_id();
            if index.insert(id, ids.len()).is_some() {
                return Err(AnalyticsError::InvalidData(
                    "duplicate projection team".into(),
                ));
            }
            ids.push(id);
        }
        let mut initial_participants = Vec::with_capacity(plan.initial_team_ids.len());
        let mut initial_seen = HashSet::new();
        for id in &plan.initial_team_ids {
            let team = *index
                .get(id)
                .ok_or_else(|| AnalyticsError::InvalidData("unknown opening stage team".into()))?;
            if !initial_seen.insert(team) {
                return Err(AnalyticsError::InvalidData(
                    "duplicate opening stage team".into(),
                ));
            }
            initial_participants.push(team);
        }
        if initial_participants.len() < 2 {
            return Err(AnalyticsError::InvalidData(
                "opening stage needs two teams".into(),
            ));
        }
        let mut known_fixtures = Vec::with_capacity(plan.stages.len());
        let mut known_ties = Vec::new();
        for (position, stage) in plan.stages.iter().enumerate() {
            if stage.definition.stage_order_index() as usize != position {
                return Err(AnalyticsError::InvalidData(
                    "stage order is not sequential".into(),
                ));
            }
            if position > 0 && (!stage.known_fixtures.is_empty() || !stage.known_ties.is_empty()) {
                return Err(AnalyticsError::InvalidData(
                    "future stage fixtures must not be materialized".into(),
                ));
            }
            let mut fixtures = Vec::with_capacity(stage.known_fixtures.len());
            let mut distinct = HashSet::new();
            for fixture in &stage.known_fixtures {
                let home = *index.get(&fixture.home_team_id).ok_or_else(|| {
                    AnalyticsError::InvalidData("unknown fixture home team".into())
                })?;
                let away = *index.get(&fixture.away_team_id).ok_or_else(|| {
                    AnalyticsError::InvalidData("unknown fixture away team".into())
                })?;
                if position == 0 && (!initial_seen.contains(&home) || !initial_seen.contains(&away))
                {
                    return Err(AnalyticsError::InvalidData(
                        "opening fixture has a nonparticipant".into(),
                    ));
                }
                if home == away || !distinct.insert((fixture.round_index, home, away)) {
                    return Err(AnalyticsError::InvalidData(
                        "invalid projection fixture".into(),
                    ));
                }
                fixtures.push(IndexedFixture {
                    round: fixture.round_index,
                    home,
                    away,
                    neutral: fixture.neutral_venue,
                });
            }
            if position == 0
                && fixtures.is_empty()
                && stage.definition.stage_type() != StageType::KnockoutBracket
            {
                return Err(AnalyticsError::InvalidData(
                    "opening stage fixtures are missing".into(),
                ));
            }
            if position == 0
                && stage.definition.stage_type() == StageType::KnockoutBracket
                && !fixtures.is_empty()
                && stage.known_ties.is_empty()
            {
                return Err(AnalyticsError::InvalidData(
                    "opening knockout ties are missing".into(),
                ));
            }
            known_fixtures.push(fixtures);
            if position == 0 {
                for tie in &stage.known_ties {
                    let high = *index
                        .get(&tie.high_team_id)
                        .ok_or_else(|| AnalyticsError::InvalidData("unknown high seed".into()))?;
                    let low = *index
                        .get(&tie.low_team_id)
                        .ok_or_else(|| AnalyticsError::InvalidData("unknown low seed".into()))?;
                    if high == low || tie.high_seed_number == tie.low_seed_number {
                        return Err(AnalyticsError::InvalidData(
                            "invalid projected knockout tie".into(),
                        ));
                    }
                    known_ties.push(IndexedTie {
                        high,
                        low,
                        high_seed: tie.high_seed_number,
                        low_seed: tie.low_seed_number,
                    });
                }
            }
        }
        Ok(Self {
            plan,
            ids,
            index,
            known_fixtures,
            known_ties,
            initial_participants,
        })
    }
}
