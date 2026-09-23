use crate::error::{EngineError, EngineResult};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PossessionState {
    possessor_team_id: Uuid,
    next_call_team_id: Uuid,
    ball_position_mirim: f64,
}

impl PossessionState {
    pub fn new(
        team_id: Uuid,
        ball_position_mirim: f64,
        pitch_length_mirim: f64,
    ) -> EngineResult<Self> {
        Self::validate_position(ball_position_mirim, pitch_length_mirim)?;
        Ok(Self {
            possessor_team_id: team_id,
            next_call_team_id: team_id,
            ball_position_mirim,
        })
    }

    pub fn possessor_team_id(&self) -> Uuid {
        self.possessor_team_id
    }
    pub fn next_call_team_id(&self) -> Uuid {
        self.next_call_team_id
    }
    pub fn ball_position_mirim(&self) -> f64 {
        self.ball_position_mirim
    }

    pub fn turnover(self, team_id: Uuid) -> EngineResult<Self> {
        if team_id == self.possessor_team_id {
            return Err(EngineError::InvalidTransition(
                "turnover must change the possessor".into(),
            ));
        }
        Ok(Self {
            possessor_team_id: team_id,
            ..self
        })
    }

    pub fn with_possessor(self, team_id: Uuid) -> Self {
        Self {
            possessor_team_id: team_id,
            ..self
        }
    }

    pub fn with_ball_position(
        self,
        position_mirim: f64,
        pitch_length_mirim: f64,
    ) -> EngineResult<Self> {
        Self::validate_position(position_mirim, pitch_length_mirim)?;
        Ok(Self {
            ball_position_mirim: position_mirim,
            ..self
        })
    }

    pub fn award_next_call(
        self,
        team_id: Uuid,
        position_mirim: f64,
        pitch_length_mirim: f64,
    ) -> EngineResult<Self> {
        Self::validate_position(position_mirim, pitch_length_mirim)?;
        Ok(Self {
            possessor_team_id: team_id,
            next_call_team_id: team_id,
            ball_position_mirim: position_mirim,
        })
    }

    fn validate_position(position_mirim: f64, pitch_length_mirim: f64) -> EngineResult<()> {
        if !position_mirim.is_finite()
            || !pitch_length_mirim.is_finite()
            || position_mirim < 0.0
            || position_mirim > pitch_length_mirim
        {
            return Err(EngineError::InvalidTransition(
                "ball position is outside the pitch".into(),
            ));
        }
        Ok(())
    }
}
