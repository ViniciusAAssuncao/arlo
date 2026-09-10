use crate::artrine::{
    compute_carry_target_lane, compute_forward_target_pos, detect_drive_crossings,
    filter_blocker_helpers,
};
use crate::physical::FatigueState;
use crate::possession::TouchActionType;
use crate::resolution::duel_profiles::get_duel_profiles;
use crate::resolution::group_rating::calculate_player_duel_rating_with_state;
use crate::resolution::resolver::resolve_duel_with_fatigue;
use crate::resolution::{AttributedDuelOutcome, DuelKind};
use crate::rng::RngStream;
use crate::spatial::{
    run_carrier_tick_loop_with_collision, CollisionResolution, DynamicSpatialMap, LiveCollision,
    MovementContext,
};
use crate::team_identity::marking::recalibrate_defenders_for_carrier;
use crate::team_identity::tempo::effort_multiplier_from_value;
use crate::time::DurationComponentKind;
use crate::world_state::match_state::MatchState;
use crate::world_state::step::open_play_loop::action_context::OpenPlayIterationContext;
use crate::world_state::step::open_play_loop::loop_state::OpenPlayLoopState;
use crate::world_state::step::setup::CallToActionContext;
use arlo_domain::{Player, Position as DomainPosition};
use arlo_math::units::{Duration, Position as VectorPosition, MIRIM_TO_METERS};
use std::collections::HashSet;
use uuid::Uuid;

