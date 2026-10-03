use arlo_analytics::context::MatchAnalysisContext;
use arlo_analytics::performance::live::{LiveRatingConfig, PlayerPerformanceAggregator};
use arlo_analytics::performance::rating::PlayerPerformanceSnapshot;
use arlo_engine::MatchInput;
use arlo_events::MatchEventEnvelope;
use crate::error::{ControllerError, ControllerResult};

pub fn evaluate_match_performance(
    input: &MatchInput,
    envelopes: &[MatchEventEnvelope],
    home_score: u32,
    away_score: u32,
) -> ControllerResult<Vec<PlayerPerformanceSnapshot>> {
    let mut context = MatchAnalysisContext::new();

    let home_team_id = input.home().team_id();
    let home_formation = input.home().formation();
    let home_lineup = input.home().lineup();

    for assignment in home_lineup.assignments() {
        let slot_idx = assignment.formation_slot_index();
        if let Some(slot) = home_formation.slots().get(slot_idx) {
            context
                .register_starter(assignment.player_id(), home_team_id, slot_idx, slot)
                .map_err(|e| ControllerError::InvalidData(e.to_string()))?;
        }
    }

    let away_team_id = input.away().team_id();
    let away_formation = input.away().formation();
    let away_lineup = input.away().lineup();

    for assignment in away_lineup.assignments() {
        let slot_idx = assignment.formation_slot_index();
        if let Some(slot) = away_formation.slots().get(slot_idx) {
            context
                .register_starter(assignment.player_id(), away_team_id, slot_idx, slot)
                .map_err(|e| ControllerError::InvalidData(e.to_string()))?;
        }
    }

    let config = LiveRatingConfig::default().with_record_snapshots_automatically(false);
    let mut aggregator = PlayerPerformanceAggregator::with_config(config);
    aggregator.set_context(context);

    aggregator.handle_envelopes(envelopes);

    let format_rules = input.format();
    let reg_duration = (format_rules.regulation_periods()
        * format_rules.regulation_period_duration_minutes()
        * 60) as f64;
    let max_elapsed = envelopes
        .iter()
        .map(|e| e.clock().total_elapsed_seconds())
        .fold(reg_duration, f64::max);
    aggregator.advance_time(max_elapsed);

    aggregator.finalize_with_scores_and_config(home_team_id, home_score, away_team_id, away_score);

    Ok(aggregator.all_player_snapshots())
}