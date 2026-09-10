use crate::match_decision::play_outcome::DetailedPlayOutcome;
use crate::officiating::{ambiguity_from_duel_outcome, ReviewableCall, ReviewableCallKind};
use crate::possession::TransitionResult;
use crate::resolution::AttributedDuelOutcome;
use crate::world_state::constants::IMMEDIATE_CONTROL_THRESHOLD_SECONDS;
use crate::world_state::play_transition::possession_resolver::determine_countdown_reason;
use crate::world_state::play_transition::publisher::EventPublisher;
use arlo_events::EventSink;
use arlo_math::units::MIRIM_TO_METERS;
use arlo_math::Probability;

pub fn resolve_turnover_and_down_events(
    publisher: &mut EventPublisher<'_, impl EventSink>,
    detailed_outcome: &DetailedPlayOutcome,
    transition_result: &TransitionResult,
    play_duels: &[AttributedDuelOutcome],
    previous_down: u32,
) {
    if let Some(new_offense) = detailed_outcome.turnover {
        publisher.emit_turnover_event(
            detailed_outcome.offense_team_id,
            new_offense,
            detailed_outcome.recovering_player_id,
            detailed_outcome.lost_by_player_id,
            !detailed_outcome.out_of_bounds,
            detailed_outcome.last_valid_possession_point,
        );
    }

    if detailed_outcome.out_of_bounds {
        let was_immediate = detailed_outcome
            .possession_control_seconds
            .map(|s| s < IMMEDIATE_CONTROL_THRESHOLD_SECONDS)
            .unwrap_or(false);
        publisher.emit_out_of_bounds_event(
            detailed_outcome.offense_team_id,
            Some(detailed_outcome.artrine_id),
            detailed_outcome.last_valid_possession_point,
            was_immediate,
        );
    }

    let end_x_mirim = detailed_outcome.last_valid_possession_point.raw().0 / MIRIM_TO_METERS;
    let new_down = transition_result.snapshot.down() as u32;
    let is_possession_change =
        transition_result.snapshot.role().offense() != detailed_outcome.offense_team_id;
    let is_first_down = (transition_result.snapshot.down() == 1 && previous_down > 1)
        || is_possession_change
        || transition_result.snapshot.down() == 1;

    if detailed_outcome.turnover.is_some() || detailed_outcome.out_of_bounds {
        let kind = if detailed_outcome.turnover.is_some() {
            ReviewableCallKind::TurnoverClassification
        } else {
            ReviewableCallKind::OutOfBoundsClassification
        };
        let ambiguity = play_duels
            .last()
            .map(|d| ambiguity_from_duel_outcome(d.outcome()))
            .unwrap_or_else(|| Probability::new_clamped(0.0));
        let on_field_favors_offense =
            detailed_outcome.turnover.is_none() && !is_possession_change;
        let call = ReviewableCall::new(
            kind,
            ambiguity,
            on_field_favors_offense,
            on_field_favors_offense,
        );
        publisher
            .state_mut()
            .set_last_reviewable_call(detailed_outcome.offense_team_id, call);
    }

    publisher.emit_down_advanced_event(
        previous_down,
        new_down,
        detailed_outcome.mirins_advanced,
        transition_result.snapshot.advanced_mirins(),
        is_first_down,
        end_x_mirim,
    );

    if transition_result.countdown_to_size_triggered {
        let reason = determine_countdown_reason(detailed_outcome, is_possession_change);
        publisher.emit_countdown_event(
            transition_result.snapshot.offense(),
            end_x_mirim,
            reason,
        );
    }
}