use super::ratings::RatingIndex;
use crate::error::{EngineError, EngineResult};
use crate::input::TeamInput;
use arlo_domain::{AttributeKey, Position, PositionLine};
use rand::Rng;
use rand_chacha::ChaCha8Rng;
use uuid::Uuid;

#[derive(Clone, Copy)]
pub(super) enum ActorRole {
    Carrier,
    Receiver,
    Defender,
}

pub(super) fn select_actor(
    ratings: &RatingIndex,
    team: &TeamInput,
    role: ActorRole,
    exclude: Option<Uuid>,
    rng: &mut ChaCha8Rng,
) -> EngineResult<Uuid> {
    let mut candidates = Vec::with_capacity(team.lineup().assignments().len());
    let mut total_weight = 0.0;
    for assignment in team.lineup().assignments() {
        if Some(assignment.player_id()) == exclude {
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
                (ratings.player_value(team, player_id, AttributeKey::HandsReception)?
                    + ratings.player_value(team, player_id, AttributeKey::Positioning)?)
                    * 0.5
            }
            ActorRole::Defender => {
                (ratings.player_value(team, player_id, AttributeKey::DefensiveContainment)?
                    + ratings.player_value(team, player_id, AttributeKey::Anticipation)?)
                    * 0.5
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
            * (0.5 + aptitude.max(0.0) / 20.0);
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
