use crate::possession::TouchActionType;
use crate::time::DurationComponentKind;
use crate::world_state::cta_pass::PassPhaseResult;
use crate::world_state::match_state::MatchState;
use crate::world_state::step::open_play_loop::action_context::OpenPlayIterationContext;
use crate::world_state::step::open_play_loop::distribution_reception::resolve_distribution_reception;
use crate::world_state::step::open_play_loop::distribution_scoring::check_distribution_scoring_opportunity;
use crate::world_state::step::open_play_loop::loop_state::OpenPlayLoopState;
use crate::world_state::step::setup::CallToActionContext;
use arlo_domain::{ArtrineDecisionKind, Player};
use rand::Rng;

pub fn execute_distribution_action<R: Rng + ?Sized>(
    state: &mut MatchState,
    context: &CallToActionContext,
    iter_ctx: &OpenPlayIterationContext<'_>,
    pass_phase: &PassPhaseResult<'_>,
    loop_state: &mut OpenPlayLoopState,
    current_carrier: &Player,
    defense_players: &[&Player],
    chosen_decision: ArtrineDecisionKind,
    rng: &mut R,
) {
    let pitch = *state.pitch();
    let carrier_pos = loop_state.current_carrier_pos;

    let action_type = if chosen_decision == ArtrineDecisionKind::ShortPass {
        TouchActionType::ShortPass
    } else {
        TouchActionType::LongLaunch
    };
    let zone = pitch.zone_at_position(carrier_pos);
    let current_time = state.clock().seconds_in_period();
    state.possession_mut().live_sequence_mut().record_touch(
        current_carrier.id(),
        action_type,
        zone,
        current_time,
    );

    let result = resolve_distribution_reception(
        state,
        context,
        iter_ctx,
        current_carrier,
        defense_players,
        chosen_decision,
        carrier_pos,
        rng,
    );

    loop_state.accumulated_duels.extend(result.duels);
    loop_state.accumulated_duration_ledger.record_live(
        DurationComponentKind::DistributionEngagement,
        result.engagement_duration,
    );

    if result.flight_duration.value() > 0.0 {
        loop_state.accumulated_duration_ledger.record_live(
            DurationComponentKind::DistributionFlight,
            result.flight_duration,
        );
    }

    loop_state.last_distribution_flight = result.flight_info;

    if !result.caught {
        loop_state.ball_in_play = false;
        if result.turnover_team.is_some() {
            loop_state.turnover_team = result.turnover_team;
            loop_state.recovering_player = result.recovering_player;
        }
        return;
    }

    let rec_zone = pitch.zone_at_position(result.reception_point);
    state.possession_mut().live_sequence_mut().record_touch(
        result.receiver.id(),
        TouchActionType::Reception,
        rec_zone,
        current_time + result.flight_duration.value(),
    );

    loop_state.last_receiver_id = Some(result.receiver.id());
    loop_state.accumulated_mirins_advanced += result.advance_mirim;
    loop_state.current_carrier_id = result.receiver.id();
    loop_state.current_carrier_pos = result.reception_point;

    check_distribution_scoring_opportunity(
        state,
        context,
        iter_ctx,
        pass_phase,
        loop_state,
        result.receiver,
        current_carrier,
        defense_players,
        result.receiver_rating,
        rng,
    );
}
