use crate::attributes::{PlayerAttributeTable, DEFAULT_PLAYER_ATTRIBUTE_TABLE};
use crate::lineup_runtime::calculate_player_contribution;
use crate::physical::PhysicalState;
use arlo_domain::{Player, Position, RotationPolicy};
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

pub struct SubstitutionValueAssessment<'a> {
    pub replacement: &'a Arc<Player>,
    pub net_gain: f64,
    pub outgoing_contribution: f64,
    pub incoming_contribution: f64,
}

pub fn evaluate_best_substitution_value<'a>(
    outgoing_player: &Player,
    outgoing_pos: Position,
    outgoing_state: &PhysicalState,
    candidates: &'a [Arc<Player>],
    attribute_tables: &HashMap<Uuid, PlayerAttributeTable>,
    rotation_policy: RotationPolicy,
) -> Option<SubstitutionValueAssessment<'a>> {
    if candidates.is_empty() {
        return None;
    }

    let out_table = attribute_tables
        .get(&outgoing_player.id())
        .unwrap_or(&DEFAULT_PLAYER_ATTRIBUTE_TABLE);
    let out_contrib = calculate_player_contribution(outgoing_player, out_table, outgoing_pos, outgoing_state).value();

    let minimum_required_gain = match rotation_policy {
        RotationPolicy::StrictCore => 12.0,
        RotationPolicy::Situational => 3.0,
        RotationPolicy::HighRotation => -5.0,
    };

    let mut best: Option<SubstitutionValueAssessment<'a>> = None;

    for candidate in candidates {
        let cand_table = attribute_tables
            .get(&candidate.id())
            .unwrap_or(&DEFAULT_PLAYER_ATTRIBUTE_TABLE);
        let cand_state = PhysicalState::initial();
        let in_contrib = calculate_player_contribution(candidate.as_ref(), cand_table, outgoing_pos, &cand_state).value();
        let net_gain = in_contrib - out_contrib;

        if net_gain >= minimum_required_gain {
            if let Some(ref current_best) = best {
                if net_gain > current_best.net_gain {
                    best = Some(SubstitutionValueAssessment {
                        replacement: candidate,
                        net_gain,
                        outgoing_contribution: out_contrib,
                        incoming_contribution: in_contrib,
                    });
                }
            } else {
                best = Some(SubstitutionValueAssessment {
                    replacement: candidate,
                    net_gain,
                    outgoing_contribution: out_contrib,
                    incoming_contribution: in_contrib,
                });
            }
        }
    }

    best
}
