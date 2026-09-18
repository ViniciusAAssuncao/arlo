use crate::attributes::PlayerAttributeTable;
use crate::lineup_runtime::Lineup;
use arlo_domain::sport_constants::{HOME_FIELD_ADVANTAGE_LOGIT, MISDIRECTION_MAX_LOGIT_MULTIPLIER};
use arlo_domain::{AttributeKey, Player, Position, PositionLine, SlotRole};
use arlo_tactics::{PlayCall, RouteAssignment};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct FalseArtrineDebuffResult {
    false_artrine_id: Uuid,
    bluff_score: f64,
    defense_reading: f64,
    debuff_amount: f64,
}

impl FalseArtrineDebuffResult {
    pub fn new(
        false_artrine_id: Uuid,
        bluff_score: f64,
        defense_reading: f64,
        debuff_amount: f64,
    ) -> Self {
        Self {
            false_artrine_id,
            bluff_score,
            defense_reading,
            debuff_amount,
        }
    }

    pub fn false_artrine_id(&self) -> Uuid {
        self.false_artrine_id
    }

    pub fn bluff_score(&self) -> f64 {
        self.bluff_score
    }

    pub fn defense_reading(&self) -> f64 {
        self.defense_reading
    }

    pub fn debuff_amount(&self) -> f64 {
        self.debuff_amount
    }

    pub fn is_effective(&self) -> bool {
        self.debuff_amount > 0.0
    }
}

pub fn find_false_artrine<'a>(
    lineup: &'a Lineup,
    role_index: Option<&HashMap<Uuid, SlotRole>>,
) -> Option<&'a Player> {
    if let Some(roles) = role_index {
        for assignment in lineup.assignments() {
            let pid = assignment.player().id();
            if roles.get(&pid) == Some(&SlotRole::FalseArtrine) {
                return Some(assignment.player());
            }
        }
    }
    for assignment in lineup.assignments() {
        if assignment.slot_role() == SlotRole::FalseArtrine {
            return Some(assignment.player());
        }
    }
    None
}

pub fn calculate_defense_tactical_reading(
    defense_lineup: &Lineup,
    defense_pos_index: &HashMap<Uuid, Position>,
    tables: &HashMap<Uuid, PlayerAttributeTable>,
) -> f64 {
    let mut total_reading = 0.0;
    let mut count = 0;

    for assignment in defense_lineup.assignments() {
        let pid = assignment.player().id();
        let pos = defense_pos_index
            .get(&pid)
            .copied()
            .unwrap_or_else(|| assignment.slot().defensive_position());

        if matches!(
            pos,
            Position::Lineback | Position::OutsideZonerback | Position::MiddleZonerback
        ) {
            if let Some(table) = tables.get(&pid) {
                let tk = table.get(AttributeKey::TacticalKnowledge);
                let ant = table.get(AttributeKey::Anticipation);
                let dec = table.get(AttributeKey::Decisions);
                let reading = tk * 0.50 + ant * 0.30 + dec * 0.20;
                total_reading += reading;
                count += 1;
            }
        }
    }

    if count > 0 {
        total_reading / (count as f64)
    } else {
        10.0
    }
}

pub fn calculate_false_artrine_debuff(
    offense_lineup: &Lineup,
    offense_role_index: Option<&HashMap<Uuid, SlotRole>>,
    defense_lineup: &Lineup,
    defense_pos_index: &HashMap<Uuid, Position>,
    tables: &HashMap<Uuid, PlayerAttributeTable>,
) -> Option<FalseArtrineDebuffResult> {
    let false_artrine = find_false_artrine(offense_lineup, offense_role_index)?;
    let fa_table = tables.get(&false_artrine.id())?;

    let bluff_score = fa_table.get(AttributeKey::FalseArtrineBluff);
    let defense_reading =
        calculate_defense_tactical_reading(defense_lineup, defense_pos_index, tables);

    let diff = bluff_score - defense_reading;
    let debuff_amount = if diff > 0.0 {
        (diff * 0.35).clamp(0.0, 5.0)
    } else {
        0.0
    };

    Some(FalseArtrineDebuffResult::new(
        false_artrine.id(),
        bluff_score,
        defense_reading,
        debuff_amount,
    ))
}

pub fn apply_false_artrine_debuff_to_table(
    table: &mut PlayerAttributeTable,
    position: Position,
    debuff: &FalseArtrineDebuffResult,
) {
    if position.line() == PositionLine::DefenseLine && debuff.is_effective() {
        let ant = table.get(AttributeKey::Anticipation);
        let pos_val = table.get(AttributeKey::Positioning);
        table.set(
            AttributeKey::Anticipation,
            (ant - debuff.debuff_amount()).max(1.0),
        );
        table.set(
            AttributeKey::Positioning,
            (pos_val - debuff.debuff_amount()).max(1.0),
        );
    }
}

pub fn apply_false_artrine_debuff_to_defense_tables(
    offense_lineup: &Lineup,
    offense_role_index: Option<&HashMap<Uuid, SlotRole>>,
    defense_lineup: &Lineup,
    defense_pos_index: &HashMap<Uuid, Position>,
    tables: &mut HashMap<Uuid, PlayerAttributeTable>,
) -> Option<FalseArtrineDebuffResult> {
    let result = calculate_false_artrine_debuff(
        offense_lineup,
        offense_role_index,
        defense_lineup,
        defense_pos_index,
        tables,
    )?;

    if result.is_effective() {
        for assignment in defense_lineup.assignments() {
            let pid = assignment.player().id();
            let pos = defense_pos_index
                .get(&pid)
                .copied()
                .unwrap_or_else(|| assignment.slot().defensive_position());

            if pos.line() == PositionLine::DefenseLine {
                if let Some(table) = tables.get_mut(&pid) {
                    let ant = table.get(AttributeKey::Anticipation);
                    let pos_val = table.get(AttributeKey::Positioning);
                    table.set(
                        AttributeKey::Anticipation,
                        (ant - result.debuff_amount()).max(1.0),
                    );
                    table.set(
                        AttributeKey::Positioning,
                        (pos_val - result.debuff_amount()).max(1.0),
                    );
                }
            }
        }
    }

    Some(result)
}

pub fn resolve_misdirection_logit_offset(
    play_call: Option<&PlayCall>,
    route_index: &HashMap<Uuid, RouteAssignment>,
    lineup: &Lineup,
) -> f64 {
    if let Some(pc) = play_call {
        if let Some(misdirection) = pc.misdirection() {
            let decoy_player = lineup.player_at_slot_index(misdirection.decoy_slot_index());
            let true_carrier_player =
                lineup.player_at_slot_index(misdirection.true_carrier_slot_index());

            if let (Some(decoy), Some(true_carrier)) = (decoy_player, true_carrier_player) {
                let decoy_route = route_index.get(&decoy.id());
                let true_route = route_index.get(&true_carrier.id());

                if let (Some(d_route), Some(t_route)) = (decoy_route, true_route) {
                    let similarity = misdirection.geometric_similarity(d_route, t_route);
                    return -MISDIRECTION_MAX_LOGIT_MULTIPLIER
                        * HOME_FIELD_ADVANTAGE_LOGIT
                        * similarity;
                }
            }
        }
    }
    0.0
}