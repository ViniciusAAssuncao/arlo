use crate::artrine::execution_outcome::ArtrineExecutionOutcome;
use crate::artrine::execution_security::resolve_ball_security;
use crate::match_decision::scoring::ScoringDecision;
use crate::resolution::aggregate_progression::AggregateProgressionStrategy;
use crate::resolution::duel_profiles::get_duel_profiles;
use crate::resolution::group_rating::{calculate_anchored_side_rating, calculate_side_rating};
use crate::resolution::progression_strategy::ProgressionResolutionStrategy;
use crate::resolution::resolver::resolve_duel;
use crate::resolution::{DuelContext, DuelKind};
use arlo_domain::pitch::Pitch;
use arlo_domain::{ArtrineDecisionKind, AttributeKey, Player};
use arlo_math::units::{Position as VectorPosition, MIRIM_TO_METERS};
use rand::Rng;
use std::collections::HashMap;
use uuid::Uuid;

pub fn execute_distribution<R: Rng + ?Sized>(
    decision_kind: ArtrineDecisionKind,
    artrine: &Player,
    offense_helpers: &[&Player],
    defenders: &[&Player],
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    pitch: &Pitch,
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
    let attacker_rating = calculate_anchored_side_rating(
        artrine,
        offense_helpers,
        attribute_keys,
        &offense_profile,
    );
    let defender_rating = calculate_side_rating(defenders, attribute_keys, &defense_profile);
    let dist_duel = resolve_duel(duel_kind, attacker_rating, defender_rating, context, rng);

    if !dist_duel.attacker_won() {
        let sec_result = resolve_ball_security(
            DuelKind::BallSecurityDistribution,
            artrine,
            defenders,
            attribute_keys,
            defense_team_id,
            context,
            rng,
        );
        return ArtrineExecutionOutcome {
            mirins_advanced: 0.0,
            drives_recorded: 0,
            drive_row_indices: Vec::new(),
            turnover: sec_result.turnover_team_id,
            recovering_player_id: sec_result.recovering_player_id,
            scoring_decision: ScoringDecision::NoOpportunity,
            elapsed_seconds: 1.0,
            end_position: start_pos,
            duels: vec![dist_duel, sec_result.duel_outcome],
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
    let elapsed_seconds = (mirins_advanced * 0.25).max(1.0);

    ArtrineExecutionOutcome {
        mirins_advanced,
        drives_recorded: 0,
        drive_row_indices: Vec::new(),
        turnover: None,
        recovering_player_id: None,
        scoring_decision: ScoringDecision::NoOpportunity,
        elapsed_seconds,
        end_position,
        duels: vec![dist_duel],
    }
}