use crate::artrine::{resolve_primary_lead_defender, DistributionFlightInfo};
use crate::match_decision::scoring::{
    evaluate_scoring_opportunity, resolve_scoring_attempt_with_fatigue, ScoringOpportunity,
};
use crate::match_decision::target_selection::{select_target_with_fatigue, ReceptionRole};
use crate::physical::FatigueState;
use crate::possession::TouchActionType;
use crate::resolution::duel_profiles::get_duel_profiles;
use crate::resolution::group_rating::{
    calculate_anchored_side_rating_from_index_with_fatigue,
    calculate_player_duel_rating_with_state, calculate_side_rating_from_index_with_fatigue,
};
use crate::resolution::resolver::resolve_duel_with_fatigue;
use crate::resolution::{AttributedDuelOutcome, DuelKind};
use crate::rng::RngStream;
use crate::spatial::ball_kinematics::{ball_flight_duration, calculate_pass_speed_with_state};
use crate::team_identity::{long_launch_advance_multiplier, short_pass_advance_multiplier};
use crate::time::DurationComponentKind;
use crate::world_state::cta_pass::PassPhaseResult;
use crate::world_state::match_state::MatchState;
use crate::world_state::step::open_play_loop::action_context::OpenPlayIterationContext;
use crate::world_state::step::open_play_loop::finish_action::find_defense_goalguard;
use crate::world_state::step::open_play_loop::loop_state::OpenPlayLoopState;
use crate::world_state::step::setup::CallToActionContext;
use arlo_domain::sport_constants::MINIMUM_ENGAGEMENT_SECONDS;
use arlo_domain::{ArtrineDecisionKind, Player, Position as DomainPosition};
use arlo_math::units::{Duration, Length, Velocity, MIRIM_TO_METERS};
use uuid::Uuid;

const OPEN_PLAY_MAX_FINISH_DISTANCE_MIRIM: f64 = 50.0;

fn check_distribution_scoring_opportunity<F>(
    state: &mut MatchState,
    context: &CallToActionContext,
    iter_ctx: &OpenPlayIterationContext<'_>,
    pass_phase: &PassPhaseResult<'_>,
    loop_state: &mut OpenPlayLoopState,
    receiver_player: &Player,
    current_carrier: &Player,
    defense_players: &[&Player],
    rec_att_rating: f64,
    fatigue_lookup: &F,
) where
    F: Fn(&Uuid) -> FatigueState,
{
    let pitch = *state.pitch();
    let rx_mirim = loop_state.current_carrier_pos.raw().0 / MIRIM_TO_METERS;
    let dist_to_goal_mirim = if context.is_home_offense {
        (pitch.length_mirim() - rx_mirim).max(0.0)
    } else {
        rx_mirim.max(0.0)
    };

    let total_drives = state.drives_in_current_series() + loop_state.accumulated_drives_recorded;
    let total_adv = state.possession().series_state().advanced_mirins()
        + loop_state.accumulated_mirins_advanced;

    let opportunity = evaluate_scoring_opportunity(
        state.possession().is_bonus_phase(),
        total_drives,
        total_adv,
        rec_att_rating,
    );

    if opportunity != ScoringOpportunity::None
        && dist_to_goal_mirim <= OPEN_PLAY_MAX_FINISH_DISTANCE_MIRIM
    {
        let goalguard = find_defense_goalguard(defense_players);
        let seq_fin = state.next_sequence();
        let mut fin_rng = state
            .rng_provider()
            .indexed_rng_for(RngStream::DuelResolution, seq_fin);

        let shot_zone = pitch.zone_at_position(loop_state.current_carrier_pos);
        let current_time = state.clock().seconds_in_period();
        state.possession_mut().live_sequence_mut().record_touch(
            receiver_player.id(),
            TouchActionType::FinishingAttempt,
            shot_zone,
            current_time,
        );

        let assister_id = state
            .possession()
            .live_sequence()
            .primary_assister(receiver_player.id())
            .or(Some(current_carrier.id()));

        let (score_dec, fin_duel) = resolve_scoring_attempt_with_fatigue(
            receiver_player,
            goalguard,
            state.attribute_keys(),
            context.offense_team_id,
            pass_phase.artrine.id(),
            assister_id,
            opportunity,
            total_drives,
            total_adv,
            &fatigue_lookup(&receiver_player.id()),
            &fatigue_lookup(&goalguard.id()),
            &iter_ctx.duel_context.for_duel_kind(DuelKind::FinishingAttempt),
            &mut fin_rng,
        );

        loop_state.accumulated_duels.push(fin_duel);
        loop_state.scoring_decision = score_dec;
        loop_state.ball_in_play = false;
    }
}

