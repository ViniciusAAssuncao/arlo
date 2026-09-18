use crate::attributes::profiles::get_duel_attribute_profiles as get_duel_profiles;
use crate::resolution::group_rating::calculate_player_duel_rating_from_table;
use crate::resolution::resolver::{resolve_duel, DuelResolutionRequest};
use crate::resolution::{AttributedDuelOutcome, DuelKind};
use crate::world_state::match_state::MatchState;
use crate::world_state::step::down_resolution::contest_stage::ActionContestOutcome;
use crate::world_state::step::down_resolution::context::DownResolutionContext;
use arlo_math::stats::contrast::logistic;
use arlo_math::Probability;
use rand::Rng;
use smallvec::smallvec;

pub fn resolve_carry_phase<'a, R: Rng + ?Sized>(
    ctx: &'a DownResolutionContext<'a>,
    state: &MatchState,
    rng: &mut R,
) -> ActionContestOutcome<'a> {
    let duel_kind = if ctx.is_true_artrine {
        DuelKind::ArtroBreakthrough
    } else {
        DuelKind::RunBreakthrough
    };

    let (att_prof, def_prof) = get_duel_profiles(duel_kind);
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
        duel_kind,
        att_rating,
        def_rating,
        ctx.carrier,
        ctx.primary_defender,
        ctx.carrier_fatigue,
        ctx.primary_defender_fatigue,
        state.attribute_keys(),
        &ctx.duel_context,
    )
    .with_tables(Some(&ctx.carrier_table), Some(&ctx.primary_defender_table))
    .with_team_powers(
        Some(offense_power.offensive_power() * ctx.artrine_axis_multiplier),
        Some(defense_power.defensive_power()),
    );

    let raw_duel = resolve_duel(req, rng);
    let attacker_won = raw_duel.attacker_won();
    let net_advantage = raw_duel.net_advantage();

    let to_base = if attacker_won { -3.5 } else { -1.5 };
    let to_p = (logistic(to_base - 0.20 * net_advantage)
        / ctx.risk_profile.tolerance_index())
    .clamp(0.005, 0.45);

    let turnover_team = if Probability::new_clamped(to_p).sample(rng) {
        Some(ctx.defense_team_id)
    } else {
        None
    };

    let recovering_player_id = turnover_team.map(|_| ctx.primary_defender.id());

    let primary_duel = AttributedDuelOutcome::new(
        raw_duel,
        smallvec![ctx.carrier.id()],
        smallvec![ctx.primary_defender.id()],
    );

    ActionContestOutcome {
        primary_duel: Some(primary_duel),
        secondary_duel: None,
        receiver: Some(ctx.carrier),
        turnover_team,
        recovering_player_id,
        attacker_won,
        net_advantage,
        is_aerial: false,
        distribution_flight: None,
    }
}
