use crate::domain::manager_profile::{
    ArtrineDependency, DefensiveApproach, OffensiveApproach, RotationPolicy,
};
use crate::domain::validation::{
    validate_float_range, validate_integer_range, validate_no_duplicate_keys,
};
use crate::error::DomainResult;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ManagerTacticalProfile {
    id: Uuid,
    manager_id: Uuid,
    offensive_approach: OffensiveApproach,
    defensive_approach: DefensiveApproach,
    rotation_policy: RotationPolicy,
    artrine_dependency: ArtrineDependency,
    flexibility_tendency: f64,
    preferred_formation_ids: Vec<Uuid>,
}

impl ManagerTacticalProfile {
    pub fn new(
        id: Uuid,
        manager_id: Uuid,
        offensive_approach: OffensiveApproach,
        defensive_approach: DefensiveApproach,
        rotation_policy: RotationPolicy,
        artrine_dependency: ArtrineDependency,
        flexibility_tendency: f64,
        preferred_formation_ids: Vec<Uuid>,
    ) -> DomainResult<Self> {
        validate_float_range(flexibility_tendency, 0.0, 1.0, "flexibility_tendency")?;
        validate_integer_range(
            preferred_formation_ids.len() as i32,
            0,
            3,
            "preferred_formation_ids",
        )?;
        validate_no_duplicate_keys(
            &preferred_formation_ids,
            |id| *id,
            "preferred_formation_ids",
            "formation_id",
        )?;

        Ok(Self {
            id,
            manager_id,
            offensive_approach,
            defensive_approach,
            rotation_policy,
            artrine_dependency,
            flexibility_tendency,
            preferred_formation_ids,
        })
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn manager_id(&self) -> Uuid {
        self.manager_id
    }

    pub fn offensive_approach(&self) -> OffensiveApproach {
        self.offensive_approach
    }

    pub fn defensive_approach(&self) -> DefensiveApproach {
        self.defensive_approach
    }

    pub fn rotation_policy(&self) -> RotationPolicy {
        self.rotation_policy
    }

    pub fn artrine_dependency(&self) -> ArtrineDependency {
        self.artrine_dependency
    }

    pub fn flexibility_tendency(&self) -> f64 {
        self.flexibility_tendency
    }

    pub fn preferred_formation_ids(&self) -> &[Uuid] {
        &self.preferred_formation_ids
    }
}