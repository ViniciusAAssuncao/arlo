use crate::domain::tactics::formation::Formation;
use crate::domain::tactics::formation_slot::FormationSlot;
use crate::error::DomainResult;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct FormationBuilder {
    id: Uuid,
    name: String,
    slots: Vec<FormationSlot>,
}

impl FormationBuilder {
    pub fn new(id: Uuid, name: impl Into<String>) -> Self {
        Self {
            id,
            name: name.into(),
            slots: Vec::new(),
        }
    }

    pub fn with_slot(mut self, slot: FormationSlot) -> Self {
        self.slots.push(slot);
        self
    }

    pub fn add_slot(mut self, slot: FormationSlot) -> Self {
        self.slots.push(slot);
        self
    }

    pub fn with_slots(mut self, slots: impl IntoIterator<Item = FormationSlot>) -> Self {
        self.slots.extend(slots);
        self
    }

    pub fn build(self) -> DomainResult<Formation> {
        Formation::new(self.id, self.name, self.slots)
    }
}