pub fn execute_distribution_action<F>(
    state: &mut MatchState,
    context: &CallToActionContext,
    iter_ctx: &OpenPlayIterationContext<'_>,
    pass_phase: &PassPhaseResult<'_>,
    loop_state: &mut OpenPlayLoopState,
    current_carrier: &Player,
    defense_players: &[&Player],
    chosen_decision: ArtrineDecisionKind,
    fatigue_lookup: &F,
) where
    F: Fn(&Uuid) -> FatigueState,
{
    let pitch = *state.pitch();
    let attribute_keys = state.attribute_keys().clone();
    let carrier_pos = loop_state.current_carrier_pos;

    let action_type = if chosen_decision == ArtrineDecisionKind::ShortPass {
        TouchActionType::ShortPass
    } else {
        TouchActionType::LongLaunch
    };
    let zone = pitch.zone_at_position(carrier_pos);
    let current_time = state.clock().seconds_in_period();
    state.possession_mut().live_sequence_mut().record_touch(
        current_carrier.id(),
        action_type,
        zone,
        current_time,
    );

    let carrier_pos_domain = context
        .offense_pos_index
        .get(&current_carrier.id())
        .copied()
        .unwrap_or_else(|| {
            current_carrier
                .positions()
                .first()
                .map(|pp| pp.position())
                .unwrap_or(DomainPosition::CenterOffense)
        });

    let duel_kind = if chosen_decision == ArtrineDecisionKind::ShortPass {
        DuelKind::ShortDistribution
    } else {
        DuelKind::LongDistribution
    };

    let (off_prof, def_prof) = get_duel_profiles(duel_kind);
    let att_rating = calculate_anchored_side_rating_from_index_with_fatigue(
        current_carrier,
        carrier_pos_domain,
        &iter_ctx.target_candidates,
        &context.offense_pos_index,
        &attribute_keys,
        &off_prof,
        fatigue_lookup,
    );
    let def_rating = calculate_side_rating_from_index_with_fatigue(
        defense_players,
        &context.defense_pos_index,
        &attribute_keys,
        &def_prof,
        fatigue_lookup,
    );

    let contest_radius = Length::new(2.0 * iter_ctx.defense_pressing_multiplier * MIRIM_TO_METERS);
    let seq_duel = state.next_sequence();
    let mut d_rng = state
        .rng_provider()
        .indexed_rng_for(RngStream::DuelResolution, seq_duel);

    let lead_defender = resolve_primary_lead_defender(
        current_carrier.id(),
        &context.offense_pos_index,
        carrier_pos,
        Velocity::zero(),
        defense_players,
        state.spatial_map(),
        &context.defense_instructions_index,
        &attribute_keys,
        fatigue_lookup,
        contest_radius,
        None,
        &mut d_rng,
    );

    let dist_context = iter_ctx.duel_context.for_duel_kind(duel_kind);
    let raw_throw_duel = resolve_duel_with_fatigue(
        duel_kind,
        att_rating,
        def_rating,
        current_carrier,
        lead_defender,
        &fatigue_lookup(&current_carrier.id()),
        &fatigue_lookup(&lead_defender.id()),
        &attribute_keys,
        &dist_context,
        &mut d_rng,
    );

    let throw_duel = AttributedDuelOutcome::new(
        raw_throw_duel,
        vec![current_carrier.id()],
        vec![lead_defender.id()],
    );
    loop_state.accumulated_duels.push(throw_duel);

    let dist_duration = Duration::new(MINIMUM_ENGAGEMENT_SECONDS);
    loop_state
        .accumulated_duration_ledger
        .record_live(DurationComponentKind::DistributionEngagement, dist_duration);

    if !raw_throw_duel.attacker_won() {
        loop_state.ball_in_play = false;
        return;
    }

    let offense_instructions = *state.instructions_for_team(context.offense_team_id);
    let passing_range = offense_instructions.in_possession().passing_range();

    let throw_advance = if chosen_decision == ArtrineDecisionKind::ShortPass {
        (4.0 * short_pass_advance_multiplier(passing_range)).max(1.0)
    } else {
        (12.0 * long_launch_advance_multiplier(passing_range)).max(3.0)
    };

    let pass_speed = calculate_pass_speed_with_state(
        current_carrier,
        &attribute_keys,
        &fatigue_lookup(&current_carrier.id()),
    );
    let flight_duration = ball_flight_duration(throw_advance, pass_speed);
    loop_state
        .accumulated_duration_ledger
        .record_live(DurationComponentKind::DistributionFlight, flight_duration);

    let receiver_id = select_target_with_fatigue(
        &iter_ctx.target_candidates,
        state.spatial_map(),
        &pitch,
        &context.offense_pos_index,
        &context.offense_instructions_index,
        &attribute_keys,
        context.is_home_offense,
        ReceptionRole::OpenPlayReceiver,
        &iter_ctx.openness_by_player,
        fatigue_lookup,
        &mut d_rng,
    )
    .unwrap_or(current_carrier.id());

    let receiver_player = iter_ctx
        .target_candidates
        .iter()
        .copied()
        .find(|p| p.id() == receiver_id)
        .unwrap_or(current_carrier);

    let is_aerial = chosen_decision == ArtrineDecisionKind::LongLaunch;
    let rec_duel_kind = if is_aerial {
        DuelKind::AerialDuel
    } else {
        DuelKind::RouteContest
    };

    let (rec_off, rec_def) = get_duel_profiles(rec_duel_kind);
    let rec_att_rating = calculate_player_duel_rating_with_state(
        receiver_player,
        context
            .offense_pos_index
            .get(&receiver_id)
            .copied()
            .unwrap_or(DomainPosition::CenterOffense),
        &attribute_keys,
        &rec_off,
        &fatigue_lookup(&receiver_id),
    );
    let rec_def_rating = calculate_side_rating_from_index_with_fatigue(
        defense_players,
        &context.defense_pos_index,
        &attribute_keys,
        &rec_def,
        fatigue_lookup,
    );

    let raw_rec_duel = resolve_duel_with_fatigue(
        rec_duel_kind,
        rec_att_rating,
        rec_def_rating,
        receiver_player,
        lead_defender,
        &fatigue_lookup(&receiver_id),
        &fatigue_lookup(&lead_defender.id()),
        &attribute_keys,
        &iter_ctx.duel_context.for_duel_kind(rec_duel_kind),
        &mut d_rng,
    );

    let rec_attributed = AttributedDuelOutcome::new(
        raw_rec_duel,
        vec![receiver_id],
        vec![lead_defender.id()],
    );
    loop_state.accumulated_duels.push(rec_attributed);

    let rec_pos = state
        .spatial_map()
        .get_position(&receiver_id)
        .unwrap_or(carrier_pos);

    let caught = raw_rec_duel.attacker_won();
    loop_state.last_distribution_flight = Some(DistributionFlightInfo {
        receiver_id,
        passer_id: current_carrier.id(),
        decision_kind: chosen_decision,
        is_aerial,
        reception_point: rec_pos,
        distance_mirim: throw_advance,
        caught,
    });

    if !caught {
        loop_state.ball_in_play = false;
        if raw_rec_duel.net_advantage() <= -2.5 {
            loop_state.turnover_team = Some(context.defense_team_id);
            loop_state.recovering_player = Some(lead_defender.id());
        }
        return;
    }

    let rec_zone = pitch.zone_at_position(rec_pos);
    state.possession_mut().live_sequence_mut().record_touch(
        receiver_id,
        TouchActionType::Reception,
        rec_zone,
        current_time + flight_duration.value(),
    );

    loop_state.last_receiver_id = Some(receiver_id);
    loop_state.accumulated_mirins_advanced += throw_advance;
    loop_state.current_carrier_id = receiver_id;
    loop_state.current_carrier_pos = rec_pos;

    check_distribution_scoring_opportunity(
        state,
        context,
        iter_ctx,
        pass_phase,
        loop_state,
        receiver_player,
        current_carrier,
        defense_players,
        rec_att_rating,
        fatigue_lookup,
    );
}
