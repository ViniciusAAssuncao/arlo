use crate::attributes::PlayerAttributeTable;
use crate::injury::contact::{evaluate_and_resolve_contact_injury, ContactInjuryContext};
use crate::injury::outcome::InjuryIncidentResolution;
use crate::officiating::foul::{evaluate_and_resolve_foul, FoulEvaluationContext, FoulResolution};
use crate::physical::PhysicalState;
use crate::play_resolution::contact_events::{evaluate_contact_likelihood, sample_contact_event};
use crate::play_resolution::field_context::PitchState;
use crate::resolution::DuelOutcome;
use crate::world_state::match_state::MatchState;
use crate::world_state::step::open_play_loop::action_context::OpenPlayIterationContext;
use crate::world_state::step::setup::CallToActionContext;
use arlo_domain::Player;
use rand::Rng;

pub fn evaluate_contact_events<R: Rng + ?Sized>(
    state: &mut MatchState,
    context: &CallToActionContext,
    iter_ctx: &OpenPlayIterationContext<'_>,
    carrier: &Player,
    defender: &Player,
    carrier_fatigue: PhysicalState,
    defender_fatigue: PhysicalState,
    carrier_table: PlayerAttributeTable,
    defender_table: PlayerAttributeTable,
    duel_outcome: DuelOutcome,
    rng: &mut R,
) -> (Vec<FoulResolution>, Vec<InjuryIncidentResolution>) {
    let mut fouls = Vec::new();
    let mut injuries = Vec::new();

    let carrier_susceptibility = state
        .player_injury_profile(&carrier.id())
        .injury_susceptibility_multiplier();
    let defender_susceptibility = state
        .player_injury_profile(&defender.id())
        .injury_susceptibility_multiplier();
    let referee_table = state.head_referee_attribute_table();
    let offense_instructions = *state.instructions_for_team(context.offense_team_id);
    let defense_instructions = *state.instructions_for_team(context.defense_team_id);

    let pitch_state = PitchState::new(
        state.possession().down(),
        state
            .possession()
            .series_state()
            .remaining_mirins_to_target(),
        iter_ctx.zone,
        iter_ctx.channel,
        iter_ctx.normalized_proximity,
        state.drives_in_current_series(),
        state.possession().is_bonus_phase(),
    );

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

    let peace_referee_table = state.peace_referee_attribute_table();
    let fault_catalog = state.fault_catalog_arc();
    let foul_eval_ctx = FoulEvaluationContext::new(
        carrier.id(),
        context.offense_team_id,
        defender.id(),
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
        iter_ctx.zone,
    );

    if let Some(foul_res) = evaluate_and_resolve_foul(&foul_eval_ctx, &fault_catalog, rng) {
        fouls.push(foul_res);
    }

    let injury_catalog = state.injury_catalog_arc();
    let contact_injury_ctx = ContactInjuryContext::new(
        contact_sampling.contact_severity,
        carrier.id(),
        context.offense_team_id,
        &carrier_table,
        carrier_fatigue,
        state.player_injury_profile(&carrier.id()),
        25.0,
        defender.id(),
        context.defense_team_id,
        &defender_table,
        defender_fatigue,
        state.player_injury_profile(&defender.id()),
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

    (fouls, injuries)
}
