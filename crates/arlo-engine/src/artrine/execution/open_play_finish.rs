use crate::artrine::execution::context::ActionExecutionContext;
use crate::match_decision::scoring::{
    evaluate_scoring_opportunity, resolve_scoring_attempt_with_fatigue, ScoringDecision,
    ScoringOpportunity,
};
use crate::physical::systems::degradation::calculate_effective_player_speed;
use crate::physical::FatigueState;
use crate::resolution::duel_profiles::get_duel_profiles;
use crate::resolution::duel_timing::derive_duel_duration;
use crate::resolution::group_rating::calculate_player_duel_rating_with_state;
use crate::resolution::{AttributedDuelOutcome, DuelKind};
use crate::spatial::ball_kinematics::{ball_flight_duration, calculate_shot_speed_with_state};
use crate::spatial::DynamicSpatialMap;
use crate::time::{DurationComponentKind, DurationLedger};
use arlo_domain::{Player, Position as DomainPosition};
use arlo_math::units::{Position as VectorPosition, MIRIM_TO_METERS};
use rand::Rng;
use uuid::Uuid;

const OPEN_PLAY_MAX_FINISH_DISTANCE_MIRIM: f64 = 50.0;

pub fn resolve_open_play_finish_attempt<F, R>(
    ctx: &ActionExecutionContext<'_, F>,
    artrine_id: Uuid,
    receiver_player: &Player,
    end_position: VectorPosition,
    total_territory: f64,
    spatial_map: &DynamicSpatialMap,
    ledger: &mut DurationLedger,
    duels: &mut Vec<AttributedDuelOutcome>,
    rng: &mut R,
) -> (Option<Uuid>, Option<Uuid>, ScoringDecision)
where
    F: Fn(&Uuid) -> FatigueState,
    R: Rng + ?Sized,
{
    let receiver_state = ctx.fatigue(&receiver_player.id());
    let (attacker_profile, _) = get_duel_profiles(DuelKind::FinishingAttempt);
    let receiver_rating = calculate_player_duel_rating_with_state(
        receiver_player,
        DomainPosition::CenterOffense,
        ctx.attribute_keys,
        &attacker_profile,
        &receiver_state,
    );

    let opportunity = evaluate_scoring_opportunity(
        ctx.is_bonus_phase,
        ctx.drives_in_series,
        total_territory,
        receiver_rating,
    );

    let rx_mirim = end_position.raw().0 / MIRIM_TO_METERS;
    let dist_to_goal_mirim = if ctx.attacking_positive_x {
        (ctx.pitch.length_mirim() - rx_mirim).max(0.0)
    } else {
        rx_mirim.max(0.0)
    };

    if opportunity != ScoringOpportunity::None
        && dist_to_goal_mirim <= OPEN_PLAY_MAX_FINISH_DISTANCE_MIRIM
    {
        let finish_context = ctx.duel_context.for_duel_kind(DuelKind::FinishingAttempt);
        let assister_id = if artrine_id != receiver_player.id() {
            Some(artrine_id)
        } else {
            None
        };

        let (decision, finish_duel) = resolve_scoring_attempt_with_fatigue(
            receiver_player,
            ctx.goalguard,
            ctx.attribute_keys,
            ctx.offense_team_id,
            artrine_id,
            assister_id,
            opportunity,
            ctx.drives_in_series,
            total_territory,
            &receiver_state,
            &ctx.fatigue(&ctx.goalguard.id()),
            &finish_context,
            rng,
        );

        let finisher_spd =
            calculate_effective_player_speed(receiver_player, ctx.attribute_keys, &receiver_state);
        let goalguard_spd = calculate_effective_player_speed(
            ctx.goalguard,
            ctx.attribute_keys,
            &ctx.fatigue(&ctx.goalguard.id()),
        );
        let goalguard_pos = spatial_map
            .get_position(&ctx.goalguard.id())
            .unwrap_or(end_position);
        let finish_dur =
            derive_duel_duration(end_position, finisher_spd, goalguard_pos, goalguard_spd);
        let shot_spd =
            calculate_shot_speed_with_state(receiver_player, ctx.attribute_keys, &receiver_state);
        let shot_flight = ball_flight_duration(dist_to_goal_mirim, shot_spd);

        ledger.record_live(DurationComponentKind::FinishingEngagement, finish_dur);
        ledger.record_live(DurationComponentKind::ShotFlight, shot_flight);
        duels.push(finish_duel);

        let (turnover, recovering_player_id) = match &decision {
            ScoringDecision::Missed { .. } => (Some(ctx.defense_team_id), None),
            _ => (None, None),
        };

        (turnover, recovering_player_id, decision)
    } else {
        (None, None, ScoringDecision::NoOpportunity)
    }
}
