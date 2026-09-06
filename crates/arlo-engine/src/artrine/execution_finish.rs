use crate::artrine::execution_outcome::ArtrineExecutionOutcome;
use crate::physical::FatigueState;
use crate::match_decision::finisher_selection::select_finisher_with_fatigue;
use crate::match_decision::scoring::{
    evaluate_scoring_opportunity, resolve_scoring_attempt_with_fatigue,
};
use crate::physical::systems::degradation::calculate_effective_player_speed;
use crate::resolution::duel_profiles::get_duel_profiles;
use crate::resolution::duel_timing::derive_duel_duration;
use crate::resolution::group_rating::calculate_player_duel_rating_with_state;
use crate::resolution::{DuelContext, DuelKind};
use crate::spatial::ball_kinematics::{
    ball_flight_duration, calculate_cross_speed_with_state, calculate_shot_speed_with_state,
};
use crate::spatial::proximity::calculate_distance_mirim;
use crate::spatial::DynamicSpatialMap;
use crate::time::{DurationComponentKind, DurationLedger};
use arlo_domain::pitch::Pitch;
use arlo_domain::{AttributeKey, Player, Position as DomainPosition};
use arlo_math::units::{Position as VectorPosition, MIRIM_TO_METERS};
use rand::Rng;
use std::collections::HashMap;
use uuid::Uuid;

pub fn execute_self_finish<F, R>(
    artrine: &Player,
    goalguard: &Player,
    spatial_map: &DynamicSpatialMap,
    pitch: &Pitch,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    offense_team_id: Uuid,
    defense_team_id: Uuid,
    drives_in_series: u32,
    accumulated_advance_mirim: f64,
    is_last_down: bool,
    is_bonus_phase: bool,
    attacking_positive_x: bool,
    start_pos: VectorPosition,
    context: &DuelContext,
    fatigue_for: &F,
    rng: &mut R,
) -> ArtrineExecutionOutcome
where
    F: Fn(&Uuid) -> FatigueState,
    R: Rng + ?Sized,
{
    let finisher_pos = spatial_map.get_position(&artrine.id()).unwrap_or(start_pos);
    let goalguard_pos = spatial_map.get_position(&goalguard.id()).unwrap_or(start_pos);

    let finisher_state = fatigue_for(&artrine.id());
    let goalguard_state = fatigue_for(&goalguard.id());

    let finisher_speed = calculate_effective_player_speed(artrine, attribute_keys, &finisher_state);
    let goalguard_speed = calculate_effective_player_speed(goalguard, attribute_keys, &goalguard_state);
    let finishing_duration = derive_duel_duration(
        finisher_pos,
        finisher_speed,
        goalguard_pos,
        goalguard_speed,
    );

    let shot_speed = calculate_shot_speed_with_state(artrine, attribute_keys, &finisher_state);
    let finisher_x_mirim = finisher_pos.raw().0 / MIRIM_TO_METERS;
    let dist_to_goal_mirim = if attacking_positive_x {
        (pitch.length_mirim() - finisher_x_mirim).max(0.0)
    } else {
        finisher_x_mirim.max(0.0)
    };
    let shot_flight = ball_flight_duration(dist_to_goal_mirim, shot_speed);

    let mut ledger = DurationLedger::new();
    ledger.record_live(
        DurationComponentKind::FinishingEngagement,
        finishing_duration,
    );
    ledger.record_live(
        DurationComponentKind::ShotFlight,
        shot_flight,
    );

    execute_finishing_with_player(
        artrine,
        artrine,
        goalguard,
        attribute_keys,
        offense_team_id,
        defense_team_id,
        drives_in_series,
        accumulated_advance_mirim,
        is_last_down,
        is_bonus_phase,
        start_pos,
        ledger,
        context,
        fatigue_for,
        rng,
    )
}