pub fn execute_carry_action<F>(
    state: &mut MatchState,
    context: &CallToActionContext,
    iter_ctx: &OpenPlayIterationContext<'_>,
    loop_state: &mut OpenPlayLoopState,
    current_carrier: &Player,
    defense_players: &[&Player],
    fatigue_lookup: &F,
    is_true_artrine: bool,
) where
    F: Fn(&Uuid) -> FatigueState,
{
    let pitch = *state.pitch();
    let attribute_keys = state.attribute_keys().clone();
    let carrier_pos = loop_state.current_carrier_pos;

    let zone = pitch.zone_at_position(carrier_pos);
    let current_time = state.clock().seconds_in_period();
    state.possession_mut().live_sequence_mut().record_touch(
        current_carrier.id(),
        TouchActionType::Carry,
        zone,
        current_time,
    );

    let target_channel_y_m = compute_carry_target_lane(carrier_pos, &pitch);
    let target_carry_pos = compute_forward_target_pos(
        carrier_pos,
        target_channel_y_m,
        &pitch,
        context.is_home_offense,
    );

    let _blockers = filter_blocker_helpers(
        &iter_ctx.target_candidates,
        &context.offense_role_index,
        &context.offense_route_index,
    );

    let mut movers =
        Vec::with_capacity(1 + iter_ctx.target_candidates.len() + defense_players.len());
    movers.push((current_carrier, target_carry_pos));

    for &helper in &iter_ctx.target_candidates {
        if let Some(pos) = state.spatial_map().get_position(&helper.id()) {
            let offset_x = if context.is_home_offense {
                10.0 * 0.7 * MIRIM_TO_METERS
            } else {
                -10.0 * 0.7 * MIRIM_TO_METERS
            };
            let helper_target =
                VectorPosition::from_components(pos.raw().0 + offset_x, pos.raw().1, 0.0);
            movers.push((helper, helper_target));
        }
    }

    let def_targets = recalibrate_defenders_for_carrier(
        defense_players,
        &context.defense_pos_index,
        state.spatial_map(),
        carrier_pos,
        iter_ctx.offensive_gravity_mult,
        &pitch,
        context.is_home_offense,
        &attribute_keys,
    );

    for &defender in defense_players {
        let def_target = def_targets
            .get(&defender.id())
            .copied()
            .unwrap_or(target_carry_pos);
        movers.push((defender, def_target));
    }

    state
        .spatial_map_mut()
        .set_position(current_carrier.id(), carrier_pos);

    let defense_pressing_value = (iter_ctx.defense_pressing_multiplier - 1.0).max(0.0);
    let offense_ids: HashSet<Uuid> = std::iter::once(current_carrier.id())
        .chain(iter_ctx.target_candidates.iter().map(|p| p.id()))
        .collect();

    let offense_tempo_val = iter_ctx.offense_tempo_value;
    let effort_multiplier_for = |id: &Uuid| {
        if offense_ids.contains(id) {
            effort_multiplier_from_value(offense_tempo_val)
        } else {
            effort_multiplier_from_value(defense_pressing_value)
        }
    };

    let def_ids: Vec<Uuid> = defense_players.iter().map(|p| p.id()).collect();
    let mut local_duels = Vec::new();
    let mut carry_turnover = None;
    let mut carry_recovering = None;
    let mut carry_halted = false;

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

    let rng_provider = *state.rng_provider();
    let mut local_seq = state.event_sequence();
    let defense_team_id = context.defense_team_id;
    let defense_pos_index = &context.defense_pos_index;
    let duel_ctx = iter_ctx.duel_context;

    let collision_cb =
        |_s_map: &mut DynamicSpatialMap, col: &LiveCollision, spd: &mut f64| {
            let def_player = defense_players
                .iter()
                .copied()
                .find(|p| p.id() == col.defender_id)
                .unwrap_or(defense_players[0]);

            let (off_prof, def_prof) = get_duel_profiles(DuelKind::ArtroBreakthrough);
            let att_rating = calculate_player_duel_rating_with_state(
                current_carrier,
                carrier_pos_domain,
                &attribute_keys,
                off_prof,
                &fatigue_lookup(&current_carrier.id()),
            );
            let def_rating = calculate_player_duel_rating_with_state(
                def_player,
                defense_pos_index
                    .get(&def_player.id())
                    .copied()
                    .unwrap_or(DomainPosition::Centerback),
                &attribute_keys,
                def_prof,
                &fatigue_lookup(&def_player.id()),
            );

            local_seq += 1;
            let mut d_rng = rng_provider.indexed_rng_for(RngStream::DuelResolution, local_seq);
            let c_context = duel_ctx.for_duel_kind(DuelKind::ArtroBreakthrough);

            let duel_raw = resolve_duel_with_fatigue(
                DuelKind::ArtroBreakthrough,
                att_rating,
                def_rating,
                current_carrier,
                def_player,
                &fatigue_lookup(&current_carrier.id()),
                &fatigue_lookup(&def_player.id()),
                &attribute_keys,
                &c_context,
                &mut d_rng,
            );

            let attributed = AttributedDuelOutcome::new(
                duel_raw,
                vec![current_carrier.id()],
                vec![def_player.id()],
            );
            local_duels.push(attributed);

            if duel_raw.attacker_won() {
                let mit = duel_raw.velocity_mitigation_factor();
                *spd *= mit;
                CollisionResolution::Continue {
                    velocity_mitigation: mit,
                }
            } else {
                carry_halted = true;
                let (sec_off, sec_def) = get_duel_profiles(DuelKind::BallSecurityCarry);
                let sec_att = calculate_player_duel_rating_with_state(
                    current_carrier,
                    carrier_pos_domain,
                    &attribute_keys,
                    sec_off,
                    &fatigue_lookup(&current_carrier.id()),
                );
                let sec_df = calculate_player_duel_rating_with_state(
                    def_player,
                    defense_pos_index
                        .get(&def_player.id())
                        .copied()
                        .unwrap_or(DomainPosition::Centerback),
                    &attribute_keys,
                    sec_def,
                    &fatigue_lookup(&def_player.id()),
                );

                local_seq += 1;
                let mut sec_rng = rng_provider.indexed_rng_for(RngStream::DuelResolution, local_seq);
                let sec_raw = resolve_duel_with_fatigue(
                    DuelKind::BallSecurityCarry,
                    sec_att,
                    sec_df,
                    current_carrier,
                    def_player,
                    &fatigue_lookup(&current_carrier.id()),
                    &fatigue_lookup(&def_player.id()),
                    &attribute_keys,
                    &duel_ctx.for_duel_kind(DuelKind::BallSecurityCarry),
                    &mut sec_rng,
                );

                let sec_attr = AttributedDuelOutcome::new(
                    sec_raw,
                    vec![current_carrier.id()],
                    vec![def_player.id()],
                );
                local_duels.push(sec_attr);

                if !sec_raw.attacker_won() {
                    carry_turnover = Some(defense_team_id);
                    carry_recovering = Some(def_player.id());
                }

                CollisionResolution::Halt {
                    turnover_team: carry_turnover,
                    recovering_player: carry_recovering,
                }
            }
        };

    let tick_result = run_carrier_tick_loop_with_collision(
        state.spatial_map_mut(),
        &movers,
        current_carrier.id(),
        &def_ids,
        &attribute_keys,
        MovementContext::LivePlay,
        &pitch,
        fatigue_lookup,
        &effort_multiplier_for,
        collision_cb,
    );

    while state.event_sequence() < local_seq {
        state.next_sequence();
    }

    let end_pos = state
        .spatial_map()
        .get_position(&current_carrier.id())
        .unwrap_or(target_carry_pos);

    let adv_mirim = (end_pos.raw().0 - carrier_pos.raw().0).abs() / MIRIM_TO_METERS;

    if is_true_artrine {
        let crossed = detect_drive_crossings(
            current_carrier.id(),
            &pitch,
            &tick_result,
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

    loop_state
        .accumulated_trajectories
        .extend(tick_result.trajectories().clone());
    loop_state.accumulated_duels.extend(local_duels);
    loop_state.accumulated_duration_ledger.record_live(
        DurationComponentKind::CarrierMovement,
        Duration::new(tick_result.elapsed_seconds()),
    );
    loop_state.accumulated_mirins_advanced += adv_mirim;
    loop_state.current_carrier_pos = end_pos;

    if carry_turnover.is_some() || carry_halted {
        loop_state.turnover_team = carry_turnover;
        loop_state.recovering_player = carry_recovering;
        loop_state.ball_in_play = false;
    }
}
