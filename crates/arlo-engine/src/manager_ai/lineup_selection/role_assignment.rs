use crate::ai::cognitive::decision_threshold::action_probability;
use crate::attributes::PlayerAttributeTable;
use crate::manager_ai::context::ManagerSnapshot;
use crate::spatial::decision_vector::extract_attribute_value;
use arlo_domain::sport_constants::manager_cognition::{
    DECISION_THRESHOLD_LOGIT_STEEPNESS, SIGNAL_DETECTION_BASE_SENSITIVITY,
    SIGNAL_DETECTION_JUDGMENT_ATTRIBUTE_SCALE,
};
use arlo_domain::sport_constants::managerial::{
    BLOCKER_ROLE_BASE_THRESHOLD, BLOCKER_ROLE_PHYSICALITY_ADJUSTMENT,
    LAUNCHER_PASSING_RANGE_PREFERENCE_WEIGHT,
};
use arlo_domain::{
    ArtrineDependency, AttributeKey, Formation, FormationSlot, Player, Position, SlotRole,
};
use arlo_tactics::{is_role_eligible_for_position, max_concurrent_count};
use std::collections::HashMap;
use uuid::Uuid;

fn evaluate_candidate_suitability(
    player: &Player,
    role: SlotRole,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
) -> f64 {
    let table = PlayerAttributeTable::from_player(player, attribute_keys);
    match role {
        SlotRole::FalseArtrine => {
            let bluff = extract_attribute_value(&table, AttributeKey::FalseArtrineBluff);
            let tech = extract_attribute_value(&table, AttributeKey::Technique);
            let flair = extract_attribute_value(&table, AttributeKey::Flair);
            bluff * 0.5 + tech * 0.3 + flair * 0.2
        }
        SlotRole::Launcher => {
            let pass = extract_attribute_value(&table, AttributeKey::Passing);
            let vision = extract_attribute_value(&table, AttributeKey::Vision);
            let tech = extract_attribute_value(&table, AttributeKey::Technique);
            pass * 0.5 + vision * 0.3 + tech * 0.2
        }
        SlotRole::Safeguard => {
            let block = extract_attribute_value(&table, AttributeKey::OffensiveBlocking);
            let strength = extract_attribute_value(&table, AttributeKey::Strength);
            let pos = extract_attribute_value(&table, AttributeKey::Positioning);
            block * 0.5 + strength * 0.3 + pos * 0.2
        }
        SlotRole::Kicker => {
            let kick = extract_attribute_value(&table, AttributeKey::GoalKicking);
            let finish = extract_attribute_value(&table, AttributeKey::Finishing);
            kick * 0.6 + finish * 0.4
        }
        SlotRole::Blocker => {
            let block = extract_attribute_value(&table, AttributeKey::OffensiveBlocking);
            let strength = extract_attribute_value(&table, AttributeKey::Strength);
            block * 0.6 + strength * 0.4
        }
        SlotRole::Standard => 0.0,
    }
}

fn assign_best_candidate_for_role(
    role: SlotRole,
    assignments: &[(usize, Player)],
    slots: &[FormationSlot],
    roles: &mut HashMap<Uuid, SlotRole>,
    role_counts: &mut HashMap<SlotRole, u32>,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
) {
    if let Some(max) = max_concurrent_count(role) {
        let current_count = role_counts.get(&role).copied().unwrap_or(0);
        if current_count < max {
            let candidate = assignments
                .iter()
                .filter(|(idx, p)| {
                    let pos = slots
                        .get(*idx)
                        .map(|s| s.position())
                        .unwrap_or(Position::Midcenter);
                    roles.get(&p.id()) == Some(&SlotRole::Standard)
                        && is_role_eligible_for_position(role, pos)
                })
                .max_by(|(_, a), (_, b)| {
                    let score_a = evaluate_candidate_suitability(a, role, attribute_keys);
                    let score_b = evaluate_candidate_suitability(b, role, attribute_keys);
                    score_a
                        .partial_cmp(&score_b)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });

            if let Some((_, player)) = candidate {
                roles.insert(player.id(), role);
                *role_counts.entry(role).or_insert(0) += 1;
            }
        }
    }
}

