use crate::attributes::{
    PlayerAttributeTable, RefereeAttributeTable, DEFAULT_PLAYER_ATTRIBUTE_TABLE,
};
use crate::officiating::foul::{evaluate_and_resolve_foul, FoulEvaluationContext, FoulResolution};
use crate::physical::FatigueState;
use crate::resolution::duel_profiles::get_duel_profiles;
use crate::resolution::group_rating::calculate_player_duel_rating_from_table;
use crate::resolution::resolver::{resolve_duel, DuelResolutionRequest};
use crate::resolution::{AttributedDuelOutcome, DuelContext, DuelKind};
use crate::spatial::live_collisions::{CollisionResolution, LiveCollision};
use crate::world_state::context_analyzer::GameStatePressure;
use arlo_domain::pitch::Pitch;
use arlo_domain::{AttributeKey, FaultCatalog, Player, Position as DomainPosition};
use rand::Rng;
use smallvec::smallvec;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct CarryCollisionResult {
    pub duels: Vec<AttributedDuelOutcome>,
    pub resolution: CollisionResolution,
    pub foul: Option<FoulResolution>,
}

impl CarryCollisionResult {
    pub fn new(
        duels: Vec<AttributedDuelOutcome>,
        resolution: CollisionResolution,
        foul: Option<FoulResolution>,
    ) -> Self {
        Self {
            duels,
            resolution,
            foul,
        }
    }
}

pub fn resolve_carry_collision<F, R>(
    col: &LiveCollision,
    spd: &mut f64,
    current_carrier: &Player,
    carrier_table: &PlayerAttributeTable,
    carrier_pos_domain: DomainPosition,
    defense_players: &[&Player],
    defender_tables: &HashMap<Uuid, PlayerAttributeTable>,
    defense_pos_index: &HashMap<Uuid, DomainPosition>,
    defense_team_id: Uuid,
    fatigue_for: &F,
    duel_ctx: &DuelContext,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    head_referee_table: &RefereeAttributeTable,
    peace_referee_table: &RefereeAttributeTable,
    game_state_pressure: GameStatePressure,
    fault_catalog: &FaultCatalog,
    pitch: &Pitch,
    rng: &mut R,
) -> CarryCollisionResult
where
    F: Fn(&Uuid) -> FatigueState,
    R: Rng + ?Sized,
{
    let def_player = defense_players
        .iter()
        .copied()
        .find(|p| p.id() == col.defender_id)
        .unwrap_or(defense_players[0]);

    let def_table_ref = defender_tables
        .get(&def_player.id())
        .unwrap_or(&DEFAULT_PLAYER_ATTRIBUTE_TABLE);

    let (off_prof, def_prof) = get_duel_profiles(DuelKind::ArtroBreakthrough);
    let carrier_fatigue = fatigue_for(&current_carrier.id());
    let def_fatigue = fatigue_for(&def_player.id());

    let att_rating = calculate_player_duel_rating_from_table(
        current_carrier,
        carrier_pos_domain,
        carrier_table,
        off_prof,
        &carrier_fatigue,
    );
    let def_rating = calculate_player_duel_rating_from_table(
        def_player,
        defense_pos_index
            .get(&def_player.id())
            .copied()
            .unwrap_or(DomainPosition::Centerback),
        def_table_ref,
        def_prof,
        &def_fatigue,
    );

    let c_context = duel_ctx.for_duel_kind(DuelKind::ArtroBreakthrough);

    let req = DuelResolutionRequest::with_states(
        DuelKind::ArtroBreakthrough,
        att_rating,
        def_rating,
        current_carrier,
        def_player,
        carrier_fatigue,
        def_fatigue,
        attribute_keys,
        &c_context,
    )
    .with_tables(Some(carrier_table), Some(def_table_ref));
    let duel_raw = resolve_duel(req, rng);

    let defender_zone = pitch.zone_at_position(col.defender_position);

    let foul_ctx = FoulEvaluationContext::new(
        current_carrier.id(),
        current_carrier.team_id().unwrap_or_default(),
        def_player.id(),
        def_player.team_id().unwrap_or(defense_team_id),
        carrier_table,
        def_table_ref,
        carrier_fatigue,
        def_fatigue,
        head_referee_table,
        peace_referee_table,
        duel_raw,
        c_context,
        col.contact_severity,
        game_state_pressure,
        true,
        defender_zone,
    );
    let foul = evaluate_and_resolve_foul(&foul_ctx, fault_catalog, rng);

    let attributed = AttributedDuelOutcome::new(
        duel_raw,
        smallvec![current_carrier.id()],
        smallvec![def_player.id()],
    );
    let mut duels = vec![attributed];

    if duel_raw.attacker_won() {
        let mit = duel_raw.velocity_mitigation_factor();
        *spd *= mit;
        CarryCollisionResult::new(
            duels,
            CollisionResolution::Continue {
                velocity_mitigation: mit,
            },
            foul,
        )
    } else {
        let (sec_off, sec_def) = get_duel_profiles(DuelKind::BallSecurityCarry);
        let sec_att = calculate_player_duel_rating_from_table(
            current_carrier,
            carrier_pos_domain,
            carrier_table,
            sec_off,
            &carrier_fatigue,
        );
        let sec_df = calculate_player_duel_rating_from_table(
            def_player,
            defense_pos_index
                .get(&def_player.id())
                .copied()
                .unwrap_or(DomainPosition::Centerback),
            def_table_ref,
            sec_def,
            &def_fatigue,
        );

        let sec_context = duel_ctx.for_duel_kind(DuelKind::BallSecurityCarry);
        let sec_req = DuelResolutionRequest::with_states(
            DuelKind::BallSecurityCarry,
            sec_att,
            sec_df,
            current_carrier,
            def_player,
            carrier_fatigue,
            def_fatigue,
            attribute_keys,
            &sec_context,
        )
        .with_tables(Some(carrier_table), Some(def_table_ref));
        let sec_raw = resolve_duel(sec_req, rng);

        let sec_attr = AttributedDuelOutcome::new(
            sec_raw,
            smallvec![current_carrier.id()],
            smallvec![def_player.id()],
        );
        duels.push(sec_attr);

        let (turnover_team, recovering_player) = if !sec_raw.attacker_won() {
            (Some(defense_team_id), Some(def_player.id()))
        } else {
            (None, None)
        };

        CarryCollisionResult::new(
            duels,
            CollisionResolution::Halt {
                turnover_team,
                recovering_player,
            },
            foul,
        )
    }
}