pub mod down_resolution;
pub mod play_resolution;
pub mod readiness;
pub mod setup;
pub mod step_outcome;

pub use down_resolution::resolve_down;
pub use play_resolution::*;
pub use readiness::peek_pending_manager_decisions;
pub use setup::{setup_call_to_action_context, CallToActionContext};
pub use step_outcome::PlayStepOutcome;

use crate::error::EngineResult;
use crate::manager_ai::orchestrator::ManagerAiEngine;
use crate::match_decision::play_outcome::DetailedPlayOutcome;
use crate::match_decision::scoring::ScoringDecision;
use crate::officiating::punishment::capture_play_reversal_snapshot;
use crate::rng::RngStream;
use crate::time::{DurationComponentKind, DurationLedger};
use crate::world_state::cta_pass::resolve_pass_phase;
use crate::world_state::match_state::MatchState;
use crate::world_state::play_transition::{apply_play_transition, EventPublisher};
use arlo_domain::ArtrineDecisionKind;
use arlo_events::EventSink;
use arlo_manager_control::ManagerDecisionInbox;
use arlo_math::units::Duration;
use uuid::Uuid;

fn build_finished_match_outcome(state: &MatchState) -> DetailedPlayOutcome {
    let scrimmage_x = state.possession().scrimmage_x_mirim();
    let center_y = state.pitch().width_mirim() * 0.5;
    DetailedPlayOutcome {
        offense_team_id: state.possession().offense(),
        defense_team_id: state.possession().defense(),
        passer_id: Uuid::nil(),
        artrine_id: Uuid::nil(),
        down_number: state.possession().down() as u32,
        scrimmage_x_mirim: scrimmage_x,
        pass_completed: false,
        pass_is_aerial: false,
        reception_x_mirim: scrimmage_x,
        reception_y_mirim: center_y,
        drives_recorded: 0,
        mirins_advanced: 0.0,
        duels: Vec::new(),
        turnover: None,
        recovering_player_id: None,
        receiver_id: None,
        lost_by_player_id: None,
        out_of_bounds: false,
        arbitral_stoppage: true,
        last_valid_x_mirim: scrimmage_x,
        last_valid_y_mirim: center_y,
        possession_control_seconds: None,
        scoring_decision: ScoringDecision::NoOpportunity,
    }
}

