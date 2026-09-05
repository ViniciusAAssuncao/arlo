use crate::artrine::execution_outcome::ArtrineExecutionOutcome;
use crate::artrine::execution_security::resolve_ball_security;
use crate::match_decision::scoring::ScoringDecision;
use crate::possession::drive::artrine_identity::TrueArtrine;
use crate::possession::drive::validator::validate_drive;
use crate::resolution::aggregate_progression::AggregateProgressionStrategy;
use crate::resolution::duel_profiles::get_duel_profiles;
use crate::resolution::duel_timing::{derive_duel_duration, nearest_opponent};
use crate::resolution::group_rating::{
    calculate_anchored_side_rating_from_index, calculate_side_rating_from_index,
    identify_lead_player_from_index,
};
use crate::resolution::progression_strategy::ProgressionResolutionStrategy;
use crate::resolution::resolver::resolve_duel;
use crate::resolution::{DuelContext, DuelKind};
use crate::spatial::decision_vector::calculate_player_speed;
use crate::spatial::proximity::calculate_distance_mirim;
use crate::spatial::run_spatial_tick_loop;
use crate::spatial::DynamicSpatialMap;
use crate::time::{DurationComponentKind, DurationLedger};
use arlo_domain::pitch::{artro_rows_for_pitch, Pitch};
use arlo_domain::sport_constants::{MINIMUM_ENGAGEMENT_SECONDS, PROXIMITY_CONTEST_RADIUS_MIRIM};
use arlo_domain::{AttributeKey, Player, Position as DomainPosition};
use arlo_math::units::{Duration, Position as VectorPosition, MIRIM_TO_METERS};
use rand::Rng;
use std::collections::HashMap;
use uuid::Uuid;

pub fn execute_carry<R: Rng + ?Sized>(
    artrine: &Player,
    offense_helpers: &[&Player],
    offense_position_index: &HashMap<Uuid, DomainPosition>,
    defenders: &[&Player],
    defense_position_index: &HashMap<Uuid, DomainPosition>,
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
    let attacker_rating = calculate_anchored_side_rating_from_index(
        artrine,
        DomainPosition::Artrine,
        offense_helpers,
        offense_position_index,
        attribute_keys,
        &offense_profile,
    );
    let defender_rating = calculate_side_rating_from_index(
        defenders,
        defense_position_index,
        attribute_keys,
        &defense_profile,
    );
    let lead_defender = identify_lead_player_from_index(
        defenders,
        defense_position_index,
        attribute_keys,
        &defense_profile,
    )
    .unwrap_or(defenders[0]);

    let artro_duel = resolve_duel(
        DuelKind::ArtroBreakthrough,
        attacker_rating,
        defender_rating,
        artrine,
        lead_defender,
        attribute_keys,
        context,
        rng,
    );

    let artrine_speed = calculate_player_speed(artrine, attribute_keys);
    let (artro_duration, nearest_def_opt) = match nearest_opponent(start_pos, defenders, spatial_map) {
        Some((d, pos)) => {
            let d_spd = calculate_player_speed(d, attribute_keys);
            (
                derive_duel_duration(start_pos, artrine_speed, pos, d_spd),
                Some((d, pos)),
            )
        }
        None => (Duration::new(MINIMUM_ENGAGEMENT_SECONDS), None),
    };

    if !artro_duel.attacker_won() {
        let mut ledger = DurationLedger::new();
        ledger.record_live(
            DurationComponentKind::ArtroBreakthroughEngagement,
            artro_duration,
        );

        let (close_defenders, closest_def_info) = match nearest_def_opt {
            Some((d, pos)) if calculate_distance_mirim(start_pos, pos) <= PROXIMITY_CONTEST_RADIUS_MIRIM => {
                let list: Vec<&Player> = defenders
                    .iter()
                    .copied()
                    .filter(|cand| {
                        spatial_map
                            .get_position(&cand.id())
                            .map(|p| calculate_distance_mirim(start_pos, p) <= PROXIMITY_CONTEST_RADIUS_MIRIM)
                            .unwrap_or(false)
                    })
                    .collect();
                (list, Some((d, pos)))
            }
            _ => (Vec::new(), None),
        };

        let (turnover, recovering_player_id, duels) = if !close_defenders.is_empty() {
            if let Some((closest_def, closest_pos)) = closest_def_info {
                let closest_def_speed = calculate_player_speed(closest_def, attribute_keys);
                let sec_duration = derive_duel_duration(
                    start_pos,
                    artrine_speed,
                    closest_pos,
                    closest_def_speed,
                );
                ledger.record_live(
                    DurationComponentKind::BallSecurityEngagement,
                    sec_duration,
                );
            }
            let sec_result = resolve_ball_security(
                DuelKind::BallSecurityCarry,
                artrine,
                DomainPosition::Artrine,
                &close_defenders,
                defense_position_index,
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

        return ArtrineExecutionOutcome {
            mirins_advanced: 0.0,
            drives_recorded: 0,
            drive_row_indices: Vec::new(),
            turnover,
            recovering_player_id,
            scoring_decision: ScoringDecision::NoOpportunity,
            duration_ledger: ledger,
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

    let mut ledger = DurationLedger::new();
    ledger.record_live(
        DurationComponentKind::ArtroBreakthroughEngagement,
        artro_duration,
    );
    ledger.record_live(
        DurationComponentKind::CarrierMovement,
        Duration::new(tick_result.elapsed_seconds()),
    );

    ArtrineExecutionOutcome {
        mirins_advanced,
        drives_recorded,
        drive_row_indices,
        turnover: None,
        recovering_player_id: None,
        scoring_decision: ScoringDecision::NoOpportunity,
        duration_ledger: ledger,
        end_position,
        duels: vec![artro_duel],
    }
}