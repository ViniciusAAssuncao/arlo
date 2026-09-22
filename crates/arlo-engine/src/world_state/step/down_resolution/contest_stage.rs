use crate::artrine::DistributionFlightInfo;
use crate::caching::get_cached_duel_profiles;
use crate::match_decision::target_selection::{select_finisher, select_target, ReceptionRole};
use crate::resolution::group_rating::{
    calculate_anchored_side_rating, calculate_player_duel_rating_from_table, calculate_side_rating,
    RatingParticipants,
};
use crate::resolution::resolver::{resolve_duel, DuelResolutionRequest};
use crate::resolution::{AttributedDuelOutcome, DuelContext, DuelKind};
use crate::world_state::match_state::MatchState;
use crate::world_state::step::down_resolution::context::{DownStaticContext, TouchDynamicContext};
use crate::world_state::step::down_resolution::power_pair::derive_power_pair;
use arlo_domain::{ArtrineDecisionKind, Player, Position};
use arlo_math::stats::contrast::logistic;
use arlo_math::Probability;
use rand::Rng;
use smallvec::smallvec;
use std::collections::HashMap;
use uuid::Uuid;

pub struct ActionContestOutcome<'a> {
    pub primary_duel: Option<AttributedDuelOutcome>,
    pub secondary_duel: Option<AttributedDuelOutcome>,
    pub receiver: Option<&'a Player>,
    pub turnover_team: Option<Uuid>,
    pub recovering_player_id: Option<Uuid>,
    pub attacker_won: bool,
    pub net_advantage: f64,
    pub is_aerial: bool,
    pub distribution_flight: Option<DistributionFlightInfo>,
}

impl<'a> ActionContestOutcome<'a> {
    pub fn all_duels(&self) -> Vec<AttributedDuelOutcome> {
        let mut duels = Vec::with_capacity(2);
        if let Some(d) = &self.primary_duel {
            duels.push(d.clone());
        }
        if let Some(d) = &self.secondary_duel {
            duels.push(d.clone());
        }
        duels
    }
}

