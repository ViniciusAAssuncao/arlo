use super::ratings::RatingIndex;
use crate::error::{EngineError, EngineResult};
use crate::input::TeamInput;
use arlo_domain::{AttributeKey, Position, PositionLine};
use arlo_tactics::{PlayCall, SlotAssignment};
use rand::Rng;
use rand_chacha::ChaCha8Rng;
use uuid::Uuid;

#[derive(Clone, Copy)]
pub(super) enum ActorRole {
    Carrier,
    Receiver,
    Defender,
    Shooter,
}

pub(super) fn select_actor(
    ratings: &RatingIndex,
    team: &TeamInput,
    role: ActorRole,
    exclude: Option<Uuid>,
    rng: &mut ChaCha8Rng,
) -> EngineResult<Uuid> {
    select_weighted_actor(ratings, team, role, exclude, rng, |_| Ok(1.0))
}

pub(super) fn select_receiver(
    ratings: &RatingIndex,
    team: &TeamInput,
    holder_id: Uuid,
    previous_holder_id: Option<Uuid>,
    selected_play_call: Option<&PlayCall>,
    has_drive: bool,
    rng: &mut ChaCha8Rng,
) -> EngineResult<Uuid> {
    select_weighted_actor(
        ratings,
        team,
        ActorRole::Receiver,
        Some(holder_id),
        rng,
        |assignment| {
            let return_weight = if Some(assignment.player_id()) == previous_holder_id {
                0.45
            } else {
                1.0
            };
            let route_weight = route_weight(selected_play_call, assignment);
            let threat_weight = if has_drive && assignment.position().line() == PositionLine::OffensiveLine {
                let finisher = 0.5
                    * ratings.player_value(team, assignment.player_id(), AttributeKey::Finishing)?
                    + 0.3
                        * ratings.player_value(
                            team,
                            assignment.player_id(),
                            AttributeKey::Anticipation,
                        )?
                    + 0.2
                        * ratings.player_value(team, assignment.player_id(), AttributeKey::Composure)?;
                (1.0 + (finisher - 10.0) * 0.025).clamp(0.75, 1.25)
            } else {
                1.0
            };
            Ok(return_weight * route_weight * threat_weight)
        },
    )
}

pub(super) fn select_shooter(
    ratings: &RatingIndex,
    team: &TeamInput,
    holder_id: Uuid,
    selected_play_call: Option<&PlayCall>,
    rng: &mut ChaCha8Rng,
) -> EngineResult<Uuid> {
    let emphasis = selected_play_call
        .map(|call| *call.decision_emphasis())
        .unwrap_or_else(|| team.tactics().instructions().default_decision_emphasis());
    select_weighted_actor(ratings, team, ActorRole::Shooter, None, rng, |assignment| {
        let depth = team
            .formation()
            .slots()
            .get(assignment.formation_slot_index())
            .and_then(|slot| slot.pitch_length_ratio())
            .unwrap_or(match assignment.position().line() {
                PositionLine::OffensiveLine => 0.7,
                PositionLine::BackLine => 0.45,
                PositionLine::DefenseLine => 0.2,
                PositionLine::Goalguard => 0.05,
            })
            .clamp(0.0, 1.0);
        let holder_weight = if assignment.player_id() == holder_id {
            0.75 + 1.5 * emphasis.self_finish().value()
        } else {
            1.0
        };
        Ok((0.25 + depth).powi(3)
            * holder_weight
            * route_weight(selected_play_call, assignment))
    })
}

fn route_weight(selected_play_call: Option<&PlayCall>, assignment: &SlotAssignment) -> f64 {
    selected_play_call.map_or(1.0, |call| {
        if call.routes().is_empty() {
            1.0
        } else {
            call.routes()
                .iter()
                .find(|route| route.slot_index() == assignment.formation_slot_index())
                .map_or(0.6, |route| 1.0 + 2.0 * route.read_priority().value())
        }
    })
}

