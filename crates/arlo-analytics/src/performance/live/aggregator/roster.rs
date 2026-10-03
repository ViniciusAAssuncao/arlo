use super::PlayerPerformanceAggregator;
use crate::context::{MatchAnalysisContext, PlayerAssignment};
use crate::error::{AnalyticsError, AnalyticsResult};
use crate::performance::live::player_state::LivePlayerState;
use crate::performance::live::seed::InitialParticipantSeed;
use arlo_domain::tactics::Formation;
use arlo_domain::{Position, SlotRole};
use arlo_events::{AvailabilityStatus, MatchEvent};
use uuid::Uuid;

impl PlayerPerformanceAggregator {
    pub fn seed_from_context(&mut self, context: &MatchAnalysisContext) {
        let mut assignments: Vec<&PlayerAssignment> = context.all_assignments().values().collect();
        assignments.sort_by_key(|assignment| (assignment.slot_index(), assignment.player_id()));
        for assignment in assignments {
            self.seed_participant(
                assignment.player_id(),
                assignment.team_id(),
                assignment.offensive_position(),
                assignment.defensive_position(),
                assignment.slot_role(),
                assignment.is_starter(),
            );
        }
    }

    pub fn seed_participant(
        &mut self,
        player_id: Uuid,
        team_id: Uuid,
        offensive_position: Position,
        defensive_position: Position,
        slot_role: SlotRole,
        is_active: bool,
    ) {
        let seed = InitialParticipantSeed::new(
            player_id,
            team_id,
            offensive_position,
            defensive_position,
            slot_role,
            is_active,
        );

        if let Some(position) = self
            .initial_seeds
            .iter()
            .position(|existing| existing.player_id() == player_id)
        {
            self.initial_seeds[position] = seed;
        } else {
            self.initial_seeds.push(seed);
        }

        self.apply_seed(&seed);
    }

    pub(super) fn apply_seed(&mut self, seed: &InitialParticipantSeed) {
        let mut state = LivePlayerState::new(
            seed.player_id(),
            seed.team_id(),
            seed.offensive_position(),
            seed.defensive_position(),
            seed.slot_role(),
            seed.is_active(),
            &self.config,
        );
        state.set_starter(seed.is_active());
        if seed.is_active() {
            state.set_entry_time_seconds(0.0);
        }
        self.players.insert(seed.player_id(), state);
    }

    pub fn seed_participants<I>(&mut self, participants: I)
    where
        I: IntoIterator<Item = (Uuid, Uuid, Position, Position, SlotRole, bool)>,
    {
        for (player_id, team_id, offensive, defensive, role, active) in participants {
            self.seed_participant(player_id, team_id, offensive, defensive, role, active);
        }
    }

    pub fn seed_team_starters(
        &mut self,
        team_id: Uuid,
        formation: &Formation,
        player_ids: &[Uuid],
    ) -> AnalyticsResult<()> {
        let slots = formation.slots();
        if player_ids.len() != slots.len() {
            return Err(AnalyticsError::InvalidData(format!(
                "Formation expects {} players, but {} were provided",
                slots.len(),
                player_ids.len()
            )));
        }

        for (&player_id, slot) in player_ids.iter().zip(slots.iter()) {
            self.seed_participant(
                player_id,
                team_id,
                slot.offensive_position(),
                slot.defensive_position(),
                slot.role(),
                true,
            );
        }

        Ok(())
    }

