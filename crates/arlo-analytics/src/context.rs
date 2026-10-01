use crate::error::{AnalyticsError, AnalyticsResult};
use arlo_domain::tactics::{Formation, FormationSlot};
use arlo_domain::{Position, SlotRole};
use arlo_events::SubstitutionMade;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayerAssignment {
    player_id: Uuid,
    team_id: Uuid,
    slot_index: usize,
    offensive_position: Position,
    defensive_position: Position,
    slot_role: SlotRole,
    is_starter: bool,
    substituted_player_id: Option<Uuid>,
    entry_time_seconds: f64,
}

impl PlayerAssignment {
    pub fn new(
        player_id: Uuid,
        team_id: Uuid,
        slot_index: usize,
        offensive_position: Position,
        defensive_position: Position,
        slot_role: SlotRole,
        is_starter: bool,
        substituted_player_id: Option<Uuid>,
        entry_time_seconds: f64,
    ) -> AnalyticsResult<Self> {
        if !entry_time_seconds.is_finite() || entry_time_seconds < 0.0 {
            return Err(AnalyticsError::NonFinite {
                field: "entry_time_seconds".into(),
            });
        }

        Ok(Self {
            player_id,
            team_id,
            slot_index,
            offensive_position,
            defensive_position,
            slot_role,
            is_starter,
            substituted_player_id,
            entry_time_seconds,
        })
    }

    pub fn player_id(&self) -> Uuid {
        self.player_id
    }

    pub fn team_id(&self) -> Uuid {
        self.team_id
    }

    pub fn slot_index(&self) -> usize {
        self.slot_index
    }

    pub fn offensive_position(&self) -> Position {
        self.offensive_position
    }

    pub fn defensive_position(&self) -> Position {
        self.defensive_position
    }

    pub fn slot_role(&self) -> SlotRole {
        self.slot_role
    }

    pub fn is_starter(&self) -> bool {
        self.is_starter
    }

    pub fn substituted_player_id(&self) -> Option<Uuid> {
        self.substituted_player_id
    }

