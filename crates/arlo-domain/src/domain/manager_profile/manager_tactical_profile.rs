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
    passing_range_preference: f64,
    aeriality_preference: f64,
    structure_preference: f64,
    physicality_preference: f64,
    transition_pace_preference: f64,
    press_block_shape_preference: f64,
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
        passing_range_preference: f64,
        aeriality_preference: f64,
        structure_preference: f64,
        physicality_preference: f64,
        transition_pace_preference: f64,
        press_block_shape_preference: f64,
        preferred_formation_ids: Vec<Uuid>,
    ) -> DomainResult<Self> {
        validate_float_range(flexibility_tendency, 0.0, 1.0, "flexibility_tendency")?;
        validate_float_range(
            passing_range_preference,
            -1.0,
            1.0,
            "passing_range_preference",
        )?;
        validate_float_range(aeriality_preference, -1.0, 1.0, "aeriality_preference")?;
        validate_float_range(structure_preference, -1.0, 1.0, "structure_preference")?;
        validate_float_range(physicality_preference, 0.0, 1.0, "physicality_preference")?;
        validate_float_range(
            transition_pace_preference,
            0.0,
            1.0,
            "transition_pace_preference",
        )?;
        validate_float_range(
            press_block_shape_preference,
            0.0,
            1.0,
            "press_block_shape_preference",
        )?;
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
            passing_range_preference,
            aeriality_preference,
            structure_preference,
            physicality_preference,
            transition_pace_preference,
            press_block_shape_preference,
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

    pub fn passing_range_preference(&self) -> f64 {
        self.passing_range_preference
    }

    pub fn aeriality_preference(&self) -> f64 {
        self.aeriality_preference
    }

    pub fn structure_preference(&self) -> f64 {
        self.structure_preference
    }

    pub fn physicality_preference(&self) -> f64 {
        self.physicality_preference
    }

    pub fn transition_pace_preference(&self) -> f64 {
        self.transition_pace_preference
    }

    pub fn press_block_shape_preference(&self) -> f64 {
        self.press_block_shape_preference
    }

    pub fn preferred_formation_ids(&self) -> &[Uuid] {
        &self.preferred_formation_ids
    }
}