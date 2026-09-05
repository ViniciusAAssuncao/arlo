use crate::error::EngineResult;
use crate::match_decision::play_outcome::DetailedPlayOutcome;
use crate::match_decision::scoring::ScoringDecision;
use crate::world_state::cta_finishing::resolve_finishing_phase;
use crate::world_state::cta_pass::resolve_pass_phase;
use crate::world_state::cta_progression::resolve_progression_phase;
use crate::world_state::cta_transition::apply_play_transition;
use crate::world_state::match_state::MatchState;
use arlo_domain::Player;
use arlo_events::EventSink;
use arlo_math::units::MIRIM_TO_METERS;
use uuid::Uuid;

fn build_finished_match_outcome(state: &MatchState) -> DetailedPlayOutcome {
    let scrimmage = state.possession().scrimmage_point();
    DetailedPlayOutcome {
        offense_team_id: state.possession().offense(),
        defense_team_id: state.possession().defense(),
        passer_id: Uuid::nil(),
        artrine_id: Uuid::nil(),
        down_number: state.possession().down() as u32,
        scrimmage_x_mirim: scrimmage.raw().0 / MIRIM_TO_METERS,
        pass_completed: false,
        pass_is_aerial: false,
        reception_point: scrimmage,
        drives_recorded: 0,
        mirins_advanced: 0.0,
        duels: Vec::new(),
        turnover: None,
        recovering_player_id: None,
        out_of_bounds: false,
        arbitral_stoppage: true,
        last_valid_possession_point: scrimmage,
        possession_control_seconds: None,
        scoring_decision: ScoringDecision::NoOpportunity,
    }
}

pub fn step_call_to_action(
    state: &mut MatchState,
    sink: &mut impl EventSink,
) -> EngineResult<DetailedPlayOutcome> {
    if state.is_match_finished() {
        return Ok(build_finished_match_outcome(state));
    }

    let is_home_offense = state.possession().role().is_offense(state.home_team_id());
    let (offense_team_id, defense_team_id) = if is_home_offense {
        (state.home_team_id(), state.away_team_id())
    } else {
        (state.away_team_id(), state.home_team_id())
    };

    let (offense_lineup, defense_lineup) = if is_home_offense {
        (state.home_lineup().clone(), state.away_lineup().clone())
    } else {
        (state.away_lineup().clone(), state.home_lineup().clone())
    };

    let offense_players: Vec<&Player> = offense_lineup.players();
    let defense_players: Vec<&Player> = defense_lineup.players();

    let pass_phase = resolve_pass_phase(
        state,
        &offense_players,
        &defense_players,
        is_home_offense,
        offense_team_id,
        defense_team_id,
        sink,
    );

    let prog_phase = resolve_progression_phase(
        state,
        &pass_phase,
        &offense_players,
        &defense_players,
        is_home_offense,
        sink,
    );

    let finishing_phase = resolve_finishing_phase(
        state,
        &pass_phase,
        &prog_phase,
        &offense_players,
        is_home_offense,
        offense_team_id,
        sink,
    );

    let detailed_outcome = apply_play_transition(
        state,
        pass_phase,
        prog_phase,
        finishing_phase,
        offense_team_id,
        defense_team_id,
        sink,
    );

    Ok(detailed_outcome)
}