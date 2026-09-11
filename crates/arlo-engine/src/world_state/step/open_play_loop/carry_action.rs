use crate::artrine::detect_drive_crossings;
use crate::attributes::PlayerAttributeTable;
use crate::officiating::foul::FoulResolution;
use crate::open_play::{compute_carry_target_lane, compute_forward_target_pos};
use crate::possession::TouchActionType;
use crate::spatial::{
    run_carrier_tick_loop_with_collision, CollisionResolution, DynamicSpatialMap, LiveCollision,
    MovementContext,
};
use crate::team_identity::marking::recalibrate_defenders_for_carrier_from_tables;
use crate::team_identity::tempo::effort_multiplier_from_value;
use crate::time::DurationComponentKind;
use crate::world_state::match_state::MatchState;
use crate::world_state::step::open_play_loop::action_context::OpenPlayIterationContext;
use crate::world_state::step::open_play_loop::carry_collision::resolve_carry_collision;
use crate::world_state::step::open_play_loop::loop_state::OpenPlayLoopState;
use crate::world_state::step::setup::CallToActionContext;
use arlo_domain::{Player, Position as DomainPosition};
use arlo_math::units::{Duration, Position as VectorPosition, MIRIM_TO_METERS};
use rand::Rng;
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

pub fn execute_carry_action<R: Rng + ?Sized>(
    state: &mut MatchState,
    context: &CallToActionContext,
    iter_ctx: &OpenPlayIterationContext<'_>,
    loop_state: &mut OpenPlayLoopState,
    current_carrier: &Player,
    defense_players: &[&Player],
    is_true_artrine: bool,
    rng: &mut R,
) {
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

    let def_targets = recalibrate_defenders_for_carrier_from_tables(
        defense_players,
        &context.defense_pos_index,
        state.spatial_map(),
        state.teams.player_attribute_tables(),
        carrier_pos,
        iter_ctx.offensive_gravity_mult,
        &pitch,
        context.is_home_offense,
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

    let defense_team_id = context.defense_team_id;
    let defense_pos_index = &context.defense_pos_index;
    let duel_ctx = iter_ctx.duel_context;
    let fatigue_tracker = state.fatigue.clone();

    let carrier_table: PlayerAttributeTable =
        state.attribute_table_for(&current_carrier.id()).clone();
    let defender_tables: HashMap<Uuid, PlayerAttributeTable> = defense_players
        .iter()
        .map(|p| (p.id(), state.attribute_table_for(&p.id()).clone()))
        .collect();

    let head_referee_table = state.head_referee_attribute_table();
    let peace_referee_table = state.peace_referee_attribute_table();
    let game_state_pressure = iter_ctx.game_state_pressure;
    let fault_catalog = state.fault_catalog_arc();

    let mut local_duels = Vec::new();
    let mut local_fouls: Vec<FoulResolution> = Vec::new();
    let mut collision_resolution = None;

    let collision_cb = |_s_map: &mut DynamicSpatialMap, col: &LiveCollision, spd: &mut f64| {
        let outcome = resolve_carry_collision(
            col,
            spd,
            current_carrier,
            &carrier_table,
            carrier_pos_domain,
            defense_players,
            &defender_tables,
            defense_pos_index,
            defense_team_id,
            &|id| fatigue_tracker.fatigue_for(id),
            &duel_ctx,
            &attribute_keys,
            head_referee_table,
            peace_referee_table,
            game_state_pressure,
            &fault_catalog,
            rng,
        );
        local_duels.extend(outcome.duels);
        if let Some(foul) = outcome.foul.clone() {
            local_fouls.push(foul);
        }
        let res = outcome.resolution;
        collision_resolution = Some(res);
        res
    };

    let tick_result = run_carrier_tick_loop_with_collision(
        &mut state.spatial_map,
        &movers,
        current_carrier.id(),
        &def_ids,
        state.teams.player_attribute_tables(),
        MovementContext::LivePlay,
        &pitch,
        &(|id| fatigue_tracker.fatigue_for(id)),
        &effort_multiplier_for,
        collision_cb,
    );

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
    loop_state.accumulated_fouls.extend(local_fouls);
    loop_state.accumulated_duration_ledger.record_live(
        DurationComponentKind::CarrierMovement,
        Duration::new(tick_result.elapsed_seconds()),
    );
    loop_state.accumulated_mirins_advanced += adv_mirim;
    loop_state.current_carrier_pos = end_pos;

    if let Some(CollisionResolution::Halt {
        turnover_team,
        recovering_player,
    }) = collision_resolution
    {
        loop_state.turnover_team = turnover_team;
        loop_state.recovering_player = recovering_player;
        loop_state.ball_in_play = false;
    }
}