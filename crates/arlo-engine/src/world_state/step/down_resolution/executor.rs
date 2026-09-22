use crate::artrine::{ArtrineExecutionOutcome, DistributionFlightInfo};
use crate::artrine::event_translation::translate_artrine_decision_made;
use crate::error::EngineResult;
use crate::match_decision::event_translation::create_envelope;
use crate::match_decision::scoring::ScoringDecision;
use crate::possession::TouchActionType;
use crate::possession_flow::{initialize_chain_budget, is_budget_exhausted, sample_continuation};
use crate::possession_flow::touch_action_selection::select_touch_action;
use crate::time::{DurationComponentKind, DurationLedger};
use crate::world_state::cta_pass::PassPhaseResult;
use crate::world_state::match_state::MatchState;
use crate::world_state::step::down_resolution::collateral_stage::resolve_collateral_events;
use crate::world_state::step::down_resolution::contest_stage::resolve_contest;
use crate::world_state::step::down_resolution::context::{DownStaticContext, TouchDynamicContext};
use crate::world_state::step::down_resolution::progression_stage::resolve_progression;
use crate::world_state::step::down_resolution::scoring_stage::resolve_scoring;
use crate::world_state::step::setup::CallToActionContext;
use arlo_domain::{ArtrineDecisionKind, Player};
use arlo_events::EventSink;
use rand::Rng;

