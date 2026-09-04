use crate::domain::validation::validate_integer_range;
use crate::error::DomainResult;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlayerPosition {
    position_id: Uuid,
    proficiency: i32,
}

impl PlayerPosition {
    pub fn new(position_id: Uuid, proficiency: i32) -> DomainResult<Self> {
        validate_integer_range(proficiency, 0, 10, "proficiency")?;
        Ok(Self {
            position_id,
            proficiency,
        })
    }

    pub fn position_id(&self) -> Uuid {
        self.position_id
    }

    pub fn proficiency(&self) -> i32 {
        self.proficiency
    }
}