fn select_weighted_actor(
    ratings: &RatingIndex,
    team: &TeamInput,
    role: ActorRole,
    exclude: Option<Uuid>,
    rng: &mut ChaCha8Rng,
    extra_weight: impl Fn(&SlotAssignment) -> EngineResult<f64>,
) -> EngineResult<Uuid> {
    let mut candidates = Vec::with_capacity(team.lineup().assignments().len());
    let mut total_weight = 0.0;
    for assignment in team.lineup().assignments() {
        if Some(assignment.player_id()) == exclude || !ratings.is_active(team, assignment.player_id()) {
            continue;
        }
        let position = assignment.position();
        let player_id = assignment.player_id();
        let aptitude = match role {
            ActorRole::Carrier => {
                (ratings.player_value(team, player_id, AttributeKey::ArloControl)?
                    + ratings.player_value(team, player_id, AttributeKey::Decisions)?)
                    * 0.5
            }
            ActorRole::Receiver => {
                0.35 * ratings.player_value(team, player_id, AttributeKey::HandsReception)?
                    + 0.30 * ratings.player_value(team, player_id, AttributeKey::Positioning)?
                    + 0.20 * ratings.player_value(team, player_id, AttributeKey::Anticipation)?
                    + 0.15 * ratings.player_value(team, player_id, AttributeKey::Acceleration)?
            }
            ActorRole::Defender => {
                (ratings.player_value(team, player_id, AttributeKey::DefensiveContainment)?
                    + ratings.player_value(team, player_id, AttributeKey::Anticipation)?)
                    * 0.5
            }
            ActorRole::Shooter => {
                0.50 * ratings.player_value(team, player_id, AttributeKey::Finishing)?
                    + 0.25 * ratings.player_value(team, player_id, AttributeKey::Positioning)?
                    + 0.15 * ratings.player_value(team, player_id, AttributeKey::Anticipation)?
                    + 0.10 * ratings.player_value(team, player_id, AttributeKey::Composure)?
            }
        };
        let position_weight = match (role, position.line()) {
            (ActorRole::Carrier, PositionLine::OffensiveLine) => 1.5,
            (ActorRole::Carrier, PositionLine::BackLine) => 2.0,
            (ActorRole::Carrier, PositionLine::DefenseLine) => 0.45,
            (ActorRole::Receiver, PositionLine::OffensiveLine) => 2.0,
            (ActorRole::Receiver, PositionLine::BackLine) => 1.5,
            (ActorRole::Receiver, PositionLine::DefenseLine) => 0.55,
            (ActorRole::Defender, PositionLine::DefenseLine) => 2.0,
            (ActorRole::Defender, PositionLine::BackLine) => 1.0,
            (ActorRole::Defender, PositionLine::OffensiveLine) => 0.5,
            (ActorRole::Shooter, _) => 1.0,
            (_, PositionLine::Goalguard) => 0.12,
        };
        let specialist_weight = match (role, position) {
            (ActorRole::Carrier, Position::Artrine) => 2.0,
            (ActorRole::Carrier, Position::Passer) => 1.5,
            (ActorRole::Receiver, Position::Artrine) => 1.5,
            _ => 1.0,
        };
        let tactical_weight = match role {
            ActorRole::Defender => {
                1.0
                    + assignment
                        .player_instructions()
                        .out_of_possession()
                        .engagement_bias()
                        .value()
                        * 0.3
            }
            _ => {
                0.65
                    + assignment
                        .player_instructions()
                        .in_possession()
                        .involvement_priority()
                        .value()
                        * 0.7
            }
        };
        let weight = position_weight
            * specialist_weight
            * tactical_weight
            * (0.5 + aptitude.max(0.0) / 20.0)
            * extra_weight(assignment)?;
        total_weight += weight;
        candidates.push((player_id, total_weight));
    }
    if total_weight <= 0.0 {
        return Err(EngineError::InvalidInput(
            "lineup has no eligible actor".into(),
        ));
    }
    let roll = rng.gen_range(0.0..total_weight);
    Ok(candidates
        .iter()
        .find(|(_, cumulative)| roll < *cumulative)
        .map(|(player_id, _)| *player_id)
        .unwrap_or(candidates[candidates.len() - 1].0))
}
