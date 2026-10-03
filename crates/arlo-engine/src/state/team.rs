use crate::error::{EngineError, EngineResult};
use crate::input::TeamInput;
use crate::state::{DriveProgress, Score};
use arlo_domain::{sport_constants::IMMEDIATE_POSSESSION_CONTROL_SECONDS, Position};
use arlo_events::AvailabilityStatus;
use std::collections::HashMap;
use uuid::Uuid;

mod plans;
mod realignment;

#[derive(Debug, Clone)]
pub struct TeamState {
    team_id: Uuid,
    artrine_id: Uuid,
    passer_id: Uuid,
    goalguard_id: Uuid,
    drive_eligible: bool,
    active_player_ids: Vec<Uuid>,
    reserve_player_ids: Vec<Uuid>,
    suspended_players: Vec<(Uuid, f64)>,
    expelled_players: Vec<Uuid>,
    injured_players: Vec<Uuid>,
    slot_replacements: HashMap<Uuid, Uuid>,
    drive_progress: DriveProgress,
    drives_in_series: u32,
    time_calls_used_in_period: u32,
    last_time_call_at: Option<f64>,
    challenges_used: u32,
    last_voluntary_substitution_at: Option<f64>,
    active_tactical_profile_id: Uuid,
    tactical_switches: u8,
    last_tactical_switch_at: Option<f64>,
    last_tactical_realignment_at: Option<f64>,
    active_layout: Option<arlo_tactics::TacticalLayout>,
    active_plan_id: Option<Uuid>,
    last_plan_activation_at: Option<f64>,
    score: Score,
}

impl TeamState {
    pub(crate) fn from_input(input: &TeamInput) -> Self {
        let active_player_ids: Vec<_> = input
            .lineup()
            .assignments()
            .iter()
            .map(|a| a.player_id())
            .collect();
        let reserve_player_ids = input
            .roster()
            .iter()
            .map(|player| player.id())
            .filter(|id| !active_player_ids.contains(id))
            .collect();
        let artrine_id = input
            .lineup()
            .assignments()
            .iter()
            .find(|a| a.position() == Position::Artrine)
            .map(|a| a.player_id())
            .expect("validated lineup has an Artrine");
        let passer_id = input
            .lineup()
            .assignments()
            .iter()
            .find(|a| a.position() == Position::Passer)
            .map(|a| a.player_id())
            .expect("validated lineup has a Passer");
        let goalguard_id = input
            .lineup()
            .assignments()
            .iter()
            .find(|a| a.position() == Position::Goalguard)
            .map(|a| a.player_id())
            .expect("validated lineup has a Goalguard");
        Self {
            team_id: input.team_id(),
            artrine_id,
            passer_id,
            goalguard_id,
            drive_eligible: true,
            active_player_ids,
            reserve_player_ids,
            suspended_players: Vec::new(),
            expelled_players: Vec::new(),
            injured_players: Vec::new(),
            slot_replacements: HashMap::new(),
            drive_progress: DriveProgress::default(),
            drives_in_series: 0,
            time_calls_used_in_period: 0,
            last_time_call_at: None,
            challenges_used: 0,
            last_voluntary_substitution_at: None,
            active_tactical_profile_id: input.tactics().id(),
            tactical_switches: 0,
            last_tactical_switch_at: None,
            last_tactical_realignment_at: None,
            active_layout: None,
            active_plan_id: input.prepared_plans().first().map(|plan| plan.id),
            last_plan_activation_at: None,
            score: Score::default(),
        }
    }

    pub fn team_id(&self) -> Uuid {
        self.team_id
    }
    pub fn artrine_id(&self) -> Uuid {
        let assigned = self.slot_player_id(self.artrine_id);
        if self.active_player_ids.contains(&assigned) {
            assigned
        } else {
            self.active_player_ids
                .iter()
                .copied()
                .find(|id| {
                    *id != self.slot_player_id(self.passer_id)
                        && *id != self.slot_player_id(self.goalguard_id)
                })
                .or_else(|| {
                    self.active_player_ids
                        .iter()
                        .copied()
                        .find(|id| *id != self.slot_player_id(self.goalguard_id))
                })
                .unwrap_or(self.artrine_id)
        }
    }
    pub fn drive_eligible(&self) -> bool {
        self.drive_eligible
    }

