use crate::error::{DbError, DbResult};
use crate::models::league_calendar::league_calendar_config_codes::{
    parse_games_per_week_conflict_scope, parse_neutral_opener_selection_strategy,
    parse_postponement_strategy_kind, parse_schedule_algorithm_kind,
};
use crate::models::league_calendar::promotion_relegation_codes::{
    parse_league_movement_rule_kind, LeagueMovementRuleKind,
};
use arlo_domain::{
    CompetitionGroup, GamesPerWeekPolicy, LeagueCalendarConfig, LeagueMovementRule,
    NeutralOpenerPolicy, PostponementPolicy, PromotionRelegationPolicy, QtaWeightingPolicy,
    SeasonTiming, SpaScoringPolicy, StageDefinition, TieBreakCriterion,
};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct LeagueCalendarConfigRow {
    pub id: String,
    pub competition_id: String,
    pub schedule_algorithm_kind: String,
    pub season_start_month_order_index: i32,
    pub season_start_day_of_month: i32,
    pub season_length_weeks: i32,
    pub max_games_per_team_per_week: i32,
    pub games_per_week_conflict_scope: String,
    pub postponement_strategy_kind: String,
    pub neutral_opener_enabled: i32,
    pub neutral_opener_selection_strategy: Option<String>,
    pub spa_win_weight: f64,
    pub spa_draw_weight: f64,
    pub spa_loss_weight: f64,
    pub spa_feo_k_factor: f64,
    pub qta_home_win_weight: f64,
    pub qta_away_win_weight: f64,
    pub qta_home_draw_weight: f64,
    pub qta_away_draw_weight: f64,
    pub qta_home_loss_weight: f64,
    pub qta_away_loss_weight: f64,
    pub standings_stage_order_index: i32,
    pub promotion_rule_kind: String,
    pub promotion_count: Option<i32>,
    pub promotion_playoff_stage_order_index: Option<i32>,
    pub promotion_target_league_id: Option<String>,
    pub relegation_rule_kind: String,
    pub relegation_count: Option<i32>,
    pub relegation_playoff_stage_order_index: Option<i32>,
    pub relegation_target_league_id: Option<String>,
    pub created_at_unix_seconds: i64,
}

impl LeagueCalendarConfigRow {
    pub fn to_domain(
        &self,
        allowed_weekdays: Vec<u32>,
        stages: Vec<StageDefinition>,
        groups: Vec<CompetitionGroup>,
        tie_break_criteria: Vec<TieBreakCriterion>,
    ) -> DbResult<LeagueCalendarConfig> {
        let id = Uuid::parse_str(&self.id)?;
        let league_id = Uuid::parse_str(&self.competition_id)?;
        let algorithm = parse_schedule_algorithm_kind(&self.schedule_algorithm_kind)?;
        let timing = SeasonTiming::new(
            self.season_start_month_order_index as u32,
            self.season_start_day_of_month as u32,
            self.season_length_weeks as u32,
            allowed_weekdays,
        )?;
        let conflict_scope =
            parse_games_per_week_conflict_scope(&self.games_per_week_conflict_scope)?;
        let games_per_week =
            GamesPerWeekPolicy::new(self.max_games_per_team_per_week as u32, conflict_scope)?;
        let postponement_strategy =
            parse_postponement_strategy_kind(&self.postponement_strategy_kind)?;
        let postponement = PostponementPolicy::new(postponement_strategy);
        let neutral_opener_strategy = parse_neutral_opener_selection_strategy(
            self.neutral_opener_selection_strategy.as_deref(),
        )?;
        let neutral_opener =
            NeutralOpenerPolicy::new(self.neutral_opener_enabled != 0, neutral_opener_strategy)?;
        let spa_scoring_policy = SpaScoringPolicy::new(
            self.spa_win_weight,
            self.spa_draw_weight,
            self.spa_loss_weight,
            self.spa_feo_k_factor,
        )?;
        let qta_weighting_policy = QtaWeightingPolicy::new(
            self.qta_home_win_weight,
            self.qta_away_win_weight,
            self.qta_home_draw_weight,
            self.qta_away_draw_weight,
            self.qta_home_loss_weight,
            self.qta_away_loss_weight,
        )?;

        let promotion_rule_kind = parse_league_movement_rule_kind(&self.promotion_rule_kind)?;
        let promotion_rule = match promotion_rule_kind {
            LeagueMovementRuleKind::None => LeagueMovementRule::None,
            LeagueMovementRuleKind::Automatic => {
                let count = self.promotion_count.ok_or_else(|| {
                    DbError::InvalidData("Automatic promotion requires promotion_count".to_string())
                })?;
                LeagueMovementRule::Automatic {
                    count: count as u32,
                }
            }
            LeagueMovementRuleKind::PlayoffStage => {
                let stage_idx = self.promotion_playoff_stage_order_index.ok_or_else(|| {
                    DbError::InvalidData(
                        "PlayoffStage promotion requires promotion_playoff_stage_order_index"
                            .to_string(),
                    )
                })?;
                let count = self.promotion_count.ok_or_else(|| {
                    DbError::InvalidData(
                        "PlayoffStage promotion requires promotion_count".to_string(),
                    )
                })?;
                LeagueMovementRule::PlayoffStage {
                    stage_order_index: stage_idx as u32,
                    count: count as u32,
                }
            }
        };

        let promotion_target_league_id = self
            .promotion_target_league_id
            .as_deref()
            .map(Uuid::parse_str)
            .transpose()?;

        let relegation_rule_kind = parse_league_movement_rule_kind(&self.relegation_rule_kind)?;
        let relegation_rule = match relegation_rule_kind {
            LeagueMovementRuleKind::None => LeagueMovementRule::None,
            LeagueMovementRuleKind::Automatic => {
                let count = self.relegation_count.ok_or_else(|| {
                    DbError::InvalidData("Automatic relegation requires relegation_count".to_string())
                })?;
                LeagueMovementRule::Automatic {
                    count: count as u32,
                }
            }
            LeagueMovementRuleKind::PlayoffStage => {
                let stage_idx = self.relegation_playoff_stage_order_index.ok_or_else(|| {
                    DbError::InvalidData(
                        "PlayoffStage relegation requires relegation_playoff_stage_order_index"
                            .to_string(),
                    )
                })?;
                let count = self.relegation_count.ok_or_else(|| {
                    DbError::InvalidData(
                        "PlayoffStage relegation requires relegation_count".to_string(),
                    )
                })?;
                LeagueMovementRule::PlayoffStage {
                    stage_order_index: stage_idx as u32,
                    count: count as u32,
                }
            }
        };

        let relegation_target_league_id = self
            .relegation_target_league_id
            .as_deref()
            .map(Uuid::parse_str)
            .transpose()?;

        let promotion_relegation_policy = PromotionRelegationPolicy::new(
            self.standings_stage_order_index as u32,
            promotion_rule,
            promotion_target_league_id,
            relegation_rule,
            relegation_target_league_id,
        )?;

        LeagueCalendarConfig::new(
            id,
            league_id,
            algorithm,
            timing,
            games_per_week,
            postponement,
            neutral_opener,
            spa_scoring_policy,
            qta_weighting_policy,
            tie_break_criteria,
            promotion_relegation_policy,
            stages,
            groups,
        )
        .map_err(Into::into)
    }
}