    pub fn entry_time_seconds(&self) -> f64 {
        self.entry_time_seconds
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubstitutionRecord {
    team_id: Uuid,
    player_out_id: Uuid,
    player_in_id: Uuid,
    slot_index: usize,
    clock_seconds: f64,
}

impl SubstitutionRecord {
    pub fn new(
        team_id: Uuid,
        player_out_id: Uuid,
        player_in_id: Uuid,
        slot_index: usize,
        clock_seconds: f64,
    ) -> Self {
        Self {
            team_id,
            player_out_id,
            player_in_id,
            slot_index,
            clock_seconds,
        }
    }

    pub fn team_id(&self) -> Uuid {
        self.team_id
    }

    pub fn player_out_id(&self) -> Uuid {
        self.player_out_id
    }

    pub fn player_in_id(&self) -> Uuid {
        self.player_in_id
    }

    pub fn slot_index(&self) -> usize {
        self.slot_index
    }

    pub fn clock_seconds(&self) -> f64 {
        self.clock_seconds
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct MatchAnalysisContext {
    assignments: HashMap<Uuid, PlayerAssignment>,
    active_slot_players: HashMap<(Uuid, usize), Uuid>,
    substitutions_by_player_out: HashMap<Uuid, Uuid>,
    substitutions_by_player_in: HashMap<Uuid, Uuid>,
    substitution_history: Vec<SubstitutionRecord>,
}

impl MatchAnalysisContext {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_starter(
        &mut self,
        player_id: Uuid,
        team_id: Uuid,
        slot_index: usize,
        slot: &FormationSlot,
    ) -> AnalyticsResult<()> {
        if self.assignments.contains_key(&player_id) {
            return Err(AnalyticsError::DuplicatePlayer(player_id));
        }

        let assignment = PlayerAssignment::new(
            player_id,
            team_id,
            slot_index,
            slot.offensive_position(),
            slot.defensive_position(),
            slot.role(),
            true,
            None,
            0.0,
        )?;

        self.assignments.insert(player_id, assignment);
        self.active_slot_players
            .insert((team_id, slot_index), player_id);
        Ok(())
    }

    pub fn register_formation_starters(
        &mut self,
        team_id: Uuid,
        formation: &Formation,
        starter_player_ids: &[Uuid],
    ) -> AnalyticsResult<()> {
        let slots = formation.slots();
        if starter_player_ids.len() != slots.len() {
            return Err(AnalyticsError::InvalidData(format!(
                "Formation expects {} players, but {} were provided",
                slots.len(),
                starter_player_ids.len()
            )));
        }

        for (slot_index, (&player_id, slot)) in starter_player_ids.iter().zip(slots.iter()).enumerate() {
            self.register_starter(player_id, team_id, slot_index, slot)?;
        }

        Ok(())
    }

    pub fn register_substitution(
        &mut self,
        team_id: Uuid,
        player_out_id: Uuid,
        player_in_id: Uuid,
        clock_seconds: f64,
    ) -> AnalyticsResult<()> {
        if self.assignments.contains_key(&player_in_id) {
            return Err(AnalyticsError::DuplicatePlayer(player_in_id));
        }

        let out_assignment = self
            .assignments
            .get(&player_out_id)
            .ok_or(AnalyticsError::PlayerNotFound(player_out_id))?
            .clone();

        if out_assignment.team_id() != team_id {
            return Err(AnalyticsError::InvalidData(format!(
                "Player {} belongs to team {}, not team {}",
                player_out_id,
                out_assignment.team_id(),
                team_id
            )));
        }

        let in_assignment = PlayerAssignment::new(
            player_in_id,
            team_id,
            out_assignment.slot_index(),
            out_assignment.offensive_position(),
            out_assignment.defensive_position(),
            out_assignment.slot_role(),
            false,
            Some(player_out_id),
            clock_seconds,
        )?;

        self.assignments.insert(player_in_id, in_assignment);
        self.active_slot_players
            .insert((team_id, out_assignment.slot_index()), player_in_id);
        self.substitutions_by_player_out
            .insert(player_out_id, player_in_id);
        self.substitutions_by_player_in
            .insert(player_in_id, player_out_id);
        self.substitution_history.push(SubstitutionRecord::new(
            team_id,
            player_out_id,
            player_in_id,
            out_assignment.slot_index(),
            clock_seconds,
        ));

        Ok(())
    }

    pub fn handle_substitution_event(&mut self, event: &SubstitutionMade) -> AnalyticsResult<()> {
        self.register_substitution(
            event.team_id(),
            event.player_out(),
            event.player_in(),
            event.match_clock().total_elapsed_seconds(),
        )
    }

    pub fn assignment(&self, player_id: &Uuid) -> Option<&PlayerAssignment> {
        self.assignments.get(player_id)
    }

    pub fn active_player_for_slot(&self, team_id: Uuid, slot_index: usize) -> Option<Uuid> {
        self.active_slot_players.get(&(team_id, slot_index)).copied()
    }

    pub fn team_id(&self, player_id: &Uuid) -> Option<Uuid> {
        self.assignments.get(player_id).map(|a| a.team_id())
    }

    pub fn offensive_position(&self, player_id: &Uuid) -> Option<Position> {
        self.assignments.get(player_id).map(|a| a.offensive_position())
    }

    pub fn defensive_position(&self, player_id: &Uuid) -> Option<Position> {
        self.assignments.get(player_id).map(|a| a.defensive_position())
    }

    pub fn slot_role(&self, player_id: &Uuid) -> Option<SlotRole> {
        self.assignments.get(player_id).map(|a| a.slot_role())
    }

    pub fn is_starter(&self, player_id: &Uuid) -> bool {
        self.assignments
            .get(player_id)
            .map(|a| a.is_starter())
            .unwrap_or(false)
    }

    pub fn substituted_player(&self, player_in_id: &Uuid) -> Option<Uuid> {
        self.substitutions_by_player_in.get(player_in_id).copied()
    }

    pub fn substitute_for(&self, player_out_id: &Uuid) -> Option<Uuid> {
        self.substitutions_by_player_out.get(player_out_id).copied()
    }

    pub fn all_assignments(&self) -> &HashMap<Uuid, PlayerAssignment> {
        &self.assignments
    }

    pub fn substitution_history(&self) -> &[SubstitutionRecord] {
        &self.substitution_history
    }

    pub fn player_ids_for_team(&self, team_id: &Uuid) -> Vec<Uuid> {
        self.assignments
            .values()
            .filter(|a| a.team_id() == *team_id)
            .map(|a| a.player_id())
            .collect()
    }
}