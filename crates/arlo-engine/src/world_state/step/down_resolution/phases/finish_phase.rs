use crate::world_state::step::down_resolution::contest_stage::ActionContestOutcome;
use crate::world_state::step::down_resolution::context::DownResolutionContext;

pub fn resolve_finish_phase<'a>(
    ctx: &'a DownResolutionContext<'a>,
) -> ActionContestOutcome<'a> {
    ActionContestOutcome {
        primary_duel: None,
        secondary_duel: None,
        receiver: Some(ctx.carrier),
        turnover_team: None,
        recovering_player_id: None,
        attacker_won: true,
        net_advantage: 0.0,
        is_aerial: false,
        distribution_flight: None,
    }
}
