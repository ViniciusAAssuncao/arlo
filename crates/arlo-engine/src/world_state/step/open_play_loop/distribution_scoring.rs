use crate::lineup_runtime::find_goalguard;
use crate::match_decision::scoring::{
    duel_kind_for_opportunity, evaluate_scoring_opportunity, resolve_scoring_attempt,
    ScoringAttemptRequest, ScoringOpportunity,
};
use crate::possession::TouchActionType;
use crate::world_state::cta_pass::PassPhaseResult;
use crate::world_state::match_state::MatchState;
use crate::world_state::step::open_play_loop::action_context::OpenPlayIterationContext;
use crate::world_state::step::open_play_loop::loop_state::OpenPlayLoopState;
use crate::world_state::step::setup::CallToActionContext;
use arlo_domain::Player;
use arlo_math::units::MIRIM_TO_METERS;
use rand::Rng;

pub const OPEN_PLAY_MAX_FINISH_DISTANCE_MIRIM: f64 = 50.0;

pub fn check_distribution_scoring_opportunity<R: Rng + ?Sized>(
    state: &mut MatchState,
    context: &CallToActionContext,
    iter_ctx: &OpenPlayIterationContext<'_>,
    pass_phase: &PassPhaseResult<'_>,
    loop_state: &mut OpenPlayLoopState,
    receiver_player: &Player,
    current_carrier: &Player,
    defense_players: &[&Player],
    rec_att_rating: f64,
    rng: &mut R,
) {
    let pitch = *state.pitch();
    let rx_mirim = loop_state.current_carrier_pos.raw().0 / MIRIM_TO_METERS;
    let dist_to_goal_mirim = if context.is_home_offense {
        (pitch.length_mirim() - rx_mirim).max(0.0)
    } else {
        rx_mirim.max(0.0)
    };

    let total_drives = state.drives_in_current_series() + loop_state.accumulated_drives_recorded;
    let total_adv = state.possession().series_state().advanced_mirins()
        + loop_state.accumulated_mirins_advanced;

    let opportunity = evaluate_scoring_opportunity(
        state.possession().is_bonus_phase(),
        total_drives,
        total_adv,
        rec_att_rating,
    );

    if opportunity != ScoringOpportunity::None
        && dist_to_goal_mirim <= OPEN_PLAY_MAX_FINISH_DISTANCE_MIRIM
    {
        let goalguard = match find_goalguard(defense_players) {
            Ok(g) => g,
            Err(_) => return,
        };

        let shot_zone = pitch.zone_at_position(loop_state.current_carrier_pos);
        let current_time = state.clock().seconds_in_period();
        state.possession_mut().live_sequence_mut().record_touch(
            receiver_player.id(),
            TouchActionType::FinishingAttempt,
            shot_zone,
            current_time,
        );

        let assister_id = state
            .possession()
            .live_sequence()
            .primary_assister(receiver_player.id())
            .or(Some(current_carrier.id()));

        let finish_ctx = iter_ctx
            .duel_context
            .for_duel_kind(duel_kind_for_opportunity(opportunity));
        let rec_fatigue = state.fatigue_lookup().get(&receiver_player.id());
        let gg_fatigue = state.fatigue_lookup().get(&goalguard.id());
        let rec_table = state
            .teams
            .player_attribute_tables()
            .get(&receiver_player.id());
        let gg_table = state.teams.player_attribute_tables().get(&goalguard.id());
        let req = ScoringAttemptRequest::new(
            receiver_player,
            goalguard,
            state.attribute_keys(),
            context.offense_team_id,
            pass_phase.artrine.id(),
            assister_id,
            opportunity,
            total_drives,
            total_adv,
            &finish_ctx,
        )
        .with_fatigue(rec_fatigue, gg_fatigue)
        .with_tables(rec_table, gg_table);

        let (score_dec, fin_duel) = resolve_scoring_attempt(req, rng);

        loop_state.accumulated_duels.push(fin_duel);
        loop_state.scoring_decision = score_dec;
        loop_state.ball_in_play = false;
    }
}