    pub(crate) fn disable_drives(&mut self) {
        self.drive_eligible = false;
        self.reset_drives();
    }
    pub fn passer_id(&self) -> Uuid {
        let assigned = self.slot_player_id(self.passer_id);
        if self.active_player_ids.contains(&assigned) {
            assigned
        } else {
            self.active_player_ids
                .iter()
                .copied()
                .find(|id| {
                    *id != self.artrine_id() && *id != self.slot_player_id(self.goalguard_id)
                })
                .unwrap_or(self.passer_id)
        }
    }
    pub fn active_player_ids(&self) -> &[Uuid] {
        &self.active_player_ids
    }
    pub fn slot_player_id(&self, original_id: Uuid) -> Uuid {
        self.slot_replacements
            .get(&original_id)
            .copied()
            .unwrap_or(original_id)
    }
    pub fn injured_player_ids(&self) -> &[Uuid] {
        &self.injured_players
    }
    pub(crate) fn record_injury(
        &mut self,
        player_id: Uuid,
        withdraw: bool,
        replacement_id: Option<Uuid>,
    ) {
        if !self.injured_players.contains(&player_id) {
            self.injured_players.push(player_id);
        }
        if withdraw {
            self.active_player_ids.retain(|id| *id != player_id);
        }
        if let Some(replacement_id) = replacement_id {
            self.reserve_player_ids.retain(|id| *id != replacement_id);
            self.active_player_ids.push(replacement_id);
            let original = self
                .slot_replacements
                .iter()
                .find(|(_, current)| **current == player_id)
                .map(|(original, _)| *original)
                .unwrap_or(player_id);
            self.slot_replacements.insert(original, replacement_id);
        }
    }
    pub(crate) fn preserve_medical_lineup(&mut self, current: &Self) {
        self.injured_players = current.injured_players.clone();
        self.active_player_ids = current.active_player_ids.clone();
        self.reserve_player_ids = current.reserve_player_ids.clone();
        self.slot_replacements = current.slot_replacements.clone();
    }
    pub(crate) fn withdraw_injured_player(
        &mut self,
        player_id: Uuid,
        replacement_id: Option<Uuid>,
    ) {
        self.active_player_ids.retain(|id| *id != player_id);
        if let Some(replacement_id) = replacement_id {
            self.reserve_player_ids.retain(|id| *id != replacement_id);
            self.active_player_ids.push(replacement_id);
            let original = self
                .slot_replacements
                .iter()
                .find(|(_, current)| **current == player_id)
                .map(|(original, _)| *original)
                .unwrap_or(player_id);
            self.slot_replacements.insert(original, replacement_id);
        }
    }
    pub fn reserve_player_ids(&self) -> &[Uuid] {
        &self.reserve_player_ids
    }
    pub fn is_available_reserve(&self, player_id: Uuid) -> bool {
        self.reserve_player_ids.contains(&player_id)
            && !self.injured_players.contains(&player_id)
            && !self.expelled_players.contains(&player_id)
            && !self
                .suspended_players
                .iter()
                .any(|(id, _)| *id == player_id)
    }
    pub fn drive_progress(&self) -> DriveProgress {
        self.drive_progress
    }
    pub fn drives_in_series(&self) -> u32 {
        self.drives_in_series
    }
    pub fn time_calls_used_in_period(&self) -> u32 {
        self.time_calls_used_in_period
    }
    pub fn last_time_call_at(&self) -> Option<f64> {
        self.last_time_call_at
    }
    pub fn challenges_used(&self) -> u32 {
        self.challenges_used
    }
    pub fn last_voluntary_substitution_at(&self) -> Option<f64> {
        self.last_voluntary_substitution_at
    }
    pub fn active_tactical_profile_id(&self) -> Uuid {
        self.active_tactical_profile_id
    }
    pub fn tactical_switches(&self) -> u8 {
        self.tactical_switches
    }
    pub fn last_tactical_switch_at(&self) -> Option<f64> {
        self.last_tactical_switch_at
    }
    pub fn score(&self) -> Score {
        self.score
    }

    pub(crate) fn availability_status(&self, player_id: Uuid) -> Option<AvailabilityStatus> {
        if self.expelled_players.contains(&player_id) {
            Some(AvailabilityStatus::Expelled)
        } else if self
            .suspended_players
            .iter()
            .any(|(id, _)| *id == player_id)
        {
            Some(AvailabilityStatus::Suspended)
        } else if self.injured_players.contains(&player_id)
            && !self.active_player_ids.contains(&player_id)
        {
            Some(AvailabilityStatus::Injured)
        } else if self.active_player_ids.contains(&player_id) {
            Some(AvailabilityStatus::Active)
        } else {
            None
        }
    }

