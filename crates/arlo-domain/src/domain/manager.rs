use crate::domain::manager_attribute_value::ManagerAttributeValue;
use crate::domain::manager_profile::ManagerTacticalProfile;
use crate::domain::person::Person;
use crate::domain::validation::validate_no_duplicate_keys;
use crate::error::DomainResult;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Manager {
    person: Person,
    team_id: Option<Uuid>,
    attributes: Vec<ManagerAttributeValue>,
    tactical_profile: Option<ManagerTacticalProfile>,
}

impl Manager {
    pub fn new(
        person: Person,
        team_id: Option<Uuid>,
        attributes: Vec<ManagerAttributeValue>,
        tactical_profile: Option<ManagerTacticalProfile>,
    ) -> DomainResult<Self> {
        validate_no_duplicate_keys(
            &attributes,
            |a| a.attribute_definition_id(),
            "attributes",
            "attribute_definition_id",
        )?;

        Ok(Self {
            person,
            team_id,
            attributes,
            tactical_profile,
        })
    }

    pub fn builder(person: Person) -> ManagerBuilder {
        ManagerBuilder::new(person)
    }

    pub fn id(&self) -> Uuid {
        self.person.id()
    }

    pub fn person(&self) -> &Person {
        &self.person
    }

    pub fn team_id(&self) -> Option<Uuid> {
        self.team_id
    }

    pub fn attributes(&self) -> &[ManagerAttributeValue] {
        &self.attributes
    }

    pub fn tactical_profile(&self) -> Option<&ManagerTacticalProfile> {
        self.tactical_profile.as_ref()
    }
}

#[derive(Debug, Clone)]
pub struct ManagerBuilder {
    person: Person,
    team_id: Option<Uuid>,
    attributes: Vec<ManagerAttributeValue>,
    tactical_profile: Option<ManagerTacticalProfile>,
}

impl ManagerBuilder {
    pub fn new(person: Person) -> Self {
        Self {
            person,
            team_id: None,
            attributes: Vec::new(),
            tactical_profile: None,
        }
    }

    pub fn with_team_id(mut self, team_id: Option<Uuid>) -> Self {
        self.team_id = team_id;
        self
    }

    pub fn with_attributes(mut self, attributes: Vec<ManagerAttributeValue>) -> Self {
        self.attributes = attributes;
        self
    }

    pub fn with_tactical_profile(
        mut self,
        tactical_profile: Option<ManagerTacticalProfile>,
    ) -> Self {
        self.tactical_profile = tactical_profile;
        self
    }

    pub fn build(self) -> DomainResult<Manager> {
        Manager::new(
            self.person,
            self.team_id,
            self.attributes,
            self.tactical_profile,
        )
    }
}