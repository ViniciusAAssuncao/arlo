use crate::artrine::{resolve_primary_lead_defender_from_tables, DistributionFlightInfo};
use crate::lineup_runtime::find_goalguard;
use crate::match_decision::target_selection::{select_target_from_tables, ReceptionRole};
use crate::officiating::foul::FoulResolution;
use crate::officiating::line_fault::{
    estimate_last_defender_position, evaluate_and_resolve_line_fault, identify_last_defender,
    is_line_fault, LineFaultEvaluationContext,
};
use crate::play_resolution::ball_kinematics::{ball_flight_duration, calculate_pass_speed};
use crate::resolution::duel_profiles::get_duel_profiles;
use crate::resolution::group_rating::{
    calculate_anchored_side_rating, calculate_player_duel_rating_from_table, calculate_side_rating,
    RatingParticipants,
};
use crate::resolution::resolver::{resolve_duel, DuelResolutionRequest};
use crate::resolution::{
    sample_action_progression, ActionProgressionKind, AttributedDuelOutcome, DuelKind,
};
use crate::team_identity::{long_launch_advance_multiplier, short_pass_advance_multiplier};
use crate::world_state::match_state::MatchState;
use crate::world_state::step::open_play_loop::action_context::OpenPlayIterationContext;
use crate::world_state::step::setup::CallToActionContext;
use arlo_domain::sport_constants::MINIMUM_ENGAGEMENT_SECONDS;
use arlo_domain::{ArtrineDecisionKind, Player, Position as DomainPosition};
use arlo_math::units::{Duration, Length, Position as VectorPosition, Velocity, MIRIM_TO_METERS};
use rand::Rng;
use smallvec::smallvec;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct DistributionReceptionResult<'a> {
    pub receiver: &'a Player,
    pub caught: bool,
    pub reception_point: VectorPosition,
    pub duels: Vec<AttributedDuelOutcome>,
    pub advance_mirim: f64,
    pub flight_duration: Duration,
    pub engagement_duration: Duration,
    pub flight_info: Option<DistributionFlightInfo>,
    pub turnover_team: Option<Uuid>,
    pub recovering_player: Option<Uuid>,
    pub receiver_rating: f64,
    pub foul: Option<FoulResolution>,
}

