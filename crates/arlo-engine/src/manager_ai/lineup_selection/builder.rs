use crate::error::{EngineError, EngineResult};
use crate::manager_ai::context::ManagerSnapshot;
use crate::manager_ai::lineup_selection::formation_preference::select_best_formation;
use crate::manager_ai::lineup_selection::player_selection::assign_players;
use crate::manager_ai::lineup_selection::role_assignment::assign_roles;
use arlo_domain::sport_constants::TOTAL_PLAYERS_PER_TEAM;
use arlo_domain::{AttributeKey, Formation, Manager, Player, SlotRole};
use arlo_tactics::TacticalLineup;
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

pub struct LineupSelectionEngine;

impl LineupSelectionEngine {
    pub fn select(
        roster: &[Player],
        formations: &[Formation],
        manager: &Manager,
        attribute_keys: &HashMap<Uuid, AttributeKey>,
    ) -> EngineResult<(TacticalLineup, Vec<Player>)> {
        let expected_size = TOTAL_PLAYERS_PER_TEAM as usize;
        if roster.len() < expected_size {
            return Err(EngineError::InvalidLineupSize {
                expected: expected_size,
                actual: roster.len(),
            });
        }

        if formations.is_empty() {
            return Err(EngineError::MissingRequiredPosition(
                "no formations available".to_string(),
            ));
        }

        let preferred_ids = manager
            .tactical_profile()
            .map(|p| p.preferred_formation_ids())
            .unwrap_or(&[]);

        let best_formation =
            select_best_formation(formations, roster, preferred_ids).ok_or_else(|| {
                EngineError::MissingRequiredPosition("no formations available".to_string())
            })?;

        let snapshot = ManagerSnapshot::from_manager(manager, attribute_keys);

        let assignments = assign_players(best_formation, roster, attribute_keys);

        if assignments.len() < expected_size {
            return Err(EngineError::InvalidLineupSize {
                expected: expected_size,
                actual: assignments.len(),
            });
        }

        let roles = assign_roles(&assignments, best_formation, &snapshot, attribute_keys);

        let team_id = manager
            .team_id()
            .or_else(|| roster.first().and_then(|p| p.team_id()))
            .unwrap_or_else(Uuid::new_v4);

        let lineup_id = Uuid::new_v4();
        let lineup_name = format!("{} Lineup", manager.person().name());

        let mut builder =
            TacticalLineup::builder(lineup_id, team_id, lineup_name).with_formation(best_formation);

        for &(slot_idx, ref player) in &assignments {
            let pid = player.id();
            let role = roles.get(&pid).copied().unwrap_or(SlotRole::Standard);
            builder = builder.assign(slot_idx, pid).designate_role(pid, role);
        }

        let tactical_lineup = builder.build(roster)?;

        let starter_ids: HashSet<Uuid> = assignments.iter().map(|(_, p)| p.id()).collect();
        let bench: Vec<Player> = roster
            .iter()
            .filter(|p| !starter_ids.contains(&p.id()))
            .cloned()
            .collect();

        Ok((tactical_lineup, bench))
    }
}
