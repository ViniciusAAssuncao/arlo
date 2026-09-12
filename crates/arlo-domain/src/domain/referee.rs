use crate::domain::person::Person;
use crate::domain::referee_attribute_value::RefereeAttributeValue;
use crate::domain::scope::Scope;
use crate::domain::validation::validate_no_duplicate_keys;
use crate::error::DomainResult;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Referee {
    person: Person,
    primary_league_id: Option<Uuid>,
    tier: Scope,
    attributes: Vec<RefereeAttributeValue>,
}

impl Referee {
    pub fn new(
        person: Person,
        primary_league_id: Option<Uuid>,
        tier: Scope,
        attributes: Vec<RefereeAttributeValue>,
    ) -> DomainResult<Self> {
        validate_no_duplicate_keys(
            &attributes,
            |a| a.attribute_definition_id(),
            "attributes",
            "attribute_definition_id",
        )?;

        Ok(Self {
            person,
            primary_league_id,
            tier,
            attributes,
        })
    }

    pub fn id(&self) -> Uuid {
        self.person.id()
    }

    pub fn person(&self) -> &Person {
        &self.person
    }

    pub fn primary_league_id(&self) -> Option<Uuid> {
        self.primary_league_id
    }

    pub fn tier(&self) -> Scope {
        self.tier
    }

    pub fn attributes(&self) -> &[RefereeAttributeValue] {
        &self.attributes
    }
}