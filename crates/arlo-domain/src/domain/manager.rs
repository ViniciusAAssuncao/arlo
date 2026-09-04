use crate::domain::person::Person;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Manager {
    person: Person,
    team_id: Option<Uuid>,
}

impl Manager {
    pub fn new(person: Person, team_id: Option<Uuid>) -> Self {
        Self { person, team_id }
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
}
