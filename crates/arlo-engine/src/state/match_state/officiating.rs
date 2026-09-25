use super::MatchState;
use arlo_events::RefereeDecisionResolved;
use arlo_domain::PunishmentKind;
use arlo_events::{AvailabilityStatus, PlayerAvailabilityChanged};
use crate::error::EngineResult;
use crate::error::EngineError;
use crate::state::{MatchPhase, SeriesState};

impl MatchState {
    pub(crate) fn queue_referee_decision(&mut self, decision: RefereeDecisionResolved) {
        self.pending_referee_decisions.push(decision);
    }

    pub(crate) fn take_referee_decisions(&mut self) -> Vec<RefereeDecisionResolved> {
        std::mem::take(&mut self.pending_referee_decisions)
    }

    pub(crate) fn pending_referee_decisions(&self) -> &[RefereeDecisionResolved] {
        &self.pending_referee_decisions
    }

    pub(crate) fn has_pending_referee_decisions(&self) -> bool {
        !self.pending_referee_decisions.is_empty()
    }

    pub(crate) fn availability_status(&self, team_id: uuid::Uuid, player_id: uuid::Uuid) -> Option<AvailabilityStatus> {
        self.team(team_id).ok()?.availability_status(player_id)
    }

    pub(crate) fn release_expired_players(&mut self) {
        let elapsed = self.clock.total_elapsed_seconds();
        let home = self.home.release_expired(elapsed);
        let away = self.away.release_expired(elapsed);
        for id in home {
            self.pending_availability_events.push(PlayerAvailabilityChanged::new(
                id, self.home.team_id(), AvailabilityStatus::Suspended, AvailabilityStatus::Active, None,
            ));
        }
        for id in away {
            self.pending_availability_events.push(PlayerAvailabilityChanged::new(
                id, self.away.team_id(), AvailabilityStatus::Suspended, AvailabilityStatus::Active, None,
            ));
        }
    }

    pub(crate) fn take_availability_events(&mut self) -> Vec<PlayerAvailabilityChanged> {
        std::mem::take(&mut self.pending_availability_events)
    }

    pub(crate) fn rollback_scoring_segment(&mut self, prior: &MatchState) -> EngineResult<()> {
        let clock = self.clock.stop();
        let rng = self.rng.clone();
        let decisions = self.take_referee_decisions();
        *self = prior.clone();
        self.clock = clock;
        self.rng = rng;
        self.pending_referee_decisions = decisions;
        if self.phase == MatchPhase::Live && self.series.team_id() != self.possessor_team_id() {
            let possessor = self.possessor_team_id();
            let position = self.possession.ball_position_mirim();
            self.resolve_out(possessor, position)?;
        } else {
            self.series = self.series.resume_for_new_call();
            self.phase = MatchPhase::Stopped;
            self.pending_call_outcome = None;
        }
        self.release_expired_players();
        Ok(())
    }

    pub(crate) fn stop_for_serious_foul(&mut self) -> EngineResult<()> {
        if self.phase != MatchPhase::Live {
            return Err(EngineError::InvalidTransition("serious foul requires live play".into()));
        }
        let team_id = self.possessor_team_id();
        let position = self.possession.ball_position_mirim();
        self.resolve_out(team_id, position)?;
        Ok(())
    }

    pub(crate) fn apply_punishment(&mut self, team_id: uuid::Uuid, player_id: uuid::Uuid, kind: PunishmentKind, magnitude: Option<i32>) -> EngineResult<()> {
        let amount = magnitude.unwrap_or(match kind { PunishmentKind::YardageLoss => 5, PunishmentKind::TimePenalty => 2, _ => 1 }).max(0);
        match kind {
            PunishmentKind::TimePenalty => {
                let until = self.clock.total_elapsed_seconds() + f64::from(amount) * 60.0;
                self.team_mut(team_id)?.suspend_player(player_id, until);
            }
            PunishmentKind::Expulsion => self.team_mut(team_id)?.expel_player(player_id),
            PunishmentKind::LossOfDrive => self.team_mut(team_id)?.lose_drives(amount as u32),
            PunishmentKind::LossOfDown | PunishmentKind::YardageLoss => {
                if self.series.team_id() == team_id {
                    let exhausted = kind == PunishmentKind::LossOfDown && u32::from(self.series.down()) + amount as u32 > 4;
                    self.apply_series_penalty(kind, amount)?;
                    if exhausted { self.forfeit_series(team_id)?; }
                } else if self.suspended_restart.is_some_and(|saved| saved.team_id == team_id) {
                    let exhausted = kind == PunishmentKind::LossOfDown
                        && self.suspended_restart.is_some_and(|saved| u32::from(saved.series.down()) + amount as u32 > 4);
                    let saved = self.suspended_restart.as_mut().expect("checked suspended restart");
                    match kind {
                        PunishmentKind::LossOfDown => saved.series = saved.series.lose_downs(amount as u32),
                        PunishmentKind::YardageLoss => {
                            saved.series = saved.series.lose_yardage(f64::from(amount));
                            let direction = if team_id == self.home.team_id() { -1.0 } else { 1.0 };
                            saved.position_mirim = (saved.position_mirim + direction * f64::from(amount)).clamp(0.0, self.pitch_length_mirim);
                        }
                        _ => {}
                    }
                    if exhausted { self.suspended_restart = None; }
                } else {
                    self.deferred_series_penalties.push((team_id, kind, amount));
                }
            }
            PunishmentKind::KickFoulAwarded | PunishmentKind::InvalidatePreviousPlay => {}
        }
        Ok(())
    }

    fn apply_series_penalty(&mut self, kind: PunishmentKind, amount: i32) -> EngineResult<()> {
        match kind {
            PunishmentKind::LossOfDown => self.series = self.series.lose_downs(amount as u32),
            PunishmentKind::YardageLoss => {
                self.series = self.series.lose_yardage(f64::from(amount));
                if self.possessor_team_id() == self.series.team_id() {
                    let direction = if self.series.team_id() == self.home.team_id() { -1.0 } else { 1.0 };
                    let position = (self.possession.ball_position_mirim() + direction * f64::from(amount)).clamp(0.0, self.pitch_length_mirim);
                    self.possession = self.possession.with_ball_position(position, self.pitch_length_mirim)?;
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn forfeit_series(&mut self, offender_team_id: uuid::Uuid) -> EngineResult<()> {
        let recipient = self.opponent_id(offender_team_id)?;
        let position = self.possession.ball_position_mirim();
        self.series = SeriesState::new(recipient, position, self.pitch_length_mirim)?;
        self.possession = self.possession.award_next_call(recipient, position, self.pitch_length_mirim)?;
        self.suspended_restart = None;
        self.pending_call_outcome = None;
        self.clock = self.clock.stop();
        self.phase = MatchPhase::Stopped;
        self.team_mut(recipient)?.reset_series_drives();
        Ok(())
    }

    pub(crate) fn apply_deferred_series_penalties(&mut self) -> EngineResult<()> {
        let team_id = self.series.team_id();
        let pending = std::mem::take(&mut self.deferred_series_penalties);
        for (id, kind, amount) in pending {
            if id == team_id {
                let exhausted = kind == PunishmentKind::LossOfDown && u32::from(self.series.down()) + amount as u32 > 4;
                self.apply_series_penalty(kind, amount)?;
                if exhausted { self.forfeit_series(id)?; }
            }
            else { self.deferred_series_penalties.push((id, kind, amount)); }
        }
        Ok(())
    }
}
