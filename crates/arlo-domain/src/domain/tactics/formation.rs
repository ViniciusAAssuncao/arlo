use crate::domain::position::Position;
use crate::domain::sport_constants::{
    REQUIRED_ARTRINES_PER_FORMATION, REQUIRED_GOALGUARDS_PER_FORMATION,
    REQUIRED_PASSERS_PER_FORMATION, TOTAL_PLAYERS_PER_TEAM,
};
use crate::domain::tactics::formation_builder::FormationBuilder;
use crate::domain::tactics::formation_slot::FormationSlot;
use crate::domain::validation::{
    validate_exact_count, validate_no_duplicate_keys, validate_not_empty,
};
use crate::error::DomainResult;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Formation {
    id: Uuid,
    name: String,
    slots: Vec<FormationSlot>,
}

impl Formation {
    pub fn new(
        id: Uuid,
        name: impl Into<String>,
        slots: Vec<FormationSlot>,
    ) -> DomainResult<Self> {
        let name = name.into();
        validate_not_empty(&name, "name")?;
        validate_exact_count(
            &slots,
            |_| true,
            TOTAL_PLAYERS_PER_TEAM as usize,
            "slots",
        )?;
        validate_exact_count(
            &slots,
            |s| s.position() == Position::Goalguard,
            REQUIRED_GOALGUARDS_PER_FORMATION,
            "slots",
        )?;
        validate_exact_count(
            &slots,
            |s| s.position() == Position::Passer,
            REQUIRED_PASSERS_PER_FORMATION,
            "slots",
        )?;
        validate_exact_count(
            &slots,
            |s| s.position() == Position::Artrine,
            REQUIRED_ARTRINES_PER_FORMATION,
            "slots",
        )?;
        validate_no_duplicate_keys(
            &slots,
            |s| {
                let x = if s.pitch_length_ratio() == 0.0 { 0.0 } else { s.pitch_length_ratio() };
                let y = if s.pitch_width_ratio() == 0.0 { 0.0 } else { s.pitch_width_ratio() };
                (x.to_bits(), y.to_bits())
            },
            "slots",
            "coordinates",
        )?;

        Ok(Self { id, name, slots })
    }

    pub fn builder(id: Uuid, name: impl Into<String>) -> FormationBuilder {
        FormationBuilder::new(id, name)
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn slots(&self) -> &[FormationSlot] {
        &self.slots
    }
}
