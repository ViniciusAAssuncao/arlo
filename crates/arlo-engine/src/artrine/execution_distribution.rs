use crate::artrine::execution_outcome::ArtrineExecutionOutcome;
use crate::artrine::execution_security::resolve_ball_security;
use crate::match_decision::scoring::ScoringDecision;
use crate::resolution::aggregate_progression::AggregateProgressionStrategy;
use crate::resolution::duel_profiles::get_duel_profiles;
use crate::resolution::duel_timing::{derive_duel_duration, nearest_opponent};
use crate::resolution::group_rating::{
    calculate_anchored_side_rating_from_index, calculate_side_rating_from_index,
};
use crate::resolution::progression_strategy::ProgressionResolutionStrategy;
use crate::resolution::resolver::resolve_duel;
use crate::resolution::{DuelContext, DuelKind};
use crate::spatial::ball_kinematics::{
    ball_flight_duration, calculate_cross_speed, calculate_pass_speed,
};
use crate::spatial::decision_vector::calculate_player_speed;
use crate::spatial::proximity::calculate_distance_mirim;
use crate::spatial::DynamicSpatialMap;
use crate::time::{DurationComponentKind, DurationLedger};
use arlo_domain::pitch::Pitch;
use arlo_domain::sport_constants::{MINIMUM_ENGAGEMENT_SECONDS, PROXIMITY_CONTEST_RADIUS_MIRIM};
use arlo_domain::{ArtrineDecisionKind, AttributeKey, Player, Position as DomainPosition};
use arlo_math::units::{Duration, Position as VectorPosition, MIRIM_TO_METERS};
use rand::Rng;
use std::collections::HashMap;
use uuid::Uuid;

pub fn execute_distribution<R: Rng + ?Sized>(
    decision_kind: ArtrineDecisionKind,
    artrine: &Player,
    offense_helpers: &[&Player],
    offense_position_index: &HashMap<Uuid, DomainPosition>,
    defenders: &[&Player],
    defense_position_index: &HashMap<Uuid, DomainPosition>,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    pitch: &Pitch,
    spatial_map: &DynamicSpatialMap,
    start_pos: VectorPosition,
    attacking_positive_x: bool,
    defense_team_id: Uuid,
    context: &DuelContext,
    rng: &mut R,
) -> ArtrineExecutionOutcome {
    let duel_kind = match decision_kind {
        ArtrineDecisionKind::ShortPass => DuelKind::ShortDistribution,
        ArtrineDecisionKind::LongLaunch => DuelKind::LongDistribution,
        ArtrineDecisionKind::Cross => DuelKind::CrossDistribution,
        _ => DuelKind::ShortDistribution,
    };

    let (offense_profile, defense_profile) = get_duel_profiles(duel_kind);
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
    let dist_duel = resolve_duel(duel_kind, attacker_rating, defender_rating, context, rng);

    let artrine_speed = calculate_player_speed(artrine, attribute_keys);
    let (dist_duration, nearest_def_opt) = match nearest_opponent(start_pos, defenders, spatial_map) {
        Some((d, pos)) => {
            let d_spd = calculate_player_speed(d, attribute_keys);
            (
                derive_duel_duration(start_pos, artrine_speed, pos, d_spd),
                Some((d, pos)),
            )
        }
        None => (Duration::new(MINIMUM_ENGAGEMENT_SECONDS), None),
    };

    if !dist_duel.attacker_won() {
        let mut ledger = DurationLedger::new();
        ledger.record_live(
            DurationComponentKind::DistributionEngagement,
            dist_duration,
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
                DuelKind::BallSecurityDistribution,
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
                vec![dist_duel, sec_result.duel_outcome],
            )
        } else {
            (None, None, vec![dist_duel])
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
    let raw_advance = progression_strategy.resolve_progression(&dist_duel, rng);

    let start_x_mirim = start_pos.raw().0 / MIRIM_TO_METERS;
    let end_x_mirim = if attacking_positive_x {
        (start_x_mirim + raw_advance).min(pitch.length_mirim())
    } else {
        (start_x_mirim - raw_advance).max(0.0)
    };

    let mirins_advanced = (end_x_mirim - start_x_mirim).abs();
    let end_position = VectorPosition::from_components(
        end_x_mirim * MIRIM_TO_METERS,
        start_pos.raw().1,
        0.0,
    );

    let ball_speed = match decision_kind {
        ArtrineDecisionKind::Cross => calculate_cross_speed(artrine, attribute_keys),
        _ => calculate_pass_speed(artrine, attribute_keys),
    };
    let flight_duration = ball_flight_duration(mirins_advanced, ball_speed);

    let mut ledger = DurationLedger::new();
    ledger.record_live(
        DurationComponentKind::DistributionEngagement,
        dist_duration,
    );
    ledger.record_live(
        DurationComponentKind::DistributionFlight,
        flight_duration,
    );

    ArtrineExecutionOutcome {
        mirins_advanced,
        drives_recorded: 0,
        drive_row_indices: Vec::new(),
        turnover: None,
        recovering_player_id: None,
        scoring_decision: ScoringDecision::NoOpportunity,
        duration_ledger: ledger,
        end_position,
        duels: vec![dist_duel],
    }
}
