use crate::artrine::detect_drive_crossings;
use crate::injury::contact::evaluate_and_resolve_contact_injury;
use crate::injury::contact::ContactInjuryContext;
use crate::officiating::foul::{evaluate_and_resolve_foul, FoulEvaluationContext};
use crate::play_resolution::action_resolution::resolve_carry_action;
use crate::play_resolution::contact_events::{evaluate_contact_likelihood, sample_contact_event};
use crate::play_resolution::field_context::PitchState;
use crate::play_resolution::space_index::calculate_team_space_rating;
use crate::possession::TouchActionType;
use crate::resolution::duel_kind::DuelKind;
use crate::resolution::outcome::DuelOutcome;
use crate::resolution::AttributedDuelOutcome;
use crate::time::DurationComponentKind;
use crate::world_state::cta_pass::PassPhaseResult;
use crate::world_state::match_state::MatchState;
use crate::world_state::step::open_play_loop::action_context::OpenPlayIterationContext;
use crate::world_state::step::open_play_loop::distribution_scoring::check_distribution_scoring_opportunity;
use crate::world_state::step::open_play_loop::loop_state::OpenPlayLoopState;
use crate::world_state::step::setup::CallToActionContext;
use arlo_domain::Player;
use arlo_math::units::{Duration, Position as VectorPosition, MIRIM_TO_METERS};
use rand::Rng;
use smallvec::smallvec;

