use crate::artrine::execution_outcome::ArtrineExecutionOutcome;
use crate::artrine::execution_security::resolve_ball_security;
use crate::match_decision::scoring::ScoringDecision;
use crate::possession::drive::artrine_identity::TrueArtrine;
use crate::possession::drive::validator::validate_drive;
use crate::resolution::aggregate_progression::AggregateProgressionStrategy;
use crate::resolution::duel_profiles::get_duel_profiles;
use crate::resolution::group_rating::{calculate_anchored_side_rating, calculate_side_rating};
use crate::resolution::progression_strategy::ProgressionResolutionStrategy;
use crate::resolution::resolver::resolve_duel;
use crate::resolution::{DuelContext, DuelKind};
use crate::spatial::proximity::calculate_distance_mirim;
use crate::spatial::run_spatial_tick_loop;
use crate::spatial::DynamicSpatialMap;
use arlo_domain::pitch::{artro_rows_for_pitch, Pitch};
use arlo_domain::sport_constants::PROXIMITY_CONTEST_RADIUS_MIRIM;
use arlo_domain::{AttributeKey, Player};
use arlo_math::units::{Position as VectorPosition, MIRIM_TO_METERS};
use rand::Rng;
use std::collections::HashMap;
use uuid::Uuid;

pub fn execute_carry<R: Rng + ?Sized>(
    artrine: &Player,
    offense_helpers: &[&Player],
    defenders: &[&Player],
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    pitch: &Pitch,
    spatial_map: &mut DynamicSpatialMap,
    start_pos: VectorPosition,
    attacking_positive_x: bool,
    defense_team_id: Uuid,
    context: &DuelContext,
    rng: &mut R,
) -> ArtrineExecutionOutcome {
    let (offense_profile, defense_profile) = get_duel_profiles(DuelKind::ArtroBreakthrough);
    let attacker_rating = calculate_anchored_side_rating(
        artrine,
        offense_helpers,
        attribute_keys,
        &offense_profile,
    );
    let defender_rating = calculate_side_rating(defenders, attribute_keys, &defense_profile);
    let artro_duel = resolve_duel(
        DuelKind::ArtroBreakthrough,
        attacker_rating,
        defender_rating,
        context,
        rng,
    );

    if !artro_duel.attacker_won() {
        let close_defenders: Vec<&Player> = defenders
            .iter()
            .copied()
            .filter(|d| {
                spatial_map
                    .get_position(&d.id())
                    .map(|pos| calculate_distance_mirim(start_pos, pos) < PROXIMITY_CONTEST_RADIUS_MIRIM)
                    .unwrap_or(false)
            })
            .collect();

        let (turnover, recovering_player_id, duels) = if !close_defenders.is_empty() {
            let sec_result = resolve_ball_security(
                DuelKind::BallSecurityCarry,
                artrine,
                &close_defenders,
                attribute_keys,
                defense_team_id,
                context,
                rng,
            );
            (
                sec_result.turnover_team_id,
                sec_result.recovering_player_id,
                vec![artro_duel, sec_result.duel_outcome],
            )
        } else {
            (None, None, vec![artro_duel])
        };

        let elapsed_seconds = (8.0f64 + rng.gen_range(0.0f64..6.0f64)).clamp(8.0f64, 14.0f64);
        return ArtrineExecutionOutcome {
            mirins_advanced: 0.0,
            drives_recorded: 0,
            drive_row_indices: Vec::new(),
            turnover,
            recovering_player_id,
            scoring_decision: ScoringDecision::NoOpportunity,
            elapsed_seconds,
            end_position: start_pos,
            duels,
        };
    }

    let progression_strategy = AggregateProgressionStrategy::default();
    let raw_advance = progression_strategy.resolve_progression(&artro_duel, rng);

    let start_x_mirim = start_pos.raw().0 / MIRIM_TO_METERS;
    let end_x_mirim = if attacking_positive_x {
        (start_x_mirim + raw_advance).min(pitch.length_mirim())
    } else {
        (start_x_mirim - raw_advance).max(0.0)
    };

    let target_pos = VectorPosition::from_components(
        end_x_mirim * MIRIM_TO_METERS,
        start_pos.raw().1,
        0.0,
    );

    spatial_map.set_position(artrine.id(), start_pos);
    let tick_result =
        run_spatial_tick_loop(spatial_map, &[(artrine, target_pos)], attribute_keys);

    let end_position = spatial_map
        .get_position(&artrine.id())
        .unwrap_or(target_pos);
    let mirins_advanced = (end_x_mirim - start_x_mirim).abs();

    let all_rows = artro_rows_for_pitch(pitch);
    let true_artrine = TrueArtrine::new(artrine.id());
    let mut drive_row_indices = Vec::new();

    let trajectory = tick_result.get_trajectory(&artrine.id());
    let segments = if let Some(traj) = trajectory {
        traj.segments()
    } else {
        Vec::new()
    };

    let segments_to_test = if segments.is_empty() {
        vec![(start_pos, end_position)]
    } else {
        segments
    };

    for (seg_start, seg_end) in segments_to_test {
        for row in &all_rows {
            if drive_row_indices.contains(&row.row_index()) {
                continue;
            }
            for artro in row.artros() {
                let drive_result = validate_drive(true_artrine, seg_start, seg_end, artro, false);
                if drive_result.is_valid() {
                    drive_row_indices.push(row.row_index());
                    break;
                }
            }
        }
    }

    let drives_recorded = drive_row_indices.len() as u32;

    let elapsed_seconds = (14.0f64
        + tick_result.elapsed_seconds() * 1.5f64
        + mirins_advanced * 0.6f64
        + rng.gen_range(0.0f64..3.0f64))
    .clamp(15.0f64, 28.0f64);

    ArtrineExecutionOutcome {
        mirins_advanced,
        drives_recorded,
        drive_row_indices,
        turnover: None,
        recovering_player_id: None,
        scoring_decision: ScoringDecision::NoOpportunity,
        elapsed_seconds,
        end_position,
        duels: vec![artro_duel],
    }
}