use crate::artrine::DistributionFlightInfo;
use crate::attributes::profiles::get_duel_attribute_profiles as get_duel_profiles;
use crate::match_decision::target_selection::{select_target, ReceptionRole};
use crate::resolution::group_rating::{
    calculate_anchored_side_rating, calculate_player_duel_rating_from_table, calculate_side_rating,
    RatingParticipants,
};
use crate::resolution::resolver::{resolve_duel, DuelResolutionRequest};
use crate::resolution::{AttributedDuelOutcome, DuelKind};
use crate::world_state::match_state::MatchState;
use crate::world_state::step::down_resolution::contest_stage::ActionContestOutcome;
use crate::world_state::step::down_resolution::context::DownResolutionContext;
use arlo_domain::{ArtrineDecisionKind, Position};
use rand::Rng;
use smallvec::smallvec;
use std::collections::HashMap;
use uuid::Uuid;

pub fn resolve_long_launch_phase<'a, R: Rng + ?Sized>(
    ctx: &'a DownResolutionContext<'a>,
    state: &MatchState,
    rng: &mut R,
) -> ActionContestOutcome<'a> {
    let (att_prof, def_prof) = get_duel_profiles(DuelKind::LongDistribution);
    let offense_power = state.power_for_team(ctx.offense_team_id);
    let defense_power = state.power_for_team(ctx.defense_team_id);
    let tables = &ctx.attribute_tables;

    let att_rating = calculate_anchored_side_rating(
        ctx.carrier,
        ctx.carrier_pos_domain,
        RatingParticipants::from_slice_with_index(
            &ctx.target_candidates,
            state.offensive_position_index_for_team(ctx.offense_team_id),
        )
        .with_fatigue(&|id| state.fatigue_lookup().get(id))
        .with_attribute_tables(tables)
        .with_team_power(offense_power.control_power() * ctx.artrine_axis_multiplier),
        state.attribute_keys(),
        &att_prof,
    ) * ctx.artrine_axis_multiplier;

    let def_rating = calculate_side_rating(
        RatingParticipants::from_slice_with_index(
            &ctx.defense_players,
            state.defensive_position_index_for_team(ctx.defense_team_id),
        )
        .with_fatigue(&|id| state.fatigue_lookup().get(id))
        .with_attribute_tables(tables)
        .with_team_power(defense_power.defensive_power()),
        state.attribute_keys(),
        &def_prof,
    );

    let req = DuelResolutionRequest::with_states(
        DuelKind::LongDistribution,
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

    let raw_throw_duel = resolve_duel(req, rng);
    let throw_won = raw_throw_duel.attacker_won();

    let receiver_id = select_target(
        &ctx.target_candidates,
        state.pitch(),
        state.offensive_position_index_for_team(ctx.offense_team_id),
        state.instructions_index_for_team(ctx.offense_team_id),
        Some(state.role_index_for_team(ctx.offense_team_id)),
        tables,
        ctx.is_home_offense,
        ReceptionRole::OpenPlayReceiver,
        &HashMap::new(),
        Some(&|id: &Uuid| state.fatigue_lookup().get(id)),
        rng,
    )
    .unwrap_or(ctx.carrier.id());

    let receiver = ctx
        .target_candidates
        .iter()
        .copied()
        .find(|p| p.id() == receiver_id)
        .unwrap_or(ctx.carrier);

    let (rec_att_prof, rec_def_prof) = get_duel_profiles(DuelKind::AerialDuel);
    let rec_pos = state
        .offensive_position_index_for_team(ctx.offense_team_id)
        .get(&receiver_id)
        .copied()
        .unwrap_or(Position::CenterOffense);

    let rec_table = tables
        .get(&receiver_id)
        .copied()
        .unwrap_or_else(|| *state.attribute_table_for(&receiver_id));
    let rec_fatigue = state.fatigue_lookup().get(&receiver_id);

    let rec_att_rating = calculate_player_duel_rating_from_table(
        receiver,
        rec_pos,
        &rec_table,
        &rec_att_prof,
        &rec_fatigue,
    ) * ctx.artrine_axis_multiplier;

    let rec_def_rating = calculate_side_rating(
        RatingParticipants::from_slice_with_index(
            &ctx.defense_players,
            state.defensive_position_index_for_team(ctx.defense_team_id),
        )
        .with_fatigue(&|id| state.fatigue_lookup().get(id))
        .with_attribute_tables(tables)
        .with_team_power(defense_power.defensive_power()),
        state.attribute_keys(),
        &rec_def_prof,
    );

    let rec_req = DuelResolutionRequest::with_states(
        DuelKind::AerialDuel,
        rec_att_rating,
        rec_def_rating,
        receiver,
        ctx.primary_defender,
        rec_fatigue,
        ctx.primary_defender_fatigue,
        state.attribute_keys(),
        &ctx.duel_context,
    )
    .with_tables(Some(&rec_table), Some(&ctx.primary_defender_table))
    .with_team_powers(
        Some(offense_power.offensive_power() * ctx.artrine_axis_multiplier),
        Some(defense_power.defensive_power()),
    );

    let raw_rec_duel = resolve_duel(rec_req, rng);
    let catch_won = raw_rec_duel.attacker_won();

    let attacker_won = throw_won && catch_won;
    let net_advantage = (raw_throw_duel.net_advantage() + raw_rec_duel.net_advantage()) * 0.5;

    let turnover_team = if !attacker_won && raw_rec_duel.net_advantage() <= -2.0 {
        Some(ctx.defense_team_id)
    } else {
        None
    };
    let recovering_player_id = turnover_team.map(|_| ctx.primary_defender.id());

    let primary_duel = AttributedDuelOutcome::new(
        raw_throw_duel,
        smallvec![ctx.carrier.id()],
        smallvec![ctx.primary_defender.id()],
    );
    let secondary_duel = AttributedDuelOutcome::new(
        raw_rec_duel,
        smallvec![receiver.id()],
        smallvec![ctx.primary_defender.id()],
    );

    let flight_info = DistributionFlightInfo {
        receiver_id: receiver.id(),
        passer_id: ctx.carrier.id(),
        decision_kind: ArtrineDecisionKind::LongLaunch,
        is_aerial: true,
        distance_mirim: 0.0,
        caught: attacker_won,
    };

    ActionContestOutcome {
        primary_duel: Some(primary_duel),
        secondary_duel: Some(secondary_duel),
        receiver: Some(receiver),
        turnover_team,
        recovering_player_id,
        attacker_won,
        net_advantage,
        is_aerial: true,
        distribution_flight: Some(flight_info),
    }
}
