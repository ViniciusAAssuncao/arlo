use super::actors::select_primary_blocker;
use super::ratings::RatingIndex;
use crate::error::EngineResult;
use crate::input::TeamInput;
use crate::state::MatchState;
use arlo_domain::AttributeKey;
use arlo_events::{DuelKind, DuelResolved, MatchEvent, MatchEventEnvelope, PasserContactResolved};
use arlo_math::Probability;
use rand::Rng;
use uuid::Uuid;

pub(super) fn resolve_after_release(
    ratings: &RatingIndex,
    offense: &TeamInput,
    defense: &TeamInput,
    passer_id: Uuid,
    defender_id: Uuid,
    state: &mut MatchState,
    events: &mut Vec<MatchEventEnvelope>,
) -> EngineResult<()> {
    let pressure = ratings.player_value(defense, defender_id, AttributeKey::PasserPressure)?;
    let aggression = ratings.player_value(defense, defender_id, AttributeKey::Aggressiveness)?;
    let blocking = ratings.active_average(offense, AttributeKey::OffensiveBlocking, true)?;
    let encounter_probability =
        (0.035 + (pressure - blocking) * 0.002).clamp(0.012, 0.075);
    let defender_broke_through =
        state.rng_mut().gen_range(0.0..1.0) < encounter_probability;
    let blocker_id = select_primary_blocker(ratings, offense)?;
    let blocker_win_probability = 1.0 - encounter_probability;
    let surprise = if defender_broke_through {
        blocker_win_probability
    } else {
        encounter_probability
    };
    let net_advantage = if defender_broke_through {
        -(surprise * 2.0)
    } else {
        surprise * 2.0
    };

    events.push(state.emit(MatchEvent::DuelResolved(DuelResolved::single(
        DuelKind::PassProtection,
        blocker_id,
        defender_id,
        !defender_broke_through,
        Probability::new_clamped(blocker_win_probability),
        net_advantage,
    )))?);

    if !defender_broke_through {
        return Ok(());
    }

    let late_probability = (0.12 + (aggression - 10.0) * 0.009).clamp(0.035, 0.24);
    let late = state.rng_mut().gen_range(0.0..1.0) < late_probability;
    let rough = late && state.rng_mut().gen_range(0.0..1.0)
        < (0.22 + (aggression - 10.0) * 0.008).clamp(0.10, 0.35);
    let violent = rough && state.rng_mut().gen_range(0.0..1.0)
        < (0.08 + (aggression - 10.0) * 0.006).clamp(0.025, 0.16);
    events.push(state.emit(MatchEvent::PasserContactResolved(PasserContactResolved::new(
        passer_id, defender_id, late, rough, violent,
    )))?);
    Ok(())
}