pub fn resolve_distribution_reception<'a, R: Rng + ?Sized>(
    state: &MatchState,
    context: &CallToActionContext,
    iter_ctx: &OpenPlayIterationContext<'a>,
    current_carrier: &'a Player,
    defense_players: &[&Player],
    chosen_decision: ArtrineDecisionKind,
    carrier_pos: VectorPosition,
    rng: &mut R,
) -> DistributionReceptionResult<'a> {
    let pitch = *state.pitch();
    let attribute_keys = state.attribute_keys();

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
    let tables = state.teams.player_attribute_tables();

    let offense_power = state.power_for_team(context.offense_team_id);
    let defense_power = state.power_for_team(context.defense_team_id);

    let att_rating = calculate_anchored_side_rating(
        current_carrier,
        carrier_pos_domain,
        RatingParticipants::from_slice_with_index(
            &iter_ctx.target_candidates,
            &context.offense_pos_index,
        )
        .with_fatigue(&|id| state.fatigue_lookup().get(id))
        .with_attribute_tables(tables)
        .with_team_power(offense_power.control_power()),
        attribute_keys,
        off_prof,
    );
    let def_rating = calculate_side_rating(
        RatingParticipants::from_slice_with_index(defense_players, &context.defense_pos_index)
            .with_fatigue(&|id| state.fatigue_lookup().get(id))
            .with_attribute_tables(tables)
            .with_team_power(defense_power.defensive_power()),
        attribute_keys,
        def_prof,
    );

    let contest_radius = Length::new(2.0 * iter_ctx.defense_pressing_multiplier * MIRIM_TO_METERS);

    let lead_defender = resolve_primary_lead_defender_from_tables(
        current_carrier.id(),
        &context.offense_pos_index,
        carrier_pos,
        Velocity::zero(),
        defense_players,
        &context.defense_instructions_index,
        tables,
        &|id| state.fatigue_lookup().get(id),
        contest_radius,
        None,
        rng,
    );

    let dist_context = iter_ctx.duel_context.for_duel_kind(duel_kind);
    let carrier_fatigue = state.fatigue_lookup().get(&current_carrier.id());
    let lead_def_fatigue = state.fatigue_lookup().get(&lead_defender.id());
    let att_table = tables.get(&current_carrier.id());
    let lead_def_table = tables.get(&lead_defender.id());
    let req = DuelResolutionRequest::with_states(
        duel_kind,
        att_rating,
        def_rating,
        current_carrier,
        lead_defender,
        carrier_fatigue,
        lead_def_fatigue,
        attribute_keys,
        &dist_context,
    )
    .with_tables(att_table, lead_def_table)
    .with_team_powers(
        Some(offense_power.control_power()),
        Some(defense_power.defensive_power()),
    );
    let raw_throw_duel = resolve_duel(req, rng);

    let throw_duel = AttributedDuelOutcome::new(
        raw_throw_duel,
        smallvec![current_carrier.id()],
        smallvec![lead_defender.id()],
    );
    let mut duels = vec![throw_duel];

    let engagement_duration = Duration::new(MINIMUM_ENGAGEMENT_SECONDS);

    if !raw_throw_duel.attacker_won() {
        return DistributionReceptionResult {
            receiver: current_carrier,
            caught: false,
            reception_point: carrier_pos,
            duels,
            advance_mirim: 0.0,
            flight_duration: Duration::new(0.0),
            engagement_duration,
            flight_info: None,
            turnover_team: None,
            recovering_player: None,
            receiver_rating: 0.0,
            foul: None,
        };
    }

    let offense_instructions = *state.instructions_for_team(context.offense_team_id);
    let passing_range = offense_instructions.in_possession().passing_range();

    let (prog_kind, pass_mult) = if chosen_decision == ArtrineDecisionKind::ShortPass {
        (
            ActionProgressionKind::ShortPass,
            short_pass_advance_multiplier(passing_range),
        )
    } else {
        (
            ActionProgressionKind::LongLaunch,
            long_launch_advance_multiplier(passing_range),
        )
    };
    let throw_advance = sample_action_progression(
        prog_kind,
        raw_throw_duel.net_advantage(),
        pass_mult,
        rng,
    );

    let pass_speed = calculate_pass_speed(current_carrier, attribute_keys, &carrier_fatigue);
    let flight_duration = ball_flight_duration(throw_advance, pass_speed);

    let receiver_id = select_target_from_tables(
        &iter_ctx.target_candidates,
        &pitch,
        &context.offense_pos_index,
        &context.offense_instructions_index,
        tables,
        context.is_home_offense,
        ReceptionRole::OpenPlayReceiver,
        &iter_ctx.openness_by_player,
        Some(&|id: &Uuid| state.fatigue_lookup().get(id)),
        rng,
    )
    .unwrap_or(current_carrier.id());

    let receiver_player = iter_ctx
        .target_candidates
        .iter()
        .copied()
        .find(|p| p.id() == receiver_id)
        .unwrap_or(current_carrier);

    let is_aerial = chosen_decision == ArtrineDecisionKind::LongLaunch;

    let shift_m = throw_advance * MIRIM_TO_METERS;
    let rec_x_m = if context.is_home_offense {
        (carrier_pos.raw().0 + shift_m).min(pitch.length().value())
    } else {
        (carrier_pos.raw().0 - shift_m).max(0.0)
    };
    let rec_pos = VectorPosition::from_components(rec_x_m, carrier_pos.raw().1, 0.0);

    let goalguard_id = find_goalguard(defense_players).map(|g| g.id()).ok();
    let outfield_defenders: Vec<&Player> = defense_players
        .iter()
        .copied()
        .filter(|p| Some(p.id()) != goalguard_id)
        .collect();

    if let Some(last_defender) =
        identify_last_defender(&outfield_defenders, &context.defense_pos_index)
    {
        let receiver_table = state.attribute_table_for(&receiver_id);
        let defender_table = state.attribute_table_for(&last_defender.id());
        let defender_pos = estimate_last_defender_position(&pitch, context.is_home_offense);

        let (is_fault, margin) = is_line_fault(
            receiver_player,
            receiver_table,
            rec_pos,
            last_defender,
            defender_table,
            defender_pos,
            context.is_home_offense,
        );

        if is_fault {
            let head_referee_table = state.head_referee_attribute_table();
            let peace_referee_table = state.peace_referee_attribute_table();
            let lf_ctx = LineFaultEvaluationContext::new(
                receiver_id,
                receiver_player.team_id().unwrap_or(context.offense_team_id),
                last_defender.id(),
                last_defender.team_id().unwrap_or(context.defense_team_id),
                margin,
                &head_referee_table,
                &peace_referee_table,
            );
            if let Some(foul_res) = evaluate_and_resolve_line_fault(&lf_ctx, rng) {
                return DistributionReceptionResult {
                    receiver: receiver_player,
                    caught: false,
                    reception_point: rec_pos,
                    duels,
                    advance_mirim: 0.0,
                    flight_duration,
                    engagement_duration,
                    flight_info: Some(DistributionFlightInfo {
                        receiver_id,
                        passer_id: current_carrier.id(),
                        decision_kind: chosen_decision,
                        is_aerial,
                        reception_point: rec_pos,
                        distance_mirim: throw_advance,
                        caught: false,
                    }),
                    turnover_team: Some(context.defense_team_id),
                    recovering_player: None,
                    receiver_rating: 0.0,
                    foul: Some(foul_res),
                };
            }
        }
    }

    let rec_duel_kind = if is_aerial {
        DuelKind::AerialDuel
    } else {
        DuelKind::RouteContest
    };

    let (rec_off, rec_def) = get_duel_profiles(rec_duel_kind);
    let rec_att_rating = calculate_player_duel_rating_from_table(
        receiver_player,
        context
            .offense_pos_index
            .get(&receiver_id)
            .copied()
            .unwrap_or(DomainPosition::CenterOffense),
        state.attribute_table_for(&receiver_id),
        rec_off,
        &state.fatigue_lookup().get(&receiver_id),
    );
    let rec_def_rating = calculate_side_rating(
        RatingParticipants::from_slice_with_index(defense_players, &context.defense_pos_index)
            .with_fatigue(&|id| state.fatigue_lookup().get(id))
            .with_attribute_tables(tables)
            .with_team_power(defense_power.defensive_power()),
        attribute_keys,
        rec_def,
    );

    let rec_context = iter_ctx.duel_context.for_duel_kind(rec_duel_kind);
    let rec_fatigue = state.fatigue_lookup().get(&receiver_id);
    let lead_def_fatigue2 = state.fatigue_lookup().get(&lead_defender.id());
    let rec_table = tables.get(&receiver_id);
    let lead_def_table2 = tables.get(&lead_defender.id());
    let req = DuelResolutionRequest::with_states(
        rec_duel_kind,
        rec_att_rating,
        rec_def_rating,
        receiver_player,
        lead_defender,
        rec_fatigue,
        lead_def_fatigue2,
        attribute_keys,
        &rec_context,
    )
    .with_tables(rec_table, lead_def_table2)
    .with_team_powers(
        Some(offense_power.offensive_power()),
        Some(defense_power.defensive_power()),
    );
    let raw_rec_duel = resolve_duel(req, rng);

    let rec_attributed = AttributedDuelOutcome::new(
        raw_rec_duel,
        smallvec![receiver_id],
        smallvec![lead_defender.id()],
    );
    duels.push(rec_attributed);

    let caught = raw_rec_duel.attacker_won();
    let flight_info = DistributionFlightInfo {
        receiver_id,
        passer_id: current_carrier.id(),
        decision_kind: chosen_decision,
        is_aerial,
        reception_point: rec_pos,
        distance_mirim: throw_advance,
        caught,
    };

    let (turnover_team, recovering_player) = if !caught && raw_rec_duel.net_advantage() <= -2.5 {
        (Some(context.defense_team_id), Some(lead_defender.id()))
    } else {
        (None, None)
    };

    DistributionReceptionResult {
        receiver: receiver_player,
        caught,
        reception_point: rec_pos,
        duels,
        advance_mirim: throw_advance,
        flight_duration,
        engagement_duration,
        flight_info: Some(flight_info),
        turnover_team,
        recovering_player,
        receiver_rating: rec_att_rating,
        foul: None,
    }
}