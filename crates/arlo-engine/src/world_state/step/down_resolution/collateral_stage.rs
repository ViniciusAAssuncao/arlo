use crate::injury::contact::{evaluate_and_resolve_contact_injury, ContactInjuryContext};
use crate::injury::outcome::InjuryIncidentResolution;
use crate::officiating::foul::{evaluate_and_resolve_foul, FoulEvaluationContext, FoulResolution};
use crate::officiating::line_fault::{
    evaluate_and_resolve_line_fault, identify_last_defender, LineFaultEvaluationContext,
};
use crate::physical::models::age::calculate_player_age;
use crate::play_resolution::contact_events::{evaluate_contact_likelihood, sample_contact_event};
use crate::possession::PitchState;
use crate::world_state::match_state::MatchState;
use crate::world_state::step::down_resolution::contest_stage::ActionContestOutcome;
use crate::world_state::step::down_resolution::context::{DownStaticContext, TouchDynamicContext};
use arlo_domain::Player;
use rand::Rng;

pub struct ActionCollateralOutcome {
    pub fouls: Vec<FoulResolution>,
    pub injuries: Vec<InjuryIncidentResolution>,
}

pub fn resolve_collateral_events<R: Rng + ?Sized>(
    static_ctx: &DownStaticContext<'_>,
    touch_ctx: &TouchDynamicContext<'_>,
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
        .player_injury_profile(&touch_ctx.carrier.id())
        .injury_susceptibility_multiplier();
    let defender_susceptibility = state
        .player_injury_profile(&touch_ctx.primary_defender.id())
        .injury_susceptibility_multiplier();
    let referee_table = state.head_referee_attribute_table();
    let offense_instructions = *state.instructions_for_team(static_ctx.offense_team_id);
    let defense_instructions = *state.instructions_for_team(static_ctx.defense_team_id);

    let pitch_state = PitchState::new(
        touch_ctx.down,
        touch_ctx.remaining_advance_mirim,
        touch_ctx.zone,
        touch_ctx.channel,
        touch_ctx.normalized_proximity,
        touch_ctx.drives_in_series,
        touch_ctx.is_bonus_phase,
    );

    let contact_profile = evaluate_contact_likelihood(
        &touch_ctx.carrier_table,
        &touch_ctx.carrier_fatigue,
        carrier_susceptibility,
        &touch_ctx.primary_defender_table,
        &touch_ctx.primary_defender_fatigue,
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
        touch_ctx.carrier.id(),
        static_ctx.offense_team_id,
        touch_ctx.primary_defender.id(),
        static_ctx.defense_team_id,
        &touch_ctx.carrier_table,
        &touch_ctx.primary_defender_table,
        touch_ctx.carrier_fatigue,
        touch_ctx.primary_defender_fatigue,
        &referee_table,
        &peace_referee_table,
        *primary_duel,
        touch_ctx.duel_context,
        contact_sampling.contact_severity,
        static_ctx.game_state_pressure,
        true,
        touch_ctx.zone,
    );

    if let Some(foul_res) = evaluate_and_resolve_foul(&foul_eval_ctx, &fault_catalog, rng) {
        fouls.push(foul_res);
    }

    if let Some(receiver) = contest.receiver {
        if receiver.id() != touch_ctx.carrier.id() {
            let outfield_defenders: Vec<&Player> = static_ctx
                .defense_players
                .iter()
                .copied()
                .filter(|p| p.id() != touch_ctx.primary_defender.id())
                .collect();

            if let Some(last_defender) = identify_last_defender(
                &outfield_defenders,
                state.defensive_position_index_for_team(static_ctx.defense_team_id),
            ) {
                let rec_table = state.attribute_table_for(&receiver.id());
                let def_table = state.attribute_table_for(&last_defender.id());

                let lf_ctx = LineFaultEvaluationContext::new(
                    receiver.id(),
                    static_ctx.offense_team_id,
                    last_defender.id(),
                    static_ctx.defense_team_id,
                    rec_table,
                    def_table,
                    &referee_table,
                    &peace_referee_table,
                    &touch_ctx.duel_context,
                    touch_ctx.zone,
                );
                if let Some(lf_res) = evaluate_and_resolve_line_fault(&lf_ctx, rng) {
                    fouls.push(lf_res);
                }
            }
        }
    }

    if contact_sampling.contact_occurred {
        let match_date = state.match_date_unix_seconds();
        let carrier_age = calculate_player_age(touch_ctx.carrier, match_date);
        let defender_age = calculate_player_age(touch_ctx.primary_defender, match_date);
        let injury_tuning = *state.tuning().injury_tuning();
        let injury_catalog = state.injury_catalog_arc();

        let contact_injury_ctx = ContactInjuryContext::new(
            contact_sampling.contact_severity,
            touch_ctx.carrier.id(),
            static_ctx.offense_team_id,
            &touch_ctx.carrier_table,
            touch_ctx.carrier_fatigue,
            state.player_injury_profile(&touch_ctx.carrier.id()),
            carrier_age,
            touch_ctx.primary_defender.id(),
            static_ctx.defense_team_id,
            &touch_ctx.primary_defender_table,
            touch_ctx.primary_defender_fatigue,
            state.player_injury_profile(&touch_ctx.primary_defender.id()),
            defender_age,
        );

        if let Some(inj) =
            evaluate_and_resolve_contact_injury(true, &contact_injury_ctx, &injury_catalog, &injury_tuning, rng)
        {
            injuries.push(inj);
        }
        if let Some(inj) =
            evaluate_and_resolve_contact_injury(false, &contact_injury_ctx, &injury_catalog, &injury_tuning, rng)
        {
            injuries.push(inj);
        }
    }

    ActionCollateralOutcome { fouls, injuries }
}
