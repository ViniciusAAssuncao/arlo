use crate::domain::fault_definition::FaultDefinition;
use crate::domain::fault_punishment_option::FaultPunishmentOption;
use crate::domain::fault_severity::FaultSeverity;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct FaultCatalog {
    definitions_by_id: HashMap<Uuid, FaultDefinition>,
    definitions_by_severity: HashMap<FaultSeverity, Vec<Uuid>>,
    punishment_options_by_definition: HashMap<Uuid, Vec<FaultPunishmentOption>>,
}

impl FaultCatalog {
    pub fn new(definitions: Vec<FaultDefinition>, options: Vec<FaultPunishmentOption>) -> Self {
        let mut definitions_by_id = HashMap::with_capacity(definitions.len());
        let mut definitions_by_severity: HashMap<FaultSeverity, Vec<Uuid>> = HashMap::new();
        for def in definitions {
            definitions_by_severity
                .entry(def.severity())
                .or_default()
                .push(def.id());
            definitions_by_id.insert(def.id(), def);
        }

        let mut punishment_options_by_definition: HashMap<Uuid, Vec<FaultPunishmentOption>> =
            HashMap::new();
        for opt in options {
            punishment_options_by_definition
                .entry(opt.fault_definition_id())
                .or_default()
                .push(opt);
        }

        Self {
            definitions_by_id,
            definitions_by_severity,
            punishment_options_by_definition,
        }
    }

    pub fn definition(&self, id: &Uuid) -> Option<&FaultDefinition> {
        self.definitions_by_id.get(id)
    }

    pub fn definitions_for_severity(&self, severity: FaultSeverity) -> &[Uuid] {
        self.definitions_by_severity
            .get(&severity)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    pub fn punishment_options(&self, fault_definition_id: &Uuid) -> &[FaultPunishmentOption] {
        self.punishment_options_by_definition
            .get(fault_definition_id)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    pub fn definitions_by_id(&self) -> &HashMap<Uuid, FaultDefinition> {
        &self.definitions_by_id
    }

    pub fn definitions_by_severity(&self) -> &HashMap<FaultSeverity, Vec<Uuid>> {
        &self.definitions_by_severity
    }

    pub fn punishment_options_by_definition(&self) -> &HashMap<Uuid, Vec<FaultPunishmentOption>> {
        &self.punishment_options_by_definition
    }
}