pub fn assign_roles(
    assignments: &[(usize, Player)],
    formation: &Formation,
    manager_snapshot: &ManagerSnapshot,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
) -> HashMap<Uuid, SlotRole> {
    let slots = formation.slots();
    let mut roles: HashMap<Uuid, SlotRole> = assignments
        .iter()
        .map(|(_, p)| (p.id(), SlotRole::Standard))
        .collect();

    let mut role_counts: HashMap<SlotRole, u32> = HashMap::new();

    let dependency = manager_snapshot
        .tactical_profile
        .as_ref()
        .map(|p| p.artrine_dependency())
        .unwrap_or(ArtrineDependency::SystemDriven);

    let dep_modifier_false_artrine = match dependency {
        ArtrineDependency::ArtrineCentric => 0.65,
        ArtrineDependency::SystemDriven => 1.35,
    };

    let dep_modifier_launcher = match dependency {
        ArtrineDependency::ArtrineCentric => 0.70,
        ArtrineDependency::SystemDriven => 1.30,
    };

    let planning_norm = (manager_snapshot.offense_planning.clamp(0.0, 20.0)) / 20.0;
    let strategy_norm = (manager_snapshot.artro_strategy.clamp(0.0, 20.0)) / 20.0;
    let def_org_norm = (manager_snapshot.defense_organization.clamp(0.0, 20.0)) / 20.0;

    let passing_range_pref = manager_snapshot
        .tactical_profile
        .as_ref()
        .map(|p| p.passing_range_preference())
        .unwrap_or(0.0);
    let pass_pref_norm = (passing_range_pref.clamp(-1.0, 1.0) + 1.0) / 2.0;

    let physicality_pref = manager_snapshot
        .tactical_profile
        .as_ref()
        .map(|p| p.physicality_preference())
        .unwrap_or(0.5);

    let false_artrine_stimulus =
        ((planning_norm * 0.5 + strategy_norm * 0.5) * dep_modifier_false_artrine).clamp(0.0, 1.0);
    let false_artrine_prob = action_probability(
        false_artrine_stimulus,
        manager_snapshot.artro_strategy,
        SIGNAL_DETECTION_BASE_SENSITIVITY,
        SIGNAL_DETECTION_JUDGMENT_ATTRIBUTE_SCALE,
        DECISION_THRESHOLD_LOGIT_STEEPNESS,
    );

    if false_artrine_prob.value() >= 0.5 {
        assign_best_candidate_for_role(
            SlotRole::FalseArtrine,
            assignments,
            slots,
            &mut roles,
            &mut role_counts,
            attribute_keys,
        );
    }

    let base_launcher_tactical = planning_norm * 0.7 + strategy_norm * 0.3;
    let launcher_stimulus = (((base_launcher_tactical
        * (1.0 - LAUNCHER_PASSING_RANGE_PREFERENCE_WEIGHT))
        + (pass_pref_norm * LAUNCHER_PASSING_RANGE_PREFERENCE_WEIGHT))
        * dep_modifier_launcher)
        .clamp(0.0, 1.0);
    let launcher_prob = action_probability(
        launcher_stimulus,
        manager_snapshot.offense_planning,
        SIGNAL_DETECTION_BASE_SENSITIVITY,
        SIGNAL_DETECTION_JUDGMENT_ATTRIBUTE_SCALE,
        DECISION_THRESHOLD_LOGIT_STEEPNESS,
    );

    if launcher_prob.value() >= 0.5 {
        assign_best_candidate_for_role(
            SlotRole::Launcher,
            assignments,
            slots,
            &mut roles,
            &mut role_counts,
            attribute_keys,
        );
    }

    let safeguard_prob = action_probability(
        def_org_norm,
        manager_snapshot.defense_organization,
        SIGNAL_DETECTION_BASE_SENSITIVITY,
        SIGNAL_DETECTION_JUDGMENT_ATTRIBUTE_SCALE,
        DECISION_THRESHOLD_LOGIT_STEEPNESS,
    );

    if safeguard_prob.value() >= 0.5 {
        assign_best_candidate_for_role(
            SlotRole::Safeguard,
            assignments,
            slots,
            &mut roles,
            &mut role_counts,
            attribute_keys,
        );
    }

    assign_best_candidate_for_role(
        SlotRole::Kicker,
        assignments,
        slots,
        &mut roles,
        &mut role_counts,
        attribute_keys,
    );

    let blocker_threshold = BLOCKER_ROLE_BASE_THRESHOLD
        - (physicality_pref.clamp(0.0, 1.0) * BLOCKER_ROLE_PHYSICALITY_ADJUSTMENT);

    for (idx, player) in assignments {
        if roles.get(&player.id()) == Some(&SlotRole::Standard) {
            let pos = slots
                .get(*idx)
                .map(|s| s.position())
                .unwrap_or(Position::Midcenter);
            if is_role_eligible_for_position(SlotRole::Blocker, pos) {
                let blocking_score =
                    evaluate_candidate_suitability(player, SlotRole::Blocker, attribute_keys);
                if blocking_score >= blocker_threshold {
                    roles.insert(player.id(), SlotRole::Blocker);
                    *role_counts.entry(SlotRole::Blocker).or_insert(0) += 1;
                }
            }
        }
    }

    roles
}