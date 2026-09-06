use crate::artrine::execution_carry::execute_carry;
use crate::artrine::execution_distribution::execute_distribution;
use crate::artrine::execution_finish::{execute_cross_finish, execute_self_finish};
use crate::artrine::execution_outcome::ArtrineExecutionOutcome;
use crate::error::{EngineError, EngineResult};
use crate::fatigue::FatigueState;
use crate::resolution::DuelContext;
use crate::spatial::DynamicSpatialMap;
use arlo_domain::pitch::Pitch;
use arlo_domain::{ArtrineDecisionKind, AttributeKey, Player, Position as DomainPosition};
use arlo_math::units::Position as VectorPosition;
use rand::Rng;
use std::collections::HashMap;
use uuid::Uuid;

pub fn find_goalguard<'a>(defenders: &[&'a Player]) -> EngineResult<&'a Player> {
    defenders
        .iter()
        .copied()
        .find(|p| {
            p.positions()
                .iter()
                .any(|pos| pos.position() == DomainPosition::Goalguard && pos.proficiency() > 0)
        })
        .ok_or_else(|| EngineError::MissingRequiredPosition("Goalguard".to_string()))
}

pub fn execute_artrine_decision<F, R>(
    decision: ArtrineDecisionKind,
    artrine: &Player,
    offense_lineup_players: &[&Player],
    offense_position_index: &HashMap<Uuid, DomainPosition>,
    defense_lineup_players: &[&Player],
    defense_position_index: &HashMap<Uuid, DomainPosition>,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    pitch: &Pitch,
    spatial_map: &mut DynamicSpatialMap,
    start_pos: VectorPosition,
    attacking_positive_x: bool,
    offense_team_id: Uuid,
    defense_team_id: Uuid,
    drives_in_series: u32,
    accumulated_advance_mirim: f64,
    is_last_down: bool,
    context: &DuelContext,
    fatigue_for: &F,
    rng: &mut R,
) -> EngineResult<ArtrineExecutionOutcome>
where
    F: Fn(&Uuid) -> FatigueState,
    R: Rng + ?Sized,
{
    let offense_helpers: Vec<&Player> = offense_lineup_players
        .iter()
        .copied()
        .filter(|p| p.id() != artrine.id())
        .collect();

    let goalguard = find_goalguard(defense_lineup_players)?;

    let outcome = match decision {
        ArtrineDecisionKind::SelfCarry => execute_carry(
            artrine,
            &offense_helpers,
            offense_position_index,
            defense_lineup_players,
            defense_position_index,
            attribute_keys,
            pitch,
            spatial_map,
            start_pos,
            attacking_positive_x,
            defense_team_id,
            context,
            fatigue_for,
            rng,
        ),
        ArtrineDecisionKind::ShortPass | ArtrineDecisionKind::LongLaunch => execute_distribution(
            decision,
            artrine,
            &offense_helpers,
            offense_position_index,
            defense_lineup_players,
            defense_position_index,
            attribute_keys,
            pitch,
            spatial_map,
            start_pos,
            attacking_positive_x,
            defense_team_id,
            context,
            fatigue_for,
            rng,
        ),
        ArtrineDecisionKind::Cross => {
            let dist_outcome = execute_distribution(
                decision,
                artrine,
                &offense_helpers,
                offense_position_index,
                defense_lineup_players,
                defense_position_index,
                attribute_keys,
                pitch,
                spatial_map,
                start_pos,
                attacking_positive_x,
                defense_team_id,
                context,
                fatigue_for,
                rng,
            );

            if dist_outcome.turnover.is_some() {
                dist_outcome
            } else {
                let total_advance = accumulated_advance_mirim + dist_outcome.mirins_advanced;
                let finish_outcome = execute_cross_finish(
                    artrine,
                    offense_lineup_players,
                    offense_position_index,
                    goalguard,
                    spatial_map,
                    pitch,
                    attribute_keys,
                    offense_team_id,
                    defense_team_id,
                    drives_in_series,
                    total_advance,
                    is_last_down,
                    attacking_positive_x,
                    dist_outcome.end_position,
                    context,
                    fatigue_for,
                    rng,
                );

                let mut combined_duels = dist_outcome.duels;
                combined_duels.extend(finish_outcome.duels);

                let mut combined_ledger = dist_outcome.duration_ledger;
                combined_ledger.merge(finish_outcome.duration_ledger);

                ArtrineExecutionOutcome {
                    mirins_advanced: dist_outcome.mirins_advanced,
                    drives_recorded: 0,
                    drive_row_indices: Vec::new(),
                    turnover: finish_outcome.turnover,
                    recovering_player_id: finish_outcome.recovering_player_id,
                    scoring_decision: finish_outcome.scoring_decision,
                    duration_ledger: combined_ledger,
                    end_position: dist_outcome.end_position,
                    duels: combined_duels,
                    receiver_id: dist_outcome.receiver_id,
                    distribution_flight: dist_outcome.distribution_flight,
                }
            }
        }
        ArtrineDecisionKind::SelfFinish => execute_self_finish(
            artrine,
            goalguard,
            spatial_map,
            pitch,
            attribute_keys,
            offense_team_id,
            defense_team_id,
            drives_in_series,
            accumulated_advance_mirim,
            is_last_down,
            attacking_positive_x,
            start_pos,
            context,
            fatigue_for,
            rng,
        ),
    };

    Ok(outcome)
}