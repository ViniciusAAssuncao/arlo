use crate::error::MatchRunnerResult;
use arlo_analytics::{MatchAnalysisContext, PlayerPerformanceAggregator};
use arlo_engine::{MatchInput, MatchPhase, MatchState};
use arlo_stats::AggregatorRegistry;

pub fn build_default_aggregator_registry(
    input: &MatchInput,
) -> MatchRunnerResult<AggregatorRegistry> {
    let mut registry = AggregatorRegistry::with_default_aggregators();
    let context = build_analysis_context(input)?;
    registry.register_aggregator(PlayerPerformanceAggregator::with_context(context));
    Ok(registry)
}

pub(crate) fn finalize_player_performance(
    input: &MatchInput,
    state: &MatchState,
    registry: &mut AggregatorRegistry,
) {
    if state.phase() != MatchPhase::Finished {
        return;
    }

    let Some(aggregator) = registry.get_mut::<PlayerPerformanceAggregator>() else {
        return;
    };

    aggregator.advance_time(state.clock().total_elapsed_seconds());
    aggregator.finalize_with_scores_and_config(
        input.home().team_id(),
        state.home().score().total_points(),
        input.away().team_id(),
        state.away().score().total_points(),
    );
}

fn build_analysis_context(input: &MatchInput) -> MatchRunnerResult<MatchAnalysisContext> {
    let mut context = MatchAnalysisContext::new();
    register_team(&mut context, input.home())?;
    register_team(&mut context, input.away())?;
    Ok(context)
}

fn register_team(
    context: &mut MatchAnalysisContext,
    team: &arlo_engine::TeamInput,
) -> MatchRunnerResult<()> {
    for assignment in team.lineup().assignments() {
        let slot_index = assignment.formation_slot_index();
        let slot = team.formation().slots().get(slot_index).ok_or_else(|| {
            arlo_analytics::AnalyticsError::InvalidData(format!(
                "Formation slot {} is missing for player {}",
                slot_index,
                assignment.player_id()
            ))
        })?;

        context.register_starter_assignment(
            assignment.player_id(),
            team.team_id(),
            slot_index,
            slot.offensive_position(),
            slot.defensive_position(),
            assignment.slot_role(),
        )?;
    }

    Ok(())
}