pub fn resolve_down<'a, R: Rng + ?Sized>(
    state: &mut MatchState,
    call_context: &CallToActionContext,
    pass_phase: &PassPhaseResult<'a>,
    carrier: &'a Player,
    offense_players: &[&'a Player],
    defense_players: &[&'a Player],
    rng: &mut R,
    sink: &mut impl EventSink,
) -> EngineResult<(ArtrineDecisionKind, ArtrineExecutionOutcome)> {
    let clock_inst = state.clock().to_instant();
    
    let static_ctx = DownStaticContext::build(
        state,
        call_context,
        offense_players.to_vec(),
        defense_players.to_vec(),
    );

    let mut current_carrier = carrier;
    let mut current_x_mirim = pass_phase.scrimmage_x_mirim;
    let mut previous_advantage = pass_phase.pass_duel_outcome.outcome().net_advantage();
    
    let mut accumulated_mirins = 0.0;
    let mut accumulated_drives = 0;
    let mut duration_ledger = DurationLedger::new();
    let mut all_duels = Vec::new();
    let mut all_fouls = Vec::new();
    let mut all_injuries = Vec::new();
    let mut all_flights = Vec::new();
    
    let mut final_turnover = None;
    let mut final_recovering = None;
    let mut final_scoring = ScoringDecision::NoOpportunity;
    let mut final_receiver_id = Some(current_carrier.id());
    
    let mut chain_state = initialize_chain_budget(current_carrier.id());
    let mut first_decision = None;

    loop {
        let current_time_seconds = state.clock().seconds_in_period() + duration_ledger.total_live().value();
        let is_true_artrine = current_carrier.id() == pass_phase.artrine.id();

        let touch_ctx = TouchDynamicContext::build(
            state,
            &static_ctx,
            call_context,
            current_carrier,
            current_x_mirim,
            previous_advantage,
            is_true_artrine,
        );
        
        let decision_res = select_touch_action(&static_ctx, &touch_ctx, state.attribute_keys(), rng);
        let decision = decision_res.chosen();
        
        if first_decision.is_none() {
            first_decision = Some(decision);
        }

        if is_true_artrine {
            let decision_event = translate_artrine_decision_made(
                touch_ctx.carrier.id(),
                decision,
                touch_ctx.down as u32,
                decision_res.chosen_probability(),
            );
            sink.record(create_envelope(state.next_sequence(), clock_inst, decision_event));
        }

        let contest = resolve_contest(&static_ctx, &touch_ctx, decision, state, rng);
        let progression = resolve_progression(&static_ctx, &touch_ctx, decision, &contest, state, rng);
        let collateral = resolve_collateral_events(&static_ctx, &touch_ctx, &contest, state, rng);

        let mut duels = contest.all_duels();
        let scoring = resolve_scoring(
            &static_ctx,
            &touch_ctx,
            decision,
            &contest,
            &progression,
            state,
            call_context,
            pass_phase,
            &mut duels,
            rng,
        );

        state.possession_mut().live_sequence_mut().record_touch(
            current_carrier.id(),
            TouchActionType::from(decision),
            touch_ctx.zone,
            current_time_seconds,
        );

        if let Some(receiver) = contest.receiver {
            if receiver.id() != current_carrier.id() && contest.attacker_won {
                let reception_time = current_time_seconds + progression.live_duration.value() * 0.5;
                state.possession_mut().live_sequence_mut().record_touch(
                    receiver.id(),
                    TouchActionType::Reception,
                    progression.new_zone,
                    reception_time,
                );
            }
        }

        duration_ledger.record_live(
            match decision {
                ArtrineDecisionKind::SelfCarry => DurationComponentKind::CarrierMovement,
                ArtrineDecisionKind::ShortPass | ArtrineDecisionKind::LongLaunch => {
                    DurationComponentKind::DistributionEngagement
                }
                ArtrineDecisionKind::Cross => DurationComponentKind::CrossFlight,
                ArtrineDecisionKind::SelfFinish => DurationComponentKind::FinishingEngagement,
            },
            progression.live_duration,
        );

        accumulated_mirins += progression.mirins_advanced;
        accumulated_drives += progression.drives_recorded;
        all_duels.extend(duels);
        all_fouls.extend(collateral.fouls);
        all_injuries.extend(collateral.injuries);

        if let Some(f) = contest.distribution_flight {
            all_flights.push(DistributionFlightInfo {
                distance_mirim: progression.mirins_advanced,
                ..f
            });
        }
        
        current_x_mirim = if static_ctx.is_home_offense {
            progression.new_normalized_proximity * static_ctx.pitch_length_mirim
        } else {
            (1.0 - progression.new_normalized_proximity) * static_ctx.pitch_length_mirim
        };
        previous_advantage = contest.net_advantage;

        if let Some(to) = contest.turnover_team {
            final_turnover = Some(to);
            final_recovering = contest.recovering_player_id;
            break;
        }

        if scoring.is_scored() || matches!(scoring, ScoringDecision::Missed { .. }) {
            final_scoring = scoring;
            break;
        }
        
        if decision == ArtrineDecisionKind::Cross || decision == ArtrineDecisionKind::LongLaunch {
            if !contest.attacker_won {
                break;
            }
        } else if decision == ArtrineDecisionKind::ShortPass {
            if !contest.attacker_won {
                break;
            }
        }

        let receiver = contest.receiver.unwrap_or(current_carrier);
        final_receiver_id = Some(receiver.id());
        current_carrier = receiver;

        chain_state.record_touch(current_carrier.id(), progression.mirins_advanced);

        if is_budget_exhausted(&chain_state) {
            break;
        }

        let offense_power = state.power_for_team(static_ctx.offense_team_id);
        let defense_instructions = state.instructions_for_team(static_ctx.defense_team_id);
        let carrier_phys = state.fatigue_for(&current_carrier.id());

        let continues = sample_continuation(
            offense_power.control_power(),
            static_ctx.offense_tempo,
            static_ctx.passing_range,
            &static_ctx.game_state_pressure,
            &carrier_phys,
            defense_instructions.out_of_possession().pressing_intensity(),
            rng,
        );

        if !continues {
            break;
        }
    }

    let end_y_mirim = state.pitch().width_mirim() * 0.5;

    let outcome = ArtrineExecutionOutcome {
        mirins_advanced: accumulated_mirins,
        drives_recorded: accumulated_drives,
        turnover: final_turnover,
        recovering_player_id: final_recovering,
        scoring_decision: final_scoring,
        duration_ledger,
        end_x_mirim: current_x_mirim,
        end_y_mirim,
        duels: all_duels,
        fouls: all_fouls,
        injuries: all_injuries,
        receiver_id: final_receiver_id,
        distribution_flight: all_flights,
    };

    Ok((first_decision.unwrap_or(ArtrineDecisionKind::SelfCarry), outcome))
}
