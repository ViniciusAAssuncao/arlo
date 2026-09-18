use crate::artrine::DistributionFlightInfo;
use crate::resolution::AttributedDuelOutcome;
use crate::world_state::match_state::MatchState;
use crate::world_state::step::down_resolution::context::DownResolutionContext;
use crate::world_state::step::down_resolution::phases::{
    resolve_carry_phase, resolve_cross_phase, resolve_finish_phase, resolve_long_launch_phase,
    resolve_short_pass_phase,
};
use arlo_domain::{ArtrineDecisionKind, Player};
use rand::Rng;
use uuid::Uuid;

pub struct ActionContestOutcome<'a> {
    pub primary_duel: Option<AttributedDuelOutcome>,
    pub secondary_duel: Option<AttributedDuelOutcome>,
    pub receiver: Option<&'a Player>,
    pub turnover_team: Option<Uuid>,
    pub recovering_player_id: Option<Uuid>,
    pub attacker_won: bool,
    pub net_advantage: f64,
    pub is_aerial: bool,
    pub distribution_flight: Option<DistributionFlightInfo>,
}

impl<'a> ActionContestOutcome<'a> {
    pub fn all_duels(&self) -> Vec<AttributedDuelOutcome> {
        let mut duels = Vec::with_capacity(2);
        if let Some(d) = &self.primary_duel {
            duels.push(d.clone());
        }
        if let Some(d) = &self.secondary_duel {
            duels.push(d.clone());
        }
        duels
    }
}

pub fn resolve_contest<'a, R: Rng + ?Sized>(
    ctx: &'a DownResolutionContext<'a>,
    decision: ArtrineDecisionKind,
    state: &MatchState,
    rng: &mut R,
) -> ActionContestOutcome<'a> {
    match decision {
        ArtrineDecisionKind::SelfCarry => resolve_carry_phase(ctx, state, rng),
        ArtrineDecisionKind::ShortPass => resolve_short_pass_phase(ctx, state, rng),
        ArtrineDecisionKind::LongLaunch => resolve_long_launch_phase(ctx, state, rng),
        ArtrineDecisionKind::Cross => resolve_cross_phase(ctx, state, rng),
        ArtrineDecisionKind::SelfFinish => resolve_finish_phase(ctx),
    }
}