pub fn step_call_to_action(
    state: &mut MatchState,
    manager_decision_inbox: &ManagerDecisionInbox,
    sink: &mut impl EventSink,
) -> EngineResult<PlayStepOutcome> {
    if state.is_match_finished() {
        return Ok(PlayStepOutcome::Resolved(build_finished_match_outcome(state)));
    }

    state.refresh_team_powers_if_needed();

    let pending =
        readiness::resolve_pending_manager_decisions(state, sink, manager_decision_inbox);
    if !pending.is_empty() {
        return Ok(PlayStepOutcome::Pending(pending));
    }

    let pre_play_snapshot = capture_play_reversal_snapshot(state);

    let (last_play_call_id, last_play_failed) = state
        .last_play_outcome_summary()
        .map(|(id, failed)| (Some(id), failed))
        .unwrap_or((None, false));

    let offense_id = state.possession().offense();
    let seq = state.next_sequence();
    let mut ai_rng = state
        .rng_provider()
        .indexed_rng_for(RngStream::PlayCallSelection, seq);

    {
        let mut publisher = EventPublisher::new(state, sink);
        ManagerAiEngine::on_down_start(
            &mut publisher,
            offense_id,
            last_play_call_id,
            last_play_failed,
            &mut ai_rng,
        );
    }

    let context = setup_call_to_action_context(state);
    let active_play_call_id = context.active_play_call.as_ref().map(|pc| pc.id());
    let offense_players = context.offense_players();
    let defense_players = context.defense_players();

    let down = state.possession().down();
    let current_carrier_id = state.possession().current_carrier();

    let pass_phase = if down == 1 || current_carrier_id.is_none() {
        resolve_pass_phase(
            state,
            &offense_players,
            &context.offense_pos_index,
            &context.offense_role_index,
            &defense_players,
            context.is_home_offense,
            context.offense_team_id,
            context.defense_team_id,
            sink,
        )?
    } else {
        let passer = crate::lineup_runtime::find_player_by_position(&offense_players, arlo_domain::Position::Passer).unwrap_or(offense_players[0]);
        let artrine = crate::lineup_runtime::find_player_by_position(&offense_players, arlo_domain::Position::Artrine).unwrap_or(offense_players[0]);
        let goalguard = crate::lineup_runtime::find_player_by_position(&defense_players, arlo_domain::Position::Goalguard).unwrap_or(defense_players[0]);
        let pass_rusher = crate::lineup_runtime::find_player_by_position(&defense_players, arlo_domain::Position::PassRusher).unwrap_or(defense_players[0]);

        let dummy_outcome = crate::resolution::DuelOutcome::new(
            crate::resolution::DuelKind::PassProtection,
            true,
            10.0,
            10.0,
            arlo_math::Probability::new_clamped(1.0),
            0.0,
        );
        let pass_duel_outcome = crate::resolution::AttributedDuelOutcome::new(
            dummy_outcome,
            smallvec::smallvec![passer.id()],
            smallvec::smallvec![pass_rusher.id()],
        );

        crate::world_state::cta_pass::PassPhaseResult {
            passer,
            artrine,
            pass_rusher,
            goalguard,
            pass_duel_outcome,
            pass_completed: true,
            is_aerial: false,
            reception_x_mirim: state.possession().scrimmage_x_mirim(),
            reception_y_mirim: state.pitch().width_mirim() * 0.5,
            down_number: down as u32,
            scrimmage_x_mirim: state.possession().scrimmage_x_mirim(),
            duration_ledger: DurationLedger::new(),
        }
    };

    let carrier = if down == 1 || current_carrier_id.is_none() || !pass_phase.pass_completed {
        pass_phase.artrine
    } else {
        let cid = current_carrier_id.unwrap();
        offense_players.iter().copied().find(|p| p.id() == cid).unwrap_or(pass_phase.artrine)
    };

    let (chosen_decision, execution_outcome) = if !pass_phase.pass_completed {
        let center_y = state.pitch().width_mirim() * 0.5;
        let mut ledger = DurationLedger::new();
        ledger.record_live(
            DurationComponentKind::PassProtectionEngagement,
            Duration::new(14.0),
        );
        (
            ArtrineDecisionKind::SelfCarry,
            crate::artrine::ArtrineExecutionOutcome {
                mirins_advanced: 0.0,
                drives_recorded: 0,
                turnover: None,
                recovering_player_id: None,
                scoring_decision: ScoringDecision::NoOpportunity,
                duration_ledger: ledger,
                end_x_mirim: pass_phase.scrimmage_x_mirim,
                end_y_mirim: center_y,
                duels: Vec::new(),
                fouls: Vec::new(),
                injuries: Vec::new(),
                receiver_id: None,
                distribution_flight: None,
            },
        )
    } else {
        let seq = state.event_sequence();
        let mut rng = state
            .rng_provider()
            .iteration_rng(RngStream::DuelResolution, seq, 1);
        down_resolution::resolve_down(
            state,
            &context,
            &pass_phase,
            carrier,
            &offense_players,
            &defense_players,
            &mut rng,
            sink,
        )?
    };

    let detailed_outcome = apply_play_transition(
        state,
        pass_phase,
        chosen_decision,
        execution_outcome,
        context.offense_team_id,
        context.defense_team_id,
        active_play_call_id,
        pre_play_snapshot,
        manager_decision_inbox,
        sink,
    );

    Ok(PlayStepOutcome::Resolved(detailed_outcome))
}