pub fn resolve_contest<'a, R: Rng + ?Sized>(
    static_ctx: &DownStaticContext<'a>,
    touch_ctx: &TouchDynamicContext<'a>,
    carrier: &'a Player,
    primary_defender: &'a Player,
    decision: ArtrineDecisionKind,
    state: &MatchState,
    rng: &mut R,
) -> ActionContestOutcome<'a> {
    match decision {
        ArtrineDecisionKind::SelfCarry => {
            let duel_kind = if touch_ctx.is_true_artrine {
                DuelKind::ArtroBreakthrough
            } else {
                DuelKind::RunBreakthrough
            };

            let (att_prof, def_prof) = get_cached_duel_profiles(duel_kind);
            let att_rating = calculate_player_duel_rating_from_table(
                carrier,
                touch_ctx.carrier_pos_domain,
                &touch_ctx.carrier_table,
                att_prof,
                &touch_ctx.carrier_fatigue,
            );
            let def_rating = calculate_player_duel_rating_from_table(
                primary_defender,
                touch_ctx.primary_defender_pos_domain,
                &touch_ctx.primary_defender_table,
                def_prof,
                &touch_ctx.primary_defender_fatigue,
            );

            let offense_power = state.power_for_team(static_ctx.offense_team_id);
            let defense_power = state.power_for_team(static_ctx.defense_team_id);

            let power_pair = derive_power_pair(
                offense_power,
                defense_power,
                duel_kind,
                &state.tuning().league_strength_scale,
                &state.tuning().team_strength_profile,
                &state.tuning().home_advantage_profile,
                Some(touch_ctx.carrier_pos_domain),
                Some(touch_ctx.primary_defender_pos_domain),
                touch_ctx.duel_context.attacker_is_home(),
                touch_ctx.duel_context.defender_is_home(),
            );

            let req = DuelResolutionRequest::with_states(
                duel_kind,
                att_rating,
                def_rating,
                carrier,
                primary_defender,
                touch_ctx.carrier_fatigue,
                touch_ctx.primary_defender_fatigue,
                state.attribute_keys(),
                &touch_ctx.duel_context,
            )
            .with_tables(Some(&touch_ctx.carrier_table), Some(&touch_ctx.primary_defender_table))
            .with_power_pair(Some(power_pair));

            let raw_duel = resolve_duel(req, rng);
            let attacker_won = raw_duel.attacker_won();
            let net_advantage = raw_duel.net_advantage();

            let to_base = if attacker_won { -5.0 } else { -3.0 };
            let to_p = (logistic(to_base - 0.20 * net_advantage)
                / touch_ctx.risk_profile.tolerance_index())
            .clamp(0.03, 0.25);

            let turnover_team = if Probability::new_clamped(to_p).sample(rng) {
                Some(static_ctx.defense_team_id)
            } else {
                None
            };

            let recovering_player_id = turnover_team.map(|_| primary_defender.id());

            let primary_duel = AttributedDuelOutcome::new(
                raw_duel,
                smallvec![carrier.id()],
                smallvec![primary_defender.id()],
            );

            ActionContestOutcome {
                primary_duel: Some(primary_duel),
                secondary_duel: None,
                receiver: Some(carrier),
                turnover_team,
                recovering_player_id,
                attacker_won,
                net_advantage,
                is_aerial: false,
                distribution_flight: None,
            }
        }
        ArtrineDecisionKind::ShortPass | ArtrineDecisionKind::LongLaunch => {
            let is_aerial = decision == ArtrineDecisionKind::LongLaunch;
            let throw_kind = if is_aerial {
                DuelKind::LongDistribution
            } else {
                DuelKind::ShortDistribution
            };

            let throw_initiative_offset = if is_aerial { 0.50 } else { 1.20 };
            let throw_context = DuelContext::with_offsets(
                touch_ctx.duel_context.orientation(),
                touch_ctx.duel_context.is_home_offense(),
                touch_ctx.duel_context.home_advantage_duel_logit(),
                touch_ctx.duel_context.aggression_logit_offset(),
                touch_ctx.duel_context.misdirection_logit_offset(),
                touch_ctx.duel_context.physicality_logit_offset() + throw_initiative_offset,
            );

            let (att_prof, def_prof) = get_cached_duel_profiles(throw_kind);
            let tables = state.teams.player_attribute_tables();
            let offense_power = state.power_for_team(static_ctx.offense_team_id);
            let defense_power = state.power_for_team(static_ctx.defense_team_id);

            let throw_power_pair = derive_power_pair(
                offense_power,
                defense_power,
                throw_kind,
                &state.tuning().league_strength_scale,
                &state.tuning().team_strength_profile,
                &state.tuning().home_advantage_profile,
                Some(touch_ctx.carrier_pos_domain),
                Some(touch_ctx.primary_defender_pos_domain),
                throw_context.attacker_is_home(),
                throw_context.defender_is_home(),
            );

            let att_rating = calculate_anchored_side_rating(
                carrier,
                touch_ctx.carrier_pos_domain,
                RatingParticipants::from_slice_with_index(
                    &touch_ctx.target_candidates,
                    state.offensive_position_index_for_team(static_ctx.offense_team_id),
                )
                .with_fatigue(&|id| state.fatigue_lookup().get(id))
                .with_attribute_tables(tables),
                state.attribute_keys(),
                att_prof,
            );
            let def_rating = calculate_side_rating(
                RatingParticipants::from_slice_with_index(
                    &static_ctx.defense_players,
                    state.defensive_position_index_for_team(static_ctx.defense_team_id),
                )
                .with_fatigue(&|id| state.fatigue_lookup().get(id))
                .with_attribute_tables(tables),
                state.attribute_keys(),
                def_prof,
            );

            let req = DuelResolutionRequest::with_states(
                throw_kind,
                att_rating,
                def_rating,
                carrier,
                primary_defender,
                touch_ctx.carrier_fatigue,
                touch_ctx.primary_defender_fatigue,
                state.attribute_keys(),
                &throw_context,
            )
            .with_tables(
                Some(&touch_ctx.carrier_table),
                Some(&touch_ctx.primary_defender_table),
            )
            .with_power_pair(Some(throw_power_pair));

            let raw_throw_duel = resolve_duel(req, rng);
            let throw_won = raw_throw_duel.attacker_won();

            let receiver_id = select_target(
                &touch_ctx.target_candidates,
                state.pitch(),
                state.offensive_position_index_for_team(static_ctx.offense_team_id),
                state.instructions_index_for_team(static_ctx.offense_team_id),
                Some(state.role_index_for_team(static_ctx.offense_team_id)),
                tables,
                static_ctx.is_home_offense,
                ReceptionRole::OpenPlayReceiver,
                &HashMap::new(),
                Some(&|id: &Uuid| state.fatigue_lookup().get(id)),
                rng,
            )
            .unwrap_or(carrier.id());

            let receiver = touch_ctx
                .target_candidates
                .iter()
                .copied()
                .find(|p| p.id() == receiver_id)
                .unwrap_or(carrier);

            let rec_duel_kind = if is_aerial {
                DuelKind::AerialDuel
            } else {
                DuelKind::RouteContest
            };

            let rec_initiative_offset = if is_aerial { 0.20 } else { 0.60 };
            let rec_context = DuelContext::with_offsets(
                touch_ctx.duel_context.orientation(),
                touch_ctx.duel_context.is_home_offense(),
                touch_ctx.duel_context.home_advantage_duel_logit(),
                touch_ctx.duel_context.aggression_logit_offset(),
                touch_ctx.duel_context.misdirection_logit_offset(),
                touch_ctx.duel_context.physicality_logit_offset() + rec_initiative_offset,
            );

            let (rec_att_prof, rec_def_prof) = get_cached_duel_profiles(rec_duel_kind);
            let rec_pos = state
                .offensive_position_index_for_team(static_ctx.offense_team_id)
                .get(&receiver_id)
                .copied()
                .unwrap_or(Position::CenterOffense);

            let rec_power_pair = derive_power_pair(
                offense_power,
                defense_power,
                rec_duel_kind,
                &state.tuning().league_strength_scale,
                &state.tuning().team_strength_profile,
                &state.tuning().home_advantage_profile,
                Some(rec_pos),
                Some(touch_ctx.primary_defender_pos_domain),
                rec_context.attacker_is_home(),
                rec_context.defender_is_home(),
            );

            let rec_att_rating = calculate_player_duel_rating_from_table(
                receiver,
                rec_pos,
                state.attribute_table_for(&receiver_id),
                rec_att_prof,
                &state.fatigue_lookup().get(&receiver_id),
            );
            let rec_def_rating = calculate_side_rating(
                RatingParticipants::from_slice_with_index(
                    &static_ctx.defense_players,
                    state.defensive_position_index_for_team(static_ctx.defense_team_id),
                )
                .with_fatigue(&|id| state.fatigue_lookup().get(id))
                .with_attribute_tables(tables),
                state.attribute_keys(),
                rec_def_prof,
            );

            let rec_fatigue = state.fatigue_lookup().get(&receiver_id);
            let rec_table = state.attribute_table_for(&receiver_id);
            let rec_req = DuelResolutionRequest::with_states(
                rec_duel_kind,
                rec_att_rating,
                rec_def_rating,
                receiver,
                primary_defender,
                rec_fatigue,
                touch_ctx.primary_defender_fatigue,
                state.attribute_keys(),
                &rec_context,
            )
            .with_tables(Some(rec_table), Some(&touch_ctx.primary_defender_table))
            .with_power_pair(Some(rec_power_pair));

            let raw_rec_duel = resolve_duel(rec_req, rng);
            let catch_won = raw_rec_duel.attacker_won();

            let attacker_won = throw_won && catch_won;
            let net_advantage = (raw_throw_duel.net_advantage() + raw_rec_duel.net_advantage()) * 0.5;

            let turnover_threshold = if is_aerial { -4.0 } else { -4.5 };
            let turnover_team = if !attacker_won && raw_rec_duel.net_advantage() <= turnover_threshold {
                Some(static_ctx.defense_team_id)
            } else {
                None
            };
            let recovering_player_id = turnover_team.map(|_| primary_defender.id());

            let primary_duel = AttributedDuelOutcome::new(
                raw_throw_duel,
                smallvec![carrier.id()],
                smallvec![primary_defender.id()],
            );
            let secondary_duel = AttributedDuelOutcome::new(
                raw_rec_duel,
                smallvec![receiver.id()],
                smallvec![primary_defender.id()],
            );

            let flight_info = DistributionFlightInfo {
                receiver_id: receiver.id(),
                passer_id: carrier.id(),
                decision_kind: decision,
                is_aerial,
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
                is_aerial,
                distribution_flight: Some(flight_info),
            }
        }
        ArtrineDecisionKind::Cross => {
            let finisher_id = select_finisher(
                &touch_ctx.target_candidates,
                Some(state.role_index_for_team(static_ctx.offense_team_id)),
                state.pitch(),
                state.offensive_position_index_for_team(static_ctx.offense_team_id),
                state.instructions_index_for_team(static_ctx.offense_team_id),
                state.teams.player_attribute_tables(),
                static_ctx.is_home_offense,
                &HashMap::new(),
                Some(&|id: &Uuid| state.fatigue_lookup().get(id)),
                rng,
            )
            .unwrap_or(carrier.id());

            let finisher = touch_ctx
                .target_candidates
                .iter()
                .copied()
                .find(|p| p.id() == finisher_id)
                .unwrap_or(carrier);

            let (att_prof, def_prof) = get_cached_duel_profiles(DuelKind::CrossDistribution);
            let att_rating = calculate_player_duel_rating_from_table(
                carrier,
                touch_ctx.carrier_pos_domain,
                &touch_ctx.carrier_table,
                att_prof,
                &touch_ctx.carrier_fatigue,
            );
            let def_rating = calculate_player_duel_rating_from_table(
                primary_defender,
                touch_ctx.primary_defender_pos_domain,
                &touch_ctx.primary_defender_table,
                def_prof,
                &touch_ctx.primary_defender_fatigue,
            );

            let offense_power = state.power_for_team(static_ctx.offense_team_id);
            let defense_power = state.power_for_team(static_ctx.defense_team_id);

            let power_pair = derive_power_pair(
                offense_power,
                defense_power,
                DuelKind::CrossDistribution,
                &state.tuning().league_strength_scale,
                &state.tuning().team_strength_profile,
                &state.tuning().home_advantage_profile,
                Some(touch_ctx.carrier_pos_domain),
                Some(touch_ctx.primary_defender_pos_domain),
                touch_ctx.duel_context.attacker_is_home(),
                touch_ctx.duel_context.defender_is_home(),
            );

            let req = DuelResolutionRequest::with_states(
                DuelKind::CrossDistribution,
                att_rating,
                def_rating,
                carrier,
                primary_defender,
                touch_ctx.carrier_fatigue,
                touch_ctx.primary_defender_fatigue,
                state.attribute_keys(),
                &touch_ctx.duel_context,
            )
            .with_tables(
                Some(&touch_ctx.carrier_table),
                Some(&touch_ctx.primary_defender_table),
            )
            .with_power_pair(Some(power_pair));

            let raw_duel = resolve_duel(req, rng);
            let attacker_won = raw_duel.attacker_won();
            let net_advantage = raw_duel.net_advantage();

            let turnover_threshold = -4.0;
            let turnover_team = if !attacker_won && net_advantage <= turnover_threshold {
                Some(static_ctx.defense_team_id)
            } else {
                None
            };
            let recovering_player_id = turnover_team.map(|_| primary_defender.id());

            let primary_duel = AttributedDuelOutcome::new(
                raw_duel,
                smallvec![carrier.id()],
                smallvec![primary_defender.id()],
            );

            ActionContestOutcome {
                primary_duel: Some(primary_duel),
                secondary_duel: None,
                receiver: Some(finisher),
                turnover_team,
                recovering_player_id,
                attacker_won,
                net_advantage,
                is_aerial: true,
                distribution_flight: None,
            }
        }
        ArtrineDecisionKind::SelfFinish => ActionContestOutcome {
            primary_duel: None,
            secondary_duel: None,
            receiver: Some(carrier),
            turnover_team: None,
            recovering_player_id: None,
            attacker_won: true,
            net_advantage: 0.0,
            is_aerial: false,
            distribution_flight: None,
        },
    }
}