pub fn execute_carry_action<R: Rng + ?Sized>(
    state: &mut MatchState,
    context: &CallToActionContext,
    iter_ctx: &OpenPlayIterationContext<'_>,
    pass_phase: &PassPhaseResult<'_>,
    loop_state: &mut OpenPlayLoopState,
    current_carrier: &Player,
    defense_players: &[&Player],
    is_true_artrine: bool,
    rng: &mut R,
) {
    let pitch = *state.pitch();
    let carrier_pos = loop_state.current_carrier_pos;

    let zone = pitch.zone_at_position(carrier_pos);
    let current_time = state.clock().seconds_in_period();
    state.possession_mut().live_sequence_mut().record_touch(
        current_carrier.id(),
        TouchActionType::Carry,
        zone,
        current_time,
    );

    let carrier_table = *state.attribute_table_for(&current_carrier.id());
    let carrier_fatigue = state.fatigue_lookup().get(&current_carrier.id());
    let carrier_impulse = state.impulse_for(&current_carrier.id());

    let offense_tables: Vec<_> = iter_ctx
        .target_candidates
        .iter()
        .map(|p| state.attribute_table_for(&p.id()))
        .collect();
    let defense_tables: Vec<_> = defense_players
        .iter()
        .map(|p| state.attribute_table_for(&p.id()))
        .collect();

    let offense_instructions = *state.instructions_for_team(context.offense_team_id);
    let defense_instructions = *state.instructions_for_team(context.defense_team_id);

    let pitch_state = PitchState::new(
        state.possession().down(),
        state
            .possession()
            .series_state()
            .remaining_mirins_to_target(),
        zone,
        arlo_domain::ArtroPlacement::Central,
        iter_ctx.normalized_proximity,
        state.drives_in_current_series() + loop_state.accumulated_drives_recorded,
        state.possession().is_bonus_phase(),
    );

    let space_rating = calculate_team_space_rating(
        &carrier_table,
        &offense_tables,
        &defense_tables,
        &offense_instructions,
        &defense_instructions,
        &pitch_state,
        0.0,
    );

    let carry_result = resolve_carry_action(
        &carrier_table,
        &carrier_fatigue,
        &carrier_impulse,
        &iter_ctx.risk_profile,
        &space_rating,
        &pitch_state,
        pitch.length_mirim(),
        is_true_artrine,
        rng,
    );

    let primary_defender = defense_players.first().copied().unwrap_or(current_carrier);
    let defender_table = *state.attribute_table_for(&primary_defender.id());
    let defender_fatigue = state.fatigue_lookup().get(&primary_defender.id());
    let carrier_susceptibility = state
        .player_injury_profile(&current_carrier.id())
        .injury_susceptibility_multiplier();
    let defender_susceptibility = state
        .player_injury_profile(&primary_defender.id())
        .injury_susceptibility_multiplier();
    let referee_table = state.head_referee_attribute_table();

    let contact_profile = evaluate_contact_likelihood(
        &carrier_table,
        &carrier_fatigue,
        carrier_susceptibility,
        &defender_table,
        &defender_fatigue,
        defender_susceptibility,
        &offense_instructions,
        &defense_instructions,
        &referee_table,
        &pitch_state,
    );

    let contact_sampling = sample_contact_event(&contact_profile, rng);

    let offense_power = state.power_for_team(context.offense_team_id);
    let defense_power = state.power_for_team(context.defense_team_id);

    let duel_outcome = DuelOutcome::new(
        DuelKind::ArtroBreakthrough,
        carry_result.success,
        offense_power.offensive_power(),
        defense_power.defensive_power(),
        carry_result.win_probability,
        carry_result.net_advantage,
    );

    let attributed_duel = AttributedDuelOutcome::new(
        duel_outcome,
        smallvec![current_carrier.id()],
        smallvec![primary_defender.id()],
    );
    loop_state.accumulated_duels.push(attributed_duel);

    let peace_referee_table = state.peace_referee_attribute_table();
    let fault_catalog = state.fault_catalog_arc();
    let foul_eval_ctx = FoulEvaluationContext::new(
        current_carrier.id(),
        context.offense_team_id,
        primary_defender.id(),
        context.defense_team_id,
        &carrier_table,
        &defender_table,
        carrier_fatigue,
        defender_fatigue,
        &referee_table,
        &peace_referee_table,
        duel_outcome,
        iter_ctx.duel_context,
        contact_sampling.contact_severity,
        iter_ctx.game_state_pressure,
        true,
        zone,
    );

    if let Some(foul_res) = evaluate_and_resolve_foul(&foul_eval_ctx, &fault_catalog, rng) {
        loop_state.accumulated_fouls.push(foul_res);
    }

    let injury_catalog = state.injury_catalog_arc();
    let contact_injury_ctx = ContactInjuryContext::new(
        contact_sampling.contact_severity,
        current_carrier.id(),
        context.offense_team_id,
        &carrier_table,
        carrier_fatigue,
        state.player_injury_profile(&current_carrier.id()),
        25.0,
        primary_defender.id(),
        context.defense_team_id,
        &defender_table,
        defender_fatigue,
        state.player_injury_profile(&primary_defender.id()),
        25.0,
    );

    if contact_sampling.carrier_injured {
        if let Some(inj) =
            evaluate_and_resolve_contact_injury(true, &contact_injury_ctx, &injury_catalog, rng)
        {
            loop_state.accumulated_injuries.push(inj);
        }
    }
    if contact_sampling.defender_injured {
        if let Some(inj) =
            evaluate_and_resolve_contact_injury(false, &contact_injury_ctx, &injury_catalog, rng)
        {
            loop_state.accumulated_injuries.push(inj);
        }
    }

    let macro_advance = if carry_result.success {
        carry_result.mirins_advanced.max(8.0)
    } else {
        (carry_result.mirins_advanced * 0.3).min(3.0)
    };

    let shift_meters = macro_advance * MIRIM_TO_METERS;
    let new_x_m = if context.is_home_offense {
        (carrier_pos.raw().0 + shift_meters).min(pitch.length().value())
    } else {
        (carrier_pos.raw().0 - shift_meters).max(0.0)
    };
    let end_pos = VectorPosition::from_components(new_x_m, carrier_pos.raw().1, 0.0);

    if is_true_artrine {
        let crossed = detect_drive_crossings(
            current_carrier.id(),
            &pitch,
            carrier_pos,
            end_pos,
            context.is_home_offense,
        );
        for row in crossed {
            if !loop_state.accumulated_drive_row_indices.contains(&row) {
                loop_state.accumulated_drive_row_indices.push(row);
                loop_state.accumulated_drives_recorded += 1;
            }
        }
    }

    let carry_time_secs = if carry_result.turnover { 18.0 } else { 28.0 };
    loop_state.accumulated_duration_ledger.record_live(
        DurationComponentKind::CarrierMovement,
        Duration::new(carry_time_secs),
    );

    loop_state.accumulated_mirins_advanced += macro_advance;
    loop_state.current_carrier_pos = end_pos;

    if carry_result.turnover {
        loop_state.turnover_team = Some(context.defense_team_id);
        loop_state.recovering_player = Some(primary_defender.id());
    } else if carry_result.success {
        check_distribution_scoring_opportunity(
            state,
            context,
            iter_ctx,
            pass_phase,
            loop_state,
            current_carrier,
            current_carrier,
            defense_players,
            carry_result.net_advantage.max(10.0),
            rng,
        );
    }

    loop_state.ball_in_play = false;
}