pub fn execute_cross_finish<F, R>(
    artrine: &Player,
    teammates: &[&Player],
    offense_position_index: &HashMap<Uuid, DomainPosition>,
    goalguard: &Player,
    spatial_map: &DynamicSpatialMap,
    pitch: &Pitch,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    offense_team_id: Uuid,
    defense_team_id: Uuid,
    drives_in_series: u32,
    accumulated_advance_mirim: f64,
    is_last_down: bool,
    is_bonus_phase: bool,
    attacking_positive_x: bool,
    start_pos: VectorPosition,
    context: &DuelContext,
    fatigue_for: &F,
    rng: &mut R,
) -> ArtrineExecutionOutcome
where
    F: Fn(&Uuid) -> FatigueState,
    R: Rng + ?Sized,
{
    let eligible_teammates: Vec<&Player> = teammates
        .iter()
        .copied()
        .filter(|p| p.id() != artrine.id())
        .collect();

    let chosen_finisher_id = select_finisher_with_fatigue(
        &eligible_teammates,
        spatial_map,
        pitch,
        offense_position_index,
        attribute_keys,
        attacking_positive_x,
        fatigue_for,
        rng,
    );

    let finisher = chosen_finisher_id
        .and_then(|fid| eligible_teammates.iter().copied().find(|p| p.id() == fid))
        .unwrap_or(artrine);

    let artrine_state = fatigue_for(&artrine.id());
    let finisher_state = fatigue_for(&finisher.id());
    let goalguard_state = fatigue_for(&goalguard.id());

    let artrine_pos = spatial_map.get_position(&artrine.id()).unwrap_or(start_pos);
    let finisher_pos = spatial_map.get_position(&finisher.id()).unwrap_or(start_pos);
    let cross_dist_mirim = calculate_distance_mirim(artrine_pos, finisher_pos);
    let cross_speed = calculate_cross_speed_with_state(artrine, attribute_keys, &artrine_state);
    let cross_flight = ball_flight_duration(cross_dist_mirim, cross_speed);

    let goalguard_pos = spatial_map.get_position(&goalguard.id()).unwrap_or(start_pos);

    let finisher_speed = calculate_effective_player_speed(finisher, attribute_keys, &finisher_state);
    let goalguard_speed = calculate_effective_player_speed(goalguard, attribute_keys, &goalguard_state);
    let finishing_duration = derive_duel_duration(
        finisher_pos,
        finisher_speed,
        goalguard_pos,
        goalguard_speed,
    );

    let shot_speed = calculate_shot_speed_with_state(finisher, attribute_keys, &finisher_state);
    let finisher_x_mirim = finisher_pos.raw().0 / MIRIM_TO_METERS;
    let dist_to_goal_mirim = if attacking_positive_x {
        (pitch.length_mirim() - finisher_x_mirim).max(0.0)
    } else {
        finisher_x_mirim.max(0.0)
    };
    let shot_flight = ball_flight_duration(dist_to_goal_mirim, shot_speed);

    let mut ledger = DurationLedger::new();
    ledger.record_live(
        DurationComponentKind::CrossFlight,
        cross_flight,
    );
    ledger.record_live(
        DurationComponentKind::FinishingEngagement,
        finishing_duration,
    );
    ledger.record_live(
        DurationComponentKind::ShotFlight,
        shot_flight,
    );

    execute_finishing_with_player(
        finisher,
        artrine,
        goalguard,
        attribute_keys,
        offense_team_id,
        defense_team_id,
        drives_in_series,
        accumulated_advance_mirim,
        is_last_down,
        is_bonus_phase,
        start_pos,
        ledger,
        context,
        fatigue_for,
        rng,
    )
}

pub fn execute_finishing_with_player<F, R>(
    finisher: &Player,
    artrine: &Player,
    goalguard: &Player,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    offense_team_id: Uuid,
    defense_team_id: Uuid,
    drives_in_series: u32,
    accumulated_advance_mirim: f64,
    _is_last_down: bool,
    is_bonus_phase: bool,
    start_pos: VectorPosition,
    duration_ledger: DurationLedger,
    context: &DuelContext,
    fatigue_for: &F,
    rng: &mut R,
) -> ArtrineExecutionOutcome
where
    F: Fn(&Uuid) -> FatigueState,
    R: Rng + ?Sized,
{
    let finisher_state = fatigue_for(&finisher.id());
    let goalguard_state = fatigue_for(&goalguard.id());

    let (attacker_profile, _) = get_duel_profiles(DuelKind::FinishingAttempt);
    let finisher_rating = calculate_player_duel_rating_with_state(
        finisher,
        DomainPosition::CenterOffense,
        attribute_keys,
        &attacker_profile,
        &finisher_state,
    );

    let opportunity = evaluate_scoring_opportunity(
        is_bonus_phase,
        drives_in_series,
        accumulated_advance_mirim,
        finisher_rating,
    );

    let (scoring_decision, finish_duel) = resolve_scoring_attempt_with_fatigue(
        finisher,
        goalguard,
        attribute_keys,
        offense_team_id,
        artrine.id(),
        opportunity,
        drives_in_series,
        accumulated_advance_mirim,
        &finisher_state,
        &goalguard_state,
        context,
        rng,
    );

    let (turnover, recovering_player_id) = match &scoring_decision {
        crate::match_decision::scoring::ScoringDecision::Missed { .. } => {
            (Some(defense_team_id), None)
        }
        _ => (None, None),
    };

    ArtrineExecutionOutcome {
        mirins_advanced: 0.0,
        drives_recorded: 0,
        drive_row_indices: Vec::new(),
        turnover,
        recovering_player_id,
        scoring_decision,
        duration_ledger,
        end_position: start_pos,
        duels: vec![finish_duel],
        receiver_id: Some(finisher.id()),
        distribution_flight: None,
    }
}
