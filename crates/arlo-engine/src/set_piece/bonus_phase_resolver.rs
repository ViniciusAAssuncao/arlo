use crate::attributes::profiles::get_duel_attribute_profiles as get_duel_profiles;
use crate::attributes::DEFAULT_PLAYER_ATTRIBUTE_TABLE;
use crate::lineup_runtime::find_goalguard;
use crate::match_decision::scoring::{
    field_goal_points, resolve_scoring_attempt, ScoringAttemptRequest, ScoringDecision,
    ScoringOpportunity,
};
use crate::resolution::group_rating::calculate_player_duel_rating_from_table;
use crate::resolution::{AttributedDuelOutcome, DuelContext, DuelKind};
use crate::set_piece::kicker_selection::select_kicker;
use crate::set_piece::post_selection::select_kick_post;
use crate::world_state::match_state::MatchState;
use crate::world_state::play_transition::publisher::EventPublisher;
use crate::world_state::play_transition::scoring_handler::apply_match_score;
use arlo_domain::{Player, Position};
use arlo_events::{EventSink, ScoringPost};
use rand::Rng;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BonusPhaseResolutionOutcome {
    pub kicker_id: Uuid,
    pub goalguard_id: Uuid,
    pub post: ScoringPost,
    pub scored: bool,
    pub points: u32,
    pub scoring_decision: ScoringDecision,
    pub duel: AttributedDuelOutcome,
}

pub fn resolve_bonus_phase_field_goal<R: Rng + ?Sized>(
    state: &MatchState,
    scoring_team_id: Uuid,
    spot_x_mirim: f64,
    rng: &mut R,
) -> Option<BonusPhaseResolutionOutcome> {
    let defending_team_id = if scoring_team_id == state.home_team_id() {
        state.away_team_id()
    } else {
        state.home_team_id()
    };

    let is_home = scoring_team_id == state.home_team_id();
    let offense_lineup = state.lineup_for_team_arc(scoring_team_id);
    let defense_lineup = state.lineup_for_team_arc(defending_team_id);
    let unavailable = state.unavailable_player_ids();

    let offense_players: Vec<&Player> = offense_lineup
        .assignments()
        .iter()
        .map(|a| a.player())
        .filter(|p| !unavailable.contains(&p.id()))
        .collect();

    let defense_players: Vec<&Player> = defense_lineup
        .assignments()
        .iter()
        .map(|a| a.player())
        .filter(|p| !unavailable.contains(&p.id()))
        .collect();

    if offense_players.is_empty() || defense_players.is_empty() {
        return None;
    }

    let tables = state.teams.player_attribute_tables();
    let fatigue_lookup = state.fatigue_lookup();
    let fatigue_for = |id: &Uuid| fatigue_lookup.get(id);

    let kicker = select_kicker(
        &offense_players,
        Some(state.role_index_for_team(scoring_team_id)),
        state.pitch(),
        state.offensive_position_index_for_team(scoring_team_id),
        state.instructions_index_for_team(scoring_team_id),
        tables,
        is_home,
        Some(&fatigue_for),
        rng,
    )
    .or_else(|| offense_players.first().copied())?;

    let goalguard = find_goalguard(&defense_players)
        .or_else(|_| defense_players.first().copied().ok_or(()))
        .ok()?;

    let kicker_table = tables
        .get(&kicker.id())
        .unwrap_or(&DEFAULT_PLAYER_ATTRIBUTE_TABLE);
    let (att_prof, _) = get_duel_profiles(DuelKind::FieldGoalAttempt);
    let finisher_rating = calculate_player_duel_rating_from_table(
        kicker,
        Position::CenterOffense,
        kicker_table,
        &att_prof,
        &fatigue_lookup.get(&kicker.id()),
    );

    let pitch_len = state.pitch().length_mirim();
    let distance_to_goal = if is_home {
        (pitch_len - spot_x_mirim).max(0.0)
    } else {
        spot_x_mirim.max(0.0)
    };
    let post = select_kick_post(finisher_rating, distance_to_goal);
    let opportunity = ScoringOpportunity::FieldGoal(post);

    let duel_context = DuelContext::new(is_home, !is_home);
    let kicker_fatigue = fatigue_lookup.get(&kicker.id());
    let goalguard_fatigue = fatigue_lookup.get(&goalguard.id());
    let goalguard_table = tables.get(&goalguard.id());

    let req = ScoringAttemptRequest::new(
        kicker,
        goalguard,
        state.attribute_keys(),
        scoring_team_id,
        kicker.id(),
        None,
        opportunity,
        0,
        distance_to_goal,
        &duel_context,
    )
    .with_fatigue(kicker_fatigue, goalguard_fatigue)
    .with_tables(Some(kicker_table), goalguard_table);

    let (scoring_decision, duel) = resolve_scoring_attempt(req, rng);
    let scored = scoring_decision.is_scored();
    let points = if scored { field_goal_points(post) } else { 0 };

    Some(BonusPhaseResolutionOutcome {
        kicker_id: kicker.id(),
        goalguard_id: goalguard.id(),
        post,
        scored,
        points,
        scoring_decision,
        duel,
    })
}

pub fn execute_bonus_phase_conversion<R: Rng + ?Sized>(
    publisher: &mut EventPublisher<'_, impl EventSink>,
    scoring_team_id: Uuid,
    spot_x_mirim: f64,
    rng: &mut R,
) -> Option<BonusPhaseResolutionOutcome> {
    let outcome = resolve_bonus_phase_field_goal(publisher.state(), scoring_team_id, spot_x_mirim, rng)?;

    if outcome.scored {
        apply_match_score(publisher.state_mut(), scoring_team_id, &outcome.scoring_decision);
    }
    publisher.emit_scoring_event(&outcome.scoring_decision);

    publisher.emit_duel_events(&[outcome.duel.clone()], outcome.kicker_id);

    let (k_energy, k_w_bal) = publisher
        .state_mut()
        .apply_duel_contest_strain(outcome.kicker_id, 1.2);
    publisher.emit_physical_strain(outcome.kicker_id, k_energy, k_w_bal, 0.0);

    let (g_energy, g_w_bal) = publisher
        .state_mut()
        .apply_duel_contest_strain(outcome.goalguard_id, 1.2);
    publisher.emit_physical_strain(outcome.goalguard_id, g_energy, g_w_bal, 0.0);

    publisher.state_mut().possession_mut().series_state_mut().set_bonus_phase(false);
    publisher.state_mut().reset_drives();

    Some(outcome)
}