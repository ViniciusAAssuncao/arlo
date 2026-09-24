use crate::error::{EngineError, EngineResult};
use arlo_domain::sport_constants::ARTROS_PER_DRIVE;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct DriveProgress {
    partial_artros: u8,
    completed_drives: u32,
}

impl DriveProgress {
    pub fn partial_artros(&self) -> u8 {
        self.partial_artros
    }

    pub fn completed_drives(&self) -> u32 {
        self.completed_drives
    }

    pub fn record_artro(self) -> EngineResult<(Self, bool)> {
        if u32::from(self.partial_artros) + 1 < ARTROS_PER_DRIVE {
            return Ok((
                Self {
                    partial_artros: self.partial_artros + 1,
                    ..self
                },
                false,
            ));
        }
        let completed_drives = self
            .completed_drives
            .checked_add(1)
            .ok_or_else(|| EngineError::InvalidTransition("drive count overflow".into()))?;
        Ok((
            Self {
                partial_artros: 0,
                completed_drives,
            },
            true,
        ))
    }

    pub fn reset(self) -> Self {
        Self::default()
    }
}
