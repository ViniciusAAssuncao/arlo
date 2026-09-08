use crate::artrine::reception_phase::rac_block::resolve_rac_block;
use crate::artrine::reception_phase::rac_breakthrough::resolve_rac_breakthrough;
use crate::artrine::reception_phase::rac_context::RacContext;
use crate::artrine::reception_phase::rac_security::resolve_rac_security;
use crate::physical::FatigueState;
use crate::resolution::{AttributedDuelOutcome, DuelContext};
use crate::spatial::DynamicSpatialMap;
use crate::time::{DurationComponentKind, DurationLedger};
use arlo_domain::pitch::Pitch;
use arlo_domain::{AttributeKey, Player, Position as DomainPosition, SlotRole};
use arlo_tactics::PlayerInstructions;
use rand::Rng;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct RunAfterCatchOutcome {
    pub additional_mirins_advanced: f64,
    pub duels: Vec<AttributedDuelOutcome>,
    pub turnover: Option<Uuid>,
    pub recovering_player_id: Option<Uuid>,
    pub duration_ledger: DurationLedger,
}

pub fn resolve_run_after_catch<F, R>(
    receiver: &Player,
    receiver_pos_domain: DomainPosition,
    offense_helpers: &[&Player],
    offense_position_index: &HashMap<Uuid, DomainPosition>,
    offense_role_index: &HashMap<Uuid, SlotRole>,
    defenders: &[&Player],
    defense_position_index: &HashMap<Uuid, DomainPosition>,
    defense_instructions_index: &HashMap<Uuid, PlayerInstructions>,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    pitch: &Pitch,
    spatial_map: &DynamicSpatialMap,
    defense_team_id: Uuid,
    context: &DuelContext,
    fatigue_for: &F,
    defense_pressing_multiplier: f64,
    rng: &mut R,
) -> RunAfterCatchOutcome
where
    F: Fn(&Uuid) -> FatigueState,
    R: Rng + ?Sized,
{
    let ctx = RacContext {
        receiver,
        receiver_pos_domain,
        offense_helpers,
        offense_position_index,
        offense_role_index,
        defenders,
        defense_position_index,
        defense_instructions_index,
        attribute_keys,
        pitch,
        spatial_map,
        defense_team_id,
        duel_context: context,
        fatigue_for,
        defense_pressing_multiplier,
    };

    let block = resolve_rac_block(&ctx, rng);
    let mut ledger = DurationLedger::new();
    ledger.record_live(
        DurationComponentKind::RunAfterCatchEngagement,
        block.duration,
    );

    if !block.won {
        return RunAfterCatchOutcome {
            additional_mirins_advanced: 0.0,
            duels: vec![block.duel],
            turnover: None,
            recovering_player_id: None,
            duration_ledger: ledger,
        };
    }

    let breakthrough = resolve_rac_breakthrough(&ctx, &block, rng);
    ledger.record_live(
        DurationComponentKind::RunAfterCatchEngagement,
        breakthrough.duration,
    );

    if breakthrough.won {
        return RunAfterCatchOutcome {
            additional_mirins_advanced: breakthrough.additional_advance,
            duels: vec![block.duel, breakthrough.duel],
            turnover: None,
            recovering_player_id: None,
            duration_ledger: ledger,
        };
    }

    let security = resolve_rac_security(&ctx, block.receiver_pos_vec, breakthrough.rec_spd, rng);
    ledger.record_live(
        DurationComponentKind::BallSecurityEngagement,
        security.duration,
    );

    RunAfterCatchOutcome {
        additional_mirins_advanced: 0.0,
        duels: vec![block.duel, breakthrough.duel, security.duel],
        turnover: security.turnover,
        recovering_player_id: security.recovering_player_id,
        duration_ledger: ledger,
    }
}
