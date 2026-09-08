use crate::domain::manager_attribute_value::ManagerAttributeValue;
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
}

impl Manager {
    pub fn new(
        person: Person,
        team_id: Option<Uuid>,
        attributes: Vec<ManagerAttributeValue>,
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
        })
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
}
