use crate::error::DbResult;
use crate::models::league_calendar::league_calendar_config_codes::{
    parse_games_per_week_conflict_scope, parse_neutral_opener_selection_strategy,
    parse_postponement_strategy_kind, parse_schedule_algorithm_kind,
};
use arlo_domain::{
    CompetitionGroup, GamesPerWeekPolicy, LeagueCalendarConfig, NeutralOpenerPolicy,
    PostponementPolicy, SeasonTiming, SpaScoringPolicy, StageDefinition,
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
    pub created_at_unix_seconds: i64,
}

impl LeagueCalendarConfigRow {
    pub fn to_domain(
        &self,
        allowed_weekdays: Vec<u32>,
        stages: Vec<StageDefinition>,
        groups: Vec<CompetitionGroup>,
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

        LeagueCalendarConfig::new(
            id,
            league_id,
            algorithm,
            timing,
            games_per_week,
            postponement,
            neutral_opener,
            spa_scoring_policy,
            stages,
            groups,
        )
        .map_err(Into::into)
    }
}
