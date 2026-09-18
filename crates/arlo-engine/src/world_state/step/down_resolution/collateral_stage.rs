use crate::injury::contact::{evaluate_and_resolve_contact_injury, ContactInjuryContext};
use crate::injury::outcome::InjuryIncidentResolution;
use crate::officiating::foul::{evaluate_and_resolve_foul, FoulEvaluationContext, FoulResolution};
use crate::officiating::line_fault::{
    evaluate_and_resolve_line_fault, identify_last_defender, LineFaultEvaluationContext,
};
use crate::play_resolution::contact_events::{evaluate_contact_likelihood, sample_contact_event};
use crate::possession::PitchState;
use crate::world_state::match_state::MatchState;
use crate::world_state::step::down_resolution::contest_stage::ActionContestOutcome;
use crate::world_state::step::down_resolution::context::DownResolutionContext;
use arlo_domain::Player;
use rand::Rng;

pub struct ActionCollateralOutcome {
    pub fouls: Vec<FoulResolution>,
    pub injuries: Vec<InjuryIncidentResolution>,
}

pub fn resolve_collateral_events<R: Rng + ?Sized>(
    ctx: &DownResolutionContext<'_>,
    contest: &ActionContestOutcome<'_>,
    state: &MatchState,
    rng: &mut R,
) -> ActionCollateralOutcome {
    let mut fouls = Vec::new();
    let mut injuries = Vec::new();

    let primary_duel = match &contest.primary_duel {
        Some(d) => d.outcome(),
        None => match &contest.secondary_duel {
            Some(d) => d.outcome(),
            None => return ActionCollateralOutcome { fouls, injuries },
        },
    };

    let carrier_susceptibility = state
        .player_injury_profile(&ctx.carrier.id())
        .injury_susceptibility_multiplier();
    let defender_susceptibility = state
        .player_injury_profile(&ctx.primary_defender.id())
        .injury_susceptibility_multiplier();
    let referee_table = state.head_referee_attribute_table();
    let offense_instructions = *state.instructions_for_team(ctx.offense_team_id);
    let defense_instructions = *state.instructions_for_team(ctx.defense_team_id);

    let pitch_state = PitchState::new(
        ctx.down,
        ctx.remaining_advance_mirim,
        ctx.zone,
        ctx.channel,
        ctx.normalized_proximity,
        ctx.drives_in_series,
        ctx.is_bonus_phase,
    );

    let contact_profile = evaluate_contact_likelihood(
        &ctx.carrier_table,
        &ctx.carrier_fatigue,
        carrier_susceptibility,
        &ctx.primary_defender_table,
        &ctx.primary_defender_fatigue,
        defender_susceptibility,
        &offense_instructions,
        &defense_instructions,
        &referee_table,
        &pitch_state,
    );

    let contact_sampling = sample_contact_event(&contact_profile, rng);

    let peace_referee_table = state.peace_referee_attribute_table();
    let fault_catalog = state.fault_catalog_arc();
    let foul_eval_ctx = FoulEvaluationContext::new(
        ctx.carrier.id(),
        ctx.offense_team_id,
        ctx.primary_defender.id(),
        ctx.defense_team_id,
        &ctx.carrier_table,
        &ctx.primary_defender_table,
        ctx.carrier_fatigue,
        ctx.primary_defender_fatigue,
        &referee_table,
        &peace_referee_table,
        *primary_duel,
        ctx.duel_context,
        contact_sampling.contact_severity,
        ctx.game_state_pressure,
        true,
        ctx.zone,
    );

    if let Some(foul_res) = evaluate_and_resolve_foul(&foul_eval_ctx, &fault_catalog, rng) {
        fouls.push(foul_res);
    }

    if let Some(receiver) = contest.receiver {
        if receiver.id() != ctx.carrier.id() {
            let outfield_defenders: Vec<&Player> = ctx
                .defense_players
                .iter()
                .copied()
                .filter(|p| p.id() != ctx.primary_defender.id())
                .collect();

            if let Some(last_defender) = identify_last_defender(
                &outfield_defenders,
                state.defensive_position_index_for_team(ctx.defense_team_id),
            ) {
                let rec_table = state.attribute_table_for(&receiver.id());
                let def_table = state.attribute_table_for(&last_defender.id());

                let lf_ctx = LineFaultEvaluationContext::new(
                    receiver.id(),
                    ctx.offense_team_id,
                    last_defender.id(),
                    ctx.defense_team_id,
                    rec_table,
                    def_table,
                    &referee_table,
                    &peace_referee_table,
                    &ctx.duel_context,
                    ctx.zone,
                );
                if let Some(lf_res) = evaluate_and_resolve_line_fault(&lf_ctx, rng) {
                    fouls.push(lf_res);
                }
            }
        }
    }

    let injury_catalog = state.injury_catalog_arc();
    let contact_injury_ctx = ContactInjuryContext::new(
        contact_sampling.contact_severity,
        ctx.carrier.id(),
        ctx.offense_team_id,
        &ctx.carrier_table,
        ctx.carrier_fatigue,
        state.player_injury_profile(&ctx.carrier.id()),
        25.0,
        ctx.primary_defender.id(),
        ctx.defense_team_id,
        &ctx.primary_defender_table,
        ctx.primary_defender_fatigue,
        state.player_injury_profile(&ctx.primary_defender.id()),
        25.0,
    );

    if contact_sampling.carrier_injured {
        if let Some(inj) =
            evaluate_and_resolve_contact_injury(true, &contact_injury_ctx, &injury_catalog, rng)
        {
            injuries.push(inj);
        }
    }
    if contact_sampling.defender_injured {
        if let Some(inj) =
            evaluate_and_resolve_contact_injury(false, &contact_injury_ctx, &injury_catalog, rng)
        {
            injuries.push(inj);
        }
    }

    ActionCollateralOutcome { fouls, injuries }
}