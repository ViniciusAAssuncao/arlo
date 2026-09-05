use crate::match_decision::event_translation::{
    create_envelope, translate_duel_resolved, translate_scoring_decision,
};
use crate::match_decision::finisher_selection::select_finisher;
use crate::match_decision::scoring::{
    evaluate_scoring_opportunity, field_point_points, goal_point_points, ScoringDecision,
    ScoringOpportunity,
};
use crate::resolution::context::DuelContext;
use crate::resolution::duel_kind::DuelKind;
use crate::resolution::outcome::DuelOutcome;
use crate::resolution::resolver::resolve_duel_for_participants;
use crate::world_state::cta_pass::PassPhaseResult;
use crate::world_state::cta_progression::ProgressionPhaseResult;
use crate::world_state::match_state::MatchState;
use arlo_domain::Player;
use arlo_events::{EventSink, ScoringPost};
use uuid::Uuid;

pub struct FinishingPhaseResult {
    pub scoring_decision: ScoringDecision,
    pub finish_duel_outcome: Option<DuelOutcome>,
}

pub fn resolve_finishing_phase(
    state: &mut MatchState,
    pass_phase: &PassPhaseResult<'_>,
    prog_phase: &ProgressionPhaseResult,
    offense_players: &[&Player],
    is_home_offense: bool,
    offense_team_id: Uuid,
    sink: &mut impl EventSink,
) -> FinishingPhaseResult {
    let total_advance_in_series =
        state.possession().series_state().advanced_mirins() + prog_phase.mirins_advanced;
    let scoring_opp =
        evaluate_scoring_opportunity(state.drives_in_current_series(), total_advance_in_series);

    if !pass_phase.pass_completed || scoring_opp == ScoringOpportunity::None {
        return FinishingPhaseResult {
            scoring_decision: ScoringDecision::NoOpportunity,
            finish_duel_outcome: None,
        };
    }

    let context = if is_home_offense {
        DuelContext::attacker_home()
    } else {
        DuelContext::defender_home()
    };

    let mut finisher_rng = state.rng_provider().finisher_selection_rng();
    let selected_finisher_id = select_finisher(
        offense_players,
        state.spatial_map(),
        state.pitch(),
        is_home_offense,
        &mut finisher_rng,
    )
    .unwrap_or_else(|| pass_phase.artrine.id());

    let finisher_player = offense_players
        .iter()
        .copied()
        .find(|p| p.id() == selected_finisher_id)
        .unwrap_or(pass_phase.artrine);

    let mut duel_rng = state.rng_provider().duel_resolution_rng();
    let finish_duel_outcome = resolve_duel_for_participants(
        DuelKind::FinishingAttempt,
        &[finisher_player],
        &[pass_phase.goalguard],
        state.attribute_keys(),
        &context,
        &mut duel_rng,
    );

    let finish_duel_event = translate_duel_resolved(
        &finish_duel_outcome,
        vec![finisher_player.id()],
        vec![pass_phase.goalguard.id()],
    );
    let seq = state.next_sequence();
    let clock_inst = state.clock().to_instant();
    sink.record(create_envelope(seq, clock_inst, finish_duel_event));

    let scoring_decision = if finish_duel_outcome.attacker_won() {
        let decision = match scoring_opp {
            ScoringOpportunity::GoalPoint => {
                let pts = goal_point_points();
                state.record_goal_point(offense_team_id);
                ScoringDecision::GoalPoint {
                    team_id: offense_team_id,
                    scorer_id: finisher_player.id(),
                    artrine_id: pass_phase.artrine.id(),
                    drives_completed: state.drives_in_current_series(),
                    points: pts,
                    post: ScoringPost::Goalpost,
                }
            }
            ScoringOpportunity::FieldPoint => {
                let pts = field_point_points();
                state.record_field_point(offense_team_id);
                ScoringDecision::FieldPoint {
                    team_id: offense_team_id,
                    scorer_id: finisher_player.id(),
                    territory_advance_mirim: prog_phase.mirins_advanced,
                    drives_completed: state.drives_in_current_series(),
                    points: pts,
                    post: ScoringPost::Fieldpost,
                }
            }
            ScoringOpportunity::None => ScoringDecision::NoOpportunity,
        };

        if let Some(match_event) = translate_scoring_decision(&decision) {
            let seq = state.next_sequence();
            let clock_inst = state.clock().to_instant();
            sink.record(create_envelope(seq, clock_inst, match_event));
        }

        decision
    } else {
        let attempted_post = match scoring_opp {
            ScoringOpportunity::GoalPoint => ScoringPost::Goalpost,
            _ => ScoringPost::Fieldpost,
        };
        ScoringDecision::Missed {
            team_id: offense_team_id,
            scorer_id: finisher_player.id(),
            attempted_post,
        }
    };

    FinishingPhaseResult {
        scoring_decision,
        finish_duel_outcome: Some(finish_duel_outcome),
    }
}
