use crate::attributes::profiles::get_duel_attribute_profiles as get_duel_profiles;
use crate::match_decision::target_selection::select_finisher;
use crate::resolution::group_rating::calculate_player_duel_rating_from_table;
use crate::resolution::resolver::{resolve_duel, DuelResolutionRequest};
use crate::resolution::{AttributedDuelOutcome, DuelKind};
use crate::world_state::match_state::MatchState;
use crate::world_state::step::down_resolution::contest_stage::ActionContestOutcome;
use crate::world_state::step::down_resolution::context::DownResolutionContext;
use rand::Rng;
use smallvec::smallvec;
use std::collections::HashMap;
use uuid::Uuid;

pub fn resolve_cross_phase<'a, R: Rng + ?Sized>(
    ctx: &'a DownResolutionContext<'a>,
    state: &MatchState,
    rng: &mut R,
) -> ActionContestOutcome<'a> {
    let tables = &ctx.attribute_tables;
    let finisher_id = select_finisher(
        &ctx.target_candidates,
        Some(state.role_index_for_team(ctx.offense_team_id)),
        state.pitch(),
        state.offensive_position_index_for_team(ctx.offense_team_id),
        state.instructions_index_for_team(ctx.offense_team_id),
        tables,
        ctx.is_home_offense,
        &HashMap::new(),
        Some(&|id: &Uuid| state.fatigue_lookup().get(id)),
        rng,
    )
    .unwrap_or(ctx.carrier.id());

    let finisher = ctx
        .target_candidates
        .iter()
        .copied()
        .find(|p| p.id() == finisher_id)
        .unwrap_or(ctx.carrier);

    let (att_prof, def_prof) = get_duel_profiles(DuelKind::CrossDistribution);
    let att_rating = calculate_player_duel_rating_from_table(
        ctx.carrier,
        ctx.carrier_pos_domain,
        &ctx.carrier_table,
        &att_prof,
        &ctx.carrier_fatigue,
    ) * ctx.artrine_axis_multiplier;
    let def_rating = calculate_player_duel_rating_from_table(
        ctx.primary_defender,
        ctx.primary_defender_pos_domain,
        &ctx.primary_defender_table,
        &def_prof,
        &ctx.primary_defender_fatigue,
    );

    let offense_power = state.power_for_team(ctx.offense_team_id);
    let defense_power = state.power_for_team(ctx.defense_team_id);

    let req = DuelResolutionRequest::with_states(
        DuelKind::CrossDistribution,
        att_rating,
        def_rating,
        ctx.carrier,
        ctx.primary_defender,
        ctx.carrier_fatigue,
        ctx.primary_defender_fatigue,
        state.attribute_keys(),
        &ctx.duel_context,
    )
    .with_tables(
        Some(&ctx.carrier_table),
        Some(&ctx.primary_defender_table),
    )
    .with_team_powers(
        Some(offense_power.control_power() * ctx.artrine_axis_multiplier),
        Some(defense_power.defensive_power()),
    );

    let raw_duel = resolve_duel(req, rng);
    let attacker_won = raw_duel.attacker_won();
    let net_advantage = raw_duel.net_advantage();

    let primary_duel = AttributedDuelOutcome::new(
        raw_duel,
        smallvec![ctx.carrier.id()],
        smallvec![ctx.primary_defender.id()],
    );

    ActionContestOutcome {
        primary_duel: Some(primary_duel),
        secondary_duel: None,
        receiver: Some(finisher),
        turnover_team: None,
        recovering_player_id: None,
        attacker_won,
        net_advantage,
        is_aerial: true,
        distribution_flight: None,
    }
}
