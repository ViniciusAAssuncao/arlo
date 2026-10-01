use super::actors::{select_actor, ActorRole};
use super::ratings::RatingIndex;
use crate::error::EngineResult;
use crate::input::TeamInput;
use crate::state::MatchState;
use arlo_domain::{AttributeKey, Position, Pitch, PitchZone, SecondZone};
use arlo_events::{GoalguardRecoveryResolved, MatchEvent, MatchEventEnvelope};
use rand::Rng;
use uuid::Uuid;

pub(super) struct Recovery {
    pub player_id: Uuid,
    pub position_mirim: f64,
}

pub(super) fn resolve_recovery(
    ratings: &RatingIndex,
    defense: &TeamInput,
    pitch: Pitch,
    goal_line_mirim: f64,
    state: &mut MatchState,
    events: &mut Vec<MatchEventEnvelope>,
) -> EngineResult<Recovery> {
    let goalguard = active_goalguard_id(ratings, defense);
    let field_player_available = defense.lineup().assignments().iter()
        .any(|assignment| assignment.position() != Position::Goalguard
            && ratings.is_active_slot(defense, assignment.player_id()));
    if let Some(goalguard_id) = goalguard {
        let handling = ratings.player_value(defense, goalguard_id, AttributeKey::Handling)?;
        let rushing = ratings.player_value(defense, goalguard_id, AttributeKey::RushingOut)?;
        let decision = ratings.player_value(defense, goalguard_id, AttributeKey::Decisions)?;
        let keeper_share = (0.60 + (handling - 10.0) * 0.012).clamp(0.40, 0.78);
        if !field_player_available || state.rng_mut().gen_range(0.0..1.0) < keeper_share {
            let outside = state.rng_mut().gen_range(0.0..1.0)
                < (0.06 + (rushing - 10.0) * 0.005).clamp(0.02, 0.12);
            let distance = if outside { 7.5 + state.rng_mut().gen_range(0.0..4.5) }
                else { 1.5 + state.rng_mut().gen_range(0.0..4.5) };
            let position = position_from_goal(goal_line_mirim, pitch.length_mirim(), distance);
            let zone = pitch.zone_at_distance_to_goal(distance, SecondZone::default_awc().depth_mirim());
            let used_hands = if zone == PitchZone::FirstZone {
                state.rng_mut().gen_range(0.0..1.0) < 0.82
            } else {
                state.rng_mut().gen_range(0.0..1.0)
                    < (0.075 + (10.0 - decision) * 0.006).clamp(0.018, 0.14)
            };
            events.push(state.emit(MatchEvent::GoalguardRecoveryResolved(GoalguardRecoveryResolved::new(
                goalguard_id, defense.team_id(), position, zone, used_hands,
            )))?);
            return Ok(Recovery { player_id: goalguard_id, position_mirim: position });
        }
    }
    let player_id = select_actor(ratings, defense, ActorRole::Defender, goalguard, state.rng_mut())?;
    let distance = 5.0 + state.rng_mut().gen_range(0.0..8.0);
    Ok(Recovery {
        player_id,
        position_mirim: position_from_goal(goal_line_mirim, pitch.length_mirim(), distance),
    })
}

fn position_from_goal(goal_line_mirim: f64, pitch_length_mirim: f64, distance: f64) -> f64 {
    if goal_line_mirim > pitch_length_mirim / 2.0 {
        (goal_line_mirim - distance).clamp(0.0, pitch_length_mirim)
    } else {
        (goal_line_mirim + distance).clamp(0.0, pitch_length_mirim)
    }
}

pub(super) fn active_goalguard_id(
    ratings: &RatingIndex,
    defense: &TeamInput,
) -> Option<Uuid> {
    defense
        .lineup()
        .assignments()
        .iter()
        .find(|assignment| {
            assignment.position() == Position::Goalguard
                && ratings.is_active_slot(defense, assignment.player_id())
        })
        .map(|assignment| ratings.slot_player_id(defense, assignment.player_id()))
}
