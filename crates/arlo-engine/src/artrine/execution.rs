use crate::artrine::execution_carry::execute_carry;
use crate::artrine::execution_distribution::execute_distribution;
use crate::artrine::execution_finish::{execute_cross_finish, execute_self_finish};
use crate::artrine::execution_outcome::ArtrineExecutionOutcome;
use crate::resolution::DuelContext;
use crate::spatial::DynamicSpatialMap;
use arlo_domain::pitch::Pitch;
use arlo_domain::{ArtrineDecisionKind, AttributeKey, Player, Position as DomainPosition};
use arlo_math::units::Position as VectorPosition;
use rand::Rng;
use std::collections::HashMap;
use uuid::Uuid;

pub fn find_goalguard<'a>(defenders: &[&'a Player]) -> &'a Player {
    defenders
        .iter()
        .copied()
        .find(|p| {
            p.positions()
                .iter()
                .any(|pos| pos.position() == DomainPosition::Goalguard && pos.proficiency() > 0)
        })
        .unwrap_or_else(|| defenders[defenders.len() - 1])
}

pub fn execute_artrine_decision<R: Rng + ?Sized>(
    decision: ArtrineDecisionKind,
    artrine: &Player,
    offense_lineup_players: &[&Player],
    defense_lineup_players: &[&Player],
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
    rng: &mut R,
) -> ArtrineExecutionOutcome {
    let offense_helpers: Vec<&Player> = offense_lineup_players
        .iter()
        .copied()
        .filter(|p| p.id() != artrine.id())
        .collect();

    let goalguard = find_goalguard(defense_lineup_players);

    match decision {
        ArtrineDecisionKind::SelfCarry => execute_carry(
            artrine,
            &offense_helpers,
            defense_lineup_players,
            attribute_keys,
            pitch,
            spatial_map,
            start_pos,
            attacking_positive_x,
            defense_team_id,
            context,
            rng,
        ),
        ArtrineDecisionKind::ShortPass | ArtrineDecisionKind::LongLaunch => execute_distribution(
            decision,
            artrine,
            &offense_helpers,
            defense_lineup_players,
            attribute_keys,
            pitch,
            start_pos,
            attacking_positive_x,
            defense_team_id,
            context,
            rng,
        ),
        ArtrineDecisionKind::Cross => {
            let dist_outcome = execute_distribution(
                decision,
                artrine,
                &offense_helpers,
                defense_lineup_players,
                attribute_keys,
                pitch,
                start_pos,
                attacking_positive_x,
                defense_team_id,
                context,
                rng,
            );

            if dist_outcome.turnover.is_some() {
                dist_outcome
            } else {
                let total_advance = accumulated_advance_mirim + dist_outcome.mirins_advanced;
                let finish_outcome = execute_cross_finish(
                    artrine,
                    offense_lineup_players,
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
                    rng,
                );

                let mut combined_duels = dist_outcome.duels;
                combined_duels.extend(finish_outcome.duels);

                let total_elapsed = (dist_outcome.elapsed_seconds + finish_outcome.elapsed_seconds)
                    .clamp(20.0f64, 35.0f64);

                ArtrineExecutionOutcome {
                    mirins_advanced: dist_outcome.mirins_advanced,
                    drives_recorded: 0,
                    drive_row_indices: Vec::new(),
                    turnover: finish_outcome.turnover,
                    recovering_player_id: finish_outcome.recovering_player_id,
                    scoring_decision: finish_outcome.scoring_decision,
                    elapsed_seconds: total_elapsed,
                    end_position: dist_outcome.end_position,
                    duels: combined_duels,
                }
            }
        }
        ArtrineDecisionKind::SelfFinish => execute_self_finish(
            artrine,
            goalguard,
            attribute_keys,
            offense_team_id,
            defense_team_id,
            drives_in_series,
            accumulated_advance_mirim,
            is_last_down,
            start_pos,
            context,
            rng,
        ),
    }
}