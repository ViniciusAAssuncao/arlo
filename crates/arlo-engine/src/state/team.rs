use crate::error::{EngineError, EngineResult};
use crate::input::TeamInput;
use crate::state::{DriveProgress, Score};
use arlo_domain::{sport_constants::IMMEDIATE_POSSESSION_CONTROL_SECONDS, Position};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct TeamState {
    team_id: Uuid,
    artrine_id: Uuid,
    active_player_ids: Vec<Uuid>,
    reserve_player_ids: Vec<Uuid>,
    drive_progress: DriveProgress,
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
        Self {
            team_id: input.team_id(),
            artrine_id,
            active_player_ids,
            reserve_player_ids,
            drive_progress: DriveProgress::default(),
            score: Score::default(),
        }
    }

    pub fn team_id(&self) -> Uuid {
        self.team_id
    }
    pub fn artrine_id(&self) -> Uuid {
        self.artrine_id
    }
    pub fn active_player_ids(&self) -> &[Uuid] {
        &self.active_player_ids
    }
    pub fn reserve_player_ids(&self) -> &[Uuid] {
        &self.reserve_player_ids
    }
    pub fn drive_progress(&self) -> DriveProgress {
        self.drive_progress
    }
    pub fn score(&self) -> Score {
        self.score
    }

    pub(crate) fn record_artro(
        &mut self,
        player_id: Uuid,
        control_seconds: f64,
    ) -> EngineResult<bool> {
        if player_id != self.artrine_id
            || !control_seconds.is_finite()
            || control_seconds < IMMEDIATE_POSSESSION_CONTROL_SECONDS
        {
            return Err(EngineError::InvalidTransition(
                "Artro requires the official Artrine with established possession".into(),
            ));
        }
        let (progress, completed_drive) = self.drive_progress.record_artro()?;
        self.drive_progress = progress;
        Ok(completed_drive)
    }

    pub(crate) fn replace_score(&mut self, score: Score) {
        self.score = score;
    }

    pub(crate) fn reset_drives(&mut self) {
        self.drive_progress = self.drive_progress.reset();
    }
}
