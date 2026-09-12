use crate::domain::injury_definition::InjuryDefinition;
use crate::domain::injury_mechanism::InjuryMechanism;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct InjuryCatalog {
    definitions_by_id: HashMap<Uuid, InjuryDefinition>,
    definitions_by_mechanism: HashMap<InjuryMechanism, Vec<Uuid>>,
}

impl InjuryCatalog {
    pub fn new(definitions: Vec<InjuryDefinition>) -> Self {
        let mut definitions_by_id = HashMap::with_capacity(definitions.len());
        let mut definitions_by_mechanism: HashMap<InjuryMechanism, Vec<Uuid>> = HashMap::new();
        for def in definitions {
            definitions_by_mechanism
                .entry(def.mechanism())
                .or_default()
                .push(def.id());
            definitions_by_id.insert(def.id(), def);
        }

        Self {
            definitions_by_id,
            definitions_by_mechanism,
        }
    }

    pub fn definition(&self, id: &Uuid) -> Option<&InjuryDefinition> {
        self.definitions_by_id.get(id)
    }

    pub fn definitions_for_mechanism(&self, mechanism: InjuryMechanism) -> &[Uuid] {
        self.definitions_by_mechanism
            .get(&mechanism)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    pub fn definitions_by_id(&self) -> &HashMap<Uuid, InjuryDefinition> {
        &self.definitions_by_id
    }

    pub fn definitions_by_mechanism(&self) -> &HashMap<InjuryMechanism, Vec<Uuid>> {
        &self.definitions_by_mechanism
    }
}