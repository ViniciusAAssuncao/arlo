use crate::error::{EngineError, EngineResult};
use arlo_domain::sport_constants::{
    FIELD_GOAL_FIELDPOST_VALUE, FIELD_GOAL_GOALPOST_VALUE, FIELD_POINT_VALUE, GOAL_POINT_VALUE,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScoreKind {
    RegularGoalPoint,
    RegularFieldPoint,
    KickFoulGoalPoint,
    KickFoulFieldPoint,
    BonusFieldpost,
    BonusGoalpost,
}

impl ScoreKind {
    pub fn points(self) -> u32 {
        match self {
            Self::RegularGoalPoint | Self::KickFoulGoalPoint => GOAL_POINT_VALUE as u32,
            Self::RegularFieldPoint | Self::KickFoulFieldPoint => FIELD_POINT_VALUE as u32,
            Self::BonusFieldpost => FIELD_GOAL_FIELDPOST_VALUE as u32,
            Self::BonusGoalpost => FIELD_GOAL_GOALPOST_VALUE as u32,
        }
    }

    pub fn opens_bonus_phase(self) -> bool {
        matches!(self, Self::RegularGoalPoint)
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Score {
    goal_points: u32,
    field_points: u32,
    field_goals: u32,
    total_points: u32,
}

impl Score {
    pub fn goal_points(&self) -> u32 {
        self.goal_points
    }
    pub fn field_points(&self) -> u32 {
        self.field_points
    }
    pub fn field_goals(&self) -> u32 {
        self.field_goals
    }
    pub fn total_points(&self) -> u32 {
        self.total_points
    }

    pub fn apply(self, kind: ScoreKind) -> EngineResult<Self> {
        let mut next = self;
        next.total_points = self
            .total_points
            .checked_add(kind.points())
            .ok_or_else(|| EngineError::InvalidTransition("score overflow".into()))?;
        let counter = match kind {
            ScoreKind::RegularGoalPoint | ScoreKind::KickFoulGoalPoint => &mut next.goal_points,
            ScoreKind::RegularFieldPoint | ScoreKind::KickFoulFieldPoint => &mut next.field_points,
            ScoreKind::BonusFieldpost | ScoreKind::BonusGoalpost => &mut next.field_goals,
        };
        *counter = counter
            .checked_add(1)
            .ok_or_else(|| EngineError::InvalidTransition("score counter overflow".into()))?;
        Ok(next)
    }
}