    pub fn handle_substitution(&mut self, player_out: Uuid, player_in: Uuid, team_id: Uuid) {
        if let Some(out_state) = self.players.get_mut(&player_out) {
            out_state.set_active(false);
        }

        let clock_seconds = self.last_clock_seconds;
        let (offensive, defensive, role) = if let Some(context) = self.translator.context() {
            let offensive = context
                .offensive_position(&player_in)
                .or_else(|| context.offensive_position(&player_out))
                .unwrap_or(Position::CenterOffense);
            let defensive = context
                .defensive_position(&player_in)
                .or_else(|| context.defensive_position(&player_out))
                .unwrap_or(Position::Centerback);
            let role = context
                .slot_role(&player_in)
                .or_else(|| context.slot_role(&player_out))
                .unwrap_or(SlotRole::Standard);
            (offensive, defensive, role)
        } else if let Some(out_state) = self.players.get(&player_out) {
            (
                out_state.offensive_position(),
                out_state.defensive_position(),
                out_state.slot_role(),
            )
        } else {
            (
                Position::CenterOffense,
                Position::Centerback,
                SlotRole::Standard,
            )
        };

        if let Some(in_state) = self.players.get_mut(&player_in) {
            in_state.set_active(true);
            in_state.set_entry_time_seconds(clock_seconds);
            in_state.update_assignment(offensive, defensive, role, &self.config);
        } else {
            let mut state = LivePlayerState::new(
                player_in,
                team_id,
                offensive,
                defensive,
                role,
                true,
                &self.config,
            );
            state.set_starter(false);
            state.set_entry_time_seconds(clock_seconds);
            self.players.insert(player_in, state);
        }
    }

    pub fn handle_availability(
        &mut self,
        player_id: Uuid,
        previous_status: AvailabilityStatus,
        new_status: AvailabilityStatus,
    ) {
        match new_status {
            AvailabilityStatus::Expelled
            | AvailabilityStatus::Injured
            | AvailabilityStatus::Suspended => {
                if let Some(state) = self.players.get_mut(&player_id) {
                    state.set_active(false);
                }
            }
            AvailabilityStatus::Active if previous_status == AvailabilityStatus::Suspended => {
                if let Some(state) = self.players.get_mut(&player_id) {
                    state.set_active(true);
                }
            }
            _ => {}
        }
    }

    pub(super) fn inspect_event_for_roster_updates(&mut self, event: &MatchEvent) {
        match event {
            MatchEvent::TacticalPlanActivated(event) => {
                for assignment in &event.assignments {
                    if self.translator.context().is_some_and(|context| {
                        context
                            .active_player_for_slot(event.team_id, assignment.formation_slot_index)
                            != Some(assignment.player_id)
                    }) {
                        continue;
                    }
                    if let Some(state) = self.players.get_mut(&assignment.player_id) {
                        state.update_assignment(
                            assignment.offensive_position,
                            assignment.defensive_position,
                            assignment.slot_role,
                            &self.config,
                        );
                    }
                }
            }
            MatchEvent::TacticalRealignmentMade(event) => {
                for assignment in event.assignments() {
                    if self.translator.context().is_some_and(|context| {
                        context.active_player_for_slot(
                            event.team_id(),
                            assignment.formation_slot_index,
                        ) != Some(assignment.player_id)
                    }) {
                        continue;
                    }
                    if let Some(state) = self.players.get_mut(&assignment.player_id) {
                        state.update_assignment(
                            assignment.offensive_position,
                            assignment.defensive_position,
                            assignment.slot_role,
                            &self.config,
                        );
                    }
                }
            }
            MatchEvent::SubstitutionMade(event) => {
                self.handle_substitution(event.player_out(), event.player_in(), event.team_id());
            }
            MatchEvent::PlayerAvailabilityChanged(event) => {
                self.handle_availability(
                    event.player_id(),
                    event.previous_status(),
                    event.new_status(),
                );
            }
            _ => {}
        }
    }

    pub(super) fn ensure_player_exists(&mut self, player_id: Uuid, team_id: Uuid) {
        if self.players.contains_key(&player_id) {
            return;
        }

        let (offensive, defensive, role) = if let Some(context) = self.translator.context() {
            (
                context
                    .offensive_position(&player_id)
                    .unwrap_or(Position::CenterOffense),
                context
                    .defensive_position(&player_id)
                    .unwrap_or(Position::Centerback),
                context.slot_role(&player_id).unwrap_or(SlotRole::Standard),
            )
        } else {
            (
                Position::CenterOffense,
                Position::Centerback,
                SlotRole::Standard,
            )
        };

        let mut state = LivePlayerState::new(
            player_id,
            team_id,
            offensive,
            defensive,
            role,
            true,
            &self.config,
        );
        state.set_starter(false);
        state.set_entry_time_seconds(self.last_clock_seconds);
        self.players.insert(player_id, state);
    }
}
