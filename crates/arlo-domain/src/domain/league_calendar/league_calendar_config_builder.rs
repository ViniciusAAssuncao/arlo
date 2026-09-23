use crate::domain::league_calendar::competition_group::CompetitionGroup;
use crate::domain::league_calendar::games_per_week_policy::GamesPerWeekPolicy;
use crate::domain::league_calendar::league_calendar_config::LeagueCalendarConfig;
use crate::domain::league_calendar::neutral_opener_policy::NeutralOpenerPolicy;
use crate::domain::league_calendar::postponement_policy::PostponementPolicy;
use crate::domain::league_calendar::promotion_relegation_policy::PromotionRelegationPolicy;
use crate::domain::league_calendar::qta_weighting_policy::QtaWeightingPolicy;
use crate::domain::league_calendar::rest_gap_policy::RestGapPolicy;
use crate::domain::league_calendar::schedule_algorithm_kind::ScheduleAlgorithmKind;
use crate::domain::league_calendar::season_timing::SeasonTiming;
use crate::domain::league_calendar::spa_scoring_policy::SpaScoringPolicy;
use crate::domain::league_calendar::stage_definition::StageDefinition;
use crate::domain::league_calendar::tie_break_criterion::TieBreakCriterion;
use crate::error::DomainResult;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct LeagueCalendarConfigBuilder {
    id: Uuid,
    league_id: Uuid,
    algorithm: ScheduleAlgorithmKind,
    timing: SeasonTiming,
    games_per_week: GamesPerWeekPolicy,
    rest_gap_policy: RestGapPolicy,
    postponement: PostponementPolicy,
    neutral_opener: NeutralOpenerPolicy,
    spa_scoring_policy: SpaScoringPolicy,
    qta_weighting_policy: QtaWeightingPolicy,
    tie_break_criteria: Vec<TieBreakCriterion>,
    promotion_relegation_policy: PromotionRelegationPolicy,
    stages: Vec<StageDefinition>,
    groups: Vec<CompetitionGroup>,
    collective_agreement_ids: Vec<Uuid>,
}

impl LeagueCalendarConfigBuilder {
    pub fn new(
        id: Uuid,
        league_id: Uuid,
        algorithm: ScheduleAlgorithmKind,
        timing: SeasonTiming,
        games_per_week: GamesPerWeekPolicy,
        postponement: PostponementPolicy,
    ) -> Self {
        Self {
            id,
            league_id,
            algorithm,
            timing,
            games_per_week,
            rest_gap_policy: RestGapPolicy::default_policy(),
            postponement,
            neutral_opener: NeutralOpenerPolicy::disabled(),
            spa_scoring_policy: SpaScoringPolicy::default_policy(),
            qta_weighting_policy: QtaWeightingPolicy::default_policy(),
            tie_break_criteria: Vec::new(),
            promotion_relegation_policy: PromotionRelegationPolicy::none(),
            stages: Vec::new(),
            groups: Vec::new(),
            collective_agreement_ids: Vec::new(),
        }
    }

    pub fn with_rest_gap_policy(mut self, rest_gap_policy: RestGapPolicy) -> Self {
        self.rest_gap_policy = rest_gap_policy;
        self
    }

    pub fn with_neutral_opener(mut self, neutral_opener: NeutralOpenerPolicy) -> Self {
        self.neutral_opener = neutral_opener;
        self
    }

    pub fn with_spa_scoring_policy(mut self, spa_scoring_policy: SpaScoringPolicy) -> Self {
        self.spa_scoring_policy = spa_scoring_policy;
        self
    }

    pub fn with_qta_weighting_policy(mut self, qta_weighting_policy: QtaWeightingPolicy) -> Self {
        self.qta_weighting_policy = qta_weighting_policy;
        self
    }

    pub fn with_tie_break_criteria(mut self, tie_break_criteria: Vec<TieBreakCriterion>) -> Self {
        self.tie_break_criteria = tie_break_criteria;
        self
    }

    pub fn with_promotion_relegation_policy(
        mut self,
        promotion_relegation_policy: PromotionRelegationPolicy,
    ) -> Self {
        self.promotion_relegation_policy = promotion_relegation_policy;
        self
    }

    pub fn with_stages(mut self, stages: Vec<StageDefinition>) -> Self {
        self.stages = stages;
        self
    }

    pub fn with_groups(mut self, groups: Vec<CompetitionGroup>) -> Self {
        self.groups = groups;
        self
    }

    pub fn with_collective_agreement_ids(mut self, collective_agreement_ids: Vec<Uuid>) -> Self {
        self.collective_agreement_ids = collective_agreement_ids;
        self
    }

    pub fn build(self) -> DomainResult<LeagueCalendarConfig> {
        LeagueCalendarConfig::new(
            self.id,
            self.league_id,
            self.algorithm,
            self.timing,
            self.games_per_week,
            self.rest_gap_policy,
            self.postponement,
            self.neutral_opener,
            self.spa_scoring_policy,
            self.qta_weighting_policy,
            self.tie_break_criteria,
            self.promotion_relegation_policy,
            self.stages,
            self.groups,
            self.collective_agreement_ids,
        )
    }
}
