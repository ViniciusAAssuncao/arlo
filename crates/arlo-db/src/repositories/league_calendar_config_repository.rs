use crate::error::DbResult;
use crate::models::league_calendar::{
    CompetitionGroupRow,
    CompetitionGroupTeamRow,
    EntryRulePoolRow,
    LeagueCalendarCollectiveAgreementRow,
    LeagueCalendarConfigRow,
    LeagueCalendarMatchdayWeekdayRow,
    LeagueCalendarStageDefinitionRow,
    ScheduleBlockPoolGroupRow,
    ScheduleBlockRow,
    TieBreakCriterionRow,
};
use crate::repositories::fetch::{fetch_all_by_param, fetch_optional_by_param};
use arlo_domain::{LeagueCalendarConfig, QualificationPoolRule, ScheduleBlock, TieBreakCriterion};
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn get_by_competition_id(
    pool: &SqlitePool,
    competition_id: Uuid,
) -> DbResult<Option<LeagueCalendarConfig>> {
    let config_row = fetch_optional_by_param::<LeagueCalendarConfigRow>(
        pool,
        "SELECT id, competition_id, schedule_algorithm_kind, season_start_month_order_index, season_start_day_of_month, season_length_weeks, max_games_per_team_per_week, games_per_week_conflict_scope, postponement_strategy_kind, neutral_opener_enabled, neutral_opener_selection_strategy, spa_win_weight, spa_draw_weight, spa_loss_weight, spa_feo_k_factor, qta_home_win_weight, qta_away_win_weight, qta_home_draw_weight, qta_away_draw_weight, qta_home_loss_weight, qta_away_loss_weight, standings_stage_order_index, promotion_rule_kind, promotion_count, promotion_playoff_stage_order_index, promotion_target_league_id, relegation_rule_kind, relegation_count, relegation_playoff_stage_order_index, relegation_target_league_id, minimum_rest_gap_days, rest_gap_conflict_scope, created_at_unix_seconds FROM league_calendar_configs WHERE competition_id = ?",
        &competition_id.to_string(),
    )
    .await?;

    let config_row = match config_row {
        Some(row) => row,
        None => {
            return Ok(None);
        }
    };

    let weekday_rows = fetch_all_by_param::<LeagueCalendarMatchdayWeekdayRow>(
        pool,
        "SELECT id, league_calendar_config_id, weekday_order_index FROM league_calendar_matchday_weekdays WHERE league_calendar_config_id = ? ORDER BY weekday_order_index ASC",
        &config_row.id,
    )
    .await?;

    let allowed_weekdays: Vec<u32> = weekday_rows
        .into_iter()
        .map(|row| row.weekday_order_index as u32)
        .collect();

    let criterion_rows = fetch_all_by_param::<TieBreakCriterionRow>(
        pool,
        "SELECT id, league_calendar_config_id, order_index, criterion_kind FROM league_calendar_tie_break_criteria WHERE league_calendar_config_id = ? ORDER BY order_index ASC",
        &config_row.id,
    )
    .await?;

    let mut tie_break_criteria: Vec<TieBreakCriterion> = Vec::with_capacity(criterion_rows.len());
    for criterion_row in criterion_rows {
        tie_break_criteria.push(criterion_row.to_domain()?);
    }

    let group_rows = fetch_all_by_param::<CompetitionGroupRow>(
        pool,
        "SELECT id, league_calendar_config_id, order_index, name FROM competition_groups WHERE league_calendar_config_id = ? ORDER BY order_index ASC",
        &config_row.id,
    )
    .await?;

    let mut groups = Vec::with_capacity(group_rows.len());
    for group_row in group_rows {
        let team_rows = fetch_all_by_param::<CompetitionGroupTeamRow>(
            pool,
            "SELECT id, competition_group_id, team_id FROM competition_group_teams WHERE competition_group_id = ?",
            &group_row.id,
        )
        .await?;

        let mut team_ids = Vec::with_capacity(team_rows.len());
        for team_row in team_rows {
            team_ids.push(team_row.team_id()?);
        }

        groups.push(group_row.to_domain(team_ids)?);
    }

    let stage_rows = fetch_all_by_param::<LeagueCalendarStageDefinitionRow>(
        pool,
        "SELECT id, league_calendar_config_id, stage_order_index, stage_type, leg_format, entry_gap_days FROM league_calendar_stage_definitions WHERE league_calendar_config_id = ? ORDER BY stage_order_index ASC",
        &config_row.id,
    )
    .await?;

    let mut stages = Vec::with_capacity(stage_rows.len());
    for stage_row in stage_rows {
        let pool_rows = fetch_all_by_param::<EntryRulePoolRow>(
            pool,
            "SELECT id, league_calendar_stage_definition_id, pool_order_index, pool_kind, count, position_index, range_start_position, range_end_position, external_competition_id FROM league_calendar_stage_entry_rule_pools WHERE league_calendar_stage_definition_id = ? ORDER BY pool_order_index ASC",
            &stage_row.id,
        )
        .await?;

        let mut entry_rule_pools: Vec<QualificationPoolRule> = Vec::with_capacity(pool_rows.len());
        for pool_row in pool_rows {
            entry_rule_pools.push(pool_row.to_domain()?);
        }

        let block_rows = fetch_all_by_param::<ScheduleBlockRow>(
            pool,
            "SELECT id, league_calendar_stage_definition_id, block_order_index, block_kind, group_a_id, group_b_id, mirrored, pool_kind, rounds_count FROM league_calendar_stage_schedule_blocks WHERE league_calendar_stage_definition_id = ? ORDER BY block_order_index ASC",
            &stage_row.id,
        )
        .await?;

        let schedule_blocks: Option<Vec<ScheduleBlock>> = if block_rows.is_empty() {
            None
        } else {
            let mut blocks = Vec::with_capacity(block_rows.len());
            for block_row in block_rows {
                let pool_group_rows = fetch_all_by_param::<ScheduleBlockPoolGroupRow>(
                    pool,
                    "SELECT id, schedule_block_id, group_id FROM schedule_block_pool_groups WHERE schedule_block_id = ?",
                    &block_row.id,
                )
                .await?;

                let mut pool_group_ids = Vec::with_capacity(pool_group_rows.len());
                for pg_row in pool_group_rows {
                    pool_group_ids.push(pg_row.group_id()?);
                }

                blocks.push(block_row.to_domain(pool_group_ids)?);
            }
            Some(blocks)
        };

        stages.push(stage_row.to_domain(schedule_blocks, entry_rule_pools)?);
    }

    let collective_agreement_rows = fetch_all_by_param::<LeagueCalendarCollectiveAgreementRow>(
        pool,
        "SELECT id, league_calendar_config_id, collective_agreement_id FROM league_calendar_collective_agreements WHERE league_calendar_config_id = ?",
        &config_row.id,
    )
    .await?;

    let mut collective_agreement_ids = Vec::with_capacity(collective_agreement_rows.len());
    for ca_row in collective_agreement_rows {
        collective_agreement_ids.push(ca_row.collective_agreement_id()?);
    }

    let domain = config_row.to_domain(
        allowed_weekdays,
        stages,
        groups,
        tie_break_criteria,
        collective_agreement_ids,
    )?;
    Ok(Some(domain))
}