    pub(crate) fn record_artro(
        &mut self,
        player_id: Uuid,
        control_seconds: f64,
    ) -> EngineResult<Option<u32>> {
        if !self.drive_eligible
            || player_id != self.artrine_id()
            || !control_seconds.is_finite()
            || control_seconds < IMMEDIATE_POSSESSION_CONTROL_SECONDS
        {
            return Err(EngineError::InvalidTransition(
                "Artro requires the official Artrine with established possession".into(),
            ));
        }
        let (progress, completed_drive) = self.drive_progress.record_artro()?;
        let drives_in_series = if completed_drive {
            Some(self.drives_in_series.checked_add(1).ok_or_else(|| {
                EngineError::InvalidTransition("series drive count overflow".into())
            })?)
        } else {
            None
        };
        self.drive_progress = progress;
        if let Some(count) = drives_in_series {
            self.drives_in_series = count;
        }
        Ok(drives_in_series)
    }

    pub(crate) fn replace_score(&mut self, score: Score) {
        self.score = score;
    }

    pub(crate) fn reset_drives(&mut self) {
        self.drive_progress = self.drive_progress.reset();
        self.drives_in_series = 0;
    }

    pub(crate) fn reset_series_drives(&mut self) {
        self.drives_in_series = 0;
    }

    pub(crate) fn lose_drives(&mut self, count: u32) {
        self.drive_progress = self.drive_progress.lose_drives(count);
        self.drives_in_series = self.drives_in_series.saturating_sub(count);
    }

    pub(crate) fn record_time_call(&mut self, elapsed: f64) {
        self.time_calls_used_in_period += 1;
        self.last_time_call_at = Some(elapsed);
    }

    pub(crate) fn reset_time_calls(&mut self) {
        self.time_calls_used_in_period = 0;
    }

    pub(crate) fn record_challenge(&mut self) {
        self.challenges_used += 1;
    }

    pub(crate) fn activate_tactical_profile(&mut self, profile_id: Uuid, elapsed: f64) {
        self.active_plan_id = None;
        self.active_tactical_profile_id = profile_id;
        self.tactical_switches += 1;
        self.last_tactical_switch_at = Some(elapsed);
    }

    pub(crate) fn restore_challenges(&mut self, used: u32) {
        self.challenges_used = used;
    }

    pub(crate) fn substitute_voluntarily(&mut self, outgoing: Uuid, incoming: Uuid, elapsed: f64) {
        self.active_player_ids.retain(|id| *id != outgoing);
        self.active_player_ids.push(incoming);
        self.reserve_player_ids.retain(|id| *id != incoming);
        self.reserve_player_ids.push(outgoing);
        let original = self
            .slot_replacements
            .iter()
            .find(|(_, current)| **current == outgoing)
            .map(|(original, _)| *original)
            .unwrap_or(outgoing);
        self.slot_replacements.insert(original, incoming);
        self.last_voluntary_substitution_at = Some(elapsed);
    }

    pub(crate) fn suspend_player(&mut self, player_id: Uuid, until: f64) {
        if self.expelled_players.contains(&player_id) {
            return;
        }
        self.active_player_ids.retain(|id| *id != player_id);
        self.suspended_players.retain(|(id, _)| *id != player_id);
        self.suspended_players.push((player_id, until));
    }

    pub(crate) fn expel_player(&mut self, player_id: Uuid) {
        self.active_player_ids.retain(|id| *id != player_id);
        self.suspended_players.retain(|(id, _)| *id != player_id);
        if !self.expelled_players.contains(&player_id) {
            self.expelled_players.push(player_id);
        }
    }

    pub(crate) fn release_expired(&mut self, elapsed: f64) -> Vec<Uuid> {
        let mut returning = Vec::new();
        self.suspended_players.retain(|(id, until)| {
            if *until <= elapsed {
                returning.push(*id);
                false
            } else {
                true
            }
        });
        for &id in &returning {
            if !self.expelled_players.contains(&id) && !self.active_player_ids.contains(&id) {
                self.active_player_ids.push(id);
            }
        }
        returning
            .into_iter()
            .filter(|id| self.active_player_ids.contains(id))
            .collect()
    